This round reviews both the SPEC and the implementation master doc together. Five rounds of revision have produced a comprehensive architectural spec. The implementation master doc has tracked well, but there are now gaps between the two documents and several places where the spec's growing complexity needs concrete implementation grounding.

## 1) Resolve the 7 open questions in the implementation master doc

The implementation_master.md lists 7 open questions that have been sitting unanswered since the doc was created. The spec is now mature enough to give definitive v1 answers to each. Leaving them open creates ambiguity during implementation.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Open questions
-1. **Font rasterization strategy**: Use `freetype-rs` (C binding, battle-tested) or `ab_glyph` + `ttf-parser` (pure Rust, less mature)? Or a switchable backend?
-2. **JPEG 2000**: Accept `openjpeg-sys` C binding, or wait for pure-Rust `jpeg2k` to mature?
-3. **Rendering backend**: Start with a simple CPU rasterizer (tiny-skia?), or target a more capable backend from the start?
-4. **Color management**: Full ICC profile support from day one, or defer to a later phase?
-5. **Incremental save granularity**: Support byte-range preservation for signature-safe workflows in v1, or defer?
-6. **CMap handling**: Build custom CMap parser, or leverage an existing library?
-7. **Performance target**: Define specific throughput targets (pages/sec on benchmark docs) or keep qualitative ("obviously fast")?
+## Resolved design decisions
+
+1. **Font rasterization strategy**: `ttf-parser` (pure Rust) for font table parsing; `ab_glyph_rasterizer`
+   for glyph rasterization in the baseline. `freetype-rs` available as an optional feature-flagged
+   backend behind a `FontRasterizer` trait for environments that need hinting. The trait allows
+   switching without touching render code. v1 ships with the pure-Rust path as default.
+
+2. **JPEG 2000**: Accept `openjpeg-sys` C binding for v1 behind `monkeybee-security` isolation
+   (the decoder runs in a budget-bounded worker per the security profile). Pure-Rust `jpeg2k` is
+   the planned replacement once it reaches decode-correctness parity. JPXDecode is gated by the
+   security profile: `Hardened` mode isolates it, `Strict` mode disables it with Tier 3 reporting.
+
+3. **Rendering backend**: `tiny-skia` for the baseline CPU rasterizer (proven, pure Rust, supports
+   all needed operations: paths, anti-aliasing, compositing, clipping). The experimental exact
+   analytic area coverage rasterizer (B-ALIEN-001) replaces `tiny-skia`'s anti-aliasing path only
+   after it beats the baseline under the proof harness. The `RenderBackend` trait allows both to
+   coexist.
+
+4. **Color management**: ICC profile support is v1-gating for ICCBased color spaces (they appear
+   in the majority of real-world PDFs). Use `lcms2` (C binding) for v1 ICC profile evaluation,
+   behind a `ColorProfileProvider` trait. The experimental spectral-aware pipeline (B-ALIEN-003)
+   is an alternative implementation of the same trait. CalRGB/CalGray/Lab conversions are
+   implemented directly (no library needed — the math is straightforward).
+
+5. **Incremental save granularity**: Byte-range preservation for signature-safe workflows is
+   v1-gating per SPEC.md Part 8 and the WritePlan classification rules. The preserve-mode write
+   path and signature impact analysis are baseline v1 features, not deferred.
+
+6. **CMap handling**: Custom CMap parser. The CMap format is simple enough (begincodespacerange,
+   beginbfchar, beginbfrange, usecmap) that a purpose-built parser is smaller, faster, and more
+   controllable than adapting a general-purpose library. Ship with embedded Adobe CJK CMap data
+   for the four standard CIDSystemInfo registries (Adobe-Japan1, Adobe-CNS1, Adobe-GB1,
+   Adobe-Korea1). CMap data is lazily loaded to keep the WASM binary under the 5 MiB target.
+
+7. **Performance targets**: Quantitative targets are defined in SPEC.md Part 7 (Performance
+   doctrine). Summary: first-page render <50ms at 150 DPI (latency class), sustained 10+
+   pages/sec at 150 DPI with parallelism (throughput class), no operation >10x expected time for
+   content size (stress class), peak memory <5x file size for typical docs (memory class).
```

## 2) Add the `MonkeybeeEngine` and `OpenSession` type definitions to implementation master

The spec defines the Engine/Session/Snapshot lifecycle in Part 3, but the implementation master has no type definitions for `MonkeybeeEngine` or `OpenSession`. These are the top-level entry points for the entire library API.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Core data structures
+### Engine and session model (`monkeybee-document`)
+
+```rust
+/// Top-level engine: owns global policy, caches, and providers.
+/// Typically one per process. Thread-safe (Send + Sync).
+pub struct MonkeybeeEngine {
+    pub config: EngineConfig,
+    pub caches: CacheManager,
+    pub font_provider: Box<dyn FontProvider>,
+    pub color_profile_provider: Box<dyn ColorProfileProvider>,
+    pub crypto_provider: Option<Box<dyn CryptoProvider>>,
+    pub oracle_provider: Option<Box<dyn OracleProvider>>,
+    pub security_profile: SecurityProfile,
+}
+
+/// An open document session: binds a byte source to the engine.
+/// Created by `engine.open(byte_source, options)`.
+pub struct OpenSession {
+    pub engine: Arc<MonkeybeeEngine>,
+    pub byte_source: Box<dyn ByteSource>,
+    pub revision_chain: RevisionChain,
+    pub current_snapshot: Arc<PdfSnapshot>,
+    pub open_strategy: OpenStrategy,  // eager, lazy, or remote
+    pub exec_ctx: ExecutionContext,
+}
+
+/// Immutable, shareable document state. Identified by snapshot_id.
+/// Send + Sync — safe for concurrent page-parallel operations.
+pub struct PdfSnapshot {
+    pub snapshot_id: SnapshotId,
+    pub document: PdfDocument,
+    pub syntax_snapshot: SyntaxSnapshot,  // from monkeybee-syntax
+    pub dep_graph: DependencyGraph,
+}
+
+pub enum OpenStrategy {
+    Eager,   // parse everything available locally
+    Lazy,    // resolve objects on demand
+    Remote,  // range requests + prefetch planner
+}
+```

--- a/SPEC.md
+++ b/SPEC.md
@@ Engine / session / snapshot model
 `EditTransaction` consumes a snapshot and produces a new snapshot plus a serializable delta.
+
+**API surface:**
+
+```
+engine = MonkeybeeEngine::new(config)?;
+session = engine.open(byte_source, open_options)?;         // parses, produces first snapshot
+snapshot = session.current_snapshot();                      // Arc<PdfSnapshot>, cheap clone
+
+// Read operations (parallel-safe on snapshot):
+rendered_page = snapshot.render_page(page_index, render_opts, &exec_ctx)?;
+text = snapshot.extract_text(page_index, extract_opts, &exec_ctx)?;
+info = snapshot.inspect_object(obj_ref)?;
+
+// Mutation:
+tx = EditTransaction::new(snapshot.clone());
+tx.add_annotation(page_index, annotation)?;
+tx.set_metadata("Title", "New Title")?;
+new_snapshot = tx.commit()?;                                // produces new snapshot, delta
+
+// Write:
+plan = WritePlan::compute(&new_snapshot, write_mode)?;      // classify, check signatures
+bytes = plan.execute(&engine, &exec_ctx)?;                  // serialize to bytes
+```
+
+This API ensures:
+- No mutable access to live snapshots (all mutation goes through EditTransaction)
+- Read operations are parallelizable
+- Write planning is inspectable before committing bytes
+- The engine's caches survive across snapshots (keyed by snapshot_id)
```

