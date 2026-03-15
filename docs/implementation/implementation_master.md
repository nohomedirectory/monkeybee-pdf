# Implementation Master — Monkeybee PDF

## Purpose

This document is the APR-facing implementation reference for Monkeybee PDF. It summarizes the crate topology, module boundaries, core data structures, data flows, cross-crate dependencies, and test obligations. It links to subordinate implementation docs for deeper subsystem design.

This is not a philosophical essay and not the full codebase. It is the grounding surface that keeps the SPEC honest about implementation realities.

## Workspace topology

```
monkeybee-pdf/
├── Cargo.toml                    # workspace root
├── crates/
│   ├── monkeybee-core/           # shared primitives: object IDs, geometry, errors, execution context
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── object.rs         # PDF object type definitions
│   │   │   ├── geometry.rs       # coordinate transforms, matrices
│   │   │   ├── error.rs          # shared error taxonomy
│   │   │   ├── context.rs        # ExecutionContext (budgets, cancellation, providers)
│   │   │   ├── diagnostics.rs    # DiagnosticSink trait, Diagnostic type, VecSink, CallbackSink, FilteringSink, CountingSink
│   │   │   ├── version.rs        # PdfVersion tracking (input, feature, output), version-gated feature registry
│   │   │   └── traits.rs         # ByteSource, FontProvider, ColorProfileProvider, CryptoProvider, OracleProvider
│   │   └── Cargo.toml
│   ├── monkeybee-bytes/          # byte sources, revision chain, raw span ownership
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── source.rs         # ByteSource implementations (mmap, in-memory, range-backed)
│   │   │   ├── fetch.rs          # fetch scheduler and prefetch planning for remote/lazy sources
│   │   │   ├── revision.rs       # revision chain tracking
│   │   │   └── span.rs           # raw span ownership for preserve mode
│   │   └── Cargo.toml
│   ├── monkeybee-codec/          # filter chains, image decode/encode, predictor logic
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── filters.rs        # stream filter implementations (Flate, LZW, ASCII85, etc.)
│   │   │   ├── predictor.rs      # PNG/TIFF predictor logic
│   │   │   ├── image.rs          # image decode/encode adapters (JPEG, JPEG2000, JBIG2)
│   │   │   ├── pipeline.rs       # bounded decode pipelines
│   │   │   └── telemetry.rs      # decode telemetry for proof and diagnostics
│   │   └── Cargo.toml
│   ├── monkeybee-security/       # security profiles, worker isolation, budget broker
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── profile.rs        # security profiles (Compatible, Hardened, Strict)
│   │   │   ├── budget.rs         # budget broker and enforcement
│   │   │   ├── isolation.rs      # worker isolation / kill-on-overrun
│   │   │   └── policy.rs         # risky-decoder allow/deny, hostile-input policy
│   │   └── Cargo.toml
│   ├── monkeybee-parser/         # PDF syntax parsing, repair (delegates codec/security)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── lexer.rs          # tokenization
│   │   │   ├── object_parser.rs  # object parsing
│   │   │   ├── xref_parser.rs    # xref table/stream parsing + repair
│   │   │   ├── stream.rs         # stream dispatch (delegates to monkeybee-codec)
│   │   │   ├── content.rs        # content stream parsing
│   │   │   ├── crypt.rs          # encryption/decryption
│   │   │   ├── repair.rs         # tolerant mode, recovery strategies
│   │   │   └── diagnostics.rs    # parser diagnostics
│   │   └── Cargo.toml
│   ├── monkeybee-syntax/         # syntax/COS preservation layer (between parser and document)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── cos_object.rs     # immutable COS object representation
│   │   │   ├── provenance.rs     # token/span provenance, source byte ranges
│   │   │   ├── xref_prov.rs      # xref provenance: original vs repaired entries
│   │   │   ├── objstream.rs      # object-stream membership tracking
│   │   │   ├── formatting.rs     # raw formatting retention (whitespace, comments)
│   │   │   ├── repair_record.rs  # repair records: strategy, confidence, alternatives
│   │   │   └── boundary.rs       # preservation boundary contract enforcement
│   │   └── Cargo.toml
│   ├── monkeybee-document/       # semantic document graph built from syntax snapshots
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── document.rs       # document-level model (PdfDocument, ObjectStore)
│   │   │   ├── xref.rs           # cross-reference management
│   │   │   ├── page.rs           # page tree, inheritance
│   │   │   ├── resource.rs       # resource resolution
│   │   │   ├── ownership.rs      # Owned/ForeignPreserved/OpaqueUnsupported classification
│   │   │   ├── update.rs         # incremental update tracking
│   │   │   ├── depgraph.rs       # dependency graph and derived-artifact invalidation
│   │   │   ├── snapshot.rs       # PdfSnapshot (immutable, shareable, keyed by snapshot_id)
│   │   │   ├── transaction.rs    # EditTransaction, change tracking, snapshot-in/snapshot-out
│   │   │   └── cache.rs          # CacheManager, CacheConfig, per-cache budgets, LRU eviction, hit/miss/eviction stats
│   │   └── Cargo.toml
│   ├── monkeybee-content/        # content-stream IR and event interpreter
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── interpreter.rs    # content stream interpreter
│   │   │   ├── state.rs          # graphics state machine
│   │   │   ├── events.rs         # streaming event model
│   │   │   ├── pageplan.rs       # PagePlan immutable display list IR
│   │   │   ├── marked.rs         # marked content span tracking
│   │   │   └── sink.rs           # consumer sink adapters (RenderSink, ExtractSink, InspectSink, EditSink)
│   │   └── Cargo.toml
│   ├── monkeybee-text/           # shared text subsystem: fonts, CMaps, decode + authoring pipelines, search
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── font.rs           # font program parsing and caching
│   │   │   ├── cmap.rs           # CMap / ToUnicode handling
│   │   │   ├── unicode.rs        # Unicode fallback chain
│   │   │   ├── decode.rs         # PDF text decode pipeline: char code -> font/CMap -> CID/glyph -> Unicode/metrics
│   │   │   ├── layout.rs         # authoring layout pipeline: Unicode -> shaping/bidi/line breaking -> glyph runs
│   │   │   ├── shaping.rs        # shaping, bidi, font fallback (used by layout pipeline)
│   │   │   ├── subset.rs         # subsetting and ToUnicode generation
│   │   │   └── search.rs         # search, hit-testing, selection primitives
│   │   └── Cargo.toml
│   ├── monkeybee-render/         # page rendering (consumes content events, not own interpreter)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── text.rs           # text rendering via decode pipeline (delegates to monkeybee-text)
│   │   │   ├── font.rs           # font dispatch (delegates to monkeybee-text)
│   │   │   ├── image.rs          # image rendering
│   │   │   ├── color.rs          # color space management
│   │   │   ├── path.rs           # vector path rendering
│   │   │   ├── transparency.rs   # transparency compositing
│   │   │   ├── pattern.rs        # tiling and shading patterns
│   │   │   ├── page.rs           # page assembly
│   │   │   ├── tile.rs           # tile/band surface abstraction and scheduler
│   │   │   ├── progressive.rs    # ProgressiveRenderState: placeholder tracking, incremental refinement, completeness flags
│   │   │   └── backend/          # output backends (raster via tile sink, svg)
│   │   └── Cargo.toml
│   ├── monkeybee-compose/        # high-level authoring and composition
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── doc_builder.rs    # document builder
│   │   │   ├── page_builder.rs   # page builder
│   │   │   ├── content_builder.rs # content stream emission from high-level ops
│   │   │   ├── resource.rs       # resource naming and assembly
│   │   │   ├── appearance.rs     # annotation/widget appearance stream generation
│   │   │   ├── font_plan.rs      # font embedding planning and subsetting requests
│   │   │   └── text_emit.rs      # text emission via authoring layout pipeline
│   │   └── Cargo.toml
│   ├── monkeybee-write/          # pure serializer (no composition/authoring)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── serialize.rs      # object serialization
│   │   │   ├── xref_writer.rs    # xref generation
│   │   │   ├── stream_encode.rs  # stream compression
│   │   │   ├── rewrite.rs        # full document rewrite (deterministic mode)
│   │   │   ├── incremental.rs    # incremental append save
│   │   │   ├── plan.rs           # WritePlan computation and classification
│   │   │   ├── encrypt.rs        # final encryption and output assembly
│   │   │   └── validate.rs       # output structural validation
│   │   └── Cargo.toml
│   ├── monkeybee-edit/           # transactional structural edits
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── transaction.rs    # edit transaction framework
│   │   │   ├── gc.rs             # resource GC and deduplication
│   │   │   ├── redaction.rs      # high-assurance redaction application
│   │   │   ├── rewriter.rs       # ContentStreamRewriter: parse-filter-inject-reemit pipeline for content stream edits
│   │   │   └── optimize.rs       # compaction, recompression
│   │   └── Cargo.toml
│   ├── monkeybee-forms/          # AcroForm field tree, value model, appearance regen
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── field_tree.rs     # field tree parsing and inheritance resolution
│   │   │   ├── value.rs          # field value model (text, button, choice, signature)
│   │   │   ├── appearance.rs     # appearance regeneration for widget annotations
│   │   │   ├── calc_order.rs     # calculation order detection and preservation
│   │   │   ├── widget.rs         # widget/annotation bridge
│   │   │   └── signature.rs      # signature-field helpers
│   │   └── Cargo.toml
│   ├── monkeybee-annotate/       # non-form annotation operations
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── model.rs          # annotation type model (non-form)
│   │   │   ├── placement.rs      # geometry-aware placement
│   │   │   ├── appearance.rs     # appearance stream generation
│   │   │   ├── flatten.rs        # annotation flattening
│   │   │   └── roundtrip.rs      # round-trip validation helpers
│   │   └── Cargo.toml
│   ├── monkeybee-extract/        # multi-surface extraction and inspection
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── physical.rs       # PhysicalText: exact glyph geometry
│   │   │   ├── logical.rs        # LogicalText: reading-order with confidence
│   │   │   ├── tagged.rs         # TaggedText: structure-tree-driven extraction
│   │   │   ├── search.rs         # SearchIndex, SelectionQuads, HitTest primitives
│   │   │   ├── metadata.rs       # metadata extraction
│   │   │   ├── structure.rs      # structure inspection
│   │   │   ├── asset.rs          # image/font/embedded file extraction
│   │   │   └── diagnostic.rs     # diagnostic report generation
│   │   └── Cargo.toml
│   ├── monkeybee-validate/       # conformance validation
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── arlington.rs      # Arlington-model conformance validation
│   │   │   ├── profile.rs        # profile-specific validation (PDF/A-4, PDF/X-6)
│   │   │   ├── preflight.rs      # write preflight checks
│   │   │   └── signature.rs      # signature byte-range verification
│   │   └── Cargo.toml
│   ├── monkeybee-proof/          # validation and evidence harness
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── corpus.rs         # corpus management and indexing
│   │   │   ├── render_compare.rs # render comparison harness
│   │   │   ├── roundtrip.rs      # round-trip validation harness
│   │   │   ├── ledger.rs         # compatibility ledger
│   │   │   ├── benchmark.rs      # performance benchmarks
│   │   │   ├── fuzz.rs           # fuzz testing coordination
│   │   │   └── evidence.rs       # artifact generation
│   │   └── Cargo.toml
│   └── monkeybee-cli/            # command-line interface
│       ├── src/
│       │   └── main.rs
│       └── Cargo.toml
```

