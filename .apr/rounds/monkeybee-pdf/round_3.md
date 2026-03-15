I reviewed the README, implementation master, and SPEC together. The strongest parts are already right: the closed-loop thesis, the proof-first release doctrine, and the compatibility ledger as an explicit “no silent failure” mechanism. The biggest issue is that the plan still mixes three different things in one layer: a believable baseline-v1 engine, a long-horizon research program, and a few implementation sketches that would make preserve-mode, snapshots, and huge/remote PDFs harder than the spec implies.   

I would keep the constitutional core unchanged: closed loop, ugly-PDF target, compatibility tiers, and proof-backed release gates. What I would change is the architecture around them so the project becomes more buildable, more reliable, and more credible.

## 1) Make the syntax layer real, not just conceptual

Right now the README says the architecture has a syntax stratum, but the implementation sketch goes directly from parser stages into `PdfDocument`, and there is no dedicated long-lived syntax/COS crate. That means preserve-mode byte ownership, repair provenance, object-stream membership, and “foreign preserved” objects have no durable home other than ad hoc fields hanging off parser/document structures. A first-class `monkeybee-syntax` layer would give you a clean preservation boundary and make strict/tolerant/preserve mode composable instead of entangled.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
-1. **Byte/revision layer** — immutable source bytes plus appended revisions.
-2. **Syntax layer** — token/span preserving PDF syntax and repair provenance.
-3. **Semantic document layer** — resolved page/resource/object graph with ownership classes.
-4. **Content layer** — parsed content-stream IR and interpreter shared by render/extract/inspect/edit.
+1. **Byte/revision layer** — immutable source bytes plus appended revisions.
+2. **Syntax/COS layer (`monkeybee-syntax`)** — immutable parsed objects, token/span provenance,
+   xref provenance, object-stream membership, raw formatting retention, and repair records.
+   This is the preservation boundary.
+3. **Semantic document layer (`monkeybee-document`)** — resolved page/resource/object graph built
+   from syntax snapshots; it owns semantic meaning, not raw-byte fidelity.
+4. **Content layer** — parsed content-stream IR and interpreter shared by render/extract/inspect/edit.
@@
-`monkeybee-core` is intentionally small; it provides shared primitives rather than becoming a god crate.
+`monkeybee-core` is intentionally small; it provides shared primitives rather than becoming a god crate.
+`monkeybee-syntax` is intentionally dumb but durable: it preserves what the parser saw and what the
+repair engine inferred, without forcing the semantic layer to own raw syntax detail.
```

## 2) Rework object and stream storage around lazy handles, not eagerly-owned `Vec`s

The spec wants lazy decoding, zero-copy where practical, eager/lazy/remote open strategies, parsed-object caches, decoded-stream caches, and tight memory ceilings. But the implementation sketch still models `PdfStream` as inline `raw_data: Vec<u8>` plus `decoded_data: OnceCell<Vec<u8>>`, and the whole object store as `HashMap<ObjRef, PdfValue>`. That fights the stated goals for huge/remote PDFs, cheap snapshots, and bounded memory. Decode caches should live at the engine/session level, not inline in every semantic object.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
-The PDF object model in `monkeybee-core` must faithfully represent all PDF 2.0 object types.
-Indirect objects carry object number + generation number. Streams carry both the dictionary and the
-raw/decoded data. The object graph supports forward and reverse reference lookups. Object access is
-zero-copy where practical and always safe.
+The PDF object model in `monkeybee-core` must faithfully represent all PDF 2.0 object types.
+Indirect objects carry object number + generation number. Streams carry the dictionary plus a
+byte-backed stream handle; decoded bytes live in engine-managed caches keyed by snapshot and filter
+chain, not inline in the object graph. The object graph supports forward and reverse reference
+lookups. Object access is zero-copy where practical, lazy by default for large/remote inputs, and
+always safe.
@@
-- Stream data is lazily decoded: raw bytes are always available, decoded bytes are produced on demand and may be cached.
+- Stream data is lazily decoded: raw bytes are always available through byte spans or range-backed
+  sources; decoded bytes are produced on demand and may be cached outside the semantic object graph.
+
+`PdfSnapshot` must use structural sharing. Opening a new snapshot or saving a delta must not imply
+cloning the full object store.
```

