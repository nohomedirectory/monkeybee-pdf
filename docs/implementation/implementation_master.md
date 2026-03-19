# Implementation Master — Monkeybee PDF

## Purpose

This document is the APR-facing implementation reference for Monkeybee PDF. It summarizes the
crate topology, module boundaries, core data structures, data flows, cross-crate dependencies, and
test obligations. It links to subordinate implementation docs for deeper subsystem design.

This is not a philosophical essay and not the full codebase. It is the grounding surface that keeps
the SPEC honest about implementation realities.

The most important architectural refinement since the prior revision is simple to state and
consequential in practice: Monkeybee is no longer described merely as a layered
parser/document/render/write stack. It now has a baseline computational kernel —
`monkeybee-substrate` — that owns content-addressed roots, structural sharing, exact invalidation,
query materialization, temporal lineage, bounded ambiguity tracking, and invariant certificates.
Everything else remains distinct and domain-shaped, but those layers now project through one shared
substrate instead of each reinventing versioning, diffing, and cache semantics in parallel.

## Workspace topology

```
monkeybee-pdf/
├── Cargo.toml                    # workspace root
├── crates/
│   ├── monkeybee/                # stable public facade crate
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── probe.rs          # OpenProbe, ComplexityFingerprint, AdmissionDecision
│   │   │   ├── report.rs         # CapabilityReport, WritePlanReport, DiffReport
│   │   │   ├── receipt.rs        # WriteReceipt, InvariantCertificate, helper serializers
│   │   │   ├── session.rs        # Engine / Session / Snapshot facade
│   │   │   └── query.rs          # unstable preview surface for semantic queries/anchors
│   │   └── Cargo.toml
│   ├── monkeybee-core/           # shared primitives: IDs, geometry, errors, execution context
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── object.rs         # PDF object type definitions
│   │   │   ├── geometry.rs       # coordinate transforms, matrices
│   │   │   ├── error.rs          # shared error taxonomy
│   │   │   ├── context.rs        # ExecutionContext, budgets, determinism, provider/view state
│   │   │   ├── diagnostics.rs    # DiagnosticSink, Diagnostic, sink adapters
│   │   │   ├── version.rs        # PdfVersion tracking and version-gated feature registry
│   │   │   ├── scope.rs          # generated support/scope registry bindings
│   │   │   └── traits.rs         # ByteSource, FontProvider, ColorProfileProvider, CryptoProvider, OracleProvider
│   │   └── Cargo.toml
│   ├── monkeybee-bytes/          # byte sources, revision chain, raw span ownership
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── source.rs         # ByteSource implementations (mmap, in-memory, range-backed)
│   │   │   ├── fetch.rs          # fetch scheduler and prefetch planning for remote/lazy sources
│   │   │   ├── access_plan.rs    # reusable page/resource/byte-range access plans
│   │   │   ├── revision.rs       # revision chain tracking
│   │   │   └── span.rs           # raw span ownership for preserve mode / raw-span nodes
│   │   └── Cargo.toml
│   ├── monkeybee-security/       # security profiles, worker isolation, budget broker
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── profile.rs        # security profiles (Compatible, Hardened, Strict)
│   │   │   ├── budget.rs         # budget broker and enforcement
│   │   │   ├── isolation.rs      # worker isolation / kill-on-overrun
│   │   │   └── policy.rs         # risky-decoder allow/deny, hostile-input policy
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
│   ├── monkeybee-parser/         # PDF syntax parsing, repair (delegates codec/security)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── lexer.rs          # tokenization
│   │   │   ├── object_parser.rs  # object parsing
│   │   │   ├── xref_parser.rs    # xref table/stream parsing + repair
│   │   │   ├── stream.rs         # stream dispatch (delegates to monkeybee-codec)
│   │   │   ├── content.rs        # content stream parsing
│   │   │   ├── crypt.rs          # encryption/decryption
│   │   │   ├── repair.rs         # tolerant mode, recovery strategies, candidate generation
│   │   │   └── diagnostics.rs    # parser diagnostics
│   │   └── Cargo.toml
│   ├── monkeybee-substrate/      # persistent incremental kernel
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── digest.rs         # NodeDigest, digest traits, canonical payload hashing
│   │   │   ├── node.rs           # SubstrateNode, typed node payload normalization
│   │   │   ├── store.rs          # content-addressed node store and deduplication
│   │   │   ├── root.rs           # SnapshotRoot, root digests, lineage metadata
│   │   │   ├── lineage.rs        # snapshot lineage, temporal revision graph
│   │   │   ├── query.rs          # QueryEngine, QuerySpec, materialization records
│   │   │   ├── invalidation.rs   # digest-delta based exact invalidation
│   │   │   ├── hypothesis.rs     # HypothesisSet, candidate evidence, collapse policy
│   │   │   ├── temporal.rs       # historical frame materialization / replay scaffolding
│   │   │   └── certificate.rs    # InvariantCertificate, digest recomputation, receipt helpers
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
│   ├── monkeybee-document/       # semantic document graph built from syntax snapshots + substrate roots
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── document.rs       # semantic document model over substrate-backed indexes
│   │   │   ├── object_index.rs   # ObjRef -> node digest mapping, reverse semantic references
│   │   │   ├── xref.rs           # cross-reference management and effective object selection
│   │   │   ├── page.rs           # page tree, inheritance
│   │   │   ├── resource.rs       # resource resolution
│   │   │   ├── ownership.rs      # Owned/ForeignPreserved/OpaqueUnsupported classification
│   │   │   ├── update.rs         # incremental update tracking
│   │   │   ├── depgraph.rs       # dependency graph and invalidation inputs
│   │   │   ├── snapshot.rs       # PdfSnapshot (immutable, root-backed, shareable)
│   │   │   ├── transaction.rs    # EditTransaction, change journal, snapshot-in/snapshot-out
│   │   │   └── cache_view.rs     # document-facing cache/query namespace helpers
│   │   └── Cargo.toml
│   ├── monkeybee-catalog/        # catalog semantics: outlines, destinations, name trees, page labels, viewer prefs, OCG configs, attachments
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── outlines.rs
│   │   │   ├── destinations.rs
│   │   │   ├── page_labels.rs
│   │   │   ├── name_trees.rs
│   │   │   ├── viewer_prefs.rs
│   │   │   ├── optional_content.rs
│   │   │   └── attachments.rs
│   │   └── Cargo.toml
│   ├── monkeybee-content/        # content-stream IR and event interpreter
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── interpreter.rs    # content stream interpreter
│   │   │   ├── state.rs          # graphics state machine / algebra surface
│   │   │   ├── events.rs         # streaming event model
│   │   │   ├── pageplan.rs       # PagePlan immutable display list IR
│   │   │   ├── marked.rs         # marked content span tracking
│   │   │   └── sink.rs           # consumer sink adapters (RenderSink, ExtractSink, InspectSink, EditSink)
│   │   └── Cargo.toml
│   ├── monkeybee-text/           # shared text subsystem: fonts, CMaps, decode + authoring pipelines, search
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── font.rs
│   │   │   ├── cmap.rs
│   │   │   ├── unicode.rs
│   │   │   ├── decode.rs         # existing PDF text decode pipeline
│   │   │   ├── layout.rs         # authoring layout pipeline
│   │   │   ├── shaping.rs
│   │   │   ├── subset.rs
│   │   │   └── search.rs
│   │   └── Cargo.toml
│   ├── monkeybee-paint/          # shared paint/appearance primitives (non-raster, page-independent)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── path.rs
│   │   │   ├── color.rs
│   │   │   ├── stroke.rs
│   │   │   └── appearance.rs
│   │   └── Cargo.toml
│   ├── monkeybee-render/         # page rendering (consumes content events, not own interpreter)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── text.rs
│   │   │   ├── font.rs
│   │   │   ├── image.rs
│   │   │   ├── color.rs
│   │   │   ├── path.rs
│   │   │   ├── transparency.rs
│   │   │   ├── pattern.rs
│   │   │   ├── page.rs
│   │   │   ├── tile.rs
│   │   │   ├── progressive.rs    # ProgressiveRenderState, placeholder tracking, refinement
│   │   │   └── backend/          # output backends (raster via tile sink, svg)
│   │   └── Cargo.toml
│   ├── monkeybee-3d/             # 3D content: PRC/U3D parsing, scene graph, wgpu rendering
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── prc.rs            # PRC format parser
│   │   │   ├── u3d.rs            # U3D format parser
│   │   │   ├── scene.rs          # unified scene graph
│   │   │   ├── render.rs         # wgpu 3D render pipeline
│   │   │   ├── views.rs          # named views, camera interpolation
│   │   │   ├── modes.rs          # rendering modes (solid/wireframe/etc)
│   │   │   ├── section.rs        # cross-section computation
│   │   │   ├── structure.rs      # product structure tree
│   │   │   └── composite.rs      # 2D/3D compositing
│   │   └── Cargo.toml
│   ├── monkeybee-gpu/            # optional GPU 2D rendering backend via wgpu
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── device.rs         # wgpu device/queue management (shared with 3d)
│   │   │   ├── raster.rs         # compute shader path rasterization
│   │   │   ├── composite.rs      # GPU tile compositing
│   │   │   ├── atlas.rs          # glyph texture atlas
│   │   │   └── blend.rs          # hardware blend modes
│   │   └── Cargo.toml
│   ├── monkeybee-compose/        # high-level authoring and composition
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── doc_builder.rs
│   │   │   ├── page_builder.rs
│   │   │   ├── content_builder.rs
│   │   │   ├── resource.rs
│   │   │   ├── appearance.rs
│   │   │   ├── font_plan.rs
│   │   │   └── text_emit.rs
│   │   └── Cargo.toml
│   ├── monkeybee-write/          # pure serializer (no composition/authoring)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── serialize.rs      # object serialization
│   │   │   ├── xref_writer.rs    # xref generation
│   │   │   ├── stream_encode.rs  # stream compression
│   │   │   ├── rewrite.rs        # full document rewrite (deterministic mode)
│   │   │   ├── incremental.rs    # incremental append save
│   │   │   ├── plan.rs           # WritePlan, preservation claims, signature impact
│   │   │   ├── receipt.rs        # WriteReceipt assembly, invariant-certificate linkage
│   │   │   ├── encrypt.rs        # final encryption and output assembly
│   │   │   └── validate.rs       # output structural validation
│   │   └── Cargo.toml
│   ├── monkeybee-edit/           # transactional structural edits
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── transaction.rs    # edit transaction framework
│   │   │   ├── gc.rs             # resource GC and deduplication
│   │   │   ├── redaction.rs      # high-assurance redaction application
│   │   │   ├── assurance.rs      # redaction assurance reports and policy evaluation
│   │   │   ├── rewriter.rs       # ContentStreamRewriter pipeline for content-stream edits
│   │   │   └── optimize.rs       # compaction, recompression
│   │   └── Cargo.toml
│   ├── monkeybee-forms/          # AcroForm field tree, value model, appearance regen
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── field_tree.rs
│   │   │   ├── value.rs
│   │   │   ├── appearance.rs
│   │   │   ├── calc_order.rs
│   │   │   ├── widget.rs
│   │   │   └── signature.rs
│   │   └── Cargo.toml
│   ├── monkeybee-annotate/       # non-form annotation operations
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── model.rs
│   │   │   ├── placement.rs
│   │   │   ├── appearance.rs
│   │   │   ├── flatten.rs
│   │   │   └── roundtrip.rs
│   │   └── Cargo.toml
│   ├── monkeybee-extract/        # multi-surface extraction, semantic graph, anchors
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── physical.rs       # PhysicalText: exact glyph geometry
│   │   │   ├── logical.rs        # LogicalText: reading-order with confidence
│   │   │   ├── tagged.rs         # TaggedText: structure-tree-driven extraction
│   │   │   ├── layout_graph.rs   # shared extraction IR for spans/blocks/order/tables/tags
│   │   │   ├── semantic_graph.rs # spatial-semantic graph + edge construction
│   │   │   ├── anchors.rs        # SemanticAnchorId, alias maps, stability helpers
│   │   │   ├── proposal.rs       # typed edit/query proposal validation hooks
│   │   │   ├── search.rs         # SearchIndex, SelectionQuads, HitTest primitives
│   │   │   ├── metadata.rs
│   │   │   ├── structure.rs
│   │   │   ├── asset.rs
│   │   │   └── diagnostic.rs
│   │   └── Cargo.toml
│   ├── monkeybee-forensics/      # document security analysis and forensic inspection
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── hidden.rs         # hidden content detection
│   │   │   ├── redaction_audit.rs # redaction sufficiency verification
│   │   │   ├── post_sign.rs      # post-signing modification forensics
│   │   │   ├── cve_patterns.rs   # known exploit pattern detection
│   │   │   ├── producer_fp.rs    # producer fingerprinting
│   │   │   └── font_fp.rs        # font fingerprinting
│   │   └── Cargo.toml
│   ├── monkeybee-validate/       # conformance validation
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── arlington.rs
│   │   │   ├── profile.rs
│   │   │   ├── preflight.rs
│   │   │   └── signature.rs
│   │   └── Cargo.toml
│   ├── monkeybee-proof/          # validation and evidence harness
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── corpus.rs         # corpus management and indexing
│   │   │   ├── render_compare.rs # render comparison harness
│   │   │   ├── roundtrip.rs      # round-trip validation harness
│   │   │   ├── temporal.rs       # historical replay harness
│   │   │   ├── anchors.rs        # semantic-anchor stability harness
│   │   │   ├── hypothesis.rs     # ambiguous-recovery truthfulness harness
│   │   │   ├── certificates.rs   # invariant-certificate recomputation + audit
│   │   │   ├── ledger.rs         # compatibility and hypothesis ledgers
│   │   │   ├── benchmark.rs      # performance benchmarks
│   │   │   ├── fuzz.rs           # fuzz testing coordination
│   │   │   ├── reducer.rs        # automatic failure minimization
│   │   │   └── evidence.rs       # artifact generation
│   │   └── Cargo.toml
│   ├── monkeybee-diff/           # structural/text/render/save-impact/revision diff engine
│   ├── monkeybee-signature/      # signature dictionaries, byte-range maps, policy + verification
│   ├── monkeybee-native/         # all optional FFI/native bridges and broker adapters
│   └── monkeybee-cli/            # command-line interface
│       ├── src/
│       │   └── main.rs
│       └── Cargo.toml
```