## 3) Define the `ExecutionContext` type in the implementation master

The spec references `ExecutionContext` extensively but the implementation master has no type definition for it. It's the single most cross-cutting type in the engine — every operation takes it.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Core data structures
+### Execution context (`monkeybee-core::context`)
+
+```rust
+/// Carried by every top-level API. Cloneable per-task for parallel operations.
+/// Cancellation propagates to all clones.
+pub struct ExecutionContext {
+    pub budgets: ResourceBudgets,
+    pub cancellation: CancellationToken,
+    pub deadline: Option<Instant>,
+    pub determinism: DeterminismSettings,
+    pub diagnostic_sink: Arc<dyn DiagnosticSink>,
+    pub trace_sink: Option<Arc<dyn TraceSink>>,
+    pub security_profile: SecurityProfile,
+    pub cache_overrides: Option<CacheConfig>,  // for proof runs with smaller budgets
+}
+
+pub struct ResourceBudgets {
+    pub max_objects: u64,              // default: 10_000_000
+    pub max_decompressed_bytes: u64,   // default: 1 GiB
+    pub max_operators_per_page: u64,   // default: 5_000_000
+    pub max_nesting_depth: u32,        // default: 256
+    pub max_page_count: u32,           // default: 100_000
+}
+
+pub struct DeterminismSettings {
+    pub deterministic_output: bool,    // canonical serialization order, stable hashers
+    pub pinned_fallback_fonts: bool,   // use pinned font pack instead of system fonts
+    pub fixed_thread_count: Option<usize>,  // for reproducible benchmarks
+}
+
+/// Cooperative cancellation token — cheaply cloneable, atomically cancellable.
+/// Checked at every cancellation checkpoint (per-operator, per-tile, per-page, per-resource).
+#[derive(Clone)]
+pub struct CancellationToken {
+    cancelled: Arc<AtomicBool>,
+}
+```
```

## 4) Add the `SecurityProfile` type and its interaction with the codec pipeline

The spec defines three security profiles (Compatible, Hardened, Strict) and says "all high-risk decode jobs execute through monkeybee-security." But neither document shows the actual type or how it gates decoder invocation.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Core data structures
+### Security profiles (`monkeybee-security::profile`)
+
+```rust
+/// Security profile that controls decoder behavior, resource limits, and isolation.
+#[derive(Clone, Copy, PartialEq, Eq)]
+pub enum SecurityProfile {
+    /// Widest feature coverage. All decoders enabled.
+    Compatible,
+    /// Risky decoders run in isolated workers with bounded budgets.
+    Hardened,
+    /// Risky decoders disabled with explicit Tier 3 degradation reporting.
+    Strict,
+}
+
+/// Policy decision for a specific decoder/feature.
+pub enum DecoderPolicy {
+    Allow,                          // run directly
+    Isolate { budget: DecodeBudget }, // run in isolated worker with budget
+    Deny { reason: String },         // disable, report Tier 3 diagnostic
+}
+
+pub struct DecodeBudget {
+    pub max_output_bytes: u64,       // max decoded size
+    pub max_wall_time_ms: u64,       // max wall-clock time
+    pub max_memory_bytes: u64,       // max heap usage during decode
+}
+
+impl SecurityProfile {
+    /// Get the policy for a specific decoder.
+    pub fn policy_for(&self, decoder: DecoderType) -> DecoderPolicy { /* ... */ }
+}
+
+/// High-risk decoder types gated by the security profile.
+pub enum DecoderType {
+    JBIG2,
+    JPEG2000,
+    Type4Calculator,   // PostScript calculator functions
+    XfaXmlPacket,
+}
+```
+
+The codec pipeline checks `exec_ctx.security_profile.policy_for(decoder_type)` before invoking
+any high-risk decoder. This is not a suggestion — it is enforced by the type system: the decoder
+entry points in `monkeybee-codec` are `pub(crate)`, and the public API routes through
+`monkeybee-security` which applies the policy.

--- a/SPEC.md
+++ b/SPEC.md
@@ monkeybee-security
 - Hostile-input policy enforcement
+
+**Security-gated decoder invocation flow:**
+
+```
+Consumer (render/extract/parse) requests stream decode
+  → monkeybee-codec public API
+  → Checks ExecutionContext.security_profile.policy_for(decoder_type)
+  → Allow: invoke decoder directly with budget
+  → Isolate: spawn bounded worker, invoke decoder in worker, collect result or timeout
+  → Deny: return Err with Tier 3 diagnostic, consumer handles degradation
+```
+
+The security gate is not bypassable. The internal decoder functions in `monkeybee-codec` are
+`pub(crate)` — external crates cannot invoke JBIG2, JPEG 2000, Type 4 calculator, or XFA
+packet handlers directly. They must go through the security-gated public API.
```