## 3) Enforce exactly one content interpreter and graphics-state machine

The README and spec are very clear that render/extract/inspect/edit should share one content-stream interpreter. But the implementation topology currently duplicates `interpreter.rs` and `state.rs` in both `monkeybee-content` and `monkeybee-render`. That is a drift trap: clipping, text matrices, stack underflow handling, blend-state transitions, and BX/EX behavior will diverge over time if two interpreters exist. `monkeybee-render` should consume events or `PagePlan`; it should not reinterpret the page independently.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 #### `monkeybee-content`
@@
 - Content stream parsing and operator dispatch
 - Graphics state machine (single implementation shared by all consumers)
 - Streaming event model for one-shot execution
 - `PagePlan` IR: immutable page-scoped display list for cached/region-aware workflows
 - Marked content span tracking
 - Source-span provenance for content-stream-level debugging
+ - Consumer adapters (`RenderSink`, `ExtractSink`, `InspectSink`, `EditSink`) so downstream crates
+   do not reimplement operator semantics
@@
 #### `monkeybee-render`
@@
-- Content stream interpretation (graphics state machine)
+- Consumption of `monkeybee-content` events or `PagePlan` IR through backend adapters
@@
-The renderer is backend-agnostic: it interprets content streams and emits drawing commands to an abstract backend trait.
+The renderer is backend-agnostic: it consumes the shared content interpreter's events or `PagePlan`
+and emits drawing commands to an abstract backend trait.
```

## 4) Split the text subsystem into two pipelines: PDF decode vs authoring/layout

This is one of the biggest semantic mismatches in the current spec. `monkeybee-text` correctly says shaping/bidi/fallback are needed for generation and FreeText annotation, but the render section then says existing PDF text rendering goes through shaping/bidi/fallback too. Those are not the same problem. Rendering an existing PDF means decoding PDF character codes, CMaps, glyph IDs, widths, and text matrices. Authoring new text means Unicode input, shaping, bidi, line breaking, and font fallback. Keeping those distinct will make rendering more correct and generation more deliberate.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 #### `monkeybee-text`
@@
-- Shaping, bidi, and font fallback for generation and FreeText annotation
+- PDF text decode pipeline for existing documents:
+  character code -> font/CMap -> CID/glyph -> Unicode/metrics/selection primitives
+- Authoring layout pipeline for emitted text:
+  Unicode -> shaping/bidi/line breaking/font fallback -> positioned glyph runs
@@
-All crates that need font resolution, text decoding, shaping, subsetting, or search delegate to `monkeybee-text`
+All crates that need font resolution, text decoding, subsetting, layout, or search delegate to
+`monkeybee-text`, but they do so through explicit decode-vs-layout APIs so existing PDF text is not
+accidentally "re-shaped" during rendering or extraction.
@@
 #### `monkeybee-render`
@@
-- Text rendering via `monkeybee-text`: font selection, shaping, bidi, fallback, glyph positioning, and Unicode-aware diagnostics
+- Text rendering via `monkeybee-text`: font lookup, encoding/CMap resolution, glyph dispatch,
+  positioned-glyph realization, and Unicode-aware diagnostics
```

## 5) Split authoring/composition out of `monkeybee-write`

`monkeybee-write` currently owns serialization, page-tree construction/manipulation, resource naming, content-stream generation, font embedding/subsetting for generated content, metadata generation, and output encryption. That is too much semantic ownership for a crate that should primarily be a serializer/saveback engine. A separate `monkeybee-compose` crate would make the pipeline cleaner: edit/annotate/forms/generation produce a semantically complete document; write serializes it safely. That split also makes appearance generation and new-document authoring easier to test in isolation.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
+#### `monkeybee-compose`
+
+High-level authoring and appearance composition.
+
+Key responsibilities:
+- Document/page/content builders for new documents
+- Resource naming and assembly
+- Annotation appearance stream generation helpers
+- Form/widget appearance composition
+- Font embedding planning and subsetting requests
+- Content stream emission from high-level drawing/text operations
+
 #### `monkeybee-write`