Key topology rules:

1. `monkeybee-substrate` is baseline architecture, not a speculative research crate.
2. `monkeybee-substrate` is intentionally **not** a god crate: it owns persistence, lineage,
   query materialization, invalidation, and certificates, but not PDF semantics, rendering, or
   policy.
3. `monkeybee-syntax` remains the preservation boundary and still owns syntax-shape truth.
4. `monkeybee-document` remains the semantic layer and still owns ownership classes, page/resource
   resolution, and edit semantics.
5. `monkeybee-extract` owns semantic graph and anchor construction because those surfaces derive
   from extracted layout/structure meaning, even though their cache/invalidation lifecycles are
   substrate-backed.
6. `monkeybee-write` owns plan execution and receipt emission, but preservation algebra inputs come
   from document semantics and substrate lineage together.

## Crate dependency graph

Monkeybee now has a slightly more explicit split between **data origin** (bytes/parser/syntax),
**persistent computational kernel** (substrate), and **semantic/use-site consumers**
(document/content/render/extract/write/proof).

```
monkeybee-core                (no internal deps — shared primitives)
    ↑
monkeybee-bytes              (depends on: core)
monkeybee-security           (depends on: core)
    ↑                            ↑
monkeybee-substrate         monkeybee-codec
(depends on: core, bytes)       (depends on: core, security)
    ↑                            ↑
    └────────────── monkeybee-parser ──────────────┐
                   (depends on: core, bytes, codec, security)
                                                    ↑
monkeybee-syntax        (depends on: core, bytes, parser, substrate)    ← preservation boundary
    ↑
monkeybee-document      (depends on: core, bytes, substrate, syntax)    ← semantic layer
    ↑
monkeybee-catalog       (depends on: core, syntax, substrate, document)
monkeybee-content       (depends on: core, substrate, document)
monkeybee-text          (depends on: core, substrate, document, codec)
    ↑
monkeybee-paint         (depends on: core, text)
monkeybee-render        (depends on: core, substrate, content, document, text, codec, paint)
monkeybee-3d            (depends on: core, substrate, document, content, codec)
monkeybee-gpu           (depends on: core, render, paint)
monkeybee-compose       (depends on: core, substrate, document, text, content, paint)
monkeybee-write         (depends on: core, bytes, substrate, document, catalog, codec, validate)
monkeybee-edit          (depends on: core, substrate, document, content, compose, write)
monkeybee-forms         (depends on: core, substrate, document, text, compose, paint)
monkeybee-annotate      (depends on: core, substrate, document, content, compose, forms, paint)
monkeybee-extract       (depends on: core, substrate, content, document, text)
monkeybee-forensics     (depends on: core, substrate, document, content, extract, signature)
monkeybee-validate      (depends on: core, substrate, document, catalog)
monkeybee-diff          (depends on: core, substrate, document, content, extract, render, write)
monkeybee-signature     (depends on: core, substrate, syntax, document, write, validate)
monkeybee-proof         (depends on: core, bytes, substrate, codec, security, parser, syntax, document, content, text, render, compose, write, edit, forms, annotate, extract, validate, diff, signature)
monkeybee               (depends on: core, bytes, substrate, document, render, extract, edit, write, validate, diff, signature)
monkeybee-cli           (depends on: all above)
```

Additional notes:

- `monkeybee-substrate` sits *below* syntax/document semantics but *above* raw bytes as the place
  where structural sharing, lineage, and materialization reuse become concrete.
- `monkeybee-write` depends on `monkeybee-validate` because write execution includes mandatory
  self-parse/structural validation and preflight hooks.
- `monkeybee-native` stays behind narrow adapter boundaries and must not create dependency cycles
  back into domain crates.
- `monkeybee-extract` and `monkeybee-diff` depend on the substrate because semantic-anchor,
  temporal, and certificate-aware surfaces need direct access to root digests and query receipts.

`monkeybee` public modules:

- `probe.rs`      # bounded pre-open inspection and complexity classification
- `report.rs`     # CapabilityReport, WritePlanReport, DiffReport
- `receipt.rs`    # WriteReceipt, InvariantCertificate, schema version helpers
- `session.rs`    # Engine / Session / Snapshot facade
- `query.rs`      # semantic query/anchor preview surface (feature-gated)

### Workspace Cargo.toml structure

```toml
[workspace]
resolver = "2"
members = [
    "crates/monkeybee",
    "crates/monkeybee-core",
    "crates/monkeybee-bytes",
    "crates/monkeybee-security",
    "crates/monkeybee-codec",
    "crates/monkeybee-parser",
    "crates/monkeybee-substrate",
    "crates/monkeybee-syntax",
    "crates/monkeybee-document",
    "crates/monkeybee-catalog",
    "crates/monkeybee-content",
    "crates/monkeybee-text",
    "crates/monkeybee-paint",
    "crates/monkeybee-render",
    "crates/monkeybee-3d",
    "crates/monkeybee-gpu",
    "crates/monkeybee-compose",
    "crates/monkeybee-write",
    "crates/monkeybee-edit",
    "crates/monkeybee-forms",
    "crates/monkeybee-annotate",
    "crates/monkeybee-extract",
    "crates/monkeybee-forensics",
    "crates/monkeybee-validate",
    "crates/monkeybee-proof",
    "crates/monkeybee-diff",
    "crates/monkeybee-signature",
    "crates/monkeybee-native",
    "crates/monkeybee-cli",
]

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
indexmap = { version = "2", features = ["serde"] }
dashmap = "6"
rayon = "1"
thiserror = "2"
blake3 = "1"
wgpu = "24"
naga = "24"
```

### Feature flag strategy

Feature flags control the baseline-vs-experimental lane separation, optional native bindings, and
preview surfaces that are intentionally non-gating.

