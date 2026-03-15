The spec has matured through four rounds into a genuinely comprehensive document — the crate boundaries, lifecycle, preservation doctrine, operational mechanisms (error recovery, cancellation, caching, thread safety, diagnostics), and proof infrastructure are all well-specified. This round focuses on remaining specification gaps: places where cross-cutting contracts are underspecified, where bead decomposition doesn't match the spec's actual content, and where the spec makes promises without sufficient implementation guidance.

## 1) Specify the `StreamHandle` contract for lazy stream access

The spec mentions that streams carry a "byte-backed stream handle" and that "decoded bytes live in engine-managed caches keyed by snapshot and filter chain, not inline in the object graph." But there is no explicit `StreamHandle` type or contract. Multiple crates (parser, document, content, render, extract, write) need to interact with stream data — without a defined handle contract, each will make different assumptions about ownership, lifetime, and cache interaction.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Object model contract
 The PDF object model in `monkeybee-core` must faithfully represent all PDF 2.0 object types.
+
+### StreamHandle contract
+
+PDF streams are the engine's primary data-carrying objects. The `StreamHandle` type mediates
+between the raw byte source and consumers that need decoded data:
+
+```
+StreamHandle {
+  object_id: ObjRef,
+  raw_span: ByteSpan,              // offset + length in the byte source
+  filter_chain: Vec<FilterSpec>,   // ordered decode filters with parameters
+  expected_decoded_length: Option<u64>,  // from content dimensions, when known
+}
+```
+
+**Access patterns:**
+
+1. **Raw bytes:** `handle.raw_bytes(byte_source) -> &[u8]` — returns the undecoded stream data
+   directly from the byte source. Used by preserve-mode write, stream analysis, and hex dump.
+2. **Decoded bytes:** `handle.decoded_bytes(engine_caches, exec_ctx) -> Arc<[u8]>` — returns
+   decoded data through the engine's decode cache. The cache key is
+   `(snapshot_id, object_id, filter_chain_hash)`. If the cache misses, the handle orchestrates
+   decode through `monkeybee-codec`, respecting the `exec_ctx` security profile and budgets.
+3. **Streaming decode:** `handle.decode_stream(byte_source, exec_ctx) -> impl Read` — returns a
+   streaming reader for large streams where materializing the full decoded output is unnecessary
+   or too expensive. Used by image rendering (progressive JPEG) and large content stream parsing.
+
+**Invariants:**
+- `StreamHandle` is `Clone + Send + Sync`. It carries no decoded data — only the metadata needed
+  to locate and decode the stream.
+- Decoded data is always `Arc<[u8]>` (shared, immutable). Multiple consumers can hold references
+  to the same decoded data without coordination.
+- The filter chain is validated at parse time. Invalid filter names produce a diagnostic and the
+  handle records the failure. Attempting to decode a handle with an invalid filter chain returns
+  an error without panicking.
+- For object streams, the `StreamHandle` of the container stream is resolved first; individual
+  objects within the stream are extracted by offset after decode.
```

## 2) Define the dependency graph computation contract

The spec references a "dependency graph" in `monkeybee-document` (page → resources → forms/patterns/xobjects/fonts/images/annotations) and says PagePlan invalidation depends on it, but never specifies how the graph is computed, stored, or queried. This is a critical contract — cache invalidation correctness, resource GC, and write planning all depend on it.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ monkeybee-document
 - Dependency graph (page -> resources -> forms/patterns/xobjects/fonts/images/annotations)
+
+### Dependency graph contract
+
+The dependency graph is a directed acyclic graph (cycles are detected and reported as errors)
+mapping objects to the objects they transitively depend on. It is computed lazily and cached
+per snapshot.
+
+**Nodes:** Every indirect object is a potential node. In practice, only objects reachable from
+the page tree, catalog, or AcroForm root are tracked.
+
+**Edges:** An edge from A to B means "A references B directly." Edge types are classified:
+- `ContentRef`: page/form XObject content stream references a resource by name
+- `DictRef`: dictionary value is an indirect reference
+- `ArrayRef`: array element is an indirect reference
+- `InheritedRef`: page inherits an attribute from an ancestor
+
+**Queries:**
+1. `dependents_of(obj_id) -> Set<ObjRef>`: all objects that transitively depend on `obj_id`
+   (reverse reachability). Used for cache invalidation — editing `obj_id` invalidates all
+   dependents.
+2. `dependencies_of(obj_id) -> Set<ObjRef>`: all objects that `obj_id` transitively depends on
+   (forward reachability). Used for resource GC — an object is garbage if no root reaches it.
+3. `page_dependencies(page_index) -> Set<ObjRef>`: the full transitive closure of objects needed
+   to render/extract page N. Used for PagePlan cache keying and write planning.
+4. `edit_impact(changed_ids: Set<ObjRef>) -> EditImpact`: given a set of changed objects, report
+   which pages are affected, which caches must be invalidated, and which other objects may need
+   regeneration (e.g., widget appearances after field value change).
+
+**Computation:** The graph is built by walking the object store from known roots (catalog, page
+tree nodes, AcroForm fields). Dictionary values and array elements are scanned for indirect
+references. Content stream resource names are resolved against the page's resource dictionary to
+find the referenced objects. The walk is bounded by the object count budget in `ExecutionContext`.
+
+**Storage:** The graph is stored as adjacency lists (forward and reverse) in a `DashMap` for
+concurrent read access. The graph is immutable per snapshot — edits produce a new snapshot with
+an incrementally updated graph (only the changed subgraph is recomputed).
```

