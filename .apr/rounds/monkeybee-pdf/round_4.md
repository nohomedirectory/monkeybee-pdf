The spec is now architecturally mature after three rounds. The crate boundaries, lifecycle model, and preservation doctrine are solid. What remains are operational gaps: places where the spec promises behavior but doesn't specify the mechanism, and places where cross-cutting concerns (cancellation, memory pressure, diagnostics, parallel access) interact with the happy-path designs in ways that need explicit treatment.

## 1) Define content stream error recovery strategy

The content interpreter section specifies tolerant mode and error events, but doesn't define what happens to the graphics state and rendering pipeline when an operator fails mid-page. A failed `Do` for a form XObject, a broken inline image, or an unresolvable font all leave the interpreter in a potentially inconsistent state. Without a defined recovery strategy, each consumer (render, extract, inspect) will improvise differently, creating divergence.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Content stream contract
 - The pipeline emits events (operator dispatched, state changed, text shown, path painted, etc.) that downstream consumers can subscribe to selectively.
+
+**Content stream error recovery:**
+
+When the content interpreter encounters an error in tolerant mode, recovery follows a defined protocol:
+
+1. **Operator-level isolation:** A failing operator does not abort the page. The interpreter emits an
+   Error event, discards the current operator's effects, and advances to the next operator.
+2. **State rollback on failure:** If an operator partially modified the graphics state before failing
+   (e.g., `gs` applied some ExtGState entries before hitting an invalid one), the interpreter rolls
+   back to the state before that operator. This prevents half-applied state from corrupting
+   subsequent rendering.
+3. **Resource resolution failures:** If a `Do`, `Tf`, or `sh` operator references a resource that
+   cannot be resolved, the interpreter skips the operator, emits an Error event with the resource
+   name and type, and continues. For `Tf` (font), a fallback font is substituted so subsequent text
+   operators don't crash.
+4. **Inline image recovery:** If `BI`/`ID`/`EI` parsing fails (corrupted image data, wrong
+   dimensions), the interpreter attempts to find the `EI` marker by scanning forward, skips the
+   inline image, and continues. If `EI` cannot be found within a bounded scan (4096 bytes), the
+   rest of the content stream is abandoned with a diagnostic.
+5. **Stack underflow:** If `Q` is called with an empty graphics state stack, the interpreter resets
+   to the page's initial graphics state and emits a warning. This is common in real-world PDFs with
+   mismatched `q`/`Q` across concatenated content streams.
+6. **Recursion limit:** Form XObject and tiling pattern nesting is bounded (default: 28 levels,
+   matching Acrobat's limit). Exceeding the limit produces an Error event and the nested content
+   is skipped.
```

## 2) Specify cooperative cancellation checkpoints in the render pipeline

The `ExecutionContext` carries cancellation tokens and deadlines, but the rendering section doesn't specify where cooperative cancellation checks occur. For interactive use (viewer scrolling, region renders, thumbnail generation), the ability to cancel mid-render is critical. Without defined checkpoints, cancellation granularity depends on implementation accidents.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Page rendering specifics
 8. Apply optional content visibility
+
+### Cooperative cancellation in rendering
+
+The render pipeline checks the `ExecutionContext` cancellation token at the following checkpoints:
+
+1. **Per-operator:** After each content stream operator dispatch. This is the finest granularity and
+   ensures that even a single pathological operator (e.g., a huge mesh shading) can be interrupted.
+2. **Per-tile/band:** Before materializing each tile in the tile/band scheduler. A cancelled tile
+   produces a placeholder (transparent or diagnostic-colored region).
+3. **Per-page:** Before starting each page in a multi-page render. Already-completed pages are
+   retained; the cancelled page and subsequent pages are skipped.
+4. **Per-resource:** Before decoding each image or font resource. Large JPEG 2000 or JBIG2 decodes
+   are interruptible at the codec level (the decode pipeline checks cancellation between data blocks).
+
+When cancellation fires, the render pipeline returns a partial result with metadata indicating
+which pages/tiles completed and which were cancelled. The partial result is usable (not corrupted).
+
+Budget enforcement uses the same checkpoints: if the operator count, memory, or time budget is
+exceeded, the effect is identical to cancellation. The diagnostic carries the specific budget that
+was exhausted.
```

## 3) Add cache eviction policy for bounded memory operation

The spec mentions engine-managed decode caches and `PdfSnapshot` structural sharing, but doesn't specify how caches are bounded. For a document with 10,000 pages, decoded streams, font caches, and PagePlan caches could consume unbounded memory. The spec needs an explicit eviction policy.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Engine / session / snapshot model
 All caches, proofs, and invalidation logic key off `snapshot_id`
+
+### Cache management doctrine
+
+Engine-level caches are bounded by configurable memory budgets. The default budgets are:
+
+- **Decoded stream cache:** 256 MB. Keyed by (snapshot_id, object_id, filter_chain_hash). LRU
+  eviction. Streams currently being consumed by an active render/extract operation are pinned and
+  cannot be evicted.
+- **Font cache:** 128 MB. Keyed by font dictionary object_id. Parsed font programs, glyph outlines,
+  and CMap tables. LRU eviction. Fonts referenced by the current page's resources are pinned.
+- **PagePlan cache:** 64 MB. Keyed by (snapshot_id, page_index). The immutable display list for a
+  page. LRU eviction. Invalidated when any object in the page's dependency subgraph changes.
+- **Raster tile cache:** Configurable, default 512 MB. Keyed by (snapshot_id, page_index, tile_id,
+  dpi). LRU eviction. Used by the tile/band scheduler for progressive and region rendering.
+
+Cache budgets are exposed in `ExecutionContext` so proof runs can use smaller budgets to stress
+eviction paths. The cache reports hit/miss/eviction statistics through the trace/metrics sink.
+
+When memory pressure exceeds all cache budgets, the engine degrades gracefully: re-decoding streams
+on demand rather than caching, re-interpreting content streams instead of reusing PagePlans. This
+degradation is instrumented (diagnostics report cache pressure events).
```

## 4) Define thread-safety contracts for parallel page rendering

The spec mentions page-parallel rendering and that `PdfSnapshot` is shareable across threads, but doesn't specify the synchronization model. Which operations can run in parallel? What shared state requires synchronization? This is critical for correctness of the Rayon-based parallel render path.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Shared invariants
+### Thread-safety model
+
+`PdfSnapshot` is `Send + Sync`. Multiple threads may read from the same snapshot concurrently.
+The following operations are safe to run in parallel on the same snapshot:
+
+- Rendering different pages
+- Extracting text from different pages
+- Inspecting different objects
+- Decoding different streams (via the decode cache, which uses concurrent-safe access)
+
+The following require exclusive access and cannot run in parallel with reads on the same snapshot:
+
+- `EditTransaction::commit()` (produces a new snapshot; does not mutate the source snapshot)
+
+Engine-level caches use lock-free or sharded concurrent data structures:
+- Decoded stream cache: `DashMap<CacheKey, Arc<[u8]>>` or equivalent sharded concurrent map
+- Font cache: `DashMap<ObjRef, Arc<ParsedFont>>` with interior read-through
+- PagePlan cache: `DashMap<(SnapshotId, usize), Arc<PagePlan>>`
+
+The `ExecutionContext` is cloneable per-task (each parallel render task gets its own copy with shared
+budget counters using atomic operations). Cancellation propagates to all clones.
+
+Rayon's scoped parallelism ensures that all parallel tasks complete before the scope exits,
+preventing dangling references to snapshot data.
```

## 5) Specify the content stream edit model for redaction and flattening

The spec mentions `EditSink` as a consumer adapter in `monkeybee-content` and describes redaction application in `monkeybee-edit`, but doesn't specify how content streams are actually rewritten. Content stream editing is fundamentally different from object-graph editing — it requires parsing, filtering, and re-emitting operators. This is the mechanism behind redaction, annotation flattening, and content transformation.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ monkeybee-edit
+### Content stream rewrite model
+
+Content stream edits (redaction, annotation flattening, content removal) use a filter-and-rewrite
+pipeline:
+
+1. **Parse** the existing content stream into an operator sequence with provenance spans.
+2. **Filter** the operator sequence through an `EditSink` that decides per-operator: keep, drop,
+   or replace. The `EditSink` receives full graphics state context for each operator (not just the
+   raw operator and operands).
+3. **Inject** new operators at specified insertion points (e.g., annotation flattening appends
+   operators wrapped in `q`/`Q`).
+4. **Re-emit** the filtered/modified operator sequence as a new content stream.
+5. **Update** the page's content stream reference(s) to point to the new stream object.
+
+For redaction specifically:
+- The `EditSink` identifies all operators that produce output within the redaction region by
+  evaluating each operator's bounding box against the redaction rectangles.
+- Text operators are split if a `TJ` array partially overlaps a redaction region (individual glyph
+  positions are checked).
+- Image operators that partially overlap are handled per the `RedactionPlan` mode: `SemanticExact`
+  removes fully contained images; `SecureRasterizeRegion` replaces the region; `SecureRasterizePage`
+  replaces the entire page.
+- After rewrite, the old content stream object is marked as deleted in the change journal.
+
+For annotation flattening:
+- The annotation's appearance stream content is extracted, transformed by the annotation's
+  position matrix, wrapped in `q`/`Q`, and appended to the page content.
+- The annotation object is removed from the page's `/Annots` array.
```

## 6) Add a unified diagnostic streaming model

Diagnostics are mentioned throughout the spec (parser diagnostics, compatibility ledger, proof artifacts, error events), but there's no unified model for how diagnostics flow from deep subsystems up to the caller. This matters for: real-time diagnostic display in interactive use, CI integration, and ensuring no diagnostic is silently dropped.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ ExecutionContext definition
 - Trace / metrics sink
+
+### Diagnostic streaming model
+
+All diagnostics flow through a `DiagnosticSink` carried by `ExecutionContext`. The sink is a trait
+with a single method: `emit(diagnostic: Diagnostic)`. Implementations include:
+
+- `VecSink`: collects all diagnostics into a `Vec<Diagnostic>` (default for library use).
+- `CallbackSink`: invokes a user-provided closure per diagnostic (for real-time display).
+- `FilteringSink`: wraps another sink and filters by severity, subsystem, or error code.
+- `CountingSink`: wraps another sink and counts diagnostics by category (for budget enforcement:
+  "abort after 1000 warnings" policies).
+
+Every diagnostic carries:
+- Error code (hierarchical string, e.g., `parse.xref.wrong_offset`)
+- Severity (Fatal, Error, Warning, Info)
+- Subsystem origin (parser, renderer, writer, etc.)
+- Object context (ObjRef, page number, byte offset — whichever are applicable)
+- Human-readable message
+- Machine-readable payload (repair details, original/corrected values, feature classification)
+
+The `DiagnosticSink` is the input side; the compatibility ledger (Part 6) is the aggregated output
+side. All diagnostics emitted during a session are collected into the compatibility ledger at
+session close.
+
+Diagnostics are never silently dropped. If the `ExecutionContext` has no explicit sink configured,
+a default `VecSink` collects them. The API always returns the diagnostic collection alongside the
+operation result.
```

## 7) Specify progressive rendering behavior for partially-available documents

Workflow 8 describes progressive/lazy/remote PDF opens, but the rendering pipeline doesn't specify what happens when a page references resources that haven't been fetched yet. The render pipeline needs to define partial-render behavior.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Workflow 8: Open huge or remote PDFs progressively
+
+### Progressive rendering contract
+
+When rendering a page from a lazily/partially-loaded document, the renderer operates in
+progressive mode:
+
+1. **Available resources render immediately.** Content stream operators that reference already-fetched
+   resources (fonts, images, XObjects) are rendered normally.
+2. **Unavailable resources produce placeholders.** An image XObject whose stream data hasn't been
+   fetched renders as a gray placeholder rectangle with a loading indicator. A font that hasn't been
+   fetched uses a substitute font with a diagnostic.
+3. **Placeholder metadata.** Each placeholder carries the byte range needed to fetch the missing
+   resource. The caller can use this to prioritize fetches for the visible region.
+4. **Incremental refinement.** When a previously-missing resource becomes available, only the
+   affected tiles in the tile/band scheduler are invalidated and re-rendered. The rest of the page
+   is preserved.
+5. **Prefetch planning.** Before rendering, the render pipeline reports the set of resources needed
+   for the requested page/region. The byte source's fetch scheduler can use this to issue range
+   requests proactively.
+
+Progressive rendering is orthogonal to the tile/band scheduler: a tile may be partially rendered
+(some resources available, some not) and refined later. The cache key for progressive tiles includes
+a "completeness" flag so refined tiles replace partial ones.
```

## 8) Add explicit PDF version handling and feature gating

The spec references PDF 2.0, PDF 1.5+, and PDF 1.4 in various places but doesn't define a systematic version-awareness model. Features like object streams (1.5+), cross-reference streams (1.5+), optional content (1.5+), and AES encryption (1.6+) are version-gated. The engine needs explicit version tracking for both input and output.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Part 4 — Shared invariants
+### PDF version awareness
+
+The engine tracks the PDF version at three levels:
+
+1. **Input version:** The version declared in the file header (`%PDF-1.N` or `%PDF-2.0`) and
+   optionally overridden by the catalog's `/Version` entry (which takes precedence when present and
+   higher). The parser uses the input version to select appropriate parsing behaviors:
+   - Pre-1.5: no object streams, no cross-reference streams
+   - Pre-1.4: no transparency model
+   - Pre-1.6: no AES encryption, no OpenType/CFF embedding
+   - 2.0: additional encryption revisions (R6), new annotation types, updated color semantics
+
+2. **Feature version:** Each parsed feature is tagged with the minimum PDF version that defines it.
+   In strict mode, features that exceed the declared version produce a diagnostic. In tolerant mode,
+   they are accepted (many producers declare an older version but use newer features).
+
+3. **Output version:** The writer emits the minimum version that covers all features present in the
+   output document. If the user requests a specific output version (e.g., for downlevel
+   compatibility), features incompatible with that version produce a preflight error.
+
+Version tracking feeds into the compatibility ledger: the ledger records the declared version, the
+effective version (minimum version needed for all features actually used), and any version
+mismatches detected.
```

## 9) Define the compatibility ledger schema concretely

Part 6 references the compatibility ledger schema but doesn't define it beyond a general description. For the ledger to be the "backbone of the proof infrastructure" as stated, its schema needs to be concrete enough that tools can consume it.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Compatibility ledger
+
+### Compatibility ledger schema
+
+The compatibility ledger is a JSON document with the following top-level structure:
+
+```json
+{
+  "schema_version": "1.0",
+  "engine_version": "0.1.0",
+  "timestamp": "2026-03-15T12:00:00Z",
+  "input": {
+    "filename": "example.pdf",
+    "sha256": "abc123...",
+    "declared_version": "1.7",
+    "effective_version": "2.0",
+    "size_bytes": 1234567,
+    "page_count": 42,
+    "producer": "Adobe Acrobat 2024",
+    "creator": "Microsoft Word"
+  },
+  "features": [
+    {
+      "code": "transparency.isolated_knockout_group",
+      "tier": 1,
+      "status": "supported",
+      "pages": [3, 7, 12],
+      "details": "Isolated knockout transparency groups on 3 pages"
+    }
+  ],
+  "repairs": [
+    {
+      "code": "parse.xref.wrong_offset",
+      "severity": "warning",
+      "object": "42 0",
+      "original_value": "12345",
+      "corrected_value": "12389",
+      "strategy": "backward_scan",
+      "confidence": 0.95
+    }
+  ],
+  "degradations": [
+    {
+      "code": "compat.xfa.dynamic_no_fallback",
+      "tier": 3,
+      "severity": "error",
+      "pages": [1],
+      "description": "Dynamic XFA form with no AcroForm fallback; pages render as blank"
+    }
+  ],
+  "summary": {
+    "total_features": 156,
+    "tier1_count": 142,
+    "tier2_count": 8,
+    "tier3_count": 6,
+    "repair_count": 3,
+    "degradation_count": 2,
+    "overall_status": "degraded"
+  }
+}
+```
+
+The schema is versioned. Breaking changes increment the major version. The proof harness validates
+ledger output against the schema. Downstream tools (dashboards, CI gates, regression detectors)
+consume the ledger via the schema.
```

## Net effect

These nine changes fill operational gaps that the first three rounds left open:

* **Error recovery** in the content interpreter becomes deterministic, not ad-hoc.
* **Cancellation** and **memory bounds** make the engine viable for interactive use on huge documents.
* **Thread-safety contracts** make page-parallel rendering correct by construction.
* **Content stream editing** gets a concrete mechanism instead of a hand-wave.
* **Diagnostics** flow through a unified pipeline instead of being collected ad-hoc.
* **Progressive rendering** becomes a specified contract instead of an aspirational workflow.
* **Version handling** and the **ledger schema** close the gap between the spec's ambitions and implementability.

The single most important addition is the content stream error recovery protocol (revision 1) — without it, every consumer would reinvent error handling differently, creating exactly the kind of divergence the shared interpreter was designed to prevent.