## Crate dependency graph

```
monkeybee-core          (no internal deps — shared primitives)
    ↑
monkeybee-bytes         (depends on: core)
monkeybee-security      (depends on: core)
    ↑
monkeybee-codec         (depends on: core, security)
    ↑
monkeybee-parser        (depends on: core, bytes, codec, security)
    ↑
monkeybee-syntax        (depends on: core, bytes, parser)    ← preservation boundary
    ↑
monkeybee-document      (depends on: core, bytes, syntax)    ← semantic layer built from syntax snapshots
    ↑
monkeybee-content       (depends on: core, document)         ← sink adapters: RenderSink, ExtractSink, InspectSink, EditSink
monkeybee-text          (depends on: core, document, codec)  ← decode pipeline + authoring layout pipeline
    ↑
monkeybee-render        (depends on: core, content, document, text, codec)  ← consumes content events, no own interpreter
monkeybee-compose       (depends on: core, document, text, content)  ← authoring/builders, appearance gen
monkeybee-write         (depends on: core, bytes, document, codec)   ← pure serializer
monkeybee-edit          (depends on: core, document, content, compose, write)
monkeybee-forms         (depends on: core, document, text, compose)
monkeybee-annotate      (depends on: core, document, content, compose, forms, render)
monkeybee-extract       (depends on: core, content, document, text)
monkeybee-validate      (depends on: core, document)
monkeybee-proof         (depends on: core, bytes, codec, security, parser, syntax, document, content, text, render, compose, write, edit, forms, annotate, extract, validate)
monkeybee-cli           (depends on: all above)
```

Note: `monkeybee-syntax` sits between parser and document as the preservation boundary. `monkeybee-compose` sits between edit/annotate/forms and write, owning authoring/builder semantics while write remains a pure serializer.

Note: monkeybee-annotate depends on monkeybee-render for appearance stream generation —
specifically for rendering text and graphics within annotation appearance form XObjects.
The compose crate handles the builder API; render provides the actual glyph/path realization.

Note: monkeybee-proof already lists security in its dependency list. Verified.

### Workspace Cargo.toml structure

```toml
[workspace]
resolver = "2"
members = [
    "crates/monkeybee-core",
    "crates/monkeybee-bytes",
    "crates/monkeybee-codec",
    "crates/monkeybee-security",
    "crates/monkeybee-parser",
    "crates/monkeybee-syntax",
    "crates/monkeybee-document",
    "crates/monkeybee-content",
    "crates/monkeybee-text",
    "crates/monkeybee-render",
    "crates/monkeybee-compose",
    "crates/monkeybee-write",
    "crates/monkeybee-edit",
    "crates/monkeybee-forms",
    "crates/monkeybee-annotate",
    "crates/monkeybee-extract",
    "crates/monkeybee-validate",
    "crates/monkeybee-proof",
    "crates/monkeybee-cli",
]

[workspace.dependencies]
# Shared dependency versions pinned at workspace level
serde = { version = "1", features = ["derive"] }
serde_json = "1"
indexmap = { version = "2", features = ["serde"] }
dashmap = "6"
rayon = "1"
thiserror = "2"
```

### Feature flag strategy

Feature flags control the baseline-vs-experimental lane separation and optional native bindings:

| Flag | Crate | Effect |
|---|---|---|
| `freetype` | monkeybee-text | Enable FreeType backend for font rasterization (default: off) |
| `openjpeg` | monkeybee-codec | Enable OpenJPEG for JPEG 2000 decode (default: on in Compatible) |
| `lcms2` | monkeybee-render | Enable lcms2 for ICC profile evaluation (default: on) |
| `tiny-skia` | monkeybee-render | Enable tiny-skia rasterizer (default: on, baseline) |
| `experimental-raster` | monkeybee-render | Enable exact analytic area coverage rasterizer |
| `experimental-color` | monkeybee-render | Enable spectral-aware color pipeline |
| `experimental-sdf` | monkeybee-render | Enable SDF glyph rendering path |
| `wasm` | workspace | WASM-compatible build: no threads, no mmap, no system fonts |
| `proof` | monkeybee-proof | Enable full proof harness (pulls in all reference renderers) |

