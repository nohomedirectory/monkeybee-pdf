# Monkeybee PDF

Memory-safe, high-performance Rust PDF engine for ugly real-world PDFs.

Monkeybee PDF is being designed as a full-loop document engine, not a one-way parser or renderer.
Its architectural kernel is a persistent, content-addressed document substrate that makes parsing,
rendering, extraction, editing, save planning, reopening, and validation projections of one
coherent system rather than disconnected pipelines.

## Why this exists

Most open-source PDF libraries pick a lane: parse, render, generate, or lightly edit. Real-world
PDFs do not respect those boundaries. They arrive malformed, incrementally updated, font-broken,
producer-quirky, partially hostile, signed, encrypted, scanned, remote, or all of the above.

Proprietary engines dominate that terrain because they own the full lifecycle. Monkeybee exists to
change that. It is a single Rust engine that treats PDFs as a bidirectional semantic and
preservation-aware substrate: not a parse-and-forget tree, not a raster stream, not a thin writer,
but a living document model that can be opened, repaired, understood, rendered, inspected,
extracted from, annotated, edited, generated, serialized, saved, reopened, diffed, and validated —
all within one memory-safe system.

## Two theses, one engine

### 1. The closed loop

```
open → understand → render → inspect/extract → annotate/edit → save/generate → reopen → validate
```

This loop is the user-visible thesis. If the loop breaks — if a save corrupts structure, an
annotation drifts, a form fill destroys signatures, or a reopen yields materially different meaning
without explanation — the engine has failed, no matter how many isolated features appear to work.

### 2. The invariant document substrate

The closed loop is the proof surface. The deeper architectural thesis is that every document state
in Monkeybee must be representable as an invariant-preserving projection of a single persistent,
versioned substrate.

At minimum, the substrate must unify:

- source bytes and preserved byte spans
- parsed COS syntax with provenance, parser token tapes, and salvage indexes
- semantic object graphs and ownership classes
- content interpretation and graphics-state transitions
- derived surfaces such as page plans, render-chunk graphs, coverage-cell indexes and coverage atlases, extraction outputs, text-paint correspondence receipts, semantic anchors, geometry witnesses, scene receipts and scene normal forms, font authority/subset-closure receipts, truth/provenance surfaces, and diffs
- cross-document import provenance, alias maps, import-closure certificates, and semantic-normal-form evidence
- edit deltas, invalidation witnesses, write plans, feasibility witnesses, solver-backed frontier witnesses, emission journals, materialization receipts, and temporal lineage
- proof artifacts, reproducibility manifests, oracle-consensus records, blind-spot ledgers, compatibility ledgers, and invariant certificates

This matters because several promises that are easy to state and hard to make real — cheap
snapshots, structural sharing, exact invalidation, explainable diffs, temporal replay, and
signature-safe save planning — become natural consequences of the substrate instead of loosely
related implementation goals.

## What v1 must prove

Monkeybee v1 is not a demo, not a slide deck, and not a speculative roadmap. It must ship with
automated, repeatable evidence on ugly real-world PDFs using a correct baseline engine.

Monkeybee now has three explicit lanes:

- **Baseline v1 (release-gating):** the smallest coherent engine that proves the closed loop on ugly
  PDFs with simple, auditable implementations and a locked substrate.
- **Experimental backends (non-gating):** advanced render, color, encode, or decode paths that
  remain optional until they beat the baseline under proof and typed equivalence evidence.
- **Post-v1 intelligence/collaboration surfaces:** richer semantic graph queries,
  anchor-driven automation, and stronger provenance layers that are architected now without being
  allowed to destabilize the baseline.

Visible workflows in `SPEC.md` now include not only hostile rendering, annotation/edit round trips,
and progressive open, but also Workflow 13: **Render and interact with 3D PDF content** on
PRC/U3D annotations.

Baseline v1 must prove all of the following:

- **Reader-kernel correctness** on hard, pathological, real-world PDFs that simpler engines mishandle or refuse.
- **Interactive 3D PDF rendering on PRC and U3D content** — the first open-source implementation and a native baseline capability rather than a post-v1 escape hatch.
- **Persistent immutable snapshots** backed by a content-addressed substrate rather than whole-document cloning.
- **Exact dependency-tracked invalidation and cache reuse admissibility** so small edits only recompute affected pages, resources, and derived artifacts, with witnessable reuse/recompute causality and no cross-mode cache poisoning.
- **A first-class numeric/geometry kernel** so rendering, annotation placement, redaction, hit-testing, function/color/prepress math, and 3D cross-sections all consume one auditable tolerance, degeneracy, and escalation doctrine.
- **Round-trip integrity**: load → render → modify → save → reload → render again, without corruption or silent semantic drift.
- **Cross-document import integrity**: copy/merge/split workflows remap resources and identities explicitly, preserve provenance, emit import-closure evidence, and never silently collide.
- **Annotation and form round trips** on ugly files, including appearance regeneration and preserve-mode save planning.
- **Preservation-aware save planning**: before bytes are emitted, the engine can explain what will be preserved, rewritten, appended, or invalidated, compute save feasibility from an explicit constraint graph, cite minimal blocking sets when preserve-mode goals are unsatisfiable, and surface the nearest legal alternative plans when that frontier is cheap to compute.
- **Policy-complete operation planning**: open/import/save decisions compose security, active-content, provider, and determinism policy up front and reject invalid combinations before execution.
- **Receiptable derived artifacts, write receipts, and invariant certificates** for save, diff, redaction, render, extraction, and cache-reuse workflows.
- **Durable persisted artifacts**: file-backed saves and persisted proof artifacts publish atomically and never leave ledgers, receipts, or manifests pointing at partial blobs.
- **Ambiguity truthfulness**: materially different repair candidates stay visible as a bounded hypothesis set rather than being silently collapsed.
- **Extraction usefulness**: text with positions, metadata, structure inspection, asset inspection, diagnostics, explicit truth surfaces, and the early semantic layers needed for anchor stability.
- **Generation correctness**: documents created by Monkeybee render correctly under both Monkeybee and reference implementations.
- **Compatibility accounting**: every unsupported or degraded zone is explicitly detected, categorized, surfaced in a generated capability surface matrix, and never silently swallowed.
- **Reproducible proof evidence**: canonical runs emit pinned reproducibility manifests, environment locks, typed oracle-consensus and oracle-disagreement records with region-level explainability, blind-spot ledgers, coverage lattices, metamorphic witnesses with fixture genealogy, and plan-selection evidence linked back to ledgers, evidence bundles, and failure capsules.
- **Fault-contained execution and deterministic render classes**: operator/page/query/native failures stay contained and diagnosable, and proof-canonical rendering is explicitly separated from viewer-adaptive and experimental paths.
- **Witness-backed performance claims**: release-facing performance numbers come from schema-versioned benchmark witnesses tied to reproducibility manifests and annotated with runtime-topology evidence, work-class receipts, segmented working-set forecasts, and peak-memory witnesses, not ad hoc timing logs.
- **Operational explainability**: the engine can explain edit safety, signature impact, revision-to-revision deltas, anchor fragility, invalidation causes, and transport continuity in a way users can act on.

Baseline anti-goals remain narrow: Monkeybee is not adding OCR, document understanding,
accessibility remediation, PDF/UA-2 generation/validation, or semantic format conversion to v1.

## Expansion lanes that the architecture already has to own

Monkeybee is not trying to finish the entire PDF category inside the baseline gate, but it is
explicitly ambitious enough to own the lanes that make serious engines matter in enterprise,
compliance, prepress, and forensics environments. Those lanes are now captured in `SPEC.md` and
`docs/implementation/implementation_master.md` as additive expansion contracts rather than
hand-waved future wishes.