## 3) Specify the `WritePlan` computation and classification rules

The spec introduces `WritePlan` in Part 4 and references it as the save-planning invariant, but doesn't define how object classifications are determined. The rules for when an object gets `PreserveBytes` vs `RewriteOwned` vs `RequiresFullRewrite` are critical for signature safety and incremental save correctness.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Save planning invariant
 `WritePlan` is surfaced to the API/CLI and to the compatibility ledger.
+
+**WritePlan classification rules:**
+
+Each object in the document is classified based on its ownership status and edit history:
+
+1. **PreserveBytes:** The object was not modified in the current transaction, its ownership is
+   `ForeignPreserved`, and the write mode is incremental-append. The object's original bytes are
+   emitted verbatim. This is the default for incremental saves on unmodified objects.
+
+2. **AppendOnly:** The object is new (created in the current transaction). It is serialized and
+   appended in the incremental update section. Applies only in incremental-append mode.
+
+3. **RewriteOwned:** The object was modified in the current transaction and its ownership is
+   `Owned`. The object is re-serialized from its semantic representation. In incremental mode,
+   the new version supersedes the old via the updated cross-reference. In full-rewrite mode,
+   all `Owned` objects use this classification.
+
+4. **RegenerateAppearance:** The object is a widget annotation whose parent field value changed.
+   The appearance stream must be regenerated from the new field value before serialization.
+   This is a specialized form of `RewriteOwned` that triggers appearance generation.
+
+5. **RequiresFullRewrite:** The object cannot be safely emitted in incremental mode. Triggers
+   include: object was deleted (must be recorded in free list in full rewrite, or in incremental
+   xref), object's ownership is `OpaqueUnsupported` and an edit attempted to modify it, or the
+   object participates in a structural change (page tree reorganization) that cannot be expressed
+   as incremental updates.
+
+6. **Unsupported:** The object's structure is not understood by the engine (ownership
+   `OpaqueUnsupported`) and it was not modified. In full-rewrite mode, it is copied byte-for-byte
+   from the original. In incremental mode, it is left untouched.
+
+**Signature impact analysis:** The `WritePlan` computes which existing signatures (if any) would
+be invalidated by the planned write. For each signature's `/ByteRange`, the plan checks whether
+any classified object's original byte span overlaps. If yes, the signature is flagged as
+invalidated. In incremental-append mode with only `PreserveBytes` and `AppendOnly` objects, no
+existing signatures should be invalidated — if the plan detects otherwise, it reports an error
+before any bytes are written.
```

## 4) Add test obligation taxonomy for the proof harness

The spec defines test case classes (Part 6) and release gates (Part 8) but doesn't connect them to specific crates or define what "passes" means quantitatively. Without thresholds and per-crate obligations, the release gates are aspirational rather than enforceable.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Release gates
 - [ ] Performance benchmarks exist for all benchmark classes.
+
+### Test obligation matrix
+
+Each gated test class has a defined pass threshold and responsible crate:
+
+| Test class | Primary crate | Pass threshold | Metric |
+|---|---|---|---|
+| xref-repair | parser | 100% of corpus fixtures | All repairs produce valid xref |
+| font-fallback | text | ≥95% Unicode coverage on corpus | Extraction F1 vs ground truth |
+| transparency-compositing | render | MS-SSIM ≥0.97 vs consensus | Per-page, per-fixture |
+| producer-quirks | parser + render | ≥90% of quirk fixtures render correctly | MS-SSIM ≥0.95 |
+| incremental-update | parser + write | 100% of corpus fixtures | Parse-save-reparse |
+| encryption | parser | 100% of standard handlers (V1-V5) | Decrypt success |
+| annotation-roundtrip | annotate + write | 100% of annotation types | Geometry ≤0.5pt drift |
+| page-mutation | edit + document | 100% of mutation ops | Structural validity |
+| generation | compose + write | 100% of generation tests | Strict-mode self-parse + ref render |
+| adversarial | parser + security | 0 panics, 0 OOM on fuzz corpus | Pass/fail |
+| color-space | render | ΔE₀₀ ≤2.0 vs reference on corpus | Per-pixel average |
+| content-stream-stress | content | Complete without timeout on corpus | All ops processed |
+| signature-preserve | write | 100% byte-range integrity | Byte comparison |
+| redaction-safety | edit | 0 recoverable redacted content | Extraction scan |
+
+**Regression policy:** A test class that was passing in the previous CI run and fails in the
+current run is a blocking regression. The PR cannot merge until the regression is resolved or
+explicitly triaged as `known_regression` with a tracking issue.
+
+**Coverage requirement:** Each test class must have ≥10 fixtures in the public corpus tier.
+Classes with <10 public fixtures must have a corpus acquisition task tracked in the project.
```