Baseline v1 builds with: `tiny-skia`, `lcms2`, `openjpeg` (Compatible profile).
Experimental features are opt-in and must beat the baseline under the proof harness before
becoming default.

## Runtime and concurrency model

### Runtime layering doctrine

Core library crates (`monkeybee-core`, `monkeybee-syntax`, `monkeybee-document`, `monkeybee-content`, `monkeybee-text`, `monkeybee-render`, `monkeybee-compose`, `monkeybee-write`, `monkeybee-edit`, `monkeybee-forms`, `monkeybee-annotate`, `monkeybee-extract`, `monkeybee-validate`) are runtime-agnostic. `ExecutionContext` carries budgets, cancellation, determinism, and providers, but parse/render/write/edit must not require a specific async runtime.

Async orchestration is an adapter concern used by:
- range-backed byte acquisition (`monkeybee-bytes` fetch scheduler)
- proof harness orchestration (`monkeybee-proof`)
- artifact streaming
- external process / oracle coordination

`asupersync` is the default orchestration runtime for CLI and proof, not a semantic dependency of the core engine model. A minimal WASM build is a non-gating proof surface that validates this runtime independence.

### Async orchestration layer

Monkeybee PDF uses `asupersync` as its async runtime and orchestration layer at the CLI/proof edge. Per the upstream `asupersync` skill and runtime guidance, Monkeybee should stay native-first: thread `&Cx<'_>` through async I/O workflows, structure child tasks inside explicit scopes, and bootstrap CLI and proof-harness entrypoints with `RuntimeBuilder` plus `LabRuntime` rather than treating Tokio as the ambient runtime.

Rayon remains the CPU-bound parallel execution layer. The architectural split is deliberate:

- `asupersync` owns async file and directory I/O, corpus traversal, artifact streaming, external-process coordination, cancellation, timeout budgeting, and task supervision.
- Rayon owns page-level rendering fan-out, image and filter transforms, diff computation, extraction batches, compression work, and other bounded in-memory compute kernels.
- CPU-heavy work should be handed off from an enclosing `asupersync` scope to Rayon and then rejoined in that same structured scope for aggregation, diagnostics, and persistence.
- Detached background tasks are not the default. Long-lived background activity must remain runtime-supervised and explicitly owned.
- Tokio compatibility, if ever required for a third-party library, belongs behind a narrow adapter boundary rather than in Monkeybee's core architecture.

## Core data structures

### Engine and session model (`monkeybee-document`)

```rust
/// Top-level engine: owns global policy, caches, and providers.
/// Typically one per process. Thread-safe (Send + Sync).
pub struct MonkeybeeEngine {
    pub config: EngineConfig,
    pub caches: CacheManager,
    pub font_provider: Box<dyn FontProvider>,
    pub color_profile_provider: Box<dyn ColorProfileProvider>,
    pub crypto_provider: Option<Box<dyn CryptoProvider>>,
    pub oracle_provider: Option<Box<dyn OracleProvider>>,
    pub security_profile: SecurityProfile,
}

/// An open document session: binds a byte source to the engine.
/// Created by `engine.open(byte_source, options)`.
pub struct OpenSession {
    pub engine: Arc<MonkeybeeEngine>,
    pub byte_source: Box<dyn ByteSource>,
    pub revision_chain: RevisionChain,
    pub current_snapshot: Arc<PdfSnapshot>,
    pub open_strategy: OpenStrategy,  // eager, lazy, or remote
    pub exec_ctx: ExecutionContext,
}

/// Immutable, shareable document state. Identified by snapshot_id.
/// Send + Sync — safe for concurrent page-parallel operations.
pub struct PdfSnapshot {
    pub snapshot_id: SnapshotId,
    pub document: PdfDocument,
    pub syntax_snapshot: SyntaxSnapshot,  // from monkeybee-syntax
    pub dep_graph: DependencyGraph,
}

pub enum OpenStrategy {
    Eager,   // parse everything available locally
    Lazy,    // resolve objects on demand
    Remote,  // range requests + prefetch planner
}
```

### Execution context (`monkeybee-core::context`)

```rust
/// Carried by every top-level API. Cloneable per-task for parallel operations.
/// Cancellation propagates to all clones.
pub struct ExecutionContext {
    pub budgets: ResourceBudgets,
    pub cancellation: CancellationToken,
    pub deadline: Option<Instant>,
    pub determinism: DeterminismSettings,
    pub diagnostic_sink: Arc<dyn DiagnosticSink>,
    pub trace_sink: Option<Arc<dyn TraceSink>>,
    pub security_profile: SecurityProfile,
    pub cache_overrides: Option<CacheConfig>,  // for proof runs with smaller budgets
}

pub struct ResourceBudgets {
    pub max_objects: u64,              // default: 10_000_000
    pub max_decompressed_bytes: u64,   // default: 1 GiB
    pub max_operators_per_page: u64,   // default: 5_000_000
    pub max_nesting_depth: u32,        // default: 256
    pub max_page_count: u32,           // default: 100_000
}

pub struct DeterminismSettings {
    pub deterministic_output: bool,    // canonical serialization order, stable hashers
    pub pinned_fallback_fonts: bool,   // use pinned font pack instead of system fonts
    pub fixed_thread_count: Option<usize>,  // for reproducible benchmarks
}

/// Cooperative cancellation token — cheaply cloneable, atomically cancellable.
/// Checked at every cancellation checkpoint (per-operator, per-tile, per-page, per-resource).
#[derive(Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}
```

### Security profiles (`monkeybee-security::profile`)

```rust
/// Security profile that controls decoder behavior, resource limits, and isolation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SecurityProfile {
    /// Widest feature coverage. All decoders enabled.
    Compatible,
    /// Risky decoders run in isolated workers with bounded budgets.
    Hardened,
    /// Risky decoders disabled with explicit Tier 3 degradation reporting.
    Strict,
}

/// Policy decision for a specific decoder/feature.
pub enum DecoderPolicy {
    Allow,                          // run directly
    Isolate { budget: DecodeBudget }, // run in isolated worker with budget
    Deny { reason: String },         // disable, report Tier 3 diagnostic
}

pub struct DecodeBudget {
    pub max_output_bytes: u64,       // max decoded size
    pub max_wall_time_ms: u64,       // max wall-clock time
    pub max_memory_bytes: u64,       // max heap usage during decode
}

impl SecurityProfile {
    /// Get the policy for a specific decoder.
    pub fn policy_for(&self, decoder: DecoderType) -> DecoderPolicy { /* ... */ }
}

/// High-risk decoder types gated by the security profile.
pub enum DecoderType {
    JBIG2,
    JPEG2000,
    Type4Calculator,   // PostScript calculator functions
    XfaXmlPacket,
}
```

The codec pipeline checks `exec_ctx.security_profile.policy_for(decoder_type)` before invoking
any high-risk decoder. This is not a suggestion — it is enforced by the type system: the decoder
entry points in `monkeybee-codec` are `pub(crate)`, and the public API routes through
`monkeybee-security` which applies the policy.

### Compatibility ledger (`monkeybee-proof::ledger`)