- **Enterprise print-production and prepress:** halftones, transfer functions, black generation /
  undercolor removal, RGB overprint simulation, soft proofing, ink coverage analysis, separation
  preview, print preflight, and trap-network handling, including halftone Types 1/5/6/10/16,
  spot-function and threshold-screen inspection, document/page-level output-intent handling, and
  explicit shared proof conditions for meaningful soft-proof/separation comparisons.
- **Digital signature lifecycle:** PAdES B-B/B-T/B-LT/B-LTA, DSS/VRI, certificate-chain building,
  OCSP/CRL/TSA evidence, offline long-term validation, and signature creation rather than mere
  preservation, with explicit modeling of per-signature validation material,
  incremental-append signing workflows, reservation planning, and append-budget evidence.
- **Tagged PDF and accessibility auditing:** richer structure semantics, `/ActualText`, `/Alt`,
  `/E`, `/Lang`, pronunciation hints, artifact handling, structure destinations, the full standard
  structure-role family, PDF/UA audit reports, and reading-order visualization without promising
  remediation in baseline v1.
- **Advanced forms and interchange:** FDF/XFDF import/export, form flattening, JavaScript and
  submit-action detection, signature-field creation, barcode fields, and Tier 2 static-XFA
  flattening where safe, with form flattening treated as a distinct field-aware operation rather
  than generic annotation flattening.
- **Action and forensics surfaces:** full action-type inventory, document link-map extraction,
  execute-deny active-content analysis, and preservation of all action dictionaries during round
  trip, covering the full PDF action family from GoTo/URI/Launch through RichMediaExecute.
- **Rich document structure and multimedia cataloging:** article threads, page transitions,
  thumbnails, collections/portfolios, alternate presentations, page-piece dictionaries, web-capture
  structures, screen/movie/sound annotations, rendition actions, media clips, and rendition trees.
- **Rendering-quality uplifts:** N-dimensional sampled-function interpolation, better resampling
  kernels, shading-edge anti-aliasing, and robust matte un-premultiplication algorithms, with
  Lanczos-3 downscaling and Mitchell-Netravali upscaling as named target kernels.
- **Deep correctness and hardening surfaces:** redaction canary scanning across the entire emitted
  file, CFF subroutine decompilation/recompilation, damaged-Type-1 alternate-key recovery,
  RoleMap-chain resolution, inline-image resource leakage tolerance, name/number-tree `/Limits`
  repair, marked-content nesting repair, page/resource-level metadata enumeration, optional-content
  configuration sequences, `/Trapped` tri-state semantics, blend-mode preference lists, Type 4
  function complexity analysis, stream-extent validation, font-descriptor flag cross-checking,
  CIDFont vertical metrics, ICC-version hazard reporting, `/Identity` crypt-filter no-op handling,
  cloudy annotation borders, alternate image selection, custom spot-function cataloging, PDF 2.0
  DeviceN attributes, output-intent condition lookup, JavaScript trigger graph extraction, web
  capture provenance, structured destinations, and certification-vs-approval signature
  classification.

These are not excuses to dilute the closed-loop proof. They are the high-value adjacencies that a
closed-loop engine naturally grows into once the substrate, preservation model, and proof harness
are real.

Expansion-wave ordering is also explicit so APR and proof work do not diffuse across too many
surfaces at once:

- **Wave 1 — highest-signal adoption surfaces:** PAdES creation/LTV, PDF/UA-style audit, prepress
  inspection/proofing, FDF/XFDF plus form flattening, and full action inventory/link-map
  extraction. These are the lanes that most directly unlock legal/compliance workflows, print-shop
  evaluation, government-form interchange, and document forensics.
- **Wave 2 — surrounding document-reality inventory:** article threads, portfolios, page
  transitions, thumbnails, alternate presentations, page-piece/web-capture structures, and
  multimedia/rendition inventory. These strengthen preservation, inspection, and forensics once
  the first wave is real.
