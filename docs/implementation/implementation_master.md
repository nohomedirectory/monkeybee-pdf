# Implementation Master — Monkeybee PDF

## Purpose

This document is the APR-facing implementation reference for Monkeybee PDF. It summarizes the crate topology, module boundaries, core data structures, data flows, cross-crate dependencies, and test obligations. It links to subordinate implementation docs for deeper subsystem design.

This is not a philosophical essay and not the full codebase. It is the grounding surface that keeps the SPEC honest about implementation realities.

## Workspace topology

```
monkeybee-pdf/
├── Cargo.toml                    # workspace root
├── crates/
│   ├── monkeybee-core/           # document model, object graph, page tree, geometry
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── object.rs         # PDF object types
│   │   │   ├── xref.rs           # cross-reference management
│   │   │   ├── document.rs       # document-level model
│   │   │   ├── page.rs           # page tree, inheritance
│   │   │   ├── resource.rs       # resource resolution
│   │   │   ├── geometry.rs       # coordinate transforms, matrices
│   │   │   ├── update.rs         # incremental update tracking
│   │   │   └── error.rs          # shared error taxonomy
│   │   └── Cargo.toml
│   ├── monkeybee-parser/         # PDF byte parsing, repair, decryption
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── lexer.rs          # tokenization
│   │   │   ├── object_parser.rs  # object parsing
│   │   │   ├── xref_parser.rs    # xref table/stream parsing + repair
│   │   │   ├── stream.rs         # stream decompression, filter chains
│   │   │   ├── content.rs        # content stream parsing
│   │   │   ├── crypt.rs          # encryption/decryption
│   │   │   ├── repair.rs         # tolerant mode, recovery strategies
│   │   │   └── diagnostics.rs    # parser diagnostics
│   │   └── Cargo.toml
│   ├── monkeybee-render/         # page rendering
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── interpreter.rs    # content stream interpreter
│   │   │   ├── state.rs          # graphics state machine
│   │   │   ├── text.rs           # text rendering pipeline
│   │   │   ├── font.rs           # font handling
│   │   │   ├── image.rs          # image rendering
│   │   │   ├── color.rs          # color space management
│   │   │   ├── path.rs           # vector path rendering
│   │   │   ├── transparency.rs   # transparency compositing
│   │   │   ├── pattern.rs        # tiling and shading patterns
│   │   │   ├── page.rs           # page assembly
│   │   │   └── backend/          # output backends (raster, svg)
│   │   └── Cargo.toml
│   ├── monkeybee-write/          # serialization, generation, save
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── serialize.rs      # object serialization
│   │   │   ├── xref_writer.rs    # xref generation
│   │   │   ├── stream_encode.rs  # stream compression
│   │   │   ├── rewrite.rs        # full document rewrite
│   │   │   ├── incremental.rs    # incremental append save
│   │   │   ├── content_gen.rs    # content stream generation
│   │   │   └── validate.rs       # output structural validation
│   │   └── Cargo.toml
│   ├── monkeybee-annotate/       # annotation operations
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── model.rs          # annotation type model
│   │   │   ├── placement.rs      # geometry-aware placement
│   │   │   ├── appearance.rs     # appearance stream generation
│   │   │   ├── flatten.rs        # annotation flattening
│   │   │   └── roundtrip.rs      # round-trip validation helpers
│   │   └── Cargo.toml
│   ├── monkeybee-extract/        # extraction and inspection
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── text.rs           # text extraction with positions
│   │   │   ├── metadata.rs       # metadata extraction
│   │   │   ├── structure.rs      # structure inspection
│   │   │   ├── asset.rs          # image/font/embedded file extraction
│   │   │   └── diagnostic.rs     # diagnostic report generation
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
monkeybee-core          (no internal deps — foundation)
    ↑
monkeybee-parser        (depends on: core)
    ↑
monkeybee-render        (depends on: core, parser [for stream decoding])
monkeybee-write         (depends on: core)
monkeybee-annotate      (depends on: core, render [for appearance streams], write)
monkeybee-extract       (depends on: core, render [for text pipeline reuse])
monkeybee-proof         (depends on: core, parser, render, write, annotate, extract)
monkeybee-cli           (depends on: all above)
```

## Runtime and concurrency model

Monkeybee PDF uses `asupersync` as its async runtime and orchestration layer. Per the upstream `asupersync` skill and runtime guidance, Monkeybee should stay native-first: thread `&Cx<'_>` through async I/O workflows, structure child tasks inside explicit scopes, and bootstrap CLI and proof-harness entrypoints with `RuntimeBuilder` plus `LabRuntime` rather than treating Tokio as the ambient runtime.

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

/// Stream: dictionary + data
pub struct PdfStream {
    pub dict: PdfDictionary,
    pub raw_data: Vec<u8>,         // as stored in file
    pub decoded_data: OnceCell<Vec<u8>>,  // lazily decoded
}