```rust
/// Machine-readable record for every document processed.
/// Serialized to JSON per the schema in SPEC.md Part 6.
#[derive(Serialize, Deserialize)]
pub struct CompatibilityLedger {
    pub schema_version: String,         // "1.0"
    pub engine_version: String,
    pub timestamp: String,              // ISO 8601
    pub input: InputInfo,
    pub features: Vec<FeatureEntry>,
    pub repairs: Vec<RepairEntry>,
    pub degradations: Vec<DegradationEntry>,
    pub pages: Vec<PageLedger>,
    pub summary: LedgerSummary,
}

#[derive(Serialize, Deserialize)]
pub struct InputInfo {
    pub filename: String,
    pub sha256: String,
    pub declared_version: String,
    pub effective_version: String,
    pub size_bytes: u64,
    pub page_count: u32,
    pub producer: Option<String>,
    pub creator: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FeatureEntry {
    pub code: String,                   // e.g., "transparency.isolated_knockout_group"
    pub tier: u8,                       // 1, 2, or 3
    pub status: String,                 // "supported", "partial", "degraded", "unsupported"
    pub pages: Vec<u32>,
    pub details: String,
}

#[derive(Serialize, Deserialize)]
pub struct RepairEntry {
    pub code: String,
    pub severity: String,
    pub object: Option<String>,         // "42 0" format
    pub original_value: String,
    pub corrected_value: String,
    pub strategy: String,
    pub confidence: f64,
}

#[derive(Serialize, Deserialize)]
pub struct LedgerSummary {
    pub total_features: u32,
    pub tier1_count: u32,
    pub tier2_count: u32,
    pub tier3_count: u32,
    pub repair_count: u32,
    pub degradation_count: u32,
    pub overall_status: String,         // "clean", "repaired", "degraded", "failed"
}
```

### PDF object model (`monkeybee-core::object`)

```rust
/// Fundamental PDF value types
pub enum PdfValue {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    String(PdfString),         // byte string with encoding tracking
    Name(PdfName),             // interned name
    Array(Vec<PdfValue>),
    Dictionary(PdfDictionary),
    Stream(PdfStream),
    Reference(ObjRef),
    Null,
}

/// Object reference (indirect object identity)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjRef {
    pub num: u32,
    pub gen: u16,
}

/// Stream: dictionary + byte-backed handle (not inline Vec<u8>)
/// Decoded bytes live in engine-managed caches, not inline in the object graph.
pub struct PdfStream {
    pub dict: PdfDictionary,
    pub handle: StreamHandle,      // byte-backed source reference (span, range, or inline)
    // Decoded data lives in engine/session-level caches keyed by (snapshot_id, filter_chain)
}

/// Dictionary with insertion-order preservation
pub struct PdfDictionary {
    entries: IndexMap<PdfName, PdfValue>,
}

/// StreamHandle: mediates between raw byte source and consumers that need decoded data.
/// Clone + Send + Sync. Carries no decoded data — only metadata to locate and decode the stream.
pub struct StreamHandle {
    pub object_id: ObjRef,
    pub raw_span: ByteSpan,              // offset + length in the byte source
    pub filter_chain: Vec<FilterSpec>,   // ordered decode filters with parameters
    pub expected_decoded_length: Option<u64>,  // from content dimensions, when known
}
```

### Document model (`monkeybee-document::document`)

```rust
/// Top-level document
pub struct PdfDocument {
    pub objects: ObjectStore,      // all indirect objects
    pub xref: CrossRefTable,       // current cross-reference state
    pub trailer: PdfDictionary,    // trailer dictionary
    pub pages: PageTree,           // resolved page tree
    pub updates: Vec<IncrementalUpdate>,  // update history
    pub metadata: DocumentMetadata,
    pub encryption: Option<EncryptionState>,
    pub diagnostics: DiagnosticLog,
    pub change_journal: ChangeJournal,  // journal-based mutation tracking (ChangeEntry records)
}

/// Object store with reference resolution
pub struct ObjectStore {
    objects: HashMap<ObjRef, PdfValue>,
    // reverse index: which objects reference which
    reverse_refs: HashMap<ObjRef, Vec<ObjRef>>,
}
```

### Page model (`monkeybee-document::page`)

```rust
/// Resolved page (all inherited attributes materialized)
pub struct ResolvedPage {
    pub index: usize,
    pub media_box: Rectangle,
    pub crop_box: Rectangle,
    pub bleed_box: Option<Rectangle>,
    pub trim_box: Option<Rectangle>,
    pub art_box: Option<Rectangle>,
    pub rotate: i32,
    pub user_unit: f64,
    pub resources: ResolvedResources,
    pub contents: Vec<ObjRef>,     // content stream references
    pub annotations: Vec<ObjRef>,
}
```

### Geometry (`monkeybee-core::geometry`)

```rust
/// Affine transformation matrix [a b c d e f]
#[derive(Clone, Copy)]
pub struct Matrix {
    pub a: f64, pub b: f64,
    pub c: f64, pub d: f64,
    pub e: f64, pub f: f64,
}

/// Rectangle in PDF coordinate space
#[derive(Clone, Copy)]
pub struct Rectangle {
    pub ll_x: f64, pub ll_y: f64,  // lower-left
    pub ur_x: f64, pub ur_y: f64,  // upper-right
}

/// Point in PDF coordinate space
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64, pub y: f64,
}
```

### Graphics state (`monkeybee-content::state`)

```rust
/// Full graphics state
pub struct GraphicsState {
    pub ctm: Matrix,
    pub clipping_path: Option<ClipPath>,
    pub color_space_stroke: ColorSpace,
    pub color_space_fill: ColorSpace,
    pub color_stroke: Color,
    pub color_fill: Color,
    pub line_width: f64,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f64,
    pub dash_pattern: DashPattern,
    pub rendering_intent: RenderingIntent,
    pub flatness: f64,
    pub blend_mode: BlendMode,
    pub soft_mask: Option<SoftMask>,
    pub alpha_stroke: f64,
    pub alpha_fill: f64,
    pub overprint_stroke: bool,
    pub overprint_fill: bool,
    pub overprint_mode: i32,
    pub text_state: TextState,
}

/// Text-specific state
pub struct TextState {
    pub font: Option<FontRef>,
    pub font_size: f64,
    pub char_spacing: f64,
    pub word_spacing: f64,
    pub horizontal_scaling: f64,
    pub leading: f64,
    pub rise: f64,
    pub render_mode: TextRenderMode,
    pub text_matrix: Matrix,
    pub line_matrix: Matrix,
}
```

### Change tracking (`monkeybee-document::transaction`)

```rust
/// Journal-based change tracking (replaces HashSet-based ChangeTracker)
pub struct ChangeJournal {
    pub entries: Vec<ChangeEntry>,
}

/// Each mutation is a structured change entry with full context
pub struct ChangeEntry {
    pub object_id: ObjRef,
    pub old_fingerprint: Option<u64>,   // hash of previous value
    pub new_value: Option<PdfValue>,     // None for deletions
    pub reason: ChangeReason,            // why this change was made
    pub ownership_before: OwnershipClass,
    pub ownership_after: OwnershipClass,
    pub dependency_delta: DependencyDelta, // what refs were added/removed
}

/// WritePlan computed before any save operation
pub struct WritePlan {
    pub classifications: Vec<ObjectClassification>,
}

pub enum ObjectAction {
    PreserveBytes,           // emit original bytes verbatim
    AppendOnly,              // incremental append only
    RewriteOwned,            // semantically understood, safe to rewrite
    RegenerateAppearance,    // appearance stream needs regeneration
    RequiresFullRewrite,     // cannot be incrementally saved
    Unsupported,             // cannot be safely serialized
}

/// Object ownership classification — determines how the write path handles each object.
pub enum OwnershipClass {
    /// Semantically understood by the engine. Eligible for rewrite/canonicalization.
    Owned,
    /// Not semantically understood but carried forward byte-preservingly.
    /// Incremental-append preserves original bytes. Full-rewrite copies verbatim.
    ForeignPreserved,
    /// Detected but not safely transformable. Incompatible edits fail explicitly.
    OpaqueUnsupported,
}

/// Why a change was made — enables undo, audit, and save-impact explanation.
pub enum ChangeReason {
    UserEdit,                // explicit user/API edit
    AppearanceRegeneration,  // triggered by field value change
    ResourceGC,              // unreachable object removal
    Deduplication,           // identical object merge
    Optimization,            // compaction, recompression
    AnnotationAdd,           // annotation creation
    AnnotationFlatten,       // annotation burned into page content
    RedactionApply,          // redaction application
    PageMutation,            // page add/remove/reorder
    MetadataUpdate,          // metadata modification
}
```

