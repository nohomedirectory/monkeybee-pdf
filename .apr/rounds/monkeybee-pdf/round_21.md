I reviewed the README, implementation master, and SPEC together before proposing changes.   

My overall take: the plan is already pointed at the right center of gravity. The strongest parts are the insistence on a single persistent substrate, exact invalidation, typed receipts/certificates, policy-complete planning, geometry as a first-class kernel, and proof artifacts that are durable and release-gating rather than ornamental.    

The best additive revisions are therefore not “more random features.” They are second-order contracts that keep an engine this broad from drifting: execution-topology discipline, memory-residency truth, numerics beyond geometry, font-truth witnesses, region-level oracle disagreement, counterfactual save planning, coverage lattices, anchor fragility envelopes, and environment/evidence lockfiles. Those are the missing bindings that would make the existing alien-artifact direction even harder to exhaust.    

## 1) Add an execution-topology and work-class doctrine

The current plan already distinguishes asupersync-owned orchestration from Rayon-owned CPU work, and benchmark witnesses already record allocator, SIMD class, NUMA policy, and runtime topology. But there is still no first-class typed vocabulary for *what kinds of work* exist, how they interfere, which ones may co-reside, and which ones must never silently starve or contaminate proof-canonical runs. On a system this broad, scheduler behavior becomes part of correctness, not just performance.  

This change would make open/parse, stream decode, render, extract, query materialization, proof execution, native isolation, and persistence publication all show up as typed workload classes with explicit co-scheduling rules and measurable receipts. That would tighten reproducibility, make benchmark witnesses more causally useful, and prevent “performance regressions” that are actually topology regressions.  

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Runtime and concurrency model
+
+### Execution topology and work-class doctrine
+
+Monkeybee MUST treat execution topology as a first-class correctness and proof
+surface rather than a hidden runtime detail.
+
+```rust
+pub enum WorkClass {
+    Probe,
+    Parse,
+    Decode,
+    Render,
+    Extract,
+    QueryMaterialize,
+    SavePlan,
+    Serialize,
+    PersistArtifact,
+    NativeIsolated,
+    ProofOracle,
+}
+
+pub enum InterferenceClass {
+    LatencyCritical,
+    ThroughputPreferred,
+    MemoryBurst,
+    IOBound,
+    DeterminismSensitive,
+    IsolationRequired,
+}
+
+pub struct ExecutionLane {
+    pub lane_id: String,
+    pub work_class: WorkClass,
+    pub interference: InterferenceClass,
+    pub support_class: SupportClass,
+    pub concurrency_cap: Option<u32>,
+}
+
+pub struct TopologyPolicy {
+    pub lanes: Vec<ExecutionLane>,
+    pub allow_render_decode_co_residency: bool,
+    pub allow_oracle_parallelism: bool,
+    pub reserve_latency_lane_for_viewport_work: bool,
+}
+
+pub struct WorkReceipt {
+    pub work_class: WorkClass,
+    pub lane_id: String,
+    pub wall_time_ms: u64,
+    pub cpu_units: u64,
+    pub peak_memory_bytes: u64,
+    pub blocked_on: Vec<WorkClass>,
+}
+```
+
+Rules:
+- proof-canonical runs MUST emit topology-qualified receipts for all major work
+  classes
+- viewport-visible work MUST be isolatable from background proof, prefetch, and
+  persistent-artifact publication work
+- topology drift that changes benchmark or correctness outcomes is a typed
+  artifact, not an anecdotal performance note
````

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 ## Runtime and concurrency model
+
+- add `monkeybee-core::topology` for `WorkClass`, `InterferenceClass`,
+  `ExecutionLane`, `TopologyPolicy`, and `WorkReceipt`
+- require `BenchmarkWitness` to attach topology receipts for parse/decode/
+  render/query/save-plan phases on canonical runs
+- add stress tests proving that proof/oracle work cannot silently starve
+  viewport-critical render lanes under configured policies
```

## 2) Add a residency and memory-hierarchy truth surface

The spec already has caches, a scratch spill store, a persistent derived-artifact store, and durable publication rules. What is still missing is a unified per-artifact residency model: when something is in memory, pinned, spillable, persisted, revalidated, or deliberately evicted under pressure. Right now those ideas exist, but they are not yet unified into one typed story.  

That matters because Monkeybee is explicitly targeting huge, hostile, remote, signed, graphics-heavy files. If the engine cannot explain *why* a decoded stream stayed in RAM, why a tile spilled, or why a query artifact was dropped and recomputed, then memory pressure will become a source of hidden nondeterminism. This addition would make memory behavior auditable, testable, and benchmarkable.  

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Caching strategy:
+
+### Residency and memory-hierarchy doctrine
+
+Every substantial derived or decoded artifact MUST carry explicit residency
+state and transition history.
+
+```rust
+pub enum ResidencyClass {
+    HotMemory,
+    WarmMemory,
+    PinnedMemory,
+    ScratchSpilled,
+    PersistedReusable,
+    RevalidatedRemote,
+    Evicted,
+}
+
+pub enum ResidencyReason {
+    ViewportCritical,
+    SignatureSensitive,
+    ProofArtifact,
+    SharedAcrossPages,
+    CachePressure,
+    PolicyRestricted,
+}
+
+pub struct ResidencyTransition {
+    pub artifact_digest: [u8; 32],
+    pub from: ResidencyClass,
+    pub to: ResidencyClass,
+    pub reason: ResidencyReason,
+}
+
+pub struct PeakMemoryWitness {
+    pub operation_kind: OperationKind,
+    pub artifact_digests: Vec<[u8; 32]>,
+    pub peak_bytes: u64,
+    pub spill_bytes: u64,
+    pub eviction_count: u64,
+}
+```
+
+Rules:
+- page plans, render chunks, decoded streams, font authorities, query
+  materializations, and proof artifacts MUST be residency-classified
+- memory-pressure degradation MUST emit typed transitions rather than only
+  aggregate counters
+- benchmark witnesses SHOULD attach `PeakMemoryWitness` for canonical workload
+  classes
````

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 - `docs/implementation/store-lifecycle.md` — substrate root pinning, spill policy, persistence eligibility,
+- `docs/implementation/residency.md` — artifact residency classes, spill/evict/revalidate transitions,
+  peak-memory witnesses, cache-pressure diagnostics, and persistence boundaries
```