## 5) Specify the `OracleProvider` and `CryptoProvider` trait contracts

The spec names these provider traits in Part 0 but never defines their methods or contracts. The `CryptoProvider` is especially important — it's the extension point for signature verification, and the spec promises signature inspection in v1.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Provider interfaces include FontProvider, ColorProfileProvider, CryptoProvider, and OracleProvider.
+
+### Provider trait contracts
+
+**CryptoProvider:**
+```
+trait CryptoProvider {
+  /// Verify a CMS/PKCS#7 detached signature against the signed bytes.
+  /// Returns the verification result including certificate chain, timestamps, and trust status.
+  fn verify_cms_signature(
+    &self,
+    signed_bytes: &[u8],
+    signature_der: &[u8],
+  ) -> Result<SignatureVerification>;
+
+  /// Verify a timestamp token (RFC 3161).
+  fn verify_timestamp(
+    &self,
+    tst_der: &[u8],
+  ) -> Result<TimestampVerification>;
+
+  /// Compute a message digest (SHA-256, SHA-384, SHA-512) for byte-range integrity checks.
+  fn digest(&self, algorithm: DigestAlgorithm, data: &[u8]) -> Vec<u8>;
+}
+```
+
+The default `CryptoProvider` implementation provides digest computation only (using pure-Rust
+crypto). Full PKI verification requires a configured provider (e.g., backed by OpenSSL, ring,
+or a platform keystore). When no verification-capable provider is configured, signature
+inspection reports the signature structure (issuer, serial, algorithm, timestamps) without
+trust validation.
+
+**OracleProvider:**
+```
+trait OracleProvider {
+  /// Look up a resource by its oracle key (font name, ICC profile identifier, CMap name).
+  /// Returns the resource bytes if found, None if not available.
+  fn resolve(&self, key: &OracleKey) -> Option<Arc<[u8]>>;
+
+  /// Report the oracle manifest (versions, sources, pinning info) for proof reproducibility.
+  fn manifest(&self) -> OracleManifest;
+}
+```
+
+The oracle provides deterministic resource resolution for CI/proof runs. In non-CI mode, the
+oracle falls through to ambient system resources (fonts, ICC profiles). In CI/proof mode, the
+oracle uses pinned resource packs for reproducibility.
```

## 6) Clarify the cross-reference stream vs table decision in the writer

The spec says "baseline deterministic writer prefers classic cross-reference tables" and "compact mode may prefer cross-reference streams after proof stability." But it doesn't specify the decision criteria or how this interacts with object streams. Object streams *require* cross-reference streams — the two features are coupled, and the spec should make this coupling explicit.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Cross-reference stream output
 Cross-reference streams are more compact and support object streams
+(which require cross-reference streams).
+
+**Decision rules for xref format selection:**
+
+1. If the output uses object stream packing → cross-reference streams are mandatory.
+2. If the output version is < PDF 1.5 → classic cross-reference tables are mandatory
+   (xref streams were introduced in PDF 1.5).
+3. If incremental-append mode and the existing file uses xref tables → prefer appending
+   an xref table for structural consistency (avoids hybrid files).
+4. If incremental-append mode and the existing file uses xref streams → append an xref stream.
+5. For full-rewrite mode with baseline settings → classic xref table (simpler to audit/debug).
+6. For full-rewrite mode with compact/optimized settings → xref stream with PNG Up predictor.
+
+The baseline v1 writer does not use object stream packing by default. The combination of
+object streams + xref streams is an optimization that lands only after the baseline writer's
+proof gates are satisfied.
```