**Important:** the following are **not** feature-gated because they are baseline architecture:
- `monkeybee-substrate`
- hypothesis tracking
- preservation claims / `WriteReceipt`
- invariant-certificate schema
- exact invalidation semantics

| Flag | Crate | Effect |
|---|---|---|
| `freetype` | monkeybee-text | Enable FreeType backend for font rasterization (default: off) |
| `openjpeg` | monkeybee-codec | Enable OpenJPEG for JPEG 2000 decode (default: on in Compatible) |
| `lcms2` | monkeybee-render | Enable lcms2 for ICC profile evaluation (default: on) |
| `tiny-skia` | monkeybee-render | Enable tiny-skia rasterizer (default: on, baseline) |
| `experimental-raster` | monkeybee-render | Enable exact analytic area coverage rasterizer |
| `experimental-color` | monkeybee-render | Enable spectral-aware color pipeline |
| `experimental-sdf` | monkeybee-render | Enable SDF glyph rendering path |
| `wgpu-3d` | monkeybee-3d | Enable 3D PDF rendering via wgpu (default: on in native) |
| `wgpu-gpu2d` | monkeybee-gpu | Enable GPU 2D rendering backend (experimental, default: off) |
| `forensics` | monkeybee-forensics | Enable document forensics analysis (default: on) |
| `unstable-semantic-query` | monkeybee / monkeybee-extract | Expose preview semantic graph + anchor query API (post-v1 preview) |
| `unstable-temporal-replay` | monkeybee / monkeybee-substrate | Expose preview historical replay API beyond proof/internal use |
| `external-attestation` | monkeybee-substrate / monkeybee-write | Enable attachment of external attestations to invariant certificates |
| `wasm` | workspace | WASM-compatible build: no threads, no mmap, no system fonts |
| `proof` | monkeybee-proof | Enable full proof harness (pulls in all reference renderers) |
| `write-encryption` | monkeybee-write | Enable output encryption (default: off; non-gating) |

Baseline v1 builds with `tiny-skia`, `lcms2`, `openjpeg` (Compatible profile), `forensics`, and
`wgpu-3d` on native targets, and without `write-encryption`, `unstable-semantic-query`,
`unstable-temporal-replay`, `external-attestation`, or the experimental `wgpu-gpu2d` backend.

## Runtime and concurrency model

### Runtime layering doctrine

Core library crates (`monkeybee-core`, `monkeybee-substrate`, `monkeybee-syntax`,
`monkeybee-document`, `monkeybee-content`, `monkeybee-text`, `monkeybee-render`,
`monkeybee-compose`, `monkeybee-write`, `monkeybee-edit`, `monkeybee-forms`,
`monkeybee-annotate`, `monkeybee-extract`, `monkeybee-validate`) are runtime-agnostic. They accept
`&ExecutionContext` for cancellation, budgets, determinism, provider/view-state selection, and
diagnostics but never import asupersync directly.

The `monkeybee` facade, `monkeybee-bytes`, `monkeybee-proof`, and `monkeybee-cli` are
asupersync-native. In these crates, asupersync is not an adapter — it is the canonical
orchestration substrate:

- Session lifecycle is modeled as asupersync regions with parent-child ownership.
- Public operations return `OperationOutcome<T>` = `Outcome<OperationSuccess<T>, MonkeybeeError>`.
- Budgets use asupersync's `Budget` semiring with automatic `combine()` tightening for child operations.
- Cancellation checkpoints in core crates delegate to `cx.checkpoint()` through the `ExecutionContext` bridge.
- The proof harness uses `LabRuntime` with DPOR, oracle suite, and chaos injection for deterministic concurrency testing.
- Progressive rendering uses asupersync watch channels for tile completion.
- Fetch scheduling uses asupersync async I/O with structured region ownership.
- Query materialization is region-owned: a query's lifecycle is supervised by the same region that requested it.

A minimal WASM build validates runtime independence: WASM uses a simple `ExecutionContext`
implementation without asupersync.

### ExecutionContext as runtime bridge

`ExecutionContext` is the contract between runtime-agnostic core crates and the asupersync-native
orchestration layer.

In asupersync-native callers (facade, CLI, proof), `ExecutionContext` is derived from `&Cx`:

- `CancellationCheckpoint` trait impl delegates to `cx.checkpoint()` (budget-aware, trace-aware, scheduler-cooperative)
- `BudgetState` is derived from `cx.budget()` with field mapping: `Budget.deadline` → deadline, `Budget.cost_quota` → operator/byte budgets, `Budget.poll_quota` → checkpoint frequency, `Budget.priority` → render/query priority
- `DiagnosticSink` emits to `cx.trace()` for LabRuntime observability
- provider manifest and view-state identity are frozen at region entry so cache namespace drift cannot occur mid-operation

In runtime-agnostic callers (WASM, third-party integrations), `ExecutionContext` uses simple
implementations (AtomicBool cancellation, manual budget tracking). The bridge is zero-cost: a
single function pointer indirection for checkpoint calls.

### Async orchestration layer

Monkeybee PDF uses `asupersync` as its async runtime and orchestration substrate. Per the upstream
`asupersync` mega-skill guidance, Monkeybee threads `&Cx` through async I/O workflows, structures
child tasks inside explicit scopes and regions, and bootstraps CLI and proof-harness entrypoints
with `RuntimeBuilder` plus `LabRuntime`.

Rayon remains the CPU-bound parallel execution layer. The architectural split is deliberate:

- `asupersync` owns async file and directory I/O, corpus traversal, artifact streaming,
  external-process coordination, cancellation, timeout budgeting, session lifecycle regions,
  query-supervision scopes, and task supervision.
- Rayon owns page-level rendering fan-out, image and filter transforms, diff computation,
  extraction batches, compression work, query materialization kernels that are purely synchronous,
  and other bounded in-memory compute kernels.
- CPU-heavy work is handed off from an enclosing `asupersync` scope to Rayon via oneshot channels
  and rejoined in that same structured scope for aggregation, diagnostics, and persistence.
- Detached background tasks are not the default. Long-lived background activity must remain
  runtime-supervised and explicitly owned within asupersync regions.
- Tokio compatibility, if ever required for a third-party library, belongs behind a narrow adapter
  boundary rather than in Monkeybee's core architecture.

### Query materialization and invalidation discipline

The substrate query engine is not a magical side cache. It follows the same region/budget rules as
other work:

1. Query requests enter through a region-owned `QueryRuntime`.
2. The query engine records observed substrate digests, dependent query keys, and materialization metadata.
3. CPU-bound materialization work may run in Rayon, but the owning asupersync region supervises lifecycle, cancellation, and reporting.
4. Query results re-enter the region with a `MaterializationReceipt` that records the digests and dependent queries observed.
5. Invalidation is exact: new snapshot deltas identify changed digests, the substrate invalidation engine marks dependent queries dirty, and clean entries remain reusable.

This matters because Monkeybee's claims about "re-render only the touched page," "diff only changed
subtrees," and "preserve untouched query results across snapshots" are now runtime-enforced rather
than left to informal cache policy.

### Rayon ↔ asupersync bridge contract

The bridge between asupersync (async orchestration) and Rayon (CPU parallelism) follows these
invariants:

1. **Lifecycle ownership:** asupersync regions own the lifecycle of all work, including
   Rayon-dispatched compute. A Rayon job is always spawned from within an asupersync scope and its
   result is always collected back into that scope.
2. **Cancellation propagation:** `ExecutionContext` (derived from `Cx`) is passed into Rayon
   closures. Rayon work checks `exec_ctx.checkpoint.check()` at every content-stream operator, tile
   boundary, resource decode point, and query-materialization chunk boundary.
3. **No async in Rayon:** Rayon closures are purely synchronous. They never call `block_on()`,
   never create async runtimes, never hold async locks. The "async Rayon sandwich"
   (async → rayon → async → rayon) is forbidden.
4. **Oneshot bridge:** Results flow from Rayon to asupersync via oneshot channels. The asupersync
   task awaits the oneshot (cancellable); the Rayon closure sends the result when compute completes.
5. **Budget respecting:** Rayon work respects the same budget as the enclosing asupersync region.
   Budget exhaustion in Rayon triggers the same early-return as cancellation.
6. **Panic containment:** Rayon panics (from native decoders, malformed input, or bugs) are caught
   at the Rayon scope boundary and converted to `Outcome::Panicked` in the asupersync region. They
   do not propagate across the bridge.

### Outcome discipline

Public operations return `OperationOutcome<T>` rather than raw `Result<T, E>`:

```rust
pub type OperationOutcome<T> = Outcome<OperationSuccess<T>, MonkeybeeError>;
```

`OperationSuccess<T>` carries the value, diagnostics, optional operation reports
(Probe/Render/Extract/Write/Diff/Query), `BudgetSummary`, `CacheSummary`, and any generated receipt
artifacts.

Operations that can be cancelled return `Outcome<T, E>` rather than `Result<T, E>`. The four-valued
Outcome distinguishes:

- `Ok(T)` — operation succeeded with full result
- `Err(E)` — domain error (malformed PDF, unsupported feature, validation failure)
- `Cancelled(CancelReason)` — operation was cancelled (viewport change, user abort, budget exhaustion, shutdown). Partial results may be available.
- `Panicked(PanicPayload)` — unrecoverable failure (native decoder crash, bug). Must be surfaced to supervision/diagnostics, never silently swallowed.

The severity lattice is `Ok < Err < Cancelled < Panicked`. Aggregation is monotone.
`CancelReason` carries structured kinds: `User`, `Timeout`, `FailFast`, `ParentCancelled`,
`Shutdown`, `BudgetExhausted`.

## Core data structures

