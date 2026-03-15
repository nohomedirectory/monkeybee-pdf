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
│   │   │   └── traits.rs         # ByteSource, FontProvider, ColorProfileProvider, CryptoProvider
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
│   │   │   └── transaction.rs    # EditTransaction, change tracking, snapshot-in/snapshot-out
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
monkeybee-annotate      (depends on: core, document, content, compose, forms)
monkeybee-extract       (depends on: core, content, document, text)
monkeybee-validate      (depends on: core, document)
monkeybee-proof         (depends on: core, bytes, codec, security, parser, syntax, document, content, text, render, compose, write, edit, forms, annotate, extract, validate)
monkeybee-cli           (depends on: all above)
```

Note: `monkeybee-syntax` sits between parser and document as the preservation boundary. `monkeybee-compose` sits between edit/annotate/forms and write, owning authoring/builder semantics while write remains a pure serializer.

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

### monkeybee-content
- Unit tests: content stream interpretation, graphics state machine, event dispatch.
- Sink adapter tests: RenderSink, ExtractSink, InspectSink, EditSink receive correct events for known content streams.
- Property tests: PagePlan IR equivalence with streaming events (same content stream, same results).
- Cache tests: PagePlan cache invalidation on content/resource changes.

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

## Open questions

1. **Font rasterization strategy**: Use `freetype-rs` (C binding, battle-tested) or `ab_glyph` + `ttf-parser` (pure Rust, less mature)? Or a switchable backend?
2. **JPEG 2000**: Accept `openjpeg-sys` C binding, or wait for pure-Rust `jpeg2k` to mature?
3. **Rendering backend**: Start with a simple CPU rasterizer (tiny-skia?), or target a more capable backend from the start?
4. **Color management**: Full ICC profile support from day one, or defer to a later phase?
5. **Incremental save granularity**: Support byte-range preservation for signature-safe workflows in v1, or defer?
6. **CMap handling**: Build custom CMap parser, or leverage an existing library?
7. **Performance target**: Define specific throughput targets (pages/sec on benchmark docs) or keep qualitative ("obviously fast")?
