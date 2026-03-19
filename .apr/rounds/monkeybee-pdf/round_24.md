I reviewed the README , the implementation master , and the plan/spec  together.

Overall, this is already unusually strong. The plan’s biggest strength is that it does **not** describe Monkeybee as a parser plus renderer plus writer; it describes one content-addressed substrate with exact invalidation, policy-complete planning, provenance/trust, transport continuity, durable artifacts, and witness-backed proof/performance surfaces. That is the right center of gravity, and it already shows up consistently across the README, the spec, and the implementation topology.   

The best revisions are therefore **not** “make it smaller” or “simplify the lanes.” They are: make the remaining implicit decision machinery explicit, make the performance model self-correcting, strengthen end-to-end publication and signing contracts, and add a few more proof-bearing bridges between render/extract/edit/save so the system becomes even more alien-artifact-like without sacrificing auditability. That direction is fully consistent with the existing doctrine around experimental backends, render determinism classes, preservation-frontier witnesses, transport continuity, chunk-level invalidation, and generated capability/proof surfaces.     

Here are my highest-value additive revisions.

## 1) Add an explicit algorithm-variant governance layer

**Why this makes the project better**

Right now the plan clearly allows advanced backends and algorithmic uplifts, and it already mentions strategy tournaments, decoder-equivalence tests, variant manifests, render determinism classes, and experimental/non-gating paths. But those pieces are still too distributed. There is not yet one first-class contract that says: “this family of interchangeable kernels exists; here is how it is benchmarked, equivalence-checked, promoted, demoted, and selected at runtime.” That matters because the project explicitly wants world-class performance without allowing fast paths to become unauditable folklore.    

Making algorithm families first-class will let you pursue truly aggressive optimizations—CPU SIMD compositors, GPU raster kernels, alternative ICC evaluators, native-vs-pure-Rust decoder paths, multiple Type 4 interpreters—without letting “experimental” become a vague social label. It also creates a clean place to encode promotion doctrine: no default switch without equivalence evidence, topology-aware benchmark evidence, and explicit degradation envelopes.

**Additive diff**

````diff
+ ### Algorithm variant governance doctrine
+
+ Experimental and optimized paths are not only feature flags. They are members
+ of named algorithm families with typed promotion, demotion, and selection rules.
+
+ ```rust
+ pub struct AlgorithmFamilyId(pub [u8; 32]);
+ pub struct AlgorithmVariantId(pub [u8; 32]);
+
+ pub enum AlgorithmFamilyKind {
+     StreamDecode,
+     FontRasterization,
+     TransparencyCompositing,
+     Type4FunctionEval,
+     ICCTransform,
+     OverprintSimulation,
+     PathRasterization,
+     ThreeDSectioning,
+ }
+
+ pub enum VariantEligibilityClass {
+     BaselineAuditable,
+     ExperimentalNonGating,
+     ProofCanonicalCandidate,
+     HostAdaptiveOnly,
+ }
+
+ pub struct AlgorithmVariantManifest {
+     pub family: AlgorithmFamilyId,
+     pub variant: AlgorithmVariantId,
+     pub algorithm_id: String,
+     pub eligibility: VariantEligibilityClass,
+     pub required_support_class: SupportClass,
+     pub required_feature_modules: Vec<String>,
+     pub determinism_class: RenderDeterminismClass,
+ }
+
+ pub struct DecoderEquivalenceRecord {
+     pub family: AlgorithmFamilyId,
+     pub baseline_variant: AlgorithmVariantId,
+     pub candidate_variant: AlgorithmVariantId,
+     pub fixture_set_digest: [u8; 32],
+     pub verdict: String,
+     pub disagreement_artifacts: Vec<String>,
+ }
+
+ pub struct VariantSelectionReceipt {
+     pub family: AlgorithmFamilyId,
+     pub chosen_variant: AlgorithmVariantId,
+     pub rejected_variants: Vec<AlgorithmVariantId>,
+     pub policy_digest: [u8; 32],
+     pub hardware_manifest_digest: [u8; 32],
+     pub reason: String,
+ }
+ ```
+
+ Rules:
+ - every hot-swappable kernel family MUST publish an `AlgorithmVariantManifest`
+ - no variant may become default without benchmark witnesses plus typed
+   equivalence evidence against a baseline auditable variant
+ - proof-canonical runs MUST cite chosen variants explicitly
+ - variant promotion/demotion MUST be durable ledger events, not release-note prose
````