## 3) Extend the numeric doctrine beyond geometry into color, functions, and prepress math

Monkeybee already has one of the strongest parts of the plan here: numerically delicate geometry is centralized, robustness classes are explicit, and benchmark witnesses already include numeric robustness profiles. But the current doctrine still leans too heavily toward geometry alone. The engine’s hardest numeric problems also include Type 4 functions, transfer functions, halftone math, overprint simulation, ICC interpolation boundaries, matte handling, and 3D sectioning.  

If those surfaces do not share the same escalation doctrine, you risk a split brain: geometry is auditable, but color/function/prepress numerics are crate-local heuristics. For a project that explicitly wants prepress, signatures, and forensic credibility, that is too weak. The fix is not to simplify; it is to widen the numeric kernel into a true engine-wide numeric-policy layer.  

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Numeric robustness and geometry-kernel doctrine
+
+### Generalized numeric doctrine
+
+The numeric doctrine MUST extend beyond geometry to all mathematically delicate
+surfaces that can materially change rendering, extraction, or validation.
+
+```rust
+pub enum NumericSurfaceKind {
+    Geometry,
+    Type4Function,
+    SampledFunctionInterpolation,
+    BlendBoundaryLogic,
+    ICCInterpolation,
+    TransferFunction,
+    HalftoneEvaluation,
+    OverprintSimulation,
+    MatteUnpremultiplication,
+    SectionPlaneComputation,
+}
+
+pub struct NumericEscalationPolicy {
+    pub preferred: NumericKernelClass,
+    pub fallback: NumericKernelClass,
+    pub exactness_budget: u64,
+}
+
+pub struct NumericEscalationReceipt {
+    pub surface: NumericSurfaceKind,
+    pub initial_kernel: NumericKernelClass,
+    pub final_kernel: NumericKernelClass,
+    pub escalation_reason: String,
+    pub witness_digest: [u8; 32],
+}
+```
+
+Rules:
+- proof-canonical runs MUST pin numeric policy across geometry, function,
+  color, and prepress surfaces
+- non-canonical fast paths MAY downgrade kernels, but MUST emit an explicit
+  `NumericEscalationReceipt` whenever they escalate or degrade
+- prepress and 3D features are not exempt from numeric truth surfaces
````

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
- `monkeybee-render`, `monkeybee-extract`, and `monkeybee-validate` own `/Trapped`, ICC-version,
+- `monkeybee-core::geometry` grows a generalized numeric-policy surface consumed by
+  `monkeybee-render::function`, `monkeybee-render::prepress`, `monkeybee-3d::section`,
+  and `monkeybee-validate::print`
+- add tests proving pinned numeric policy across Type 4 functions, ICC interpolation,
+  transfer functions, and overprint simulation on proof fixtures
```

## 4) Add a font-truth, font-authority, and font-repair witness doctrine

The current font plan is already strong: fallback chains, Type 1/CFF repair, CJK handling, subsetting correctness, and hardening surfaces are all present. What is still missing is a typed distinction between “text truth,” “glyph truth,” “metric truth,” and “substitute render truth.” In ugly PDFs, those are not the same thing.  

This is especially important because Monkeybee wants to be useful for extraction, diffing, signatures, forensics, and accessibility auditing. A user needs to know whether a character came from ToUnicode, a repaired CMap, a glyph-name inference, a TrueType cmap salvage, or a shape-only fallback. The engine already has provenance/trust surfaces; fonts deserve their own specialized witness family.

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 #### Font and encoding recovery
+
+### Font truth and repair witness doctrine
+
+```rust
+pub enum FontTruthClass {
+    EmbeddedAuthoritative,
+    PdfEncodingAuthoritative,
+    ToUnicodeAuthoritative,
+    EmbeddedCmapRecovered,
+    GlyphNameInferred,
+    SubstituteRenderOnly,
+    Unmappable,
+}
+
+pub struct FontMappingWitness {
+    pub font_fingerprint: ResourceFingerprint,
+    pub truth_class: FontTruthClass,
+    pub unicode_authority: Option<String>,
+    pub metric_authority: Option<String>,
+    pub outline_authority: Option<String>,
+    pub affected_charcodes: Vec<u32>,
+}
+
+pub struct FontRepairReceipt {
+    pub font_fingerprint: ResourceFingerprint,
+    pub repairs_applied: Vec<String>,
+    pub truth_surface: Vec<FontMappingWitness>,
+    pub residual_unknowns: Vec<u32>,
+}
+```
+
+Rules:
+- extraction surfaces MUST be able to cite font truth class per span on demand
+- render and extraction reports MUST summarize substitute-render-only zones
+- subsetting, CFF repair, and damaged Type 1 recovery SHOULD emit
+  `FontRepairReceipt` artifacts when they materially affect output
````

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 ### monkeybee-text
 - Unit tests: font program parsing (Type 1, TrueType, CFF, CIDFont, Type 3), CMap parsing, ToUnicode resolution.
+- Truth-surface tests: span-level extraction can cite `FontTruthClass`, and render-side
+  substitution remains distinguishable from extraction-side Unicode authority.
+- Repair-receipt tests: CFF subroutine closure, alternate-key Type 1 recovery, and broken cmap
+  salvage emit deterministic `FontRepairReceipt` artifacts.
```