## 5) Add the `CompatibilityLedger` Rust type to implementation master

The SPEC defines a comprehensive compatibility ledger schema (Part 6) with JSON serialization, but the implementation master has no corresponding Rust type. This is one of the most important types in the proof infrastructure.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Core data structures
+### Compatibility ledger (`monkeybee-proof::ledger`)
+
+```rust
+/// Machine-readable record for every document processed.
+/// Serialized to JSON per the schema in SPEC.md Part 6.
+#[derive(Serialize, Deserialize)]
+pub struct CompatibilityLedger {
+    pub schema_version: String,         // "1.0"
+    pub engine_version: String,
+    pub timestamp: String,              // ISO 8601
+    pub input: InputInfo,
+    pub features: Vec<FeatureEntry>,
+    pub repairs: Vec<RepairEntry>,
+    pub degradations: Vec<DegradationEntry>,
+    pub pages: Vec<PageLedger>,
+    pub summary: LedgerSummary,
+}
+
+#[derive(Serialize, Deserialize)]
+pub struct InputInfo {
+    pub filename: String,
+    pub sha256: String,
+    pub declared_version: String,
+    pub effective_version: String,
+    pub size_bytes: u64,
+    pub page_count: u32,
+    pub producer: Option<String>,
+    pub creator: Option<String>,
+}
+
+#[derive(Serialize, Deserialize)]
+pub struct FeatureEntry {
+    pub code: String,                   // e.g., "transparency.isolated_knockout_group"
+    pub tier: u8,                       // 1, 2, or 3
+    pub status: String,                 // "supported", "partial", "degraded", "unsupported"
+    pub pages: Vec<u32>,
+    pub details: String,
+}
+
+#[derive(Serialize, Deserialize)]
+pub struct RepairEntry {
+    pub code: String,
+    pub severity: String,
+    pub object: Option<String>,         // "42 0" format
+    pub original_value: String,
+    pub corrected_value: String,
+    pub strategy: String,
+    pub confidence: f64,
+}
+
+#[derive(Serialize, Deserialize)]
+pub struct LedgerSummary {
+    pub total_features: u32,
+    pub tier1_count: u32,
+    pub tier2_count: u32,
+    pub tier3_count: u32,
+    pub repair_count: u32,
+    pub degradation_count: u32,
+    pub overall_status: String,         // "clean", "repaired", "degraded", "failed"
+}
+```
```

## 6) Specify the content interpreter's `PagePlan` IR structure

The spec describes `PagePlan` as an "immutable page-scoped display list" with specific contents, but neither document defines the actual IR structure. This is a critical type — it's the boundary between interpretation and consumption.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Core data structures
+### PagePlan IR (`monkeybee-content::pageplan`)
+
+```rust
+/// Immutable page-scoped display list. Cacheable, shareable, region-queryable.
+/// Produced by interpreting a page's content stream(s) through the graphics state machine.
+pub struct PagePlan {
+    pub page_index: usize,
+    pub media_box: Rectangle,
+    pub crop_box: Rectangle,
+    pub ops: Vec<DrawOp>,              // normalized draw operations in page order
+    pub text_runs: Vec<TextRun>,       // text with positions and Unicode
+    pub resource_deps: HashSet<ObjRef>, // all objects this page depends on
+    pub marked_spans: Vec<MarkedSpan>, // marked content regions
+    pub degradations: Vec<DegradationNote>, // any operator-level errors/degradations
+    pub provenance: Vec<SourceSpan>,   // byte offsets in content stream for each op
+}
+
+/// A normalized draw operation.
+pub enum DrawOp {
+    FillPath { path: Path, rule: FillRule, color: ResolvedColor, state: DrawState },
+    StrokePath { path: Path, color: ResolvedColor, stroke: StrokeParams, state: DrawState },
+    ClipPath { path: Path, rule: FillRule },
+    DrawImage { image_ref: ObjRef, rect: Rectangle, state: DrawState },
+    DrawInlineImage { data: Arc<[u8]>, params: ImageParams, rect: Rectangle, state: DrawState },
+    BeginGroup { isolated: bool, knockout: bool, blend_mode: BlendMode, soft_mask: Option<SoftMaskRef> },
+    EndGroup,
+    BeginMarkedContent { tag: String, properties: Option<ObjRef> },
+    EndMarkedContent,
+}
+
+/// A positioned text run with resolved Unicode and glyph info.
+pub struct TextRun {
+    pub glyphs: Vec<PositionedGlyph>,
+    pub unicode: String,               // decoded Unicode text for this run
+    pub font_ref: ObjRef,
+    pub font_size: f64,
+    pub render_mode: TextRenderMode,
+    pub color: ResolvedColor,
+    pub state: DrawState,
+}
+
+pub struct PositionedGlyph {
+    pub glyph_id: u32,
+    pub unicode: Option<char>,
+    pub position: Point,               // in page space (after full transform chain)
+    pub advance: f64,
+    pub quad: [Point; 4],              // bounding quad for hit-testing/selection
+}
+
+/// Shared draw state snapshot (subset of GraphicsState relevant to rendering).
+pub struct DrawState {
+    pub ctm: Matrix,
+    pub alpha: f64,
+    pub blend_mode: BlendMode,
+    pub overprint: bool,
+    pub overprint_mode: i32,
+}
+```

--- a/SPEC.md
+++ b/SPEC.md
@@ PagePlan IR
 `PagePlan` is an immutable page-scoped display list containing normalized draw ops,
+
+**PagePlan structure:**
+- `ops: Vec<DrawOp>` — normalized draw operations (FillPath, StrokePath, ClipPath, DrawImage,
+  BeginGroup/EndGroup, BeginMarkedContent/EndMarkedContent) in page painting order.
+- `text_runs: Vec<TextRun>` — positioned text with resolved Unicode, glyph IDs, per-glyph
+  bounding quads, font reference, size, render mode, and color.
+- `resource_deps: Set<ObjRef>` — all objects this page depends on, for cache invalidation.
+- `marked_spans: Vec<MarkedSpan>` — marked content regions with tags and property references.
+- `degradations: Vec<DegradationNote>` — operator-level errors or degradations encountered
+  during interpretation.
+- `provenance: Vec<SourceSpan>` — maps each op back to its byte offset in the content stream.
+
+The PagePlan is the shared currency between subsystems: render consumes DrawOps and TextRuns
+to produce visual output; extract consumes TextRuns for text extraction; inspect consumes
+everything for structure analysis; diff consumes the full plan for page comparison.
```

