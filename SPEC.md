# Monkeybee PDF Specification

## Purpose

This document is the operational master specification for Monkeybee PDF.

It translates the thesis in `NORTH_STAR.md` into build rules, crate boundaries, runtime choices, and delivery gates. If there is a conflict between implementation convenience and this document, this document wins unless it is explicitly revised.

## Closed-loop product claim

Monkeybee PDF is a closed-loop PDF engine:

`open -> understand -> render -> inspect/extract -> annotate/edit -> save/generate -> reopen -> validate`

Every major subsystem must strengthen that loop on ugly real-world PDFs. No subsystem is allowed to optimize itself by making the loop less believable.

## Runtime and concurrency contract

Monkeybee PDF standardizes on Jeffrey Emanuel's `asupersync` as its async runtime.

### Non-negotiable runtime rules

- `asupersync` owns all async I/O, orchestration, cancellation, timeout budgeting, and task supervision.
- Rayon owns CPU-bound parallelism over in-memory work.
- The two compose, but they do not overlap in responsibility: `asupersync` coordinates the work graph, and Rayon executes bounded CPU-heavy kernels.
- Tokio is not a first-class runtime in this workspace. If a third-party dependency forces Tokio interoperability later, it must be isolated behind an adapter boundary rather than becoming the project's ambient runtime model.

### Asupersync usage rules

- Native `asupersync` APIs come first. Monkeybee should not design around a Tokio-shaped abstraction and then wrap it later.
- Top-level async entrypoints thread `&Cx<'_>` through I/O-heavy workflows so cancellation, deadlines, and supervision remain explicit.
- CLI commands, proof-harness jobs, corpus walkers, and any service-style entrypoints bootstrap with `RuntimeBuilder`, execute inside `LabRuntime`, and structure child work with `Scope`.
- Detached background work is disallowed unless it is a deliberate long-lived supervised service owned by the runtime.
- Async boundaries stay at orchestration edges: file opens, buffered reads and writes, corpus traversal, artifact emission, external process coordination, and workflow scheduling.

### Rayon usage rules

- Rayon is the standard mechanism for CPU-bound parallelism: page rendering, image decode and transform stages, filter and compression work, render-diff computation, extraction batches, and other bounded compute kernels.
- CPU-heavy functions should consume already-materialized inputs and return owned results so they remain easy to schedule from `asupersync` regions.
- Rayon jobs must not hide ad hoc I/O or create their own async runtime assumptions.

### Composition rule

The intended shape is:

1. `asupersync` acquires inputs, owns task lifetimes, and defines cancellation and timeout policy.
2. CPU-heavy stages hand off pure in-memory work to Rayon.
3. Results return to the enclosing `asupersync` scope for aggregation, persistence, diagnostics, and downstream scheduling.

## Workspace architecture

Monkeybee PDF is a Rust workspace with focused crates over a shared document core:

- `monkeybee-core`: object model, cross-references, page tree, resources, shared geometry, shared errors, change tracking
- `monkeybee-parser`: byte parsing, repair, tolerant mode, decryption, diagnostics
- `monkeybee-render`: page interpretation, graphics state, fonts, images, transparency, backend output
- `monkeybee-write`: serialization, generation, rewrite, incremental save, structural validation
- `monkeybee-annotate`: annotation modeling, placement, appearance, flattening, round-trip helpers
- `monkeybee-extract`: text extraction, metadata, structure inspection, asset extraction, diagnostics
- `monkeybee-proof`: corpus harnesses, render comparisons, round-trip evidence, benchmarks, compatibility accounting
- `monkeybee-cli`: thin command layer over the libraries

## Crate-level runtime posture

- `monkeybee-core` stays runtime-agnostic and does not own async orchestration.
- `monkeybee-parser`, `monkeybee-write`, `monkeybee-extract`, and `monkeybee-proof` may expose `asupersync`-native async surfaces for I/O-heavy workflows.
- `monkeybee-render` and performance-critical inner loops stay primarily synchronous and compose with Rayon for CPU-bound execution.
- `monkeybee-cli` is the main runtime bootstrap point for local commands, batch workflows, and operator tooling.

## Architecture rules

- Shared document reality wins over subsystem-local shortcuts.
- Parsing, rendering, extraction, mutation, writeback, and proof machinery must stay separable enough to test independently.
- Compatibility handling must be explicit, observable, and ledgered.
- Any unsupported or degraded feature zone must surface diagnostics rather than fail silently.
- Memory safety is non-negotiable. `unsafe` remains exceptional and justified.

## Proof obligations

No capability is considered real until it is backed by automated evidence:

- pathological corpus coverage
- reference-guided render validation
- round-trip validation
- annotation save/reopen checks
- extraction usefulness checks
- performance baselines on representative hard inputs
- compatibility ledger entries for unsupported or degraded zones

## Delivery gates

Before work is considered complete:

- implementation aligns with this spec and the relevant implementation doc
- the affected crate tests pass
- round-trip expectations still hold for previously passing corpus cases
- diagnostics and compatibility accounting are updated when behavior changes
- runtime behavior respects the `asupersync` plus Rayon contract above

## Implementation alignment

The implementation detail document at `docs/implementation/implementation_master.md` is the code-facing companion to this spec. Subsystem docs under `docs/implementation/` refine this spec further, but they may not weaken the runtime and concurrency contract defined here.