## 5) Add region-level oracle disagreement and render explainability surfaces

The proof system already has oracle disagreement records, consensus records, blind-spot ledgers, and capability-matrix gating. That is excellent. But page-level disagreement is still too coarse for a renderer this ambitious. You need to know *where* disagreement happened and along what semantic axis: text metrics, blend boundary, overprint, halftone, soft mask, image decode, etc.  

This would make the proof harness vastly more actionable. Instead of “page 14 disagrees,” you get “the disagreement is a 92×37 region over a knockout group with non-separable blend mode and ICC alternate fallback.” That helps engineering, makes release ledgers sharper, and increases external credibility when Monkeybee is right and another renderer is wrong.  

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 pub struct OracleDisagreementRecord {
     pub disagreement_id: String,
@@
     pub blocking: bool,
 }
+
+pub enum DisagreementAxis {
+    TextMetrics,
+    GlyphSubstitution,
+    BlendMode,
+    SoftMask,
+    Overprint,
+    Halftone,
+    ICCConversion,
+    ImageDecode,
+    Geometry,
+    AnnotationAppearance,
+    ThreeDComposite,
+}
+
+pub struct RenderDisagreementRegion {
+    pub page_index: u32,
+    pub region: RegionRef,
+    pub axes: Vec<DisagreementAxis>,
+    pub monkeybee_region_digest: [u8; 32],
+    pub oracle_region_digests: Vec<(String, [u8; 32])>,
+}
+
+pub struct RenderExplainabilityReceipt {
+    pub disagreement_id: String,
+    pub regions: Vec<RenderDisagreementRegion>,
+    pub related_geometry_witnesses: Vec<[u8; 32]>,
+    pub related_numeric_receipts: Vec<[u8; 32]>,
+}
```

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 ### monkeybee-proof
 - Oracle-resolution tests: above-threshold renderer splits emit typed disagreement records with correct blocking state and resolution class.
+- Region-disagreement tests: above-threshold render splits emit bounded disagreement regions with
+  typed semantic axes and linkages to geometry/numeric witnesses.
```