---

## 2) Add a self-correcting cost-model calibration loop

**Why this makes the project better**

The spec already has `ComplexityFingerprint`, `AdmissionDecision`, `BudgetDerivationReceipt`, `CostEnvelope`, `PlanSelectionRecord`, `WorkingSetForecast`, and benchmark witnesses. That is excellent. But today those are still mostly *forward* estimates. What is missing is a mandatory *backward* loop that compares predicted cost to actual cost and records error. Without that, the engine can make elaborate plan selections while its heuristics quietly drift out of calibration on real corpora.    

A calibration layer would make the planner progressively sharper across pathological classes: incremental-history bombs, font-table explosions, mesh-heavy 3D, transparency-heavy prepress, and remote-first-paint cases. It also gives you a principled way to justify extreme optimization doctrine: the planner becomes measurable, not intuitive.

**Additive diff**

````diff
+ ### Cost-model calibration doctrine
+
+ Plan candidates may estimate cost, but release-facing planning quality also
+ requires measured calibration against actual executions.
+
+ ```rust
+ pub struct CostModelId(pub [u8; 32]);
+
+ pub enum CostConfidenceClass {
+     High,
+     Medium,
+     Low,
+     Uncalibrated,
+ }
+
+ pub struct CostModelReference {
+     pub model_id: CostModelId,
+     pub version: String,
+     pub confidence: CostConfidenceClass,
+ }
+
+ pub struct PredictionErrorWitness {
+     pub plan_kind: PlanKind,
+     pub chosen_label: String,
+     pub estimated_latency_ms: u64,
+     pub actual_latency_ms: u64,
+     pub estimated_peak_memory: u64,
+     pub actual_peak_memory: u64,
+     pub estimated_bytes_read: u64,
+     pub actual_bytes_read: u64,
+     pub fixture_digest: [u8; 32],
+ }
+
+ pub struct CalibrationReceipt {
+     pub model_id: CostModelId,
+     pub training_fixture_set_digest: [u8; 32],
+     pub error_witnesses: Vec<PredictionErrorWitness>,
+     pub promoted: bool,
+ }
+ ```
+
+ Rules:
+ - every `PlanCandidate.cost` SHOULD cite a `CostModelReference`
+ - canonical benchmark runs SHOULD emit `PredictionErrorWitness` artifacts for
+   the selected plan
+ - planner upgrades SHOULD require a `CalibrationReceipt`, not only green tests
+ - low-confidence or uncalibrated models MUST surface that state in planning artifacts
````

---

## 3) Add a viewport-aware byte-need graph for remote/progressive open

**Why this makes the project better**

The current remote story is already strong: progressive placeholders, range-backed byte sources, transport continuity receipts, sparse convergence, verified sparse blobs, resumption receipts, and prefetch planning all exist. What is still missing is a **unified graph** from *visible substructure* to *exact byte need*, so first-paint scheduling can operate at a finer granularity than “resource placeholder with a needed range.”    

That graph matters for real performance. It lets you prioritize bytes by viewport value, chunk coverage, progressive completeness debt, and repair risk. It also helps transport trust, because it gives every progressive artifact an exact dependency closure over byte ranges and fetch epochs.

**Additive diff**

````diff
+ ### Viewport-aware byte-need graph doctrine
+
+ Progressive rendering SHOULD plan remote acquisition against a byte-need graph
+ that links visible semantic/render chunks to concrete range requirements.
+
+ ```rust
+ pub struct NeedNodeId(pub [u8; 32]);
+
+ pub enum NeedNodeKind {
+     FirstPaintChunk,
+     FontSubset,
+     ImageStream,
+     FormXObject,
+     TransparencyGroup,
+     ThreeDSceneRegion,
+ }
+
+ pub struct ByteNeedNode {
+     pub need_id: NeedNodeId,
+     pub kind: NeedNodeKind,
+     pub page_index: u32,
+     pub chunk_id: Option<RenderChunkId>,
+     pub required_ranges: Vec<(u64, u64)>,
+     pub visual_priority: u32,
+     pub completeness_gain_score: u32,
+ }
+
+ pub struct ByteNeedGraph {
+     pub nodes: Vec<ByteNeedNode>,
+     pub fetch_epoch: FetchEpoch,
+     pub viewport_digest: [u8; 32],
+ }
+
+ pub struct FetchPlanReceipt {
+     pub byte_need_graph_digest: [u8; 32],
+     pub scheduled_ranges: Vec<(u64, u64)>,
+     pub deferred_ranges: Vec<(u64, u64)>,
+     pub reason: String,
+     pub policy_digest: [u8; 32],
+ }
+ ```
+
+ Rules:
+ - progressive first-paint planning SHOULD operate on `ByteNeedGraph`, not
+   only ad hoc resource-level placeholder demand
+ - `RenderChunkGraph` and `ByteNeedGraph` SHOULD be linkable by digest
+ - remote benchmark classes SHOULD emit `FetchPlanReceipt`s for first-paint runs
+ - when transport continuity breaks, only affected `ByteNeedGraph` nodes may be poisoned
````