@@
-- Page tree construction and manipulation
-- Resource dictionary management
-- Content stream generation
-- Font embedding and subsetting delegated through `monkeybee-text` so render/extract/write use one font truth
-- Metadata generation and update
-- Encryption for output files
+- Deterministic rewrite and incremental append
+- Object serialization and xref/trailer emission
+- Structural validity enforcement
+- Final compression, encryption, and output assembly
+
+`monkeybee-write` serializes a semantically complete document.
+Authoring, page assembly, appearance generation, and builder-style APIs live in `monkeybee-compose`.
```

## 6) Add a first-class `WritePlan` and replace set-based change tracking with a journal

Signature-safe modification is a marquee workflow, and the spec’s mutation section already wants immutable snapshots plus detailed change accounting. But the implementation sketch still models `ChangeTracker` as just three `HashSet`s. That is not enough for incremental-save correctness, undo, cache invalidation, signature-impact explanation, or “why did this force a full rewrite?” diagnostics. Before every save, the engine should compute a `WritePlan` that explains exactly what will be preserved, appended, rewritten, or regenerated. That makes the system safer and dramatically more useful.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
-The underlying change tracking model:
-- Every mutation is recorded as (object_id, old_value_hash, new_value). This enables both incremental save (serialize only changed objects) and undo (restore old values).
+The underlying change tracking model:
+- Every mutation is recorded as a `ChangeEntry { object_id, old_fingerprint, new_value, reason,
+  ownership_before, ownership_after, dependency_delta }`. This enables incremental save, undo,
+  precise cache invalidation, and save-impact explanation.
@@
+### Save planning invariant
+
+Before any write, Monkeybee computes a `WritePlan` that classifies each touched object as one of:
+`PreserveBytes`, `AppendOnly`, `RewriteOwned`, `RegenerateAppearance`, `RequiresFullRewrite`, or
+`Unsupported`.
+
+`WritePlan` is surfaced to the API/CLI and to the compatibility ledger. Signature-safe workflows
+must be explainable before bytes are emitted, not inferred after the fact.
@@
-`monkeybee diagnose <file>` — full compatibility report
+`monkeybee diagnose <file>` — full compatibility report
+`monkeybee plan-save <file> [--incremental|--rewrite]` — preview ownership, rewritten regions,
+signature impact, and fallback reasons before saving
```

## 7) Re-scope baseline v1 much harder and move a few dangerous items out of gating

This is the highest-leverage planning fix. The README says baseline v1 should be the smallest coherent proof engine, with advanced backends staying optional until they beat the baseline. But the SPEC still makes the v1 surface feel too wide: render beads include all ICC/color paths plus mesh shadings 4–7 and overprint; write beads include output encryption; edit beads include high-assurance redaction; proof gates still make profile validation feel very close to core release criteria; and the advanced math section is written in “must” language in the main body. That weakens credibility.

My revision would be:

* **Baseline-gating**: xref repair, common fonts, core image filters, DeviceGray/RGB/CMYK + practical ICCBased subset, core transparency, basic annotations, extraction, inspect/diagnose, page add/remove/reorder, deterministic rewrite, incremental append, signature preservation, self-parse, public corpus proof.
* **Experimental/non-gating**: mesh shadings 4–7, full overprint semantics, SVG fidelity, output encryption, PDF/A/X profile validation, advanced spot-color paths, probabilistic extraction, spectral color science, exact analytic rasterizer.
* **Post-v1 unless separately proven**: high-assurance redaction.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Functional gates
@@
-- [ ] CLI exposes all major workflows.
+- [ ] CLI exposes baseline workflows: render, extract, inspect, annotate, edit-pages, validate, diagnose.
@@
 ### Proof gates