## 6) Add a counterfactual plan frontier for open/save/import operations

The plan already has one of the strongest save-planning stories I’ve seen: explicit preservation algebra, policy-aware plan selection, feasibility witnesses, and minimal unsat cores. The next additive step is to make that more useful by surfacing the *nearest acceptable alternatives* when a requested plan is impossible.  

This is not simplification; it is deepening the same doctrine. A user should not only hear “incremental preserve is impossible.” They should hear “the smallest relaxation is to rewrite these four owned objects while preserving signed byte ranges A/B/C” or “switching provider policy from pinned-only to pinned-then-ambient would make extraction feasible but would demote trust class.” That would make Monkeybee materially more compelling in real workflows.  

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 pub struct PlanSelectionRecord {
     pub plan_kind: PlanKind,
@@
     pub trace_digest: [u8; 32],
 }
+
+pub struct CounterfactualPlan {
+    pub label: String,
+    pub policy_digest: [u8; 32],
+    pub properties_gained: Vec<PreservedProperty>,
+    pub properties_lost: Vec<PreservedProperty>,
+    pub additional_cost: CostEnvelope,
+}
+
+pub struct PropertyLossFrontier {
+    pub requested_plan: String,
+    pub nearest_feasible: Vec<CounterfactualPlan>,
+    pub minimal_blocking_sets: Vec<Vec<String>>,
+}
+
+pub struct FeasibilityFrontierReceipt {
+    pub plan_kind: PlanKind,
+    pub requested_policy_digest: [u8; 32],
+    pub frontier: PropertyLossFrontier,
+}
+
+Rules:
+- failed save/import/open planning SHOULD emit a `FeasibilityFrontierReceipt`
+  when the frontier can be computed cheaply
+- unsat-core output and counterfactual frontier output are complementary; one
+  explains why the requested plan failed, the other explains the nearest legal
+  alternatives
```

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 - Feasibility tests: save-plan constraint graphs emit minimal unsat cores and stable escalation witnesses for preserve/incremental/full-rewrite boundary cases.
+- Counterfactual-frontier tests: infeasible operations emit nearest-feasible alternative plans with
+  stable property-loss frontiers under deterministic mode.
```

## 7) Add a coverage lattice and synthesis doctrine to the proof harness

The current proof harness already has pathological corpus classes, blind-spot ledgers, metamorphic witnesses, reducer genealogy, and expansion-lane fixture representation. That is strong. The next additive step is to move from “fixture counts plus blind spots” to a true coverage lattice over feature × producer × operation × support-class combinations.  

This matters because the plan now spans roughly 181 named surfaces in the inclusive framing. At that size, the most dangerous failure is not a known failing fixture; it is an unexercised interaction. A coverage lattice plus generator/synthesis witnesses would make the release gate much harder to fool.  

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Fuzz testing strategy:
+
+### Coverage lattice and synthesis doctrine
+
+```rust
+pub enum CoverageAxis {
+    FeatureCode,
+    ProducerFamily,
+    OperationChain,
+    SupportClass,
+    SecurityProfile,
+    WriteMode,
+}
+
+pub struct CoverageCell {
+    pub axis_values: Vec<String>,
+    pub exercised_fixture_count: u32,
+    pub passing_fixture_count: u32,
+    pub disagreement_fixture_count: u32,
+    pub synthesis_backfill_count: u32,
+}
+
+pub struct CoverageLattice {
+    pub cells: Vec<CoverageCell>,
+}
+
+pub struct SynthesisWitness {
+    pub source_fixture_digest: Option<[u8; 32]>,
+    pub generator_family: String,
+    pub inserted_features: Vec<FeatureCode>,
+    pub preserved_features: Vec<FeatureCode>,
+    pub verdict: String,
+}
+```
+
+Rules:
+- blind-spot reporting SHOULD be derivable from a coverage lattice rather than
+  only free-form gap descriptions
+- release-facing claims for complex feature intersections SHOULD require lattice
+  coverage thresholds, not only isolated fixture passes
+- synthesized fixtures MUST carry witness lineage into reducers and failure
+  capsules
````

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 ### monkeybee-proof
 - Blind-spot tests: release-facing capability summaries are suppressed or qualified when coverage thresholds are not met.