---

## 4) Add a hardware-capability and kernel-dispatch manifest

**Why this makes the project better**

The plan already tracks support class, render determinism class, topology receipts, SIMD class, NUMA policy, storage class, GPU fallbacks, and feature modules. But there is still no single artifact that says: “on this host, these kernels were admissible; these were selected; these were rejected; this is why.” That becomes critical once you really start optimizing across CPU SIMD widths, GPU adapters, native isolation modes, and WASM/browser fallbacks.    

This addition would prevent one of the most common failures of ambitious systems: benchmark evidence exists, but runtime dispatch remains opaque. If you want “world-best performance characteristics” *and* proof-grade explainability, dispatch itself has to become receiptable.

**Additive diff**

````diff
+ ### Hardware capability and kernel-dispatch doctrine
+
+ Runtime kernel choice is a proof surface. Host capability discovery and
+ algorithm dispatch MUST be explicit, typed, and receiptable.
+
+ ```rust
+ pub struct HardwareCapabilityManifest {
+     pub cpu_topology: String,
+     pub simd_class: String,
+     pub numa_policy: String,
+     pub gpu_adapter_fingerprint: Option<[u8; 32]>,
+     pub webgpu_limits_digest: Option<[u8; 32]>,
+     pub native_module_manifest_id: String,
+ }
+
+ pub enum DispatchConstraint {
+     DeterminismPinned,
+     MemoryBudget,
+     IsolationPolicy,
+     AdapterUnsupported,
+     WasmRestricted,
+     ProofCanonicalDowngrade,
+ }
+
+ pub struct KernelDispatchReceipt {
+     pub family: AlgorithmFamilyId,
+     pub chosen_variant: AlgorithmVariantId,
+     pub hardware_manifest_digest: [u8; 32],
+     pub constraints: Vec<DispatchConstraint>,
+     pub downgraded_from: Option<AlgorithmVariantId>,
+ }
+ ```
+
+ Rules:
+ - proof, benchmarks, and diagnostic reports SHOULD be able to cite a
+   `HardwareCapabilityManifest`
+ - hot-path dispatch decisions SHOULD emit `KernelDispatchReceipt`
+ - proof-canonical mode MAY ignore faster host-specific variants, but the
+   downgrade must remain explicit
+ - GPU/CPU/WASM fallbacks must be dispatch artifacts, not silent internal branches
````

---

## 5) Make text-paint correspondence a first-class bridge, not only a named surface

**Why this makes the project better**

The README already names text-paint correspondence receipts as one of the derived surfaces unified by the substrate, and the spec already has `RenderChunkGraph`, `CoverageCellIndex`, `CoverageAtlas`, anchors, provenance lattices, and redaction audits. That is exactly the right ecosystem. What still deserves explicit elevation is the *bridge object* between logical/physical text spans and painted marks.   

Why this matters: it hardens extraction correctness, search hit-testing, redaction assurance, diff explainability, accessibility overlays, and anchor stability **all at once**. If a user asks “show me why this extracted span exists, where it was painted, whether it is occluded, and what redaction touched it,” that should be one substrate-native answer.

**Additive diff**