## 7) Specify the annotation flattening coordinate transform chain

The spec says annotation flattening "wraps in `q`/`Q` and appends to page content" and "correctly transforms from the annotation's coordinate space to the page's coordinate space." But the actual transform chain is not specified. This is a common source of bugs — annotation Rect, appearance stream Matrix, page Rotation, and CropBox offset all interact.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Annotation flattening
 - The annotation object is removed from the page's `/Annots` array.
+
+**Flattening coordinate transform chain:**
+
+When flattening an annotation into page content, the following transforms are composed:
+
+1. The annotation's appearance stream is a form XObject with its own `/BBox` and optional
+   `/Matrix`. The appearance content is drawn in the form's coordinate space.
+2. The annotation's `/Rect` [llx lly urx ury] defines where the appearance is placed in the
+   page's default user space (before page rotation).
+3. The transform maps the appearance's `/BBox` to the annotation's `/Rect`:
+   ```
+   scale_x = (rect_urx - rect_llx) / (bbox_urx - bbox_llx)
+   scale_y = (rect_ury - rect_lly) / (bbox_ury - bbox_lly)
+   translate_x = rect_llx - bbox_llx * scale_x
+   translate_y = rect_lly - bbox_lly * scale_y
+   ```
+4. If the appearance has a `/Matrix`, it is applied before the BBox-to-Rect mapping:
+   `final_matrix = bbox_to_rect_matrix × appearance_matrix`
+5. The flattened content is: `q [final_matrix components] cm [appearance stream operators] Q`
+6. Page rotation does NOT affect the flattening transform — both the page content and the
+   annotation are in the same unrotated coordinate space. The viewer applies rotation uniformly.
+7. CropBox offset is similarly irrelevant to flattening — the annotation Rect is already in the
+   page's coordinate system, which includes the CropBox position.
```

## 8) Add the fetch scheduler contract for range-backed byte sources

The spec mentions a "fetch scheduler" and "prefetch planning" in `monkeybee-bytes` and references it from the progressive rendering contract, but never defines the fetch scheduler's interface or its interaction with the render pipeline.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ monkeybee-bytes
 - Fetch scheduler and prefetch planning for remote/lazy byte sources
+
+### Fetch scheduler contract
+
+The fetch scheduler mediates byte-range requests for remote or lazy byte sources:
+
+```
+trait FetchScheduler {
+  /// Request bytes in the given range. Returns a future that resolves when the bytes are
+  /// available in the byte source's local buffer.
+  fn request_range(&self, offset: u64, length: u64) -> FetchHandle;
+
+  /// Submit a prefetch plan (ordered list of ranges by priority). The scheduler issues
+  /// requests in priority order, subject to concurrency limits. Returns immediately.
+  fn submit_prefetch(&self, plan: PrefetchPlan);
+
+  /// Cancel all outstanding requests. In-flight requests may or may not complete.
+  fn cancel_all(&self);
+
+  /// Report fetch statistics (requests issued, bytes fetched, latencies).
+  fn statistics(&self) -> FetchStatistics;
+}
+```
+
+**PrefetchPlan:** An ordered list of `(offset, length, priority)` tuples. The render pipeline
+generates a prefetch plan by inspecting the page's resource dependencies: font programs, image
+streams, form XObject streams. Resources visible in the current viewport get higher priority.
+
+**Concurrency:** The scheduler limits concurrent HTTP range requests (default: 6, matching
+browser connection limits). Requests are coalesced when ranges overlap or are adjacent within
+a configurable gap threshold (default: 4096 bytes — it's cheaper to fetch a few extra bytes
+than to issue a separate request).
+
+**Integration with progressive rendering:** When the render pipeline encounters a stream whose
+bytes are not yet available, it:
+1. Records a placeholder in the tile output.
+2. Returns the needed byte range to the caller.
+3. The caller submits the range to the fetch scheduler.
+4. When the fetch completes, the caller invalidates the affected tiles and re-renders.
```