- **Wave 3 — rendering-quality uplifts:** Lanczos/Mitchell resampling, N-dimensional sampled
  function interpolation, shading-edge anti-aliasing, and matte un-premultiplication. These are
  important, but they compound best after the first two waves have stable proof fixtures and report
  surfaces.

Counting policy is explicit. The locked current inventory remains **104 named algorithms and
techniques**. The first forward-looking number is **143**, meaning `104 + 39` priority uplift
families. A broader inclusive planning number of **155** adds **12** document-structure and
multimedia catalog lanes tracked separately because they are primarily parse/expose/preserve
surfaces rather than new math-heavy kernels. This revision also adds a second explicit
**deep-correctness and hardening uplift of +26**, bringing the priority-plus-hardening planning
number to **169** and the fully inclusive planning number to **181**. All of these counts are
valid when the counting policy is stated. What matters is that these additions are not cosmetic
backlog items: they are the lanes that make the engine matter in prepress, regulated e-signature,
accessibility compliance, government-form interchange, document forensics, and high-assurance
redaction/font/metadata correctness.

For APR/public-round arithmetic, the priority uplift is intentionally spelled out:

- original spec inventory: `104`
- print production: `+9`
- digital signature lifecycle: `+8`
- tagged PDF / accessibility: `+10`
- advanced rendering quality: `+4`
- advanced forms and interchange: `+7`
- full action catalog and link-map extraction: `+1`
- deep correctness and hardening uplift: `+26`
  - redaction, signatures, and active-content forensics: `+3`
  - font resilience and text correctness: `+4`
  - structure and metadata integrity: `+5`
  - parser/render hardening: `+6`
  - prepress and color fidelity: `+6`
  - OCG and annotation rendering detail: `+2`

That yields two useful APR-facing totals:

- **`104 + 39 = 143`** for the original priority uplift framing
- **`104 + 39 + 26 = 169`** for the priority-plus-hardening framing now needed by APR/proof work

Using the current APR comparison shorthand of **FrankenTUI at `30+`**, Monkeybee's original
`143`-item framing remains nearly **5x** the named algorithmic breadth, while the `169`-item
priority-plus-hardening framing is well beyond **5x**.

For APR rounds, demos, and public-signaling compression, the highest-signal additions to foreground
are:

1. **PAdES digital-signature creation and long-term validation** because it makes the engine
   immediately useful for legal and regulatory workflows rather than mere passive signature
   preservation.
2. **PDF/UA accessibility auditing** because accessibility requirements increasingly carry legal
   force and produce crisp, reportable proof artifacts.
3. **Print-production pipeline coverage** because ink coverage, soft proofing, separation preview,
   preflight, and trap analysis are where enterprise prepress value lives.
4. **FDF/XFDF round-trip and form flattening** because they create instant utility for
   government-form and regulated-document workflows.
5. **Full action catalog and link-map extraction** because they sharpen the forensics/security
   narrative: the engine can enumerate what a document would attempt to execute, launch, or
   navigate.

For the second hardening pass, the highest-signal additions to foreground are:

1. **Redaction canary scanning across the entire emitted file** because real redaction failures
   happen when text survives in metadata, names, attachments, form values, or font tables rather
   than in visible content alone.
2. **CFF subroutine recompilation and damaged-Type-1 recovery** because getting font subsetting and
   damaged embedded-font repair right is rare, technically sharp, and highly defensible.
3. **JavaScript trigger-graph extraction** because trigger timing across open/page/annotation/form/
   print/save surfaces is forensically useful even when execution is denied.
4. **Certification-vs-approval signature classification with MDP-chain validation** because it
   turns vague DocMDP support into a concrete trust-policy narrative.
5. **DeviceN/ICC/output-intent/trapped prepress hazard reporting** because it demonstrates that the
   engine understands real print semantics rather than generic RGB rendering only.

## Compatibility doctrine

Monkeybee does not hide from hard PDFs. It operates under a three-tier compatibility doctrine:

- **Tier 1 — Full native support.** If a feature can be supported safely within the architecture, implement it directly.
- **Tier 2 — Safe contained handling.** If native support is not yet practical, explore sandboxed, constrained, or partial handling that preserves safety.
- **Tier 3 — Explicit detected degradation.** If support is not yet feasible, detect the situation, surface it to diagnostics and ledgers, and degrade in principled, instrumented ways. Silent evasion is unacceptable.

This doctrine applies not only to rendering features but also to repair ambiguity, risky decoders,
active content, preserve-mode workflows, and writeback behavior. Monkeybee is allowed to say
"detected, constrained, and not yet safely transformable." It is not allowed to pretend a hard zone
does not exist.

Target categories include malformed cross-references, broken object graphs, historical font and
encoding nightmares, incremental-update oddities, encryption edge cases, transparency/mask/blend
edge cases, scanned documents, CJK and RTL text, producer-specific quirks, XFA and hybrid forms
(Tier 2/3), PostScript XObjects (Tier 2/3), active content, and hostile or adversarial inputs.

## Architecture at a glance

Monkeybee is a Rust workspace organized around six explicit strata:

1. **Byte/revision layer** — immutable source bytes, fetch-epoch continuity, sparse-convergence trust state, appended revisions, and range-backed access.
2. **Persistent substrate/query layer** — content-addressed roots, temporal lineage, structural
   sharing, exact invalidation, store lifecycle, segmented out-of-core materialization, acceleration indexes, materialization receipts, hypothesis tracking, and invariant certificates.
3. **Syntax/COS layer** — immutable parsed objects, provenance, repair records, raw formatting, and
   the preservation boundary.
4. **Semantic document layer** — resolved page/resource/object meaning, ownership classes, cross-document import provenance, and semantic normal forms.
5. **Content layer** — graphics-state interpretation, shared IR for render/extract/inspect/edit, and page-space evidence indexes.
6. **Facade/report layer** — stable public API, diff/signature/report surfaces, proof harness, and CLI.

Workspace crates:

| Crate | Responsibility |
|---|---|
| `monkeybee` | Stable public facade: semver-governed `Engine`, `OpenProbe`, `Session`, `Snapshot`, `EditTransaction`, `CapabilityReport`, `WritePlan`, `WriteReceipt`, `DiffReport`, and high-level open/render/extract/edit/save APIs |
| `monkeybee-core` | Shared primitives: object IDs, the first-class geometry/numeric-robustness kernel, errors, diagnostics, execution context, version tracking, scope bindings, provider traits, and operation-profile/policy-composition primitives |
| `monkeybee-bytes` | Byte sources, mmap/in-memory/range-backed access, fetch scheduling, access plans, revision chains, raw-span ownership |
| `monkeybee-security` | Security profiles, worker isolation, budget broker, hostile-input policy |
| `monkeybee-codec` | Filter chains, image decode/encode, predictor logic, bounded decode pipelines |
| `monkeybee-parser` | PDF tokenization, immutable parser artifact tapes, salvage indexes, syntax parsing, repair, encryption handling, tolerant/strict/preserve ingestion |
| `monkeybee-substrate` | Persistent incremental kernel: node digests, content-addressed store, snapshot roots, lineage, store lifecycle, hypothesis sets, query engine, materialized acceleration indexes, exact invalidation plus invalidation witnesses, temporal replay scaffolding, and invariant certificate generation |
| `monkeybee-syntax` | Syntax/COS preservation layer: immutable parsed objects, token/span provenance, xref provenance, object-stream membership, formatting retention, repair records. The preservation boundary |
| `monkeybee-document` | Semantic document graph built from syntax snapshots and substrate roots: page tree, inherited state, resource resolution, ownership classes, dependency graph contract, transaction/change journal, semantic object indexes, cross-document import/remap provenance, import-closure evidence, and semantic-normal-form evidence |
| `monkeybee-catalog` | Catalog semantics: outlines, named destinations, page labels, name/number trees, viewer preferences, optional-content configs, embedded-file inventory, collections, presentation metadata, and document-level rich-structure roots |
| `monkeybee-content` | Content-stream IR + interpreter shared by render/extract/inspect/edit; graphics-state algebra and sink adapters |
| `monkeybee-text` | Font programs, CMaps, Unicode mapping, font-authority graphs, deterministic subset-closure receipts, decode pipeline for existing PDF text, authoring pipeline for generated text, subsetting, search/hit-test primitives |
| `monkeybee-color` | Shared color/prepress kernel: ICC evaluation, output-intent cascade, DeviceN/Separation resolution, soft-proofing, TAC accounting, and color witness emission |
| `monkeybee-render` | Page rendering via shared content interpretation: text, images, vector graphics, masks, blending, prepress proof modes, tile/band surfaces, render-chunk graphs, cooperative cancellation, and progressive rendering |
| `monkeybee-3d` | PRC/U3D parsing, unified scene graph, wgpu rendering (Vulkan/Metal/DX12/WebGPU), named views, cross-sections, illustration modes |
| `monkeybee-gpu` | Optional GPU-accelerated 2D rendering backend via wgpu, shared device/queue with the 3D pipeline, compute shader path rasterization, hardware compositing |
| `monkeybee-compose` | High-level authoring and composition: document/page builders, appearance synthesis, resource naming, font embedding planning |
| `monkeybee-write` | Pure serializer: deterministic rewrite, incremental append, preservation-aware `WritePlan` execution, replayable emission journals, xref/trailer emission, structural validation, and final compression/encryption |
| `monkeybee-edit` | Transactional structural edits, resource GC/dedup, redaction application, content-stream rewrite pipeline |
| `monkeybee-forms` | AcroForm field tree, value model, appearance regeneration, FDF/XFDF interchange, flattening, widget/signature bridge, and static-XFA helpers |
| `monkeybee-paint` | Shared page-independent paint and appearance primitives reused by render, compose, forms, and annotate |
| `monkeybee-annotate` | Non-form annotations: creation, modification, flattening, geometry-aware placement |
| `monkeybee-extract` | Multi-surface extraction, metadata, structure inspection, accessibility semantics, text-truth/provenance surfaces, text-paint correspondence, action/link inventories, rich-structure cataloging, semantic anchors, typed query/selection helpers, diagnostics |
| `monkeybee-forensics` | Hidden-content detection, redaction audits, post-signing modification analysis, active-content analysis, print-risk analysis, exploit-pattern detection, producer/font fingerprinting |
| `monkeybee-validate` | Arlington/profile validation, print preflight, PDF/UA-style audit, PAdES/LTV checks, write preflight, signature byte-range checks |
| `monkeybee-diff` | Structural, text, render, save-impact, and revision-frame comparison engine reused by the facade, proof harness, and CLI |
| `monkeybee-signature` | Signature parsing, byte-range preservation, PAdES/DSS/VRI modeling, OCSP/CRL/TSA handling, signature creation, DocMDP/FieldMDP policy, verification plumbing, and save-impact analysis |
| `monkeybee-proof` | Pathological corpus harness, round-trip validation, render comparison, compatibility and hypothesis ledgers, reproducibility manifests, oracle-consensus/disagreement records, blind-spot ledgers, plan-selection evidence, benchmark witnesses, certificate recomputation, and regression gates |
| `monkeybee-native` | Optional native bridge quarantine: JPX/JBIG2/ICC/FreeType adapters, FFI audit surface, native-isolation attestations, subprocess-friendly broker hooks |
| `monkeybee-cli` | Command-line interface for inspection, rendering, extraction, prepress/signature/accessibility reporting, diffing, plan-save previews, diagnostics, proof execution |

Core library crates remain runtime-agnostic: they accept `&ExecutionContext` and do not import
`asupersync` directly. The facade, bytes, proof, and CLI crates are `asupersync`-native. CPU-bound
work remains Rayon-based and bridges back into structured async regions via oneshot channels.
Canonical proof also distinguishes render determinism classes and native isolation classes so
viewer-adaptive paths or quarantined native paths never masquerade as proof-canonical evidence.