## 7) Add the `OwnershipClass` and `ChangeReason` enum definitions

The spec references these in the mutation safety section and the change journal, but neither document defines the actual enum variants.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Change tracking
+/// Object ownership classification — determines how the write path handles each object.
+pub enum OwnershipClass {
+    /// Semantically understood by the engine. Eligible for rewrite/canonicalization.
+    Owned,
+    /// Not semantically understood but carried forward byte-preservingly.
+    /// Incremental-append preserves original bytes. Full-rewrite copies verbatim.
+    ForeignPreserved,
+    /// Detected but not safely transformable. Incompatible edits fail explicitly.
+    OpaqueUnsupported,
+}
+
+/// Why a change was made — enables undo, audit, and save-impact explanation.
+pub enum ChangeReason {
+    UserEdit,                // explicit user/API edit
+    AppearanceRegeneration,  // triggered by field value change
+    ResourceGC,              // unreachable object removal
+    Deduplication,           // identical object merge
+    Optimization,            // compaction, recompression
+    AnnotationAdd,           // annotation creation
+    AnnotationFlatten,       // annotation burned into page content
+    RedactionApply,          // redaction application
+    PageMutation,            // page add/remove/reorder
+    MetadataUpdate,          // metadata modification
+}
```

## 8) Reconcile the dependency graph between implementation master and SPEC

The implementation master's crate dependency graph is missing some edges that the spec implies. Specifically: `monkeybee-annotate` needs `render` for appearance generation (it delegates to render primitives per the annotation round-trip flow), and `monkeybee-proof` is missing `security` in its dependency list.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Crate dependency graph
-monkeybee-annotate      (depends on: core, document, content, compose, forms)
+monkeybee-annotate      (depends on: core, document, content, compose, forms, render)

 Note: monkeybee-annotate depends on monkeybee-render for appearance stream generation —
 specifically for rendering text and graphics within annotation appearance form XObjects.
 The compose crate handles the builder API; render provides the actual glyph/path realization.
+
+Note: monkeybee-proof already lists security in its dependency list. Verified.

--- a/SPEC.md
+++ b/SPEC.md
@@ monkeybee-annotate
 - Bridge generic annotation handling to `monkeybee-forms` for Widget annotations
+- Depends on `monkeybee-render` primitives for appearance stream content realization
+  (glyph positioning, path construction, color setting within form XObjects)
```