+- Coverage-lattice tests: feature × producer × operation × support-class intersections remain
+  represented or explicitly ledgered as gaps.
+- Synthesis-witness tests: generated adversarial and interaction fixtures retain provenance through
+  reducers and disagreement artifacts.
```

## 8) Add anchor fragility classes and stability envelopes

The plan already treats semantic anchors seriously: stable IDs, alias maps, proposal validation, and proof harnesses for anchor stability. The missing additive piece is to surface not just “stable/unstable,” but *how fragile* a given anchor is and what transforms it is expected to survive.  

That matters a lot for the post-v1 agent-safe editing story. Some anchors are page-label stable but not text-layout stable; some survive whitespace/cosmetic rewrites but not page-tree rebalance; some are geometry-dominant and break under appearance regeneration. Exposing that envelope would make automation safer and make anchor-driven workflows more honest. 

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Workflow 12: Queryable semantic anchors and agent-safe edits
+
+### Anchor fragility and stability-envelope doctrine
+
+```rust
+pub enum AnchorFragilityClass {
+    SnapshotLocal,
+    RewriteStable,
+    IncrementalStable,
+    LayoutSensitive,
+    ProviderSensitive,
+    AmbiguitySensitive,
+}
+
+pub struct AnchorStabilityEnvelope {
+    pub anchor_id: SemanticAnchorId,
+    pub fragility: AnchorFragilityClass,
+    pub survives_transforms: Vec<MetamorphicTransformKind>,
+    pub requires_alias_map_on: Vec<MetamorphicTransformKind>,
+    pub invalidated_by: Vec<String>,
+}
+
+pub struct AnchorDriftWitness {
+    pub anchor_id: SemanticAnchorId,
+    pub before_snapshot: SnapshotId,
+    pub after_snapshot: SnapshotId,
+    pub geometry_drift: Option<f64>,
+    pub logical_drift: Option<f64>,
+    pub resolved_via_alias_map: bool,
+}
+```
+
+Rules:
+- query APIs SHOULD be able to return an `AnchorStabilityEnvelope` alongside a
+  semantic anchor when requested
+- agent-facing edit APIs MUST reject anchors whose fragility class is
+  incompatible with the proposed transform unless the caller explicitly opts in
````

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 ### monkeybee-proof
 - Anchor tests: semantic-anchor stability harness computes expected alias precision.
+- Fragility tests: anchors emit stable fragility classes and transform-survival envelopes on
+  canonical metamorphic fixtures.
```

## 9) Add environment lockfiles and evidence bundles

The plan already has policy digests, provider manifests, feature-module manifests, benchmark witnesses, and manifest-last durability rules. The next additive step is to package those into a single reproducible engine-environment lock plus a transitive evidence bundle.   