````diff
+ ### Text-paint correspondence doctrine
+
+ Text extraction, search, redaction audit, and semantic anchoring MUST be able
+ to cite a stable bridge between decoded text spans and painted page evidence.
+
+ ```rust
+ pub struct TextPaintLink {
+     pub span_id: SpanId,
+     pub chunk_id: RenderChunkId,
+     pub coverage_cell_ids: Vec<u64>,
+     pub glyph_range: (u32, u32),
+     pub occlusion_state: String,
+     pub provenance: ProvenanceAtom,
+ }
+
+ pub struct TextPaintCorrespondenceReceipt {
+     pub page_index: u32,
+     pub link_count: u64,
+     pub link_digest: [u8; 32],
+     pub atlas_digest: [u8; 32],
+     pub trace_digest: [u8; 32],
+ }
+
+ pub struct SelectionStabilityWitness {
+     pub anchor_id: SemanticAnchorId,
+     pub pre_snapshot: SnapshotId,
+     pub post_snapshot: SnapshotId,
+     pub retained_links: u64,
+     pub lost_links: u64,
+     pub reason: String,
+ }
+ ```
+
+ Rules:
+ - extraction surfaces SHOULD be able to emit `TextPaintCorrespondenceReceipt`
+ - redaction audit MUST be able to query text-paint links before claiming erasure
+ - semantic-anchor stability SHOULD incorporate retained/lost text-paint linkage
+ - diff/explain surfaces SHOULD localize text deltas through span→chunk→cell evidence
````

---

## 6) Formalize append-budget and signature-reservation planning

**Why this makes the project better**

The current docs already say the signature lane includes reservation planning and append-budget evidence, and the write receipt already has an `append_budget` slot. That is a strong signal that the concept is already wanted. But it is not yet specified with enough force to become implementation-grade: there is no concrete model for remaining append headroom, reserved future signature windows, DSS/TSA growth, or multi-signature incremental chains.   

This is one of the best places to deepen the project, because real signature workflows fail on exactly these boundaries: not “can you sign once,” but “can you keep signing, timestamping, and embedding validation material without accidentally cornering future increments or breaking a DocMDP story.”

**Additive diff**

````diff
+ ### Append-budget and signature-reservation doctrine
+
+ Signature-safe incremental workflows require an explicit forecast of remaining
+ append headroom and optional reservation windows for future signature growth.
+
+ ```rust
+ pub struct ReservationWindowId(pub [u8; 32]);
+
+ pub struct ReservationWindow {
+     pub window_id: ReservationWindowId,
+     pub reserved_for: String,
+     pub byte_count: u64,
+     pub may_shift: bool,
+ }
+
+ pub struct AppendBudgetModel {
+     pub current_incremental_depth: u32,
+     pub free_append_headroom_bytes: u64,
+     pub reserved_windows: Vec<ReservationWindow>,
+     pub expected_dss_growth_bytes: u64,
+     pub expected_timestamp_growth_bytes: u64,
+ }
+
+ pub struct SignatureReservationPlan {
+     pub signature_field: ObjRef,
+     pub requested_future_steps: Vec<String>,
+     pub append_budget_model: AppendBudgetModel,
+     pub feasible: bool,
+     pub blocking_reasons: Vec<String>,
+ }
+
+ pub struct AppendBudgetReceipt {
+     pub model_digest: [u8; 32],
+     pub consumed_bytes: u64,
+     pub remaining_headroom_bytes: u64,
+     pub violated_reservations: Vec<ReservationWindowId>,
+ }
+ ```
+
+ Rules:
+ - PAdES creation/LTV workflows SHOULD be able to emit `SignatureReservationPlan`
+ - incremental signing/timestamping SHOULD surface append-headroom risk before commit
+ - `WriteReceipt.append_budget` SHOULD reference a typed `AppendBudgetReceipt`
+ - multi-signature fixtures SHOULD test reservation exhaustion and controlled escalation
````

---

## 7) Add a publication-transaction graph for multi-artifact atomicity

**Why this makes the project better**

The plan is already serious about durability: manifest-last publishing, emission journals, durable artifact publication rules, and evidence bundles are all there. But the architecture would become stronger if the *entire closure* of a save/proof publication became one explicit transaction object rather than a set of related rules spread across writeback, reproducibility, and evidence bundles.    

That matters because the project is unusual in how many child artifacts it emits per operation: save output, write receipt, invariant certificate, frontier witness, disagreement records, benchmark witnesses, failure capsules, bundle manifests. Once the project is this evidence-heavy, publication itself needs a typed closure.

**Additive diff**