### PagePlan IR (`monkeybee-content::pageplan`)

```rust
/// Immutable page-scoped display list. Cacheable, shareable, region-queryable.
/// Produced by interpreting a page's content stream(s) through the graphics state machine.
pub struct PagePlan {
    pub page_index: usize,
    pub media_box: Rectangle,
    pub crop_box: Rectangle,
    pub ops: Vec<DrawOp>,              // normalized draw operations in page order
    pub text_runs: Vec<TextRun>,       // text with positions and Unicode
    pub resource_deps: HashSet<ObjRef>, // all objects this page depends on
    pub marked_spans: Vec<MarkedSpan>, // marked content regions
    pub degradations: Vec<DegradationNote>, // any operator-level errors/degradations
    pub provenance: Vec<SourceSpan>,   // byte offsets in content stream for each op
}

/// A normalized draw operation.
pub enum DrawOp {
    FillPath { path: Path, rule: FillRule, color: ResolvedColor, state: DrawState },
    StrokePath { path: Path, color: ResolvedColor, stroke: StrokeParams, state: DrawState },
    ClipPath { path: Path, rule: FillRule },
    DrawImage { image_ref: ObjRef, rect: Rectangle, state: DrawState },
    DrawInlineImage { data: Arc<[u8]>, params: ImageParams, rect: Rectangle, state: DrawState },
    BeginGroup { isolated: bool, knockout: bool, blend_mode: BlendMode, soft_mask: Option<SoftMaskRef> },
    EndGroup,
    BeginMarkedContent { tag: String, properties: Option<ObjRef> },
    EndMarkedContent,
}

/// A positioned text run with resolved Unicode and glyph info.
pub struct TextRun {
    pub glyphs: Vec<PositionedGlyph>,
    pub unicode: String,               // decoded Unicode text for this run
    pub font_ref: ObjRef,
    pub font_size: f64,
    pub render_mode: TextRenderMode,
    pub color: ResolvedColor,
    pub state: DrawState,
}

pub struct PositionedGlyph {
    pub glyph_id: u32,
    pub unicode: Option<char>,
    pub position: Point,               // in page space (after full transform chain)
    pub advance: f64,
    pub quad: [Point; 4],              // bounding quad for hit-testing/selection
}

/// Shared draw state snapshot (subset of GraphicsState relevant to rendering).
pub struct DrawState {
    pub ctm: Matrix,
    pub alpha: f64,
    pub blend_mode: BlendMode,
    pub overprint: bool,
    pub overprint_mode: i32,
}
```

### Error taxonomy (`monkeybee-core::error`)

```rust
pub enum MonkeybeeError {
    Parse(ParseError),
    Semantic(SemanticError),
    Render(RenderError),
    Write(WriteError),
    RoundTrip(RoundTripError),
    Compatibility(CompatibilityNote),
}

/// Every error carries context
pub struct ErrorContext {
    pub subsystem: Subsystem,
    pub object_ref: Option<ObjRef>,
    pub page: Option<usize>,
    pub description: String,
    pub severity: Severity,
    pub tier: Option<CompatibilityTier>,
}
```

### Diagnostic streaming (`monkeybee-core::diagnostics`)

```rust
/// Unified diagnostic type emitted by all subsystems
pub struct Diagnostic {
    pub code: String,                   // hierarchical, e.g., "parse.xref.wrong_offset"
    pub severity: Severity,             // Fatal, Error, Warning, Info
    pub subsystem: Subsystem,           // parser, renderer, writer, etc.
    pub object_ref: Option<ObjRef>,
    pub page: Option<usize>,
    pub byte_offset: Option<u64>,
    pub message: String,
    pub payload: Option<DiagnosticPayload>, // machine-readable repair details, feature classification, etc.
}

/// Trait for receiving diagnostics — the unified streaming interface
pub trait DiagnosticSink: Send + Sync {
    fn emit(&self, diagnostic: Diagnostic);
}

/// Collects all diagnostics into a Vec (default for library use)
pub struct VecSink { /* ... */ }

/// Invokes a user-provided closure per diagnostic (for real-time display)
pub struct CallbackSink<F: Fn(Diagnostic) + Send + Sync> { /* ... */ }

/// Wraps another sink and filters by severity, subsystem, or error code
pub struct FilteringSink<S: DiagnosticSink> { /* ... */ }

/// Wraps another sink and counts diagnostics by category (for budget enforcement)
pub struct CountingSink<S: DiagnosticSink> { /* ... */ }
```

### PDF version tracking (`monkeybee-core::version`)

```rust
/// PDF version (major.minor)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdfVersion {
    pub major: u8,  // 1 or 2
    pub minor: u8,  // 0–9
}

/// Version tracking for a document session
pub struct VersionInfo {
    pub input_version: PdfVersion,         // from file header (%PDF-X.Y)
    pub catalog_version: Option<PdfVersion>, // from catalog /Version entry (takes precedence when higher)
    pub effective_version: PdfVersion,     // minimum version needed for all features actually used
    pub output_version: Option<PdfVersion>, // user-requested output version constraint
}

/// Feature-to-version association for version gating
pub struct VersionedFeature {
    pub code: String,           // e.g., "object_streams", "cross_ref_streams", "aes_encryption"
    pub min_version: PdfVersion,
}
```

### Cache management (`monkeybee-document::cache`)

```rust
/// Configuration for engine-level caches
pub struct CacheConfig {
    pub decoded_stream_budget: usize,  // bytes, default 256 MB
    pub font_cache_budget: usize,      // bytes, default 128 MB
    pub page_plan_budget: usize,       // bytes, default 64 MB
    pub raster_tile_budget: usize,     // bytes, default 512 MB
}

/// Engine-level cache manager — owns all bounded caches
/// All cache data structures are thread-safe (DashMap / sharded concurrent maps).
pub struct CacheManager {
    pub config: CacheConfig,
    pub decoded_streams: DashMap<(SnapshotId, ObjRef, u64), Arc<[u8]>>,
    pub fonts: DashMap<ObjRef, Arc<ParsedFont>>,
    pub page_plans: DashMap<(SnapshotId, usize), Arc<PagePlan>>,
    pub raster_tiles: DashMap<(SnapshotId, usize, TileId, u32), Arc<TileData>>,
}

/// Cache statistics for diagnostics and proof
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub current_bytes: usize,
    pub budget_bytes: usize,
}
```

### Content stream rewriter (`monkeybee-edit::rewriter`)

```rust
/// Filter-and-rewrite pipeline for content stream edits (redaction, flattening, content removal)
pub struct ContentStreamRewriter {
    pub filters: Vec<Box<dyn OperatorFilter>>,
    pub injections: Vec<Injection>,
}

/// Per-operator decision: keep, drop, or replace
pub trait OperatorFilter: Send + Sync {
    fn filter(&self, op: &Operator, state: &GraphicsState) -> FilterDecision;
}

pub enum FilterDecision {
    Keep,
    Drop,
    Replace(Vec<Operator>),
}

/// New operators to inject at a specified position
pub struct Injection {
    pub position: InjectionPoint,  // before/after a specific operator index, or append
    pub operators: Vec<Operator>,
    pub wrap_in_save_restore: bool, // wrap in q/Q to isolate state
}
```