## 9) Reconcile the bead decomposition with spec content

The bead list in Part 9 has several gaps relative to the actual spec content after four rounds of revision. Some beads reference content that doesn't exist in the spec, and some spec sections have no corresponding bead.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Foundation beads
+- B-CORE-006: DiagnosticSink trait and implementations (VecSink, CallbackSink, FilteringSink, CountingSink)
+- B-CORE-007: PDF version tracking (input/feature/output versions)

 @@ Byte layer beads
+- B-BYTES-004: Fetch scheduler and prefetch planning for range-backed sources

+@@ Document layer beads
+- B-DOC-009: Dependency graph computation, storage, and query
+- B-DOC-010: Cache management integration (bounded budgets, eviction, pinning)
+
 @@ Content layer beads
+- B-CONTENT-006: Consumer sink adapters (RenderSink, ExtractSink, InspectSink, EditSink)
+- B-CONTENT-007: Content stream error recovery protocol
+
+@@ Write beads
+- B-WRITE-011: WritePlan computation and classification
+- B-WRITE-012: Signature impact analysis for save planning
+
+@@ Render beads
 - B-RENDER-001: Graphics state machine
+Note: B-RENDER-001 duplicates B-CONTENT-002 (graphics state machine). The state machine is
+owned by monkeybee-content, not monkeybee-render. Remove B-RENDER-001 and ensure B-CONTENT-002
+covers the full graphics state contract from Part 5.
+
+@@ Checkpoint beads
+- B-CHECK-005: Post-proof checkpoint (proof harness operational, corpus coverage thresholds met)
```

These additions ensure that every operational contract added in rounds 1-4 has a corresponding bead, and that the bead list accurately reflects the crate where each responsibility lives.
