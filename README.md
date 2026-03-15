# Monkeybee PDF

Memory-safe, high-performance Rust PDF engine for ugly real-world PDFs.

## Why this exists

Most open-source PDF libraries choose a lane: parse, render, or generate. None of them own the full document lifecycle. The result is that real-world PDFs — malformed, quirky, hostile, produced by hundreds of different tools over decades — remain the exclusive domain of proprietary engines like Acrobat.

Monkeybee PDF exists to change that. It is a single Rust engine that treats PDFs as a bidirectional semantic substrate: not a one-way rendering stream, not a parse-and-forget pipeline, but a living document model that can be read, understood, rendered, inspected, extracted from, annotated, edited, generated, serialized, saved, reopened, and validated — all within one coherent, memory-safe system.

## The closed loop

```
open → understand → render → inspect/extract → annotate/edit → save/generate → reopen → validate
```

This loop is the thesis. Every subsystem exists to make this loop real on ugly documents. If the loop breaks — if a round trip corrupts the file, if an annotation drifts, if a save destroys structure — the engine has failed, regardless of how many individual features work in isolation.

## What v1 must prove

Monkeybee v1 is not a demo or a roadmap. It must ship with automated, repeatable evidence of the closed loop on ugly documents using a correct baseline engine.

v1 has two explicit lanes:
- **Baseline v1 (release-gating):** the smallest coherent engine that proves the closed loop on ugly documents with simple, auditable implementations.
- **Experimental backends (non-gating):** advanced algorithms, compact encoders, and specialized optimizations that remain optional until they beat the baseline under proof.

- **Rendering correctness** on hard, pathological, real-world PDFs that simpler engines mishandle or refuse.
- **Round-trip integrity**: load → render → modify → save → reload → render again, without corruption or silent drift.
- **Annotation round trips** on ugly files: add annotations, save, reopen, verify geometry and content preservation.
- **Extraction usefulness**: text with positions, metadata, structure, resource inspection, diagnostics.
- **Generation correctness**: documents created by Monkeybee render correctly under both Monkeybee and reference implementations.
- **Compatibility accounting**: every unsupported or degraded zone is explicitly detected, categorized, and surfaced — never silently swallowed.
- **Operational explainability**: the engine can explain edit safety, signature impact, and revision-to-revision deltas in a way users can act on.

## Compatibility doctrine

Monkeybee does not hide from hard PDFs. It operates under a three-tier compatibility doctrine:

- **Tier 1 — Full native support.** If a feature can be supported safely within the architecture, implement it directly.
- **Tier 2 — Safe contained handling.** If native support is not yet practical, explore sandboxed, constrained, or partial handling that preserves safety.
- **Tier 3 — Explicit detected degradation.** If support is not yet feasible, detect the situation, surface it to diagnostics, and degrade in principled, instrumented ways. Silent evasion is unacceptable.

Target categories include: malformed cross-references, broken object graphs, historical font and encoding nightmares, incremental-update oddities, encryption edge cases, transparency/mask/blend edge cases, scanned documents, CJK and RTL text, producer-specific quirks, XFA/hybrid forms (Tier 2/3), PostScript XObjects (Tier 2/3), and hostile/adversarial inputs.

## Architecture at a glance

Monkeybee is a Rust workspace organized as layered crates with explicit preservation and ownership boundaries:

| Crate | Responsibility |
|---|---|
| `monkeybee` | Stable public facade: semver-governed `Engine`, `Session`, `Snapshot`, `EditTransaction`, `WritePlan`, `CapabilityReport`, and high-level open/render/extract/edit/save APIs |
| `monkeybee-core` | Shared primitives: object IDs, geometry, errors, diagnostics, execution budgets, diagnostic streaming (DiagnosticSink), PDF version tracking, StreamHandle contract, provider traits (CryptoProvider, OracleProvider) |
| `monkeybee-bytes` | Byte sources, mmap/in-memory/range-backed access, fetch scheduler (FetchScheduler trait), prefetch planning, revision chain, raw span ownership |
| `monkeybee-codec` | Filter chains, image decode/encode, predictor logic, bounded decode pipelines |
| `monkeybee-security` | Security profiles, worker isolation, budget broker, hostile-input policy |
| `monkeybee-parser` | PDF syntax parsing, repair, tolerant/strict modes, raw token/span retention |
| `monkeybee-syntax` | Syntax/COS preservation layer: immutable parsed objects, token/span provenance, xref provenance, object-stream membership, repair records. The preservation boundary. |
| `monkeybee-document` | Semantic document graph built from syntax snapshots: page tree, inherited state, resource resolution, ownership classes, dependency graph contract, bounded cache management |
| `monkeybee-content` | Content-stream IR + event interpreter shared by render/extract/inspect/edit; consumer sink adapters (RenderSink, ExtractSink, InspectSink, EditSink) |
| `monkeybee-text` | Font programs, CMaps, Unicode mapping, decode pipeline (existing PDF text) and authoring pipeline (shaping/bidi/layout), subsetting, search/hit-test primitives |
| `monkeybee-render` | Page rendering via content events/PagePlan: positioned glyphs, images, transparency, vector graphics, masks, blending; tile/band raster surface; cooperative cancellation; progressive rendering |
| `monkeybee-compose` | High-level authoring and composition: document/page builders, resource naming, annotation/widget appearance synthesis, font embedding planning |
| `monkeybee-write` | Pure serializer: deterministic rewrite, incremental append, WritePlan classification, xref format decision rules, xref/trailer emission, structural validity, final compression/encryption |
| `monkeybee-edit` | Transactional structural edits, resource GC/dedup, redaction application, content stream rewrite pipeline |
| `monkeybee-forms` | AcroForm field tree, value model, appearance regeneration, widget/signature bridge |
| `monkeybee-annotate` | Non-form annotations: creation, modification, flattening, geometry-aware placement |
| `monkeybee-extract` | Multi-surface text extraction, metadata, structure inspection, asset extraction, diagnostics |
| `monkeybee-validate` | Arlington/profile validation, write preflight, signature byte-range checks |
| `monkeybee-proof` | Pathological corpus harness, round-trip validation, render comparison, compatibility accounting |
| `monkeybee-native` | Optional native bridge quarantine: JPX/JBIG2/ICC/FreeType adapters, FFI audit surface, subprocess-friendly broker hooks |
| `monkeybee-cli` | Command-line interface for inspection, rendering, extraction, conversion, diagnostics |

The architecture has four explicit strata: byte/revision, syntax/COS (`monkeybee-syntax` -- the preservation boundary), semantic document (`monkeybee-document`), and content. All crates share `monkeybee-core` for primitives. The syntax layer preserves what the parser saw; the semantic layer builds meaning from it. Rendering, extraction, annotation, editing, and writeback all operate on the same document model, not parallel dead-end parse trees. Core library crates are runtime-agnostic; async orchestration is an adapter concern at the CLI/proof edge.

## Repo structure

```
monkeybee-pdf/
├── README.md                     ← you are here
├── NORTH_STAR.md                 ← constitutional thesis
├── SPEC.md                       ← operational master spec
├── AGENTS.md                     ← agent/swarm coordination
├── Cargo.toml
├── crates/
│   ├── monkeybee/
│   ├── monkeybee-core/
│   ├── monkeybee-bytes/
│   ├── monkeybee-codec/
│   ├── monkeybee-security/
│   ├── monkeybee-parser/
│   ├── monkeybee-syntax/
│   ├── monkeybee-document/
│   ├── monkeybee-content/
│   ├── monkeybee-text/
│   ├── monkeybee-render/
│   ├── monkeybee-compose/
│   ├── monkeybee-write/
│   ├── monkeybee-edit/
│   ├── monkeybee-forms/
│   ├── monkeybee-annotate/
│   ├── monkeybee-extract/
│   ├── monkeybee-validate/
│   ├── monkeybee-proof/
│   └── monkeybee-cli/
├── docs/
│   ├── architecture/
│   ├── implementation/
│   ├── testing/
│   ├── compatibility/
│   └── adr/
├── tests/
│   ├── corpus/
│   ├── render/
│   ├── roundtrip/
│   ├── extraction/
│   ├── annotation/
│   └── fuzz/
└── .apr/
    └── workflows/
```

## Evidence, validation, and release gates

Monkeybee's proof is automated, not rhetorical. The project maintains:

- A **pathological PDF corpus** spanning scanned docs, form-heavy files, broken metadata, transparency edge cases, CJK/RTL, huge files, malformed inputs, complex vector art, and adversarial inputs.
- A **round-trip harness** that exercises load → modify → save → reload → validate cycles.
- **Reference-guided validation** against external renderers (PDFium, MuPDF, pdf.js, Ghostscript) for differential correctness.
- A **compatibility ledger** that tracks every detected degradation, unsupported feature zone, and failure category.
- **Performance baselines** on representative hard workloads.

No feature ships without evidence. No release gate passes on rhetoric.

## Project status

**Phase: Pre-implementation. Canonical docs under APR refinement. Bead conversion pending.**

## Contributing

Monkeybee welcomes contributors. See `AGENTS.md` for the agent/swarm coordination model and `SPEC.md` for the full operational plan.

## License

MIT OR Apache-2.0