### Progressive render state (`monkeybee-render::progressive`)

```rust
/// Tracks progressive rendering state for partially-loaded documents
pub struct ProgressiveRenderState {
    pub page_index: usize,
    pub available_resources: HashSet<ObjRef>,
    pub missing_resources: Vec<MissingResource>,
    pub completeness: f32,  // 0.0 = nothing rendered, 1.0 = fully complete
}

/// A resource that is not yet available for rendering
pub struct MissingResource {
    pub obj_ref: ObjRef,
    pub resource_type: ResourceType,  // Image, Font, XObject, etc.
    pub byte_range: Option<(u64, u64)>, // byte range needed to fetch this resource
    pub affected_tiles: Vec<TileId>,
}

/// Placeholder metadata attached to partially-rendered tiles
pub struct PlaceholderInfo {
    pub bbox: Rectangle,
    pub missing_resource: ObjRef,
    pub fetch_range: Option<(u64, u64)>,
}
```

### Dependency graph (`monkeybee-document::depgraph`)

```rust
/// Edge type classification for the dependency graph
pub enum EdgeType {
    ContentRef,    // page/form XObject content stream references a resource by name
    DictRef,       // dictionary value is an indirect reference
    ArrayRef,      // array element is an indirect reference
    InheritedRef,  // page inherits an attribute from an ancestor
}

/// Dependency graph: DAG mapping objects to their transitive dependencies.
/// Computed lazily, cached per snapshot, stored as adjacency lists in DashMap.
pub struct DependencyGraph {
    pub forward: DashMap<ObjRef, Vec<(ObjRef, EdgeType)>>,  // A -> [B, C, ...]
    pub reverse: DashMap<ObjRef, Vec<(ObjRef, EdgeType)>>,  // B -> [A, ...]
    pub snapshot_id: SnapshotId,
}

/// Result of edit_impact query
pub struct EditImpact {
    pub affected_pages: Vec<usize>,
    pub invalidated_caches: Vec<CacheKey>,
    pub regeneration_needed: Vec<ObjRef>,
}
```

### Fetch scheduler (`monkeybee-bytes::fetch`)

```rust
/// Fetch scheduler: mediates byte-range requests for remote or lazy byte sources.
pub trait FetchScheduler: Send + Sync {
    /// Request bytes in the given range. Returns a future that resolves when available.
    fn request_range(&self, offset: u64, length: u64) -> FetchHandle;

    /// Submit a prefetch plan (ordered list of ranges by priority).
    fn submit_prefetch(&self, plan: PrefetchPlan);

    /// Cancel all outstanding requests.
    fn cancel_all(&self);

    /// Report fetch statistics (requests issued, bytes fetched, latencies).
    fn statistics(&self) -> FetchStatistics;
}

/// Ordered list of (offset, length, priority) tuples for prefetch planning.
pub struct PrefetchPlan {
    pub ranges: Vec<(u64, u64, u32)>,  // (offset, length, priority)
}

pub struct FetchStatistics {
    pub requests_issued: u64,
    pub bytes_fetched: u64,
    pub avg_latency_ms: f64,
}
```

### WritePlan classification (`monkeybee-write::plan`)

```rust
/// Classification of each object for the write plan (extended from ObjectAction).
/// See SPEC.md Part 4 for full classification rules and signature impact analysis.
pub enum WritePlanClassification {
    PreserveBytes,           // unmodified + ForeignPreserved + incremental-append
    AppendOnly,              // new object, incremental-append only
    RewriteOwned,            // modified + Owned, re-serialize from semantic repr
    RegenerateAppearance,    // widget annotation needing appearance regeneration
    RequiresFullRewrite,     // cannot be incrementally saved (deleted, opaque+modified, structural)
    Unsupported,             // OpaqueUnsupported + unmodified, copy verbatim or leave untouched
}

/// Signature impact report produced by WritePlan computation.
pub struct SignatureImpact {
    pub signatures: Vec<SignatureStatus>,
}

pub struct SignatureStatus {
    pub signature_ref: ObjRef,
    pub byte_range: Vec<(u64, u64)>,
    pub invalidated: bool,
    pub reason: Option<String>,
}
```

### Provider traits (`monkeybee-core::traits`)

```rust
/// CryptoProvider: extension point for signature verification and digest computation.
pub trait CryptoProvider: Send + Sync {
    fn verify_cms_signature(
        &self,
        signed_bytes: &[u8],
        signature_der: &[u8],
    ) -> Result<SignatureVerification>;

    fn verify_timestamp(
        &self,
        tst_der: &[u8],
    ) -> Result<TimestampVerification>;

    fn digest(&self, algorithm: DigestAlgorithm, data: &[u8]) -> Vec<u8>;
}

/// OracleProvider: deterministic resource resolution for CI/proof reproducibility.
pub trait OracleProvider: Send + Sync {
    fn resolve(&self, key: &OracleKey) -> Option<Arc<[u8]>>;
    fn manifest(&self) -> OracleManifest;
}
```

### Test obligation matrix reference

The SPEC.md Part 8 defines a test obligation matrix with per-crate pass thresholds and metrics
for 14 gated test classes (xref-repair, font-fallback, transparency-compositing, producer-quirks,
incremental-update, encryption, annotation-roundtrip, page-mutation, generation, adversarial,
color-space, content-stream-stress, signature-preserve, redaction-safety). Each class has a
defined primary crate, pass threshold, and metric. See SPEC.md Part 8 "Test obligation matrix"
for the full table. The regression policy requires that any previously-passing test class that
fails in a new CI run is a blocking regression.

## Critical data flows

### Runtime orchestration flow

```
CLI / proof / library workflow
  -> RuntimeBuilder bootstrap
  -> LabRuntime entry
  -> Scope owns request region and cancellation budget
  -> asupersync performs file and artifact I/O, scheduling, and supervision
  -> Rayon executes CPU-bound kernels over in-memory page/document data
  -> asupersync aggregates results, emits diagnostics, writes artifacts, and closes the region
```

### Parse flow

```
PDF bytes
  → Lexer (tokenize)
  → Object parser (construct PdfValue tree)
  → XRef parser (build cross-reference table, repair if needed)
  → Encryption handler (decrypt if needed)
  → Syntax layer (monkeybee-syntax: immutable COS objects, provenance, repair records)
  → Document builder (construct PdfDocument from syntax snapshots with ObjectStore, PageTree, etc.)
  → Diagnostic log (record all warnings, repairs, compatibility notes)
```

### Render flow

```
PdfDocument + page index
  → ResolvedPage (materialize inherited attributes)
  → Content stream(s) (decode, concatenate if multiple)
  → Content stream interpreter in monkeybee-content (single implementation)
    → Events or PagePlan IR dispatched through RenderSink adapter
    → Text operations → Font decode pipeline → Glyph positions → Backend
    → Path operations → Path builder → Stroke/Fill → Backend
    → Image operations → Image decoder → Color conversion → Backend
    → Transparency → Compositing engine → Backend
  → Tile/band scheduler materializes full page or requested region
  → Backend produces output (raster via tile sink, SVG elements, etc.)
```

### Write flow