## Foundation freeze before fan-out

The next implementation step is not "start coding random crates." It is the substrate-first
foundation freeze, because several downstream guarantees depend on decisions that must be stable
before subsystem fan-out.

**Slice F — Foundation freeze** locks:

- identity model (`DocumentId`, `SnapshotId`, `NodeDigest`, `ResourceFingerprint`, `SemanticAnchorId`)
- content-addressed snapshot roots and lineage schema
- incremental query engine and exact invalidation semantics
- geometry-kernel tolerance policy and geometry-witness schema
- preservation algebra, `WritePlan`, `WriteReceipt`, and `InvariantCertificate` schema
- capability-surface-matrix derivation rules from scope + proof artifacts
- policy-composition validity and plan-selection evidence
- hypothesis-set and ambiguity-reporting contract
- temporal revision model and semantic-anchor stability rules
- substrate-store lifecycle and acceleration-index freshness semantics
- cross-document import provenance and semantic-normal-form rules
- cache namespace and reuse-admissibility doctrine, including determinism/provider-manifest/module/isolation boundaries
- reproducibility-manifest and oracle-disagreement schema boundaries
- scope registry/test-gate bootstrap

Only after Slice F is ratified does Monkeybee fan out into the reader kernel, preserve loop,
progressive/remote, and proof slices.

The reader-kernel slice explicitly includes baseline 3D annotation detection and baseline 3D
render for PRC/U3D annotations (PRC/U3D parsing plus static 3D scene rendering) so that 3D PDF
support is part of the v1 proof surface rather than a deferred niche lane.

## Repo structure

```
monkeybee-pdf/
├── README.md                     ← you are here
├── NORTH_STAR.md                 ← constitutional thesis
├── SPEC.md                       ← operational master spec
├── docs/implementation/implementation_master.md      ← APR-facing implementation reference
├── AGENTS.md                     ← agent/swarm coordination
├── Cargo.toml
├── crates/
│   ├── monkeybee/
│   ├── monkeybee-core/
│   ├── monkeybee-bytes/
│   ├── monkeybee-security/
│   ├── monkeybee-codec/
│   ├── monkeybee-parser/
│   ├── monkeybee-substrate/
│   ├── monkeybee-syntax/
│   ├── monkeybee-document/
│   ├── monkeybee-catalog/
│   ├── monkeybee-content/
│   ├── monkeybee-text/
│   ├── monkeybee-color/
│   ├── monkeybee-render/
│   ├── monkeybee-3d/
│   ├── monkeybee-gpu/
│   ├── monkeybee-compose/
│   ├── monkeybee-write/
│   ├── monkeybee-edit/
│   ├── monkeybee-forms/
│   ├── monkeybee-paint/
│   ├── monkeybee-annotate/
│   ├── monkeybee-extract/
│   ├── monkeybee-forensics/
│   ├── monkeybee-validate/
│   ├── monkeybee-proof/
│   ├── monkeybee-diff/
│   ├── monkeybee-signature/
│   ├── monkeybee-native/
│   └── monkeybee-cli/
├── docs/
│   ├── scope_registry.yaml
│   ├── architecture/
│   ├── implementation/
│   ├── testing/
│   ├── compatibility/
│   └── adr/
├── tests/
│   ├── corpus/
│   │   ├── public/**/expectation.yaml
│   │   ├── restricted/**/expectation.yaml
│   │   ├── generated/**/expectation.yaml
│   │   └── minimized/**/expectation.yaml
│   ├── render/
│   ├── roundtrip/
│   ├── extraction/
│   ├── annotation/
│   ├── temporal/
│   ├── anchors/
│   └── fuzz/
└── .apr/
    └── workflows/
```

## Evidence, validation, and release gates

Monkeybee's proof is automated, not rhetorical. The project maintains:

- A **pathological PDF corpus** spanning scanned docs, form-heavy files, broken metadata,
  transparency edge cases, CJK/RTL, huge files, malformed inputs, complex vector art, signed
  workflows, and adversarial inputs.
- A **round-trip harness** that exercises load → modify → save → reload → validate cycles.
- **Reference-guided validation** against external renderers (PDFium, MuPDF, pdf.js, Ghostscript)
  for differential correctness.
- A **compatibility ledger** that tracks every detected degradation, unsupported feature zone, and
  failure category.
- A **hypothesis ledger** for ambiguous repairs so materially different candidates remain visible.
- **Oracle-consensus records and blind-spot ledgers** so release-facing claims reflect both how
  disputed expectations were chosen, where fixture coverage is still thin, and which feature ×
  producer × operation intersections remain under-covered.
- **Coverage-lattice acquisition recommendations** so corpus growth is driven by typed
  under-covered cells rather than only by ad hoc fixture hunting.
- A **metamorphic proof lane with deterministic reducers and fixture genealogy** so
  representation-changing transforms, crash minimization, and repair drift stay auditable.
- **Write receipts and invariant certificates** for save-impact, preserve-mode, and redaction workflows.
- **Anchor-stability and temporal-replay harnesses** on representative fixtures so post-v1 surfaces
  are forced to grow from stable primitives instead of hand-waving.
- **Performance baselines** on representative hard workloads.
- **Schema-versioned benchmark witnesses** that bind support class, render determinism class,
  runtime-topology evidence, work-class receipts, peak-memory witnesses, and threshold verdicts to reproducibility manifests.
- **Environment locks and evidence bundles** so benchmark claims, regressions, and bug reports can
  be published as content-addressed reproducer bundles without relying on ambient host state.
- A **generated capability surface matrix** derived from the scope registry plus proof outputs so
  README, website, CLI, and CI capability claims cannot drift apart.
- **Durable artifact publication rules** so saved outputs, ledgers, capsules, and witnesses are
  published atomically and never reference partial child artifacts.

No feature ships without evidence. No release gate passes on rhetoric. No architectural promise is
accepted until a proof surface exists for it.

The current locked baseline inventory names 104 algorithms and techniques across baseline,
post-baseline, and experimental lanes (57 pre-existing plus 47 restored or added), including
restored 3D rendering, GPU backends, PDF 2.0 supplements, forensics, subpixel text, advanced path
geometry, compression, and hot-path optimizations. The priority uplift captured in the current
specification adds 39 more named capabilities for a forward-looking total of 143. A separate
supporting inventory bucket adds 12 document-structure and multimedia catalog lanes, which yields
an inclusive planning total of 155 when those preserve/expose surfaces are counted too. This
revision also adds a 26-item deep-correctness and hardening uplift, producing a 169-item
priority-plus-hardening planning total and a fully inclusive planning total of 181 when all named
uplift families are counted.

## Specification philosophy

This project's specification (`SPEC.md`) and implementation plan are designed to be exhaustively
comprehensive — an alien artifact in the only sense that matters. The specification MUST only grow
in detail, precision, and breadth. Suggestions to consolidate, simplify, streamline, or reduce the
specification are categorically rejected. Every section, struct, contract, and narrative exists
because the PDF domain demands it.

That does **not** mean implementation order is chaotic. The delivery doctrine is:

1. baseline, auditable kernels before exotic backends
2. substrate and invariants before subsystem fan-out
3. proof surfaces before release gates
4. post-v1 intelligence layers only on top of a stable kernel

Refinement means adding precision and coverage, never subtracting content.

## Project status

**Phase: Pre-implementation. Canonical docs under APR refinement. Immediate priority: Slice F foundation freeze and bead conversion.**

## Contributing

Monkeybee welcomes contributors. See `AGENTS.md` for the agent/swarm coordination model,
`SPEC.md` for the full operational plan, and `docs/implementation/implementation_master.md` for the APR-facing
implementation reference.

## License

MIT OR Apache-2.0