````diff
+ ### Publication-transaction and artifact-closure doctrine
+
+ Durable publication is a first-class transactional surface spanning emitted
+ bytes plus all child receipts, ledgers, certificates, and manifests that
+ claim those bytes.
+
+ ```rust
+ pub struct PublicationTxnId(pub [u8; 32]);
+
+ pub enum PublicationArtifactKind {
+     SavedPdf,
+     WriteReceipt,
+     InvariantCertificate,
+     FrontierWitness,
+     BenchmarkWitness,
+     FailureCapsule,
+     ReproducerBundle,
+     Manifest,
+ }
+
+ pub struct PublicationArtifactRef {
+     pub kind: PublicationArtifactKind,
+     pub digest: [u8; 32],
+     pub durable: bool,
+ }
+
+ pub struct ArtifactClosureManifest {
+     pub txn_id: PublicationTxnId,
+     pub root_artifacts: Vec<PublicationArtifactRef>,
+     pub child_artifacts: Vec<PublicationArtifactRef>,
+     pub policy_digest: [u8; 32],
+ }
+
+ pub struct PublicationReceipt {
+     pub txn_id: PublicationTxnId,
+     pub committed: bool,
+     pub quarantined_children: Vec<PublicationArtifactRef>,
+     pub manifest_digest: [u8; 32],
+ }
+ ```
+
+ Rules:
+ - save/proof publication SHOULD build an `ArtifactClosureManifest` before final publish
+ - manifest-last durability applies to the closure, not only the primary PDF blob
+ - crash recovery SHOULD quarantine closure fragments that never reached a committed `PublicationReceipt`
+ - evidence bundles SHOULD be definable as a special case of artifact-closure publication
````

---

## 8) Add semantic-delta axes for diff, replay, and explainability

**Why this makes the project better**

The project already has strong diffing ingredients: root digests, semantic normal forms, import-closure certificates, temporal replay, provenance/trust, exact invalidation, and explain surfaces. But the user-facing question “what materially changed?” still deserves one more layer of structure: typed *delta axes* that unify render, extraction, structure, signature, anchor stability, and preservation.  

This would make diffs dramatically more actionable. Instead of only “these bytes/objects/pixels changed,” Monkeybee could say: “this was a structure-only delta,” “this changed rendered appearance but not extracted text,” “this preserved semantic normal form but broke one anchor alias class,” “this invalidated signature coverage but preserved page visuals.” That is exactly the kind of alien-artifact coherence the plan wants.

**Additive diff**

````diff
+ ### Semantic-delta axis doctrine
+
+ Diff, replay, and save-impact explanations SHOULD classify changes along typed
+ semantic axes rather than only object, byte, or pixel surfaces.
+
+ ```rust
+ pub enum SemanticDeltaAxis {
+     StructureTree,
+     ExtractedText,
+     PaintedAppearance,
+     AnnotationGeometry,
+     SignatureCoverage,
+     FormValueModel,
+     ActiveContentInventory,
+     AnchorStability,
+     ImportAliasMap,
+     PreservationClaims,
+ }
+
+ pub enum MeaningChangeClass {
+     None,
+     SurfaceOnly,
+     SemanticEquivalent,
+     MaterialSemanticChange,
+ }
+
+ pub struct SemanticDeltaWitness {
+     pub axis: SemanticDeltaAxis,
+     pub change_class: MeaningChangeClass,
+     pub before_digest: [u8; 32],
+     pub after_digest: [u8; 32],
+     pub supporting_artifacts: Vec<[u8; 32]>,
+ }
+
+ pub struct AnchorFragilityReceipt {
+     pub snapshot_before: SnapshotId,
+     pub snapshot_after: SnapshotId,
+     pub retained_aliases: u64,
+     pub broken_aliases: u64,
+     pub delta_axes: Vec<SemanticDeltaAxis>,
+ }
+ ```
+
+ Rules:
+ - `DiffReport` SHOULD be able to emit `SemanticDeltaWitness` entries
+ - save-impact explanations SHOULD distinguish preservation loss by semantic axis
+ - temporal replay SHOULD classify frame-to-frame change axes, not only changed objects
+ - anchor-fragility reporting SHOULD be a typed receipt rather than a prose-only warning
````

---

## 9) Add a proof-facing corpus value model for acquisition and minimization

**Why this makes the project better**

The proof system already has a pathological corpus, coverage lattices, blind-spot ledgers, metamorphic witnesses, reducers, and acquisition recommendations. That is excellent. The next step is to make fixture *value* explicit: how much new surface area does a candidate fixture add, how much overlap does it have with existing cells, and how much reducer pressure can be spent on it before it stops paying rent.  