```
PdfDocument (with ChangeJournal)
  → WritePlan computation (classify each touched object: PreserveBytes/AppendOnly/RewriteOwned/etc.)
  → WritePlan surfaced to API/CLI and compatibility ledger
  → Mode selection (full rewrite vs. incremental append, informed by WritePlan)
  → Object serializer (PdfValue → bytes)
  → Stream encoder (apply compression filters)
  → XRef writer (build new xref table/stream)
  → Trailer writer
  → Output assembler (concatenate header + body + xref + trailer)
  → Self-validation (parse the output, verify structural correctness)
```

### Annotation round-trip flow

```
PdfDocument
  → Load existing annotations (resolve from page annotation arrays)
  → Create new annotation (type, geometry via shared pipeline, content)
  → Generate appearance stream (via monkeybee-render primitives)
  → Insert into document model (update page annotations, add objects)
  → Track change (ChangeJournal)
  → Write (incremental or full rewrite)
  → Reload and validate (annotations present, geometry preserved, content intact)
```

## External dependency strategy

### Planned dependencies (subject to evaluation)

- **`flate2`** — DEFLATE compression/decompression (FlateDecode)
- **`image`** — image decoding (JPEG, PNG, TIFF baseline)
- **`jpeg-decoder`** — DCTDecode
- **`openjpeg-sys` or `jpeg2k`** — JPXDecode (JPEG 2000)
- **`freetype-rs` or `ttf-parser` + `ab_glyph`** — font parsing and glyph rasterization
- **`indexmap`** — ordered dictionaries
- **`once_cell` / `std::sync::OnceLock`** — lazy initialization
- **`asupersync`** — async runtime, structured concurrency, cancellation, and orchestration
- **`rayon`** — CPU-bound parallelism composed under `asupersync` orchestration
- **`clap`** — CLI argument parsing
- **`serde` + `serde_json`** — structured output, compatibility ledger
- **`sha2` / `md5`** — PDF encryption handlers
- **`aes`** — AES encryption for PDF security handlers
- **`rc4`** — RC4 encryption for legacy security handlers
- **`miniz_oxide`** — alternative pure-Rust DEFLATE

### Dependency principles

- Prefer pure-Rust where quality and performance are comparable.
- Accept C/C++ bindings only for capabilities not yet available in pure Rust at required quality (e.g., JPEG 2000, complex font shaping).
- Pin all dependency versions. Audit for `unsafe` in critical-path dependencies.
- Core library crates are runtime-agnostic. `asupersync` is the CLI/proof default orchestration runtime, not a semantic dependency of the core engine model. Any async compatibility layer must stay quarantined at the edge.
- No dependency may introduce undefined behavior or memory unsafety that escapes its abstraction boundary.

## Test obligations by crate

### monkeybee-core
- Unit tests: object type creation, geometry transforms, matrix operations.
- Property tests: arbitrary object construction → serialize → deserialize → compare.
- DiagnosticSink tests: VecSink collects all diagnostics, FilteringSink filters by severity/subsystem, CountingSink counts correctly and enforces budget abort policies.
- PdfVersion tests: version parsing, comparison ordering, version-gated feature lookup, catalog version override precedence.

### monkeybee-bytes
- Unit tests: ByteSource implementations (mmap, in-memory), revision chain construction, span tracking.
- Property tests: span ownership invariants preserved across revision appends.

### monkeybee-codec
- Unit tests: each filter implementation (FlateDecode, LZW, ASCII85, etc.) on known input/output pairs.
- Property tests: encode → decode round-trip identity for all filters.
- Fuzz tests: arbitrary bytes through each decoder — no panics, bounded memory.
- Predictor tests: PNG and TIFF predictor logic on known image data.
- Pipeline tests: cascaded filter chains, including reversed-order recovery.

### monkeybee-security
- Unit tests: security profile selection, budget enforcement, allow/deny policy.
- Integration tests: risky decoder invocation through security gate — verify budgets enforced and isolation works.
- Property tests: no decoder invocation bypasses the security boundary.

### monkeybee-syntax
- Unit tests: COS object construction from parser output, provenance round-trip (source spans preserved).
- Property tests: immutability invariant (syntax objects cannot be mutated after construction).
- Preservation tests: raw formatting retention (whitespace, comments survive round-trip via syntax layer).
- Repair record tests: repair records faithfully capture strategy, confidence, and alternatives.
- Object-stream membership tests: objects correctly track their object-stream provenance.
- Xref provenance tests: original vs repaired xref entries are distinguishable.

### monkeybee-document
- Unit tests: document model construction from syntax snapshots, page tree inheritance, resource resolution, reference integrity.
- Property tests: ownership classification consistency, EditTransaction commit/rollback semantics.
- Invariant tests: change journal consistency, reverse reference index accuracy.
- Dependency graph tests: invalidation correctness — edit an object, verify only dependents invalidated.
- Snapshot tests: PdfSnapshot immutability, snapshot_id uniqueness, cache keying correctness, structural sharing (new snapshot does not clone full object store).
- Cache management tests: LRU eviction under budget pressure, pinned-entry protection during active operations, cache stats accuracy, graceful degradation when all budgets exceeded (re-decode on demand), concurrent access correctness under DashMap/sharded structures.
- Thread-safety tests: parallel page renders on shared PdfSnapshot, concurrent decode cache access, atomic budget counter correctness, Rayon scoped parallelism lifetime safety.

### monkeybee-content
- Unit tests: content stream interpretation, graphics state machine, event dispatch.
- Sink adapter tests: RenderSink, ExtractSink, InspectSink, EditSink receive correct events for known content streams.
- Property tests: PagePlan IR equivalence with streaming events (same content stream, same results).
- Cache tests: PagePlan cache invalidation on content/resource changes.
- Error recovery tests: operator-level isolation (failing operator does not abort page), state rollback on partial failure, resource resolution failure handling (missing font/image/XObject), inline image recovery (corrupted BI/ID/EI scanning), Q stack underflow recovery (reset to initial state), recursion limit enforcement (form XObject / tiling pattern nesting at 28 levels).

### monkeybee-parser
- Unit tests: lexer on known token sequences, object parsing on all types, xref parsing on well-formed and malformed tables.
- Corpus tests: parse every file in the pathological corpus, verify no panics, collect diagnostics.
- Fuzz tests: random bytes → parser → no panics, no UB, bounded memory.
- Repair tests: known malformed inputs → verify repair produces usable output.

### monkeybee-text
- Unit tests: font program parsing (Type 1, TrueType, CFF, CIDFont, Type 3), CMap parsing, ToUnicode resolution.
- Decode pipeline tests: char code -> font/CMap -> CID/glyph -> Unicode/metrics for each font type; verify existing PDF text is decoded, not re-shaped.
- Authoring pipeline tests: Unicode -> shaping/bidi/line breaking/font fallback -> positioned glyph runs.
- Unicode fallback chain tests: known fonts with broken/missing ToUnicode — verify fallback produces correct mappings.
- Shaping/bidi tests: complex scripts (Arabic, Hebrew, Devanagari), ligatures, bidi reordering (authoring pipeline only).
- Subsetting tests: subset → re-embed → verify glyph coverage and metrics round-trip.
- Search/hit-test tests: known text at known positions — verify search finds it, hit-test returns correct quads.

### monkeybee-render
- Unit tests: backend drawing operations, color space conversions, tile/band scheduling.
- Render comparison tests: render corpus documents → compare against reference renderers.
- Visual regression tests: golden-image comparisons with perceptual diff thresholds.
- Edge case tests: transparency stacking, pattern rendering, Type 3 fonts, unusual blend modes.
- Cooperative cancellation tests: cancel mid-render at each checkpoint type (per-operator, per-tile, per-page, per-resource); verify partial results are usable and not corrupted; verify budget exhaustion triggers same behavior as cancellation.
- Progressive rendering tests: render with missing resources produces correct placeholders, placeholder metadata carries correct byte ranges, incremental refinement replaces only affected tiles, completeness flag transitions correctly, prefetch planning reports correct resource sets.