### Identity model

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DocumentId(pub u128);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SnapshotId(pub u128);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RevisionFrameId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeDigest(pub [u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticAnchorId(pub [u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryKey(pub [u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectKey {
    pub document_id: DocumentId,
    pub obj_ref: ObjRef,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResourceFingerprint(pub [u8; 32]);

pub enum BlobHandle {
    InMemory(Arc<[u8]>),
    Spill(ScratchBlobId),
}
```

### Engine and session model (`monkeybee` facade + document/substrate)

```rust
/// Top-level engine: owns global policy, providers, caches, and substrate runtime.
/// Typically one per process. Thread-safe (Send + Sync).
pub struct MonkeybeeEngine {
    pub config: EngineConfig,
    pub caches: CacheManager,
    pub substrate: Arc<SubstrateRuntime>,
    pub font_provider: Box<dyn FontProvider>,
    pub color_profile_provider: Box<dyn ColorProfileProvider>,
    pub crypto_provider: Option<Box<dyn CryptoProvider>>,
    pub oracle_provider: Option<Box<dyn OracleProvider>>,
    pub engine_policy: EnginePolicy,
}

/// An open document session: binds a byte source and revision chain to the engine.
pub struct OpenSession {
    pub engine: Arc<MonkeybeeEngine>,
    pub byte_source: Box<dyn ByteSource>,
    pub revision_chain: RevisionChain,
    pub current_snapshot: Arc<PdfSnapshot>,
    pub session_config: SessionConfig,
    pub access_plan_cache: Option<PreliminaryAccessPlan>,
    pub active_hypothesis_set: Option<HypothesisSetId>,
}

pub struct EnginePolicy {
    pub default_security_profile: SecurityProfile,
    pub provider_policy: ProviderPolicy,
}

pub struct SessionConfig {
    pub open_strategy: OpenStrategy,
    pub password: Option<SecretString>,
    pub session_overrides: SessionOverrides,
}

/// Immutable, shareable document state. Identified publicly by snapshot_id and
/// internally by a content-addressed root.
pub struct PdfSnapshot {
    pub snapshot_id: SnapshotId,
    pub lineage: SnapshotLineage,
    pub document: Arc<SemanticDocumentView>,
    pub syntax_snapshot: Arc<SyntaxSnapshot>,
    pub dep_graph: Arc<CondensedDependencyGraph>,
    pub parent_snapshot: Option<SnapshotId>,
    pub delta_from_parent: Option<SnapshotDelta>,
}

pub enum OpenStrategy {
    Eager,   // parse everything available locally
    Lazy,    // resolve objects on demand
    Remote,  // range requests + prefetch planner
}
```

### Probe and capability model (`monkeybee::probe` / `monkeybee::report`)

```rust
pub struct OpenProbe {
    pub capability_report: CapabilityReport,
    pub complexity: ComplexityFingerprint,
    pub admission: AdmissionDecision,
    pub preliminary_access_plan: Option<PreliminaryAccessPlan>,
    pub recovery_candidates: Vec<RecoveryCandidateSummary>,
}

pub struct CapabilityReport {
    pub signed: bool,
    pub signature_summary: SignatureSummary,
    pub encrypted: bool,
    pub tagged: bool,
    pub structure_complexity: Option<StructureComplexity>,
    pub structure_edit_risk: Option<StructureEditRisk>,
    pub has_xfa: bool,
    pub has_javascript: bool,
    pub risky_decoder_set: Vec<DecoderType>,
    pub edit_safety: EditSafetyClass,
    pub save_constraints: SaveConstraintReport,
    pub preserve_constraints: Vec<PreserveConstraint>,
    pub expected_degradations: Vec<FeatureCode>,
    pub recovery_confidence: RecoveryConfidence,
    pub ambiguity_count: u32,
    pub active_content: ActiveContentReport,
    pub substrate_digest: [u8; 32],
    pub temporal_revision_depth: u32,
    pub hypothesis_summary: Option<HypothesisSetSummary>,
    pub semantic_surface: Option<SemanticSurfaceSummary>,
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
    pub cache_overrides: Option<CacheConfig>,
    pub provider_manifest_id: ProviderManifestId,
    pub view_state_hash: ViewStateHash,
    pub query_policy: QueryPolicy,
}

pub struct ResourceBudgets {
    pub max_objects: u64,
    pub max_decompressed_bytes: u64,
    pub max_operators_per_page: u64,
    pub max_nesting_depth: u32,
    pub max_page_count: u32,
}

pub struct DeterminismSettings {
    pub deterministic_output: bool,
    pub pinned_fallback_fonts: bool,
    pub fixed_thread_count: Option<usize>,
    pub stable_digest_seeds: bool,
}

/// Cooperative cancellation token — cheaply cloneable, atomically cancellable.
#[derive(Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

pub struct QueryPolicy {
    pub allow_materialization: bool,
    pub allow_partial_reuse: bool,
    pub emit_materialization_receipts: bool,
}
```

### Security profiles (`monkeybee-security::profile`)

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SecurityProfile {
    Compatible,
    Hardened,
    Strict,
}

pub enum DecoderPolicy {
    Allow,
    Isolate { budget: DecodeBudget },
    Deny { reason: String },
}

pub struct DecodeBudget {
    pub max_output_bytes: u64,
    pub max_wall_time_ms: u64,
    pub max_memory_bytes: u64,
}

impl SecurityProfile {
    pub fn policy_for(&self, decoder: DecoderType) -> DecoderPolicy { /* ... */ }
}

pub enum DecoderType {
    JBIG2,
    JPEG2000,
    Type4Calculator,
    XfaXmlPacket,
}
```

### Persistent substrate and lineage (`monkeybee-substrate`)

```rust
pub enum SubstrateNode {
    RawSpan { source: ByteSpanRef },
    ParsedCos { kind: CosKind, children: Vec<NodeDigest> },
    SemanticObject { kind: SemanticKind, children: Vec<NodeDigest> },
    DerivedIndex { kind: DerivedIndexKind, children: Vec<NodeDigest> },
}

pub struct SnapshotRoot {
    pub syntax_root: NodeDigest,
    pub semantic_root: NodeDigest,
    pub preserve_root: NodeDigest,
    pub lineage_root: NodeDigest,
    pub hypothesis_set: HypothesisSetId,
    pub query_epoch: u64,
}

pub struct SnapshotLineage {
    pub snapshot_id: SnapshotId,
    pub root: SnapshotRoot,
    pub parent: Option<SnapshotId>,
    pub transaction_intent: Option<TransactionIntent>,
    pub change_digest: [u8; 32],
}

pub struct SubstrateRuntime {
    pub store: Arc<SubstrateStore>,
    pub queries: Arc<QueryRuntime>,
    pub lineage_index: DashMap<SnapshotId, SnapshotLineage>,
}

pub struct SubstrateStore {
    pub nodes: DashMap<NodeDigest, Arc<SubstrateNode>>,
    pub refcounts: DashMap<NodeDigest, u64>,
}
```

### Incremental query engine (`monkeybee-substrate::query`)

```rust
pub trait QuerySpec {
    type Key: Clone + Eq + Hash;
    type Value: Send + Sync + 'static;

    fn family() -> QueryFamily;
    fn materialize(key: &Self::Key, ctx: &mut QueryContext) -> Self::Value;
}

pub struct QueryRuntime {
    pub records: DashMap<QueryKey, QueryRecord>,
    pub reverse_deps: DashMap<QueryKey, Vec<QueryKey>>,
}

pub struct QueryRecord {
    pub key: QueryKey,
    pub family: QueryFamily,
    pub input_digests: Vec<NodeDigest>,
    pub dependent_queries: Vec<QueryKey>,
    pub materialization_digest: [u8; 32],
    pub status: QueryStatus,
}

pub struct MaterializationReceipt {
    pub key: QueryKey,
    pub input_digests: Vec<NodeDigest>,
    pub dependent_queries: Vec<QueryKey>,
    pub materialization_digest: [u8; 32],
    pub reused: bool,
}

pub enum QueryStatus {
    Clean,
    Dirty,
    Materializing,
}
```

### Hypothesis sets and ambiguity tracking (`monkeybee-substrate::hypothesis`)

```rust
pub struct HypothesisSet {
    pub set_id: HypothesisSetId,
    pub candidates: Vec<HypothesisCandidate>,
    pub collapse_policy: HypothesisCollapsePolicy,
}

pub struct HypothesisCandidate {
    pub candidate_id: RecoveryCandidateId,
    pub snapshot_root: SnapshotRoot,
    pub confidence: f64,
    pub evidence: Vec<HypothesisEvidence>,
}

pub struct HypothesisEvidence {
    pub source: String,
    pub score_delta: f64,
    pub reason: String,
}

pub struct HypothesisSetSummary {
    pub set_id: HypothesisSetId,
    pub candidate_count: u32,
    pub chosen: Option<RecoveryCandidateId>,
    pub unresolved_material_ambiguities: u32,
}
```

### Invariant certificates and receipts (`monkeybee-substrate::certificate` / `monkeybee-write::receipt`)

```rust
pub enum PreservedProperty {
    ByteRange { start: u64, end: u64 },
    ObjectIdentity(ObjRef),
    StructuralValidity,
    VisualEquivalence { page_index: u32 },
    SemanticEquivalence,
    TaggedStructureIntegrity,
    FormFieldSemantics,
    AnnotationGeometry,
    SignatureValidity(ObjRef),
}

pub struct PreservationClaim {
    pub property: PreservedProperty,
    pub verdict: PreservationVerdict,
    pub reason: String,
    pub evidence: Vec<CausalRef>,
}

pub struct InvariantCertificate {
    pub certificate_version: String,
    pub before_digest: [u8; 32],
    pub after_digest: [u8; 32],
    pub delta_digest: [u8; 32],
    pub preserved: Vec<PreservationClaim>,
    pub invalidated: Vec<PreservedProperty>,
    pub trace_digest: [u8; 32],
    pub external_attestations: Vec<ExternalAttestationRef>,
}

pub struct WriteReceipt {
    pub schema_version: String,
    pub snapshot_id: SnapshotId,
    pub write_mode: WriteMode,
    pub write_plan_digest: [u8; 32],
    pub pre_snapshot_digest: [u8; 32],
    pub post_snapshot_digest: [u8; 32],
    pub delta_digest: [u8; 32],
    pub bytes_appended: u64,
    pub preservation: BytePreservationMap,
    pub signature_coverage: Vec<SignedCoverageEntry>,
    pub ownership_transitions: Vec<OwnershipTransitionRecord>,
    pub invariant_certificate: Option<InvariantCertificate>,
    pub hypothesis_set: Option<HypothesisSetSummary>,
    pub post_write_validation: Vec<ValidationFinding>,
}
```

### Temporal replay (`monkeybee-substrate::temporal`)

```rust
pub struct RevisionFrame {
    pub frame_id: RevisionFrameId,
    pub snapshot_id: SnapshotId,
    pub root: SnapshotRoot,
    pub byte_window: (u64, u64),
    pub prior_frame: Option<RevisionFrameId>,
    pub signatures_visible: Vec<ObjRef>,
}

pub struct RevisionReplay {
    pub document_id: DocumentId,
    pub frames: Vec<RevisionFrame>,
}
```

### Compatibility ledger (`monkeybee-proof::ledger`)

```rust
#[derive(Serialize, Deserialize)]
pub struct CompatibilityLedger {
    pub schema_version: String,
    pub engine_version: String,
    pub timestamp: String,
    pub input: InputInfo,
    pub features: Vec<FeatureEntry>,
    pub repairs: Vec<RepairEntry>,
    pub degradations: Vec<DegradationEntry>,
    pub hypotheses: Vec<HypothesisLedgerEntry>,
    pub receipts: Vec<ReceiptDigestRef>,
    pub pages: Vec<PageLedger>,
    pub summary: LedgerSummary,
}

#[derive(Serialize, Deserialize)]
pub struct HypothesisLedgerEntry {
    pub set_id: String,
    pub candidate_count: u32,
    pub chosen: Option<String>,
    pub unresolved_material_ambiguities: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ReceiptDigestRef {
    pub kind: String,     // "write_receipt", "invariant_certificate", "diff_receipt"
    pub digest: String,
}

#[derive(Serialize, Deserialize)]
pub struct LedgerSummary {
    pub total_features: u32,
    pub tier1_count: u32,
    pub tier2_count: u32,
    pub tier3_count: u32,
    pub repair_count: u32,
    pub degradation_count: u32,
    pub hypothesis_count: u32,
    pub overall_status: String,
}
```

### PDF object model (`monkeybee-core::object`)

```rust
pub enum PdfValue {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    String(PdfString),
    Name(PdfName),
    Array(Vec<PdfValue>),
    Dictionary(PdfDictionary),
    Stream(PdfStream),
    Reference(ObjRef),
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjRef {
    pub num: u32,
    pub gen: u16,
}

pub struct PdfStream {
    pub dict: PdfDictionary,
    pub handle: StreamHandle,
}

pub struct PdfDictionary {
    entries: IndexMap<PdfName, PdfValue>,
}

pub struct StreamHandle {
    pub object_id: ObjRef,
    pub raw_span: ByteSpan,
    pub filter_chain: Vec<FilterSpec>,
    pub expected_decoded_length: Option<u64>,
}
```

### Document model (`monkeybee-document::document`)

```rust
pub struct PdfDocument {
    pub root: SnapshotRoot,
    pub objects: SemanticObjectIndex,
    pub xref: CrossRefView,
    pub trailer: TrailerView,
    pub pages: PageTree,
    pub updates: Vec<IncrementalUpdate>,
    pub metadata: DocumentMetadata,
    pub encryption: Option<EncryptionState>,
    pub diagnostics: DiagnosticLog,
    pub change_journal: ChangeJournal,
}

pub struct SemanticObjectIndex {
    pub by_ref: HashMap<ObjRef, NodeDigest>,
    pub reverse_refs: HashMap<ObjRef, Vec<ObjRef>>,
}

pub struct SemanticDocumentView {
    pub document_id: DocumentId,
    pub root: SnapshotRoot,
    pub page_count: u32,
    pub ownership_summary: OwnershipSummary,
}
```

### Page model (`monkeybee-document::page`)

```rust
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
    pub contents: Vec<ObjRef>,
    pub annotations: Vec<ObjRef>,
}
```

### Geometry (`monkeybee-core::geometry`)

```rust
#[derive(Clone, Copy)]
pub struct Matrix {
    pub a: f64, pub b: f64,
    pub c: f64, pub d: f64,
    pub e: f64, pub f: f64,
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub ll_x: f64, pub ll_y: f64,
    pub ur_x: f64, pub ur_y: f64,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64, pub y: f64,
}
```

### Graphics state (`monkeybee-content::state`)

```rust
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
pub struct SnapshotDelta {
    pub changed_objects: Vec<ObjectChange>,
    pub changed_pages: Vec<usize>,
    pub changed_node_digests: Vec<NodeDigest>,
    pub invalidated_query_keys: Vec<QueryKey>,
    pub regenerated_artifacts: Vec<ArtifactId>,
    pub preserved_properties: Vec<PreservationClaim>,
}

pub struct ChangeJournal {
    pub entries: Vec<ChangeEntry>,
}

pub struct ChangeEntry {
    pub object_id: ObjRef,
    pub old_digest: Option<NodeDigest>,
    pub new_digest: Option<NodeDigest>,
    pub new_value: Option<PdfValue>,
    pub reason: ChangeReason,
    pub ownership_before: OwnershipClass,
    pub ownership_after: OwnershipClass,
    pub dependency_delta: DependencyDelta,
    pub preservation_effects: Vec<PreservationClaim>,
}

pub struct WritePlan {
    pub classifications: Vec<ObjectClassification>,
    pub preservation_claims: Vec<PreservationClaim>,
    pub signature_impact: SignatureImpact,
    pub plan_digest: [u8; 32],
}

pub enum ObjectAction {
    PreserveBytes,
    AppendOnly,
    RewriteOwned,
    RegenerateAppearance,
    RequiresFullRewrite,
    Unsupported,
}

pub enum OwnershipClass {
    Owned,
    ForeignPreserved,
    OpaqueUnsupported,
}
```

### PagePlan IR (`monkeybee-content::pageplan`)

```rust
pub struct PagePlan {
    pub page_index: usize,
    pub media_box: Rectangle,
    pub crop_box: Rectangle,
    pub ops: Vec<DrawOp>,
    pub text_runs: Vec<TextRun>,
    pub resource_deps: HashSet<ObjRef>,
    pub marked_spans: Vec<MarkedSpan>,
    pub degradations: Vec<DegradationNote>,
    pub provenance: Vec<SourceSpan>,
    pub source_node_digests: Vec<NodeDigest>,
    pub materialization_digest: [u8; 32],
}

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
```

### Error taxonomy (`monkeybee-core::error`)

```rust
pub enum MonkeybeeError {
    Parse(ParseError),
    Semantic(SemanticError),
    Query(QueryError),
    Render(RenderError),
    Write(WriteError),
    RoundTrip(RoundTripError),
    Compatibility(CompatibilityNote),
}

pub struct ErrorContext {
    pub subsystem: Subsystem,
    pub object_ref: Option<ObjRef>,
    pub page: Option<usize>,
    pub description: String,
    pub severity: Severity,
    pub tier: Option<CompatibilityTier>,
    pub snapshot_id: Option<SnapshotId>,
    pub query_key: Option<QueryKey>,
}
```

### Diagnostic streaming (`monkeybee-core::diagnostics`)

```rust
pub struct Diagnostic {
    pub code: String,
    pub severity: Severity,
    pub subsystem: Subsystem,
    pub object_ref: Option<ObjRef>,
    pub page: Option<usize>,
    pub byte_offset: Option<u64>,
    pub snapshot_id: Option<SnapshotId>,
    pub query_key: Option<QueryKey>,
    pub hypothesis_set: Option<HypothesisSetId>,
    pub message: String,
    pub payload: Option<DiagnosticPayload>,
    pub causal_refs: Vec<CausalRef>,
}

pub trait DiagnosticSink: Send + Sync {
    fn emit(&self, diagnostic: Diagnostic);
}
```

### PDF version tracking (`monkeybee-core::version`)

```rust
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdfVersion {
    pub major: u8,
    pub minor: u8,
}

pub struct VersionInfo {
    pub input_version: PdfVersion,
    pub catalog_version: Option<PdfVersion>,
    pub effective_version: PdfVersion,
    pub output_version: Option<PdfVersion>,
}
```

### Cache management (`monkeybee-substrate::cache`)

```rust
pub struct CacheNamespace {
    pub snapshot_id: SnapshotId,
    pub security_profile: SecurityProfile,
    pub provider_manifest_id: ProviderManifestId,
    pub determinism_class: DeterminismClass,
    pub view_state_hash: ViewStateHash,
}

pub struct CacheConfig {
    pub decoded_stream_budget: usize,
    pub font_cache_budget: usize,
    pub page_plan_budget: usize,
    pub raster_tile_budget: usize,
    pub semantic_graph_budget: usize,
    pub temporal_replay_budget: usize,
    pub certificate_budget: usize,
}

pub struct CacheManager {
    pub config: CacheConfig,
    pub decoded_streams_local: DashMap<(SnapshotId, ObjRef, u64), Arc<[u8]>>,
    pub decoded_streams_shared: DashMap<(ResourceFingerprint, u64), Arc<[u8]>>,
    pub doc_fonts: DashMap<(SnapshotId, ObjRef), Arc<ParsedFontInstance>>,
    pub shared_font_programs: DashMap<ResourceFingerprint, Arc<ParsedFontProgram>>,
    pub shared_icc_profiles: DashMap<ResourceFingerprint, Arc<ParsedIccProfile>>,
    pub shared_cmaps: DashMap<ResourceFingerprint, Arc<ParsedCMap>>,
    pub glyph_bitmaps: DashMap<(ResourceFingerprint, GlyphId, QuantizedSize, QuantizedSubpixel), Arc<GlyphBitmap>>,
    pub page_plans: DashMap<(CacheNamespace, usize, u64), Arc<PagePlan>>,
    pub raster_tiles: DashMap<(CacheNamespace, usize, TileId, u32, TileCompleteness, RenderProfileHash), Arc<TileData>>,
    pub semantic_graphs: DashMap<(CacheNamespace, usize, ExtractProfileHash), Arc<SpatialSemanticGraph>>,
    pub temporal_replays: DashMap<(DocumentId, RevisionFrameId, ViewStateHash), Arc<RevisionFrameMaterialization>>,
    pub invariant_certificates: DashMap<(SnapshotId, SnapshotId, WriteMode, ProofProfileHash), Arc<InvariantCertificate>>,
}
```

### Content stream rewriter (`monkeybee-edit::rewriter`)

```rust
pub struct ContentStreamRewriter {
    pub filters: Vec<Box<dyn OperatorFilter>>,
    pub injections: Vec<Injection>,
}

pub trait OperatorFilter: Send + Sync {
    fn filter(&self, op: &Operator, state: &GraphicsState) -> FilterDecision;
}

pub enum FilterDecision {
    Keep,
    Drop,
    Replace(Vec<Operator>),
}
```

### Progressive render state (`monkeybee-render::progressive`)

```rust
pub struct ProgressiveRenderState {
    pub page_index: usize,
    pub available_resources: HashSet<ObjRef>,
    pub missing_resources: Vec<MissingResource>,
    pub completeness: f32,
    pub dependent_query_keys: Vec<QueryKey>,
}

pub struct MissingResource {
    pub obj_ref: ObjRef,
    pub resource_type: ResourceType,
    pub byte_range: Option<(u64, u64)>,
    pub affected_tiles: Vec<TileId>,
}
```

### 3D scene model (`monkeybee-3d::scene`)

```rust
pub struct Scene3D {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub lights: Vec<Light>,
    pub cameras: Vec<Camera>,
    pub transform_tree: TransformNode,
    pub named_views: Vec<NamedView>,
    pub product_structure: Option<ProductStructureTree>,
}

pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
    pub material_id: usize,
    pub lod_levels: Vec<LodLevel>,
}

pub struct NamedView {
    pub name: String,
    pub camera: Camera,
    pub rendering_mode: RenderingMode3D,
    pub visible_parts: Option<Vec<PartId>>,
    pub cross_sections: Vec<CrossSectionPlane>,
}

pub enum RenderingMode3D {
    Solid,
    Wireframe,
    Transparent,
    Illustration,
    HiddenLine,
    SolidWireframe,
}

pub struct CrossSectionPlane {
    pub normal: [f32; 3],
    pub distance: f32,
    pub cap_color: Option<[f32; 4]>,
}
```

### Dependency graph (`monkeybee-document::depgraph`)

```rust
pub enum EdgeType {
    ContentRef,
    DictRef,
    ArrayRef,
    InheritedRef,
}

pub struct ReferenceGraph {
    pub forward: DashMap<ObjRef, Vec<(ObjRef, EdgeType)>>,
    pub reverse: DashMap<ObjRef, Vec<(ObjRef, EdgeType)>>,
    pub snapshot_id: SnapshotId,
}

pub struct CondensedDependencyGraph {
    pub scc_nodes: DashMap<SccId, Vec<ObjRef>>,
    pub dag_forward: DashMap<SccId, Vec<SccId>>,
    pub dag_reverse: DashMap<SccId, Vec<SccId>>,
    pub snapshot_id: SnapshotId,
}

pub struct EditImpact {
    pub affected_pages: Vec<usize>,
    pub invalidated_caches: Vec<CacheKey>,
    pub invalidated_queries: Vec<QueryKey>,
    pub regeneration_needed: Vec<ObjRef>,
}
```

### Fetch scheduler (`monkeybee-bytes::fetch`)

```rust
pub trait FetchScheduler: Send + Sync {
    fn request_range(&self, offset: u64, length: u64) -> FetchHandle;
    fn submit_prefetch(&self, plan: PrefetchPlan);
    fn cancel_all(&self);
    fn statistics(&self) -> FetchStatistics;
}
```

### Semantic graph and anchors (`monkeybee-extract::semantic_graph` / `anchors`)

```rust
pub struct SpatialSemanticGraph {
    pub graph_digest: [u8; 32],
    pub nodes: Vec<SemanticNode>,
    pub edges: Vec<SemanticEdge>,
    pub alias_map: Vec<AnchorAlias>,
}

pub enum SemanticNodeKind {
    Page,
    Paragraph,
    Line,
    Span,
    Table,
    TableCell,
    Figure,
    FormField,
    Annotation,
    Signature,
    Region,
}

pub struct SemanticNode {
    pub anchor_id: SemanticAnchorId,
    pub kind: SemanticNodeKind,
    pub page_index: u32,
    pub bbox: Rectangle,
    pub text_excerpt: Option<String>,
    pub depends_on: Vec<ObjRef>,
    pub source_span_ids: Vec<SpanId>,
}

pub struct AnchorAlias {
    pub old_anchor: SemanticAnchorId,
    pub new_anchor: SemanticAnchorId,
    pub reason: String,
}

pub struct EditProposal {
    pub proposal_id: String,
    pub anchors: Vec<SemanticAnchorId>,
    pub op: SemanticEditOp,
    pub precondition_snapshot: SnapshotId,
    pub precondition_digest: [u8; 32],
    pub intent: EditIntent,
}
```

### WritePlan classification (`monkeybee-write::plan`)

```rust
pub enum WritePlanClassification {
    PreserveBytes,
    AppendOnly,
    RewriteOwned,
    RegenerateAppearance,
    RequiresFullRewrite,
    Unsupported,
}

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

pub trait OracleProvider: Send + Sync {
    fn resolve(&self, key: &OracleKey) -> Option<Arc<[u8]>>;
    fn manifest(&self) -> OracleManifest;
}
```

### Test obligation matrix reference

The SPEC.md Part 8 now defines a substrate-aware test obligation matrix with per-crate pass
thresholds and metrics for both traditional PDF-engine behaviors and the new kernel guarantees. In
addition to the earlier gated classes (xref repair, font fallback, transparency compositing,
producer quirks, incremental update, annotation roundtrip, generation, adversarial input, signature
preserve, and redaction safety), the revised matrix also adds:

- `substrate-delta`
- `historical-replay`
- `hypothesis-recovery`
- `semantic-anchor-stability`

The regression policy remains blocking: any previously passing gated class that fails in a new CI
run is a merge blocker unless explicitly triaged.

## Critical data flows

### Runtime orchestration flow

```
CLI / proof / library workflow
  -> RuntimeBuilder bootstrap (CLI) or LabRuntime (proof)
  -> Root region created (engine lifetime)
  -> engine.probe() optionally performs bounded OpenProbe inside session-admission scope
  -> engine.open() creates session region (child of root, owns session lifecycle)
  -> parser + syntax build initial substrate roots and hypothesis candidates
  -> document view constructed for chosen candidate
  -> render_page() / extract_text() / plan_save() create child regions (deadline budget, CollectAll policy)
  -> ExecutionContext derived from &Cx at region boundary (bridge to runtime-agnostic core)
  -> QueryRuntime materializes or reuses substrate-backed query families
  -> Rayon executes CPU-bound kernels via oneshot bridge (cancel-safe, budget-aware)
  -> asupersync aggregates Outcome results, emits diagnostics, writes artifacts/receipts
  -> Region close guarantees quiescence (no orphan tasks, all finalizers complete)
  -> Oracle suite asserts: no obligation leaks, losers drained, cancellation protocol honored
```

### Parse flow

```
PDF bytes
  -> Lexer (tokenize)
  -> Object parser (construct PdfValue tree)
  -> XRef parser (build cross-reference table, repair if needed)
  -> Encryption handler (decrypt if needed)
  -> Candidate builder (materially different repairs become bounded hypothesis candidates)
  -> Syntax layer (immutable COS objects, provenance, repair records)
  -> Substrate insertion (raw spans, parsed COS nodes, syntax root digests)
  -> Document builder (semantic object index, page tree, ownership classes, dependency graph)
  -> SnapshotRoot + SnapshotLineage recorded
  -> Diagnostic log + compatibility/hypothesis summaries produced
```

### Render flow

```
PdfSnapshot + page index
  -> QueryRuntime.get(page_plan(page_index, mode))
  -> ResolvedPage (materialize inherited attributes)
  -> Content stream(s) decoded / concatenated
  -> Content interpreter in monkeybee-content (single implementation)
     -> events or PagePlan IR dispatched through RenderSink adapter
     -> text ops -> font decode pipeline -> glyph positions -> backend
     -> path ops -> path builder -> stroke/fill -> backend
     -> image ops -> image decoder -> color conversion -> backend
     -> transparency -> compositing engine -> backend
  -> Tile/band scheduler materializes full page or requested region
  -> Progressive mode records placeholders and missing-resource byte ranges when needed
  -> MaterializationReceipt stored with query record for reuse / invalidation
  -> Backend produces output (raster, SVG, etc.)
```

### Write flow

```
PdfSnapshot + ChangeJournal
  -> Preservation analysis (claims preserved / invalidated properties)
  -> WritePlan computation (PreserveBytes / AppendOnly / RewriteOwned / etc.)
  -> Signature impact analysis
  -> Mode selection (incremental append vs full rewrite)
  -> Object serializer (PdfValue -> bytes)
  -> Stream encoder (apply compression filters)
  -> XRef writer / trailer writer
  -> Output assembler
  -> Self-validation (parse the output, verify structural correctness)
  -> WriteReceipt + optional InvariantCertificate emitted
  -> Compatibility ledger / proof artifacts receive receipt digests
```

### Annotation round-trip flow

```
PdfSnapshot
  -> Load existing annotations
  -> Create new annotation (type, geometry, content)
  -> Generate appearance stream (compose/paint content, not raster)
  -> Insert into document model and substrate
  -> ChangeJournal records object digests + preservation effects
  -> plan_save() previews write impact / signature effect
  -> write()
  -> reload + validate (annotations present, geometry preserved, content intact)
```

### Temporal replay flow

```
Byte source + revision chain
  -> revision scanner identifies incremental frames
  -> Substrate temporal module materializes RevisionFrame sequence
  -> choose frame N or diff frame N..M
  -> build historical PdfSnapshot view for selected frame
  -> render / extract / diff using ordinary query families
  -> emit revision-scoped receipts and validation findings
```

### Semantic-anchor query and edit flow

```
PdfSnapshot + extract profile
  -> QueryRuntime.get(semantic_graph(page or snapshot))
  -> SpatialSemanticGraph + SemanticAnchorId set materialized
  -> caller selects anchors / runs typed query
  -> optional EditProposal validated against current snapshot + digest
  -> EditTransaction created only if policy / ownership / preservation checks pass
  -> ordinary write path emits receipts anchored to SemanticAnchorId values
```

## External dependency strategy

### Planned dependencies (subject to evaluation)

- **`flate2`** — DEFLATE compression/decompression (FlateDecode)
- **`image`** — image decoding (JPEG, PNG, TIFF baseline)
- **`jpeg-decoder`** — DCTDecode
- **`openjpeg-sys` or `jpeg2k`** — JPXDecode, isolated behind `monkeybee-native`
- **`lcms2`** — ICC evaluation, isolated behind `monkeybee-native`
- **`freetype-rs`** — optional hinted rasterization, isolated behind `monkeybee-native`
- **`wgpu`** — GPU abstraction for 3D rendering and optional 2D GPU backend
- **`naga`** — shader translation/validation support for wgpu pipelines
- **`indexmap`** — ordered dictionaries
- **`dashmap`** — concurrent maps for caches, substrate store, and query metadata
- **`blake3`** — baseline digest engine for substrate nodes, roots, deltas, and receipts
- **`once_cell` / `std::sync::OnceLock`** — lazy initialization
- **`asupersync`** — async runtime, structured concurrency, cancellation, Budget semiring, Outcome type, LabRuntime deterministic testing, watch/oneshot channels, DPOR, oracle suite, chaos injection
- **`rayon`** — CPU-bound parallelism; lifecycle owned by asupersync regions
- **`clap`** — CLI argument parsing
- **`serde` + `serde_json`** — structured output, compatibility ledger, receipts, certificates
- **`sha2` / `md5`** — PDF encryption handlers
- **`aes`** — AES encryption for PDF security handlers
- **`rc4`** — RC4 encryption for legacy security handlers
- **`miniz_oxide`** — alternative pure-Rust DEFLATE

### Dependency principles

- Prefer pure-Rust where quality and performance are comparable.
- Accept C/C++ bindings only for capabilities not yet available in pure Rust at required quality.
- Pin all dependency versions. Audit for `unsafe` in critical-path dependencies.
- Core library crates are runtime-agnostic. The facade, bytes, proof, and CLI crates are
  asupersync-native — asupersync is the orchestration substrate, not an adapter.
- The substrate digest model uses `blake3` in the baseline because it is fast, stable, widely
  understood, and practical for high-volume content addressing. A digest abstraction may exist
  internally, but we do not begin with a pluggable crypto buffet.
- Monkeybee is inspired by incremental-query systems such as Salsa, but does **not** take a hard
  dependency on Salsa in the baseline plan. The query engine must integrate tightly with
  asupersync/Rayon budgets, spill-aware caches, deterministic receipts, and document-scale binary
  artifacts; a focused internal query runtime is the more realistic v1 choice.
- No dependency may introduce undefined behavior or memory unsafety that escapes its abstraction boundary.

## Test obligations by crate

### monkeybee-core
- Unit tests: object type creation, geometry transforms, matrix operations.
- Property tests: arbitrary object construction -> serialize -> deserialize -> compare.
- DiagnosticSink tests: VecSink collects all diagnostics, FilteringSink filters by severity/subsystem, CountingSink counts correctly.
- ExecutionContext tests: provider-manifest/view-state identity remains stable across child clones; query policy propagates correctly.
- PdfVersion tests: version parsing, comparison ordering, version-gated feature lookup, catalog version override precedence.

### monkeybee-bytes
- Unit tests: ByteSource implementations (mmap, in-memory, range-backed), revision chain construction, span tracking.
- Property tests: span ownership invariants preserved across revision appends.
- Access-plan tests: first-paint byte planning on linearized vs non-linearized fixtures.
- Remote tests: fetch statistics and cancellation are stable under concurrent range requests.

### monkeybee-codec
- Unit tests: each filter implementation on known input/output pairs.
- Property tests: encode -> decode round-trip identity for all reversible filters.
- Fuzz tests: arbitrary bytes through each decoder — no panics, bounded memory.
- Predictor tests: PNG and TIFF predictor logic on known image data.
- Pipeline tests: cascaded filter chains, including reversed-order recovery.

### monkeybee-security
- Unit tests: security profile selection, budget enforcement, allow/deny policy.
- Integration tests: risky decoder invocation through security gate — verify budgets enforced and isolation works.
- Property tests: no decoder invocation bypasses the security boundary.
- Active-content policy tests: XFA / active-content detection never silently upgrades to native execution.

### monkeybee-parser
- Unit tests: lexer on known token sequences, object parsing on all types, xref parsing on well-formed and malformed tables.
- Corpus tests: parse every file in the pathological corpus, verify no panics, collect diagnostics.
- Fuzz tests: random bytes -> parser -> no panics, no UB, bounded memory.
- Repair tests: known malformed inputs -> verify repair produces usable output.
- Ambiguity tests: ambiguous fixtures produce multiple candidates or an explicit unresolved classification; no silent collapse.

### monkeybee-substrate
- Unit tests: node digest stability, content-addressed deduplication, root construction, lineage insertion.
- Property tests: identical normalized payload + identical child digests -> identical NodeDigest; changed child digest -> changed parent digest.
- Query tests: materialization records capture all observed digests and dependent query keys.
- Invalidation tests: changed digests dirty exactly the expected query set and no more.
- Receipt tests: invariant certificates are deterministic under deterministic mode and recomputable by proof harness.
- Hypothesis tests: chosen candidate and alternative summaries remain stable across identical opens.
- Temporal tests: historical frame materialization preserves frame-local roots and does not mutate later frames.

### monkeybee-syntax
- Unit tests: COS object construction from parser output, provenance round-trip (source spans preserved).
- Property tests: immutability invariant (syntax objects cannot be mutated after construction).
- Preservation tests: raw formatting retention (whitespace, comments survive round-trip via syntax layer).
- Repair record tests: repair records faithfully capture strategy, confidence, and alternatives.
- Object-stream membership tests: objects correctly track their object-stream provenance.
- Xref provenance tests: original vs repaired xref entries are distinguishable.
- Substrate tests: syntax roots are stable across repeated parses of identical bytes.

### monkeybee-document
- Unit tests: document model construction from syntax snapshots, page tree inheritance, resource resolution, reference integrity.
- Property tests: ownership classification consistency, EditTransaction commit/rollback semantics.
- Invariant tests: change journal consistency, reverse reference index accuracy.
- Dependency graph tests: edit an object, verify only dependents invalidated.
- Snapshot tests: PdfSnapshot immutability, snapshot_id uniqueness, root-digest lineage correctness, structural sharing (new snapshot does not clone full object store).
- Preservation tests: change journal entries emit preservation-effect deltas expected by WritePlan.
- Thread-safety tests: parallel page renders on shared PdfSnapshot, concurrent object-index access, atomic budget counter correctness.

### monkeybee-content
- Unit tests: content stream interpretation, graphics state machine, event dispatch.
- Sink adapter tests: RenderSink, ExtractSink, InspectSink, EditSink receive correct events for known content streams.
- Property tests: PagePlan IR equivalence with streaming events.
- Cache/query tests: PagePlan cache invalidation on content/resource changes.
- Error recovery tests: operator-level isolation, state rollback on partial failure, resource resolution failure handling, inline image recovery, Q stack underflow recovery, recursion limit enforcement.
- Graphics-algebra tests: save/restore and CTM composition remain consistent with formal state transitions.

### monkeybee-text
- Unit tests: font program parsing (Type 1, TrueType, CFF, CIDFont, Type 3), CMap parsing, ToUnicode resolution.
- Decode pipeline tests: char code -> font/CMap -> CID/glyph -> Unicode/metrics for each font type.
- Authoring pipeline tests: Unicode -> shaping/bidi/line breaking/font fallback -> positioned glyph runs.
- Unicode fallback chain tests: known fonts with broken/missing ToUnicode produce expected mappings.
- Shaping/bidi tests: complex scripts (Arabic, Hebrew, Devanagari), ligatures, bidi reordering (authoring pipeline only).
- Subsetting tests: subset -> re-embed -> verify glyph coverage and metrics round-trip.
- Search/hit-test tests: known text at known positions -> verify search finds it, hit-test returns correct quads.

### monkeybee-render
- Unit tests: backend drawing operations, color space conversions, tile/band scheduling.
- Render comparison tests: render corpus documents -> compare against reference renderers.
- Visual regression tests: golden-image comparisons with perceptual diff thresholds.
- Edge case tests: transparency stacking, pattern rendering, Type 3 fonts, unusual blend modes.
- Cooperative cancellation tests: cancel mid-render at each checkpoint type.
- Progressive rendering tests: missing resources produce correct placeholders, placeholder metadata carries correct byte ranges, incremental refinement replaces only affected tiles.
- Query reuse tests: repeated renders on unchanged snapshot reuse page-plan/tile materializations.

### monkeybee-3d
- Unit tests: PRC parser on known PRC files, U3D parser on known U3D files, scene graph construction from both formats.
- Render tests: 3D annotations rendered and compared against Adobe Acrobat screenshots at defined camera positions.
- Named view tests: camera interpolation produces smooth transitions, all named views are reachable.
- Cross-section tests: cross-section planes produce geometrically correct cut surfaces.
- Round-trip tests: 3D annotation dictionaries survive load-save-reload without data loss.
- WASM tests: 3D renders in WebGPU-capable browsers.

### monkeybee-gpu
- Unit tests: device/queue lifecycle, shader module compilation, atlas allocation.
- Render parity tests: GPU output matches the CPU baseline within proof thresholds.
- Stress tests: transparency-heavy and text-heavy pages stay within GPU memory budgets.
- Fallback tests: unsupported adapters and budget overruns cleanly fall back to the CPU path.

### monkeybee-paint
- Unit tests: path, color, and stroke primitives used by compose/render/annotate.
- Appearance tests: paint-state normalization remains consistent across appearance generation and raster consumption.

### monkeybee-compose
- Unit tests: document/page/content builder APIs, resource naming uniqueness, appearance stream generation.
- Integration tests: compose a document -> serialize via monkeybee-write -> parse -> verify structure.
- Appearance tests: annotation and widget appearance generation produces valid form XObjects.
- Font embedding planning tests: subsetting requests match actual glyph usage.
- Text emission tests: authoring layout pipeline produces correct positioned glyph runs.

### monkeybee-write
- Unit tests: object serialization for all types, xref generation, stream encoding.
- WritePlan tests: classification correctness (PreserveBytes/AppendOnly/RewriteOwned/etc.) on known document states.
- Preservation algebra tests: composed transform claims yield expected preserved / invalidated properties.
- WriteReceipt tests: receipt digests remain stable under deterministic mode and include correct signature-coverage entries.
- Round-trip tests: parse -> write -> re-parse -> compare object graphs.
- Self-consistency tests: write output -> parse with monkeybee-parser -> verify structural validity.
- Reference validation: write output -> open in PDFium/MuPDF -> verify renders correctly.

### monkeybee-edit
- Unit tests: EditTransaction commit/rollback, resource GC, deduplication.
- Redaction tests: text-only, image-only, mixed, reused XObjects, canary-text leakage.
- Optimization tests: compaction produces smaller valid output, recompression round-trips.
- Content stream rewrite tests: parse-filter-reemit round-trip preserves unfiltered operators exactly, operator drop removes target operators and old stream is deleted from change journal, operator replace substitutes correctly with full graphics state context, injection inserts at correct positions with q/Q wrapping, annotation flattening appends appearance stream with correct coordinate transform, TJ array splitting for partial-overlap redaction.
- Receipt tests: edit receipts point back to changed digests and affected anchors when applicable.

### monkeybee-forms
- Unit tests: field tree parsing, inheritance resolution, field value model for each type.
- Appearance regeneration tests: change field value -> regenerate appearance -> verify rendered appearance matches value.
- Round-trip tests: fill form -> save -> reload -> verify field values and appearances preserved.
- Signature-field tests: incremental-append after form fill preserves signed byte ranges.
- Calculation order tests: detection and preservation of calculation order across round-trips.

### monkeybee-annotate
- Unit tests: annotation creation, geometry calculations, appearance stream generation.
- Round-trip tests: annotate -> save -> reload -> verify annotations preserved.
- Integration tests: annotate corpus documents -> save -> open in reference viewers.
- Geometry-preservation tests: incremental append does not drift existing annotation quads.

### monkeybee-validate
- Unit tests: Arlington-model rules against known valid/invalid objects.
- Profile tests: PDF/A-4, PDF/X-6 constraint checking on known-conforming documents.
- Preflight tests: write preflight catches structural errors before serialization.
- Signature tests: byte-range verification on signed documents.
- Receipt cross-check tests: validation findings match receipt-reported post-write validation.

### monkeybee-extract
- Unit tests: text extraction on known documents with ground-truth positions.
- Multi-surface tests: PhysicalText matches exact glyph geometry, LogicalText produces correct reading order with confidence, TaggedText uses structure tree when present.
- Search/hit-test tests: SearchIndex finds known text, SelectionQuads returns correct regions, HitTest resolves correct characters.
- Metadata tests: extraction accuracy on documents with known metadata.
- Semantic graph tests: graph node/edge construction is deterministic for fixed extract profile.
- Anchor tests: semantically unchanged rewrites preserve anchors or emit explicit alias maps.
- Proposal tests: invalid EditProposal preconditions are rejected before mutation.

### monkeybee-forensics
- Unit tests: hidden content detection on planted corpus fixtures including white-on-white text, off-page content, and image-obscured text.
- Redaction audit tests: intentionally bad redactions (opaque overlay only) are detected while proper redactions pass.
- Post-signing tests: classify modifications on signed-then-modified corpus files with permitted-vs-suspicious expectations.
- Fingerprinting tests: producer and font fingerprinting remain stable on curated fixtures with known provenance.

### monkeybee-diff
- Unit tests: structural, text, render, and save-impact diffs on known fixture pairs.
- Temporal diff tests: diffing historical frames yields the same object/page deltas as manual replay.
- Receipt tests: diff surfaces can attach source snapshot digests and optional invariant certificates.

### monkeybee-signature
- Unit tests: signature dictionary parsing, byte-range maps, DocMDP / FieldMDP policy evaluation.
- Integration tests: signature impact reports agree with WritePlan and WriteReceipt outputs.
- Provider tests: CryptoProvider-backed verification results are stably surfaced into reports and ledgers.

### monkeybee-proof
- Integration tests: full proof harness runs on subset of corpus.
- Ledger tests: compatibility and hypothesis ledgers correctly categorize known encounters.
- Evidence tests: artifact generation produces valid, parseable output.
- Ledger JSON schema tests: ledger output validates against schema, version tracking fields populate correctly, schema versioning remains backward-compatible within majors.
- Corpus manifest tests: every fixture has an `ExpectationManifest`.
- Repair expectation tests: ambiguous recovery asserts chosen candidate id, semantic digest, and write-impact class unless explicitly waived.
- Temporal tests: multi-revision fixtures produce stable historical frame outputs.
- Anchor tests: semantic-anchor stability harness computes expected alias precision.
- Certificate tests: proof harness can recompute invariant-certificate digests independently.
- Regression tests: unknown degradations, hypothesis drift, or scope-class violations fail unless triaged.

## Subordinate implementation docs

Each of the following should be authored as the spec matures. They are design-to-code contracts for
their respective subsystems:

- `docs/implementation/substrate.md` — node digests, content-addressed store, root construction, lineage, query runtime
- `docs/implementation/query-engine.md` — QuerySpec model, materialization records, invalidation, cache namespaces
- `docs/implementation/preservation-algebra.md` — preserved properties, transform composition, WritePlan derivation, receipts
- `docs/implementation/document-model.md` — semantic object index, reference resolution, dependency graph, snapshots, transactions
- `docs/implementation/syntax-layer.md` — COS object representation, provenance model, preservation boundary contract, repair record schema
- `docs/implementation/parser-and-repair.md` — parser architecture, repair strategies, tolerant mode, ambiguity handling
- `docs/implementation/codec.md` — filter chains, image decode/encode, bounded pipelines, decode telemetry
- `docs/implementation/security.md` — security profiles, budget broker, worker isolation, hostile-input policy
- `docs/implementation/text.md` — font programs, CMaps, Unicode mapping, decode pipeline, authoring layout pipeline, subsetting, search/hit-test
- `docs/implementation/rendering.md` — render pipeline via content sink adapters, output backends, tile/band surface, progressive render
- `docs/implementation/forms.md` — AcroForm field tree, value model, appearance regeneration, widget bridge, signature helpers
- `docs/implementation/annotation.md` — annotation model, placement, appearance, flattening
- `docs/implementation/compose.md` — document/page builders, resource naming, appearance generation, font embedding planning
- `docs/implementation/writeback.md` — serialization, save modes, WritePlan computation, receipt emission, structural validation
- `docs/implementation/extraction.md` — multi-surface text extraction, semantic graph, anchors, search primitives, metadata, diagnostics
- `docs/implementation/temporal-replay.md` — revision frames, historical snapshot materialization, replay semantics
- `docs/implementation/proof-manifests.md` — expectation manifest schema, triage workflow, CI semantics, certificate audit workflow

## Resolved design decisions

1. **Font rasterization strategy:** `ttf-parser` (pure Rust) for font table parsing; `ab_glyph_rasterizer`
   for glyph rasterization in the baseline. `freetype-rs` remains an optional feature-flagged
   backend behind a `FontRasterizer` trait for environments that need hinting.

2. **JPEG 2000:** Accept `openjpeg-sys` C binding for v1 behind `monkeybee-security` isolation.
   Pure-Rust `jpeg2k` is the planned replacement once it reaches decode-correctness parity.

3. **Rendering backend:** `tiny-skia` for the baseline CPU rasterizer. The experimental exact
   analytic area coverage rasterizer replaces `tiny-skia`'s anti-aliasing path only after it beats
   the baseline under the proof harness.

4. **Color management:** ICC profile support is v1-gating for ICCBased color spaces. Use `lcms2`
   (C binding) for v1 ICC profile evaluation behind a `ColorProfileProvider` trait.

5. **Incremental save granularity:** Byte-range preservation for signature-safe workflows is
   v1-gating. The preserve-mode write path, preservation claims, and signature impact analysis are
   baseline v1 features, not deferred.

6. **CMap handling:** Custom CMap parser with embedded Adobe CJK CMap data for the four standard
   CIDSystemInfo registries. Lazily loaded to keep WASM binaries bounded.

7. **Performance targets:** Quantitative targets remain defined in SPEC.md Part 7. Summary:
   first-page render <50ms at 150 DPI (latency class), sustained 10+ pages/sec at 150 DPI with
   parallelism (throughput class), no operation >10x expected time for content size (stress class),
   peak memory <5x file size for typical docs (memory class).

8. **Digest engine:** BLAKE3 is the baseline digest engine for node content addressing, snapshot
   roots, deltas, receipts, and certificate recomputation. The abstraction boundary exists
   internally, but the baseline architecture assumes one pinned digest algorithm for determinism.

9. **Persistent substrate strategy:** The baseline snapshot model is a content-addressed,
   structurally shared store with raw-span leaves and digest-addressed interior nodes. We are not
   starting from mutable `HashMap<ObjRef, Object>` state and retrofitting persistence later.

10. **Incremental query engine:** The query engine is an internal substrate runtime rather than an
    external Salsa dependency in v1. The fit with asupersync budgets, spill-aware caches,
    deterministic receipts, and large binary artifacts is too important to leave vague.

11. **Preservation algebra proof level:** v1 uses an auditable rule engine plus digest-backed
    receipts and invariant certificates. Stronger external proofs, theorem provers, or zk systems
    remain explicitly post-v1 layers.

12. **Semantic anchor identity:** Anchor IDs are deterministic surfaces derived from semantic/layout
    meaning, not raw object numbers. When safe rewrites change the internal object graph without
    changing semantic meaning, Monkeybee emits alias maps rather than pretending raw IDs stayed stable.

13. **Temporal replay model:** Historical replay is derived from incremental revision frames and
    snapshot lineage, not from heuristic byte diffing of whole files after the fact.