This is not administrative garnish. A project with this scope will eventually be limited less by cleverness than by corpus economics. An explicit value model prevents the proof harness from becoming huge but low-information.

**Additive diff**

````diff
+ ### Corpus value and acquisition-economics doctrine
+
+ Fixture growth SHOULD be optimized for information gain, not only raw count.
+
+ ```rust
+ pub struct FixtureValueScore {
+     pub fixture_digest: [u8; 32],
+     pub novelty_score: f64,
+     pub breadth_gain_score: f64,
+     pub reducer_cost_score: f64,
+     pub blocking_bug_relevance: f64,
+ }
+
+ pub struct AcquisitionRecommendation {
+     pub target_feature_cells: Vec<String>,
+     pub desired_producer_phenotypes: Vec<ProducerPhenotypeId>,
+     pub desired_complexity_hazards: Vec<ComplexityHazardKind>,
+     pub expected_value_gain: f64,
+ }
+
+ pub struct ReducerTerminationReceipt {
+     pub original_fixture_digest: [u8; 32],
+     pub reduced_fixture_digest: [u8; 32],
+     pub value_retained_score: f64,
+     pub reason: String,
+ }
+ ```
+
+ Rules:
+ - corpus triage SHOULD rank fixtures by `FixtureValueScore`
+ - acquisition planning SHOULD optimize under-covered cells × value gain
+ - reducers SHOULD emit `ReducerTerminationReceipt` so minimization does not silently erase proof value
````

---

## 10) Add a formal object-neighborhood closure surface for huge-document locality

**Why this makes the project better**

The current segmented-artifact story is already good: page closures, object neighborhoods, render-chunk regions, semantic regions, revision frames, working-set forecasts, and spill receipts. What would improve it is making object-neighborhood closure a reusable cross-subsystem *planning primitive* rather than only a segmentation concept. 

This is especially valuable for huge and ugly PDFs: dense annotation webs, shared resource graphs, enormous incremental chains, and portfolio-like documents. A formal closure surface improves locality-aware open, edit closure computation, import planning, spill policy, and remote fetch planning all at once.

**Additive diff**

````diff
+ ### Object-neighborhood closure doctrine
+
+ Large-document planning SHOULD be able to operate on reusable semantic
+ neighborhoods smaller than full page closure and larger than a single object.
+
+ ```rust
+ pub struct NeighborhoodId(pub [u8; 32]);
+
+ pub enum NeighborhoodKind {
+     PageAdjacent,
+     ResourceClosure,
+     AnnotationCluster,
+     FormDependencyIsland,
+     IncrementalRevisionCone,
+ }
+
+ pub struct ObjectNeighborhood {
+     pub neighborhood_id: NeighborhoodId,
+     pub root_objects: Vec<ObjRef>,
+     pub member_objects: Vec<ObjRef>,
+     pub source_digests: Vec<NodeDigest>,
+     pub estimated_working_set_bytes: u64,
+ }
+
+ pub struct NeighborhoodReuseReceipt {
+     pub neighborhood_id: NeighborhoodId,
+     pub reused: bool,
+     pub invalidating_digests: Vec<NodeDigest>,
+ }
+ ```
+
+ Rules:
+ - edit closure, import planning, huge-doc open, and spill policy SHOULD be
+   allowed to target `ObjectNeighborhood`s directly
+ - neighborhood reuse SHOULD be auditable via `NeighborhoodReuseReceipt`
+ - segmented artifact policy SHOULD be able to map segments onto neighborhoods
````

---

## My priority order

If I were revising the plan immediately, I would prioritize these in this order:

1. **Algorithm variant governance** — because the project explicitly wants extreme optimization, and this is what keeps optimization from becoming opaque.  
2. **Cost-model calibration** — because the planner already exists conceptually and needs a truth loop.  
3. **Append-budget / signature reservation** — because it turns the signature lane from impressive to operationally formidable.  
4. **Text-paint correspondence formalization** — because it strengthens extraction, redaction, search, hit-testing, and anchors simultaneously.  
5. **Publication transaction graph** — because the evidence surface is already large enough that closure atomicity matters.  

My highest-level judgment: **do not shrink this plan**. The ambition is justified. The right move now is to keep adding explicit contracts wherever the system currently relies on “obvious” internal judgment—especially around planner calibration, algorithm promotion, signing headroom, cross-surface correspondence, and durable closure publication. That is how this becomes not just broad, but disturbingly coherent.