## 9) Add a concrete Cargo.toml workspace member list and feature flag strategy

Neither document specifies the actual workspace Cargo.toml structure or how feature flags are organized. This is critical for the baseline-vs-experimental lane separation.

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@ Workspace topology
+### Workspace Cargo.toml structure
+
+```toml
+[workspace]
+resolver = "2"
+members = [
+    "crates/monkeybee-core",
+    "crates/monkeybee-bytes",
+    "crates/monkeybee-codec",
+    "crates/monkeybee-security",
+    "crates/monkeybee-parser",
+    "crates/monkeybee-syntax",
+    "crates/monkeybee-document",
+    "crates/monkeybee-content",
+    "crates/monkeybee-text",
+    "crates/monkeybee-render",
+    "crates/monkeybee-compose",
+    "crates/monkeybee-write",
+    "crates/monkeybee-edit",
+    "crates/monkeybee-forms",
+    "crates/monkeybee-annotate",
+    "crates/monkeybee-extract",
+    "crates/monkeybee-validate",
+    "crates/monkeybee-proof",
+    "crates/monkeybee-cli",
+]
+
+[workspace.dependencies]
+# Shared dependency versions pinned at workspace level
+serde = { version = "1", features = ["derive"] }
+serde_json = "1"
+indexmap = { version = "2", features = ["serde"] }
+dashmap = "6"
+rayon = "1"
+thiserror = "2"
+```
+
+### Feature flag strategy
+
+Feature flags control the baseline-vs-experimental lane separation and optional native bindings:
+
+| Flag | Crate | Effect |
+|---|---|---|
+| `freetype` | monkeybee-text | Enable FreeType backend for font rasterization (default: off) |
+| `openjpeg` | monkeybee-codec | Enable OpenJPEG for JPEG 2000 decode (default: on in Compatible) |
+| `lcms2` | monkeybee-render | Enable lcms2 for ICC profile evaluation (default: on) |
+| `tiny-skia` | monkeybee-render | Enable tiny-skia rasterizer (default: on, baseline) |
+| `experimental-raster` | monkeybee-render | Enable exact analytic area coverage rasterizer |
+| `experimental-color` | monkeybee-render | Enable spectral-aware color pipeline |
+| `experimental-sdf` | monkeybee-render | Enable SDF glyph rendering path |
+| `wasm` | workspace | WASM-compatible build: no threads, no mmap, no system fonts |
+| `proof` | monkeybee-proof | Enable full proof harness (pulls in all reference renderers) |
+
+Baseline v1 builds with: `tiny-skia`, `lcms2`, `openjpeg` (Compatible profile).
+Experimental features are opt-in and must beat the baseline under the proof harness before
+becoming default.
```