/// Dictionary with insertion-order preservation
pub struct PdfDictionary {
    entries: IndexMap<PdfName, PdfValue>,
}
```

### Document model (`monkeybee-core::document`)

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
    pub change_tracker: ChangeTracker,  // mutation tracking
}

/// Object store with reference resolution
pub struct ObjectStore {
    objects: HashMap<ObjRef, PdfValue>,
    // reverse index: which objects reference which
    reverse_refs: HashMap<ObjRef, Vec<ObjRef>>,
}
```

### Page model (`monkeybee-core::page`)

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

### Graphics state (`monkeybee-render::state`)

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

### Change tracking (`monkeybee-core::update`)

```rust
/// Tracks mutations to the document
pub struct ChangeTracker {
    pub added: HashSet<ObjRef>,
    pub modified: HashSet<ObjRef>,
    pub deleted: HashSet<ObjRef>,
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
  → Document builder (construct PdfDocument with ObjectStore, PageTree, etc.)
  → Diagnostic log (record all warnings, repairs, compatibility notes)
```

### Render flow

```
PdfDocument + page index
  → ResolvedPage (materialize inherited attributes)
  → Content stream(s) (decode, concatenate if multiple)
  → Content stream interpreter (dispatch operators, maintain graphics state stack)
    → Text operations → Font pipeline → Glyph positions → Backend
    → Path operations → Path builder → Stroke/Fill → Backend
    → Image operations → Image decoder → Color conversion → Backend
    → Transparency → Compositing engine → Backend
  → Backend produces output (raster buffer, SVG elements, etc.)
```

### Write flow

```
PdfDocument (with ChangeTracker)
  → Mode selection (full rewrite vs. incremental append)
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
  → Track change (ChangeTracker)
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
- Keep the async model single-runtime: `asupersync` is the workspace default, and any compatibility layer must stay quarantined at the edge.
- No dependency may introduce undefined behavior or memory unsafety that escapes its abstraction boundary.

## Test obligations by crate

### monkeybee-core
- Unit tests: object creation, serialization round-trips, reference resolution, page inheritance, geometry transforms.
- Property tests: arbitrary object construction → serialize → deserialize → compare.
- Invariant tests: change tracker consistency, reverse reference index accuracy.

### monkeybee-parser
- Unit tests: lexer on known token sequences, object parsing on all types, xref parsing on well-formed and malformed tables.
- Corpus tests: parse every file in the pathological corpus, verify no panics, collect diagnostics.
- Fuzz tests: random bytes → parser → no panics, no UB, bounded memory.
- Repair tests: known malformed inputs → verify repair produces usable output.

### monkeybee-render
- Unit tests: graphics state operations, individual operator handling, color space conversions.
- Render comparison tests: render corpus documents → compare against reference renderers.
- Visual regression tests: golden-image comparisons with perceptual diff thresholds.
- Edge case tests: transparency stacking, pattern rendering, Type 3 fonts, unusual blend modes.

### monkeybee-write
- Unit tests: object serialization for all types, xref generation, stream encoding.
- Round-trip tests: parse → write → re-parse → compare object graphs.
- Self-consistency tests: write output → parse with monkeybee-parser → verify structural validity.
- Reference validation: write output → open in PDFium/MuPDF → verify renders correctly.

### monkeybee-annotate
- Unit tests: annotation creation, geometry calculations, appearance stream generation.
- Round-trip tests: annotate → save → reload → verify annotations preserved.
- Integration tests: annotate corpus documents → save → open in reference viewers.

### monkeybee-extract
- Unit tests: text extraction on known documents with ground-truth positions.
- Metadata tests: extraction accuracy on documents with known metadata.
- Coverage tests: extraction runs on entire corpus without panics.

### monkeybee-proof
- Integration tests: full proof harness runs on subset of corpus.
- Ledger tests: compatibility ledger correctly categorizes known feature encounters.
- Evidence tests: artifact generation produces valid, parseable output.

## Subordinate implementation docs

Each of the following should be authored as the spec matures. They are design-to-code contracts for their respective subsystems:

- `docs/implementation/document-model.md` — core object model, object store, reference resolution
- `docs/implementation/parser-and-repair.md` — parser architecture, repair strategies, tolerant mode
- `docs/implementation/rendering.md` — render pipeline, graphics state, output backends
- `docs/implementation/annotation.md` — annotation model, placement, appearance, flattening
- `docs/implementation/writeback.md` — serialization, save modes, structural validation
- `docs/implementation/extraction.md` — text extraction, metadata, diagnostics

## Open questions

1. **Font rasterization strategy**: Use `freetype-rs` (C binding, battle-tested) or `ab_glyph` + `ttf-parser` (pure Rust, less mature)? Or a switchable backend?
2. **JPEG 2000**: Accept `openjpeg-sys` C binding, or wait for pure-Rust `jpeg2k` to mature?
3. **Rendering backend**: Start with a simple CPU rasterizer (tiny-skia?), or target a more capable backend from the start?
4. **Color management**: Full ICC profile support from day one, or defer to a later phase?
5. **Incremental save granularity**: Support byte-range preservation for signature-safe workflows in v1, or defer?
6. **CMap handling**: Build custom CMap parser, or leverage an existing library?
7. **Performance target**: Define specific throughput targets (pages/sec on benchmark docs) or keep qualitative ("obviously fast")?