This would pay off immediately in debugging, benchmarking, and external credibility. A bug report or benchmark claim becomes a content-addressed bundle containing the exact policy digest, module/provider manifests, allocator/SIMD/NUMA topology, fixture digests, receipts, disagreement regions, and minimized reproducer. That is exactly the kind of “alien artifact” proof surface this project is aiming for.  

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Persisted artifact durability contract
+
+### Engine environment lock and evidence-bundle doctrine
+
+```rust
+pub struct EngineEnvironmentLock {
+    pub engine_version: String,
+    pub git_commit: Option<String>,
+    pub policy_digest: [u8; 32],
+    pub provider_manifest_ids: Vec<String>,
+    pub feature_module_manifest_ids: Vec<String>,
+    pub allocator: String,
+    pub simd_class: String,
+    pub numa_policy: String,
+    pub support_class: SupportClass,
+}
+
+pub struct EvidenceBundleManifest {
+    pub bundle_digest: [u8; 32],
+    pub environment: EngineEnvironmentLock,
+    pub fixture_digests: Vec<[u8; 32]>,
+    pub child_artifacts: Vec<ArtifactRef>,
+}
+
+pub struct ReproducerBundle {
+    pub manifest: EvidenceBundleManifest,
+    pub minimized_fixture: Option<ArtifactRef>,
+    pub trace_artifact: Option<ArtifactRef>,
+    pub disagreement_artifact: Option<ArtifactRef>,
+}
+```
+
+Rules:
+- proof-canonical failures SHOULD be publishable as `ReproducerBundle`s
+- benchmark claims SHOULD be attachable to an `EngineEnvironmentLock`
+- bug-report and regression artifacts MUST never rely on ambient host state
+  that is absent from the lockfile
````

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 ## Subordinate implementation docs
+- `docs/implementation/evidence-bundles.md` — environment lockfiles, bundle manifests,
+  transitive artifact closure, reproducer-bundle publication, and retention rules
```

## 10) Add a render/materialization chunk doctrine spanning CPU, GPU, progressive, and 3D composition

The plan already mentions page plans, render-chunk graphs, coverage-cell indexes, progressive refinement, GPU fallback, and 2D/3D compositing. The missing additive contract is a stable render/materialization chunk identity that unifies those ideas across backends.  

Without that, CPU tiles, GPU tiles, progressive placeholders, and 3D composite layers risk becoming backend-private artifacts. With it, invalidation, disagreement localization, cache reuse, and proof receipts all operate on the same chunk algebra. That would make the renderer both faster and more explainable.  

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Progressive rendering contract
+
+### Render chunk and composite-materialization doctrine
+
+```rust
+pub enum RenderChunkKind {
+    VectorDisplayList,
+    GlyphRun,
+    ImageSample,
+    TransparencyGroup,
+    AnnotationAppearance,
+    ThreeDComposite,
+    ProgressivePlaceholder,
+}
+
+pub struct RenderChunkId(pub [u8; 32]);
+
+pub struct RenderChunkWitness {
+    pub chunk_id: RenderChunkId,
+    pub kind: RenderChunkKind,
+    pub dependency_digests: Vec<[u8; 32]>,
+    pub coverage_cells: Vec<CoverageCellRef>,
+    pub completeness_class: String,
+}
+
+pub struct CompositeMaterializationReceipt {
+    pub page_index: u32,
+    pub chunk_witnesses: Vec<RenderChunkWitness>,
+    pub backend_class: RenderDeterminismClass,
+    pub reused_chunks: Vec<RenderChunkId>,
+    pub recomputed_chunks: Vec<RenderChunkId>,
+}
+```
+
+Rules:
+- CPU, GPU, progressive, and 3D-composited page renders SHOULD converge on the
+  same chunk identity model even if final rasterization differs
+- invalidation and proof disagreements SHOULD be localizable at chunk granularity
+  where practical
````

```diff
--- a/docs/implementation/implementation_master.md
+++ b/docs/implementation/implementation_master.md
@@
 ### monkeybee-render
 - Cache/query tests: PagePlan cache invalidation on content/resource changes and derived render-chunk graphs stay causally aligned.
+- Chunk-identity tests: CPU, progressive, and GPU paths preserve stable `RenderChunkId`
+  linkage for unchanged display subgraphs.
+
+### monkeybee-3d
+- Composite-receipt tests: 2D/3D page composites emit chunk witnesses that participate in page
+  invalidation, render disagreement localization, and proof receipts.
```

The shortest summary of my revisions is this:

The current plan already has the right first-order architecture. The highest-value additive move now is to promote the hidden second-order forces — topology, residency, numerics, font truth, disagreement localization, counterfactual planning, coverage lattices, anchor fragility, environment locks, and chunk identities — into first-class contracts.

That would make Monkeybee not just broad, but *tighter*: more reproducible, more debuggable, more performant under stress, more honest under ambiguity, and more compelling as a proof-bearing engine rather than a merely ambitious one.   

If you want, I can turn this into a single consolidated patch against `SPEC.md` and `implementation_master.md` next.