### monkeybee-compose
- Unit tests: document/page/content builder APIs, resource naming uniqueness, appearance stream generation.
- Integration tests: compose a document → serialize via monkeybee-write → parse → verify structure.
- Appearance tests: annotation and widget appearance generation produces valid form XObjects.
- Font embedding planning tests: subsetting requests match actual glyph usage.
- Text emission tests: authoring layout pipeline produces correct positioned glyph runs.

### monkeybee-write
- Unit tests: object serialization for all types, xref generation, stream encoding.
- WritePlan tests: classification correctness (PreserveBytes/AppendOnly/RewriteOwned/etc.) on known document states.
- Round-trip tests: parse → write → re-parse → compare object graphs.
- Self-consistency tests: write output → parse with monkeybee-parser → verify structural validity.
- Reference validation: write output → open in PDFium/MuPDF → verify renders correctly.

### monkeybee-edit
- Unit tests: EditTransaction commit/rollback, resource GC, deduplication.
- Redaction tests: text-only, image-only, mixed, reused XObjects, canary-text leakage.
- Optimization tests: compaction produces smaller valid output, recompression round-trips.
- Content stream rewrite tests: parse-filter-reemit round-trip preserves unfiltered operators exactly, operator drop removes target operators and old stream is deleted from change journal, operator replace substitutes correctly with full graphics state context, injection inserts at correct positions with q/Q wrapping, annotation flattening appends appearance stream with correct coordinate transform, TJ array splitting for partial-overlap redaction.

### monkeybee-forms
- Unit tests: field tree parsing, inheritance resolution, field value model for each type.
- Appearance regeneration tests: change field value → regenerate appearance → verify rendered appearance matches value.
- Round-trip tests: fill form → save → reload → verify field values and appearances preserved.
- Signature-field tests: incremental-append after form fill preserves signed byte ranges.
- Calculation order tests: detection and preservation of calculation order across round-trips.

### monkeybee-annotate
- Unit tests: annotation creation, geometry calculations, appearance stream generation.
- Round-trip tests: annotate → save → reload → verify annotations preserved.
- Integration tests: annotate corpus documents → save → open in reference viewers.

### monkeybee-validate
- Unit tests: Arlington-model rules against known valid/invalid objects.
- Profile tests: PDF/A-4, PDF/X-6 constraint checking on known-conforming documents.
- Preflight tests: write preflight catches structural errors before serialization.
- Signature tests: byte-range verification on signed documents.

### monkeybee-extract
- Unit tests: text extraction on known documents with ground-truth positions.
- Multi-surface tests: PhysicalText matches exact glyph geometry, LogicalText produces correct reading order with confidence, TaggedText uses structure tree when present.
- Search/hit-test tests: SearchIndex finds known text, SelectionQuads returns correct regions, HitTest resolves correct characters.
- Metadata tests: extraction accuracy on documents with known metadata.
- Coverage tests: extraction runs on entire corpus without panics.

### monkeybee-proof
- Integration tests: full proof harness runs on subset of corpus.
- Ledger tests: compatibility ledger correctly categorizes known feature encounters.
- Evidence tests: artifact generation produces valid, parseable output.
- Ledger JSON schema tests: ledger output validates against the JSON schema (schema_version, input block, features array, repairs array, degradations array, summary block), version tracking fields (declared_version, effective_version) are populated correctly, schema versioning — breaking changes increment major version.

## Subordinate implementation docs

Each of the following should be authored as the spec matures. They are design-to-code contracts for their respective subsystems:

- `docs/implementation/document-model.md` — core object model, object store, reference resolution, dependency graph, snapshots, structural sharing
- `docs/implementation/syntax-layer.md` — COS object representation, provenance model, preservation boundary contract, repair record schema
- `docs/implementation/parser-and-repair.md` — parser architecture, repair strategies, tolerant mode
- `docs/implementation/codec.md` — filter chains, image decode/encode, bounded pipelines, decode telemetry
- `docs/implementation/security.md` — security profiles, budget broker, worker isolation, hostile-input policy
- `docs/implementation/text.md` — font programs, CMaps, Unicode mapping, decode pipeline, authoring layout pipeline, subsetting, search/hit-test
- `docs/implementation/rendering.md` — render pipeline via content sink adapters, output backends, tile/band surface, region/thumbnail render
- `docs/implementation/forms.md` — AcroForm field tree, value model, appearance regeneration, widget bridge, signature helpers
- `docs/implementation/annotation.md` — annotation model, placement, appearance, flattening
- `docs/implementation/compose.md` — document/page builders, resource naming, appearance generation, font embedding planning
- `docs/implementation/writeback.md` — serialization, save modes, WritePlan computation, structural validation
- `docs/implementation/extraction.md` — multi-surface text extraction, search primitives, metadata, diagnostics

## Resolved design decisions

1. **Font rasterization strategy**: `ttf-parser` (pure Rust) for font table parsing; `ab_glyph_rasterizer`
   for glyph rasterization in the baseline. `freetype-rs` available as an optional feature-flagged
   backend behind a `FontRasterizer` trait for environments that need hinting. The trait allows
   switching without touching render code. v1 ships with the pure-Rust path as default.

2. **JPEG 2000**: Accept `openjpeg-sys` C binding for v1 behind `monkeybee-security` isolation
   (the decoder runs in a budget-bounded worker per the security profile). Pure-Rust `jpeg2k` is
   the planned replacement once it reaches decode-correctness parity. JPXDecode is gated by the
   security profile: `Hardened` mode isolates it, `Strict` mode disables it with Tier 3 reporting.

3. **Rendering backend**: `tiny-skia` for the baseline CPU rasterizer (proven, pure Rust, supports
   all needed operations: paths, anti-aliasing, compositing, clipping). The experimental exact
   analytic area coverage rasterizer (B-ALIEN-001) replaces `tiny-skia`'s anti-aliasing path only
   after it beats the baseline under the proof harness. The `RenderBackend` trait allows both to
   coexist.

4. **Color management**: ICC profile support is v1-gating for ICCBased color spaces (they appear
   in the majority of real-world PDFs). Use `lcms2` (C binding) for v1 ICC profile evaluation,
   behind a `ColorProfileProvider` trait. The experimental spectral-aware pipeline (B-ALIEN-003)
   is an alternative implementation of the same trait. CalRGB/CalGray/Lab conversions are
   implemented directly (no library needed — the math is straightforward).

5. **Incremental save granularity**: Byte-range preservation for signature-safe workflows is
   v1-gating per SPEC.md Part 8 and the WritePlan classification rules. The preserve-mode write
   path and signature impact analysis are baseline v1 features, not deferred.

6. **CMap handling**: Custom CMap parser. The CMap format is simple enough (begincodespacerange,
   beginbfchar, beginbfrange, usecmap) that a purpose-built parser is smaller, faster, and more
   controllable than adapting a general-purpose library. Ship with embedded Adobe CJK CMap data
   for the four standard CIDSystemInfo registries (Adobe-Japan1, Adobe-CNS1, Adobe-GB1,
   Adobe-Korea1). CMap data is lazily loaded to keep the WASM binary under the 5 MiB target.

7. **Performance targets**: Quantitative targets are defined in SPEC.md Part 7 (Performance
   doctrine). Summary: first-page render <50ms at 150 DPI (latency class), sustained 10+
   pages/sec at 150 DPI with parallelism (throughput class), no operation >10x expected time for
   content size (stress class), peak memory <5x file size for typical docs (memory class).