@@
-- [ ] Profile validation is v1-gating; profile-constrained emission is non-gating until baseline rewrite and incremental-append paths are proven stable.
+- [ ] Core Arlington validation for catalog/page tree/font/resource/writeback invariants is v1-gating.
+- [ ] PDF/A-4 and PDF/X-6 profile validation is advisory in v1 unless backed by public corpus coverage
+  and pinned oracle evidence.
@@
-#### Exact analytic area coverage for path rasterization
+#### Exact analytic area coverage for path rasterization (experimental)
@@
-#### Spectral-aware color science pipeline
+#### Spectral-aware color science pipeline (experimental)
@@
-### Render beads
+### Render beads
@@
-- B-RENDER-010: Shading types 4-7 (mesh-based gradient rendering)
-- B-RENDER-011: Overprint and overprint mode implementation
+- B-RENDER-010: Shading types 4-7 (mesh-based gradient rendering) [experimental]
+- B-RENDER-011: Overprint and overprint mode implementation [experimental until corpus-backed]
@@
-- B-WRITE-010: Output encryption (AES-256)
+- B-WRITE-010: Output encryption (AES-256) [non-gating / post-baseline]
@@
-- B-EDIT-003: Redaction application (high-assurance rewrite)
+- B-EDIT-003: Redaction application (high-assurance rewrite) [post-v1 unless separately proven]
```

## 8) Make the core runtime-agnostic; keep async orchestration at the edge

The implementation master currently blesses `asupersync` as the runtime model for the workspace, while the spec also wants lazy/remote opens, pinned CI providers, and a WASM-friendly core. That combination is much healthier if parse/render/write/edit stay runtime-agnostic and async is used where it truly helps: remote byte acquisition, proof orchestration, artifact pipelines, and external process control. This keeps the library easier to embed, easier to test, and easier to ship in nonstandard environments.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
+### Runtime layering doctrine
+
+Core library crates are runtime-agnostic.
+`ExecutionContext` carries budgets, cancellation, determinism, and providers, but parse/render/write/edit
+must not require a specific async runtime.
+
+Async orchestration is an adapter concern used by:
+- range-backed byte acquisition
+- proof harness orchestration
+- artifact streaming
+- external process / oracle coordination
+
+`asupersync` is the default orchestration runtime for CLI and proof, not a semantic dependency of
+the core engine model.
@@
-**WASM compilation target:**
+**WASM-friendly core target:**
@@
-The engine's core crates ... must be compilable to WebAssembly for browser-native use.
+The engine's core crates should remain WASM-friendly. A minimal WASM build is a non-gating proof
+surface until baseline v1 is proven.
```

## 9) Make the raster backend tile/band-oriented internally

Workflow 8 explicitly promises progressive opening of huge or remote PDFs, the CLI already wants region renders, and the performance doctrine sets memory goals that get awkward if the raster backend is fundamentally “render the whole page into one RGBA buffer.” A tile/band surface should be the default internal model, with full-page RGBA as one sink. That gives you lower peak memory, region-first rendering, more natural progressive painting, and a path to compositing budgets.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
-*Raster backend (PNG/JPEG):* Renders to an in-memory pixel buffer (RGBA, 8 bits per component).
-The buffer dimensions are determined by the page dimensions and the requested DPI.
+*Raster backend (PNG/JPEG):* Renders through a tile/band surface abstraction.
+Full-page RGBA output is one sink, not the only working set.
+Region render, thumbnail render, and remote-first first-paint reuse the same tile scheduler,
+dependency tracking, and caches.
@@
-**Page rendering specifics:**
+**Page rendering specifics:**
@@
-6. Interpret the concatenated content stream through the shared graphics state machine.
+6. Interpret the concatenated content stream through the shared graphics state machine and emit
+   commands into a tile/band scheduler that can materialize either a full page or only the requested region.
```

## Net effect

These changes do four things at once:

* They make preserve-mode and incremental append **more believable**.
* They make huge/remote PDFs and snapshots **architecturally real**, not just aspirational.
* They make the baseline v1 claim **narrower and stronger**.
* They keep the “alien artifact” ambition, but put it where it belongs: **behind proof, not ahead of proof**.

The single most important edit is the baseline/experimental split in the SPEC. The single most important architecture fix is the explicit syntax layer plus lazy snapshot-friendly storage. The single most useful user-facing addition is `WritePlan`.
