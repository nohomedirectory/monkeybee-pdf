I read the README first and then reviewed the spec.  

The core is already unusually strong: one persistent substrate spans parse/render/extract/edit/save/proof; v1 is defined by evidence rather than claims; policy composition is explicit; exact invalidation and write receipts are first-class; and the expansion lanes are concrete enough to matter in prepress, signatures, accessibility, and forensics.      

My strongest additive revisions are below. I am not reducing anything; each item widens the system, sharpens proofs, or turns implicit shared concerns into explicit subsystems.

## 1) Add a first-class numeric/geometry kernel

**Why this makes it better**

The spec already exposes `numeric_robustness_profile` in `RenderReport` and `ExtractReport`, which is a good signal, but right now numeric robustness is report-shaped rather than architecture-shaped. That is dangerous in a PDF engine because the same failure mode shows up everywhere: clipping, fills, winding, self-intersections, nearly singular transforms, redaction region checks, annotation placement, prepress region analysis, hit-testing, and 3D cross-sections. If each crate chooses its own epsilon policy, the substrate will stay coherent while the visible behavior drifts.  

A dedicated kernel gives you one auditable place for interval guards, exact-or-filtered predicates, degeneracy classes, affine conditioning, and proof-canonical tolerance rules. This is one of the highest-leverage additions because it simultaneously improves reliability, performance tuning, and explainability.

```diff
diff --git a/SPEC.md b/SPEC.md
@@
+### Numeric robustness doctrine
+
+Monkeybee MUST treat numerically delicate geometry as a shared engine concern rather than
+crate-local epsilon folklore. Rendering, extraction, annotation placement, redaction,
+prepress region analysis, hit-testing, path boolean ops, and 3D cross-sections all depend on
+the same affine/path/intersection semantics.
+
+Introduce a dedicated crate:
+
+| `monkeybee-geometry` | Robust affine/path/clip/intersection kernel: interval arithmetic, exact-or-filtered predicates, path flattening contracts, degeneracy classification, canonical tolerance policies, and geometry witnesses |
+
+pub enum NumericRobustnessClass {
+    ProofCanonical,
+    IntervalGuarded,
+    FastApproximate,
+    Experimental,
+}
+
+pub enum DegeneracyClass {
+    ZeroAreaPath,
+    NearlySingularTransform,
+    CoincidentEdges,
+    SelfIntersection,
+    CatastrophicCancellationRisk,
+}
+
+pub struct AffineConditionWitness {
+    pub matrix_digest: [u8; 32],
+    pub condition_estimate: f64,
+    pub degeneracies: Vec<DegeneracyClass>,
+}
+
+pub struct GeometryWitness {
+    pub page_index: Option<u32>,
+    pub robustness_class: NumericRobustnessClass,
+    pub transforms: Vec<AffineConditionWitness>,
+    pub clipped_regions: Vec<RegionRef>,
+    pub degraded_due_to_numeric_risk: Vec<RegionRef>,
+    pub trace_digest: [u8; 32],
+}
+
+Rules:
+- proof-canonical rendering and extraction MUST use a pinned tolerance policy from `monkeybee-geometry`
+- geometry-sensitive surfaces MAY use faster kernels in non-canonical modes, but MUST report the chosen robustness class
+- boolean/path/intersection operators MUST emit degeneracy classifications when they fall off the ideal path
+- redaction, annotation placement, and prepress region accounting MUST consume the same geometry kernel rather than private implementations
@@
 pub struct RenderReport {
     pub render_determinism_class: RenderDeterminismClass,
@@
     pub numeric_robustness_profile: Option<NumericRobustnessProfile>,
+    pub geometry_witness_digest: Option<[u8; 32]>,
@@
 pub struct ExtractReport {
@@
     pub numeric_robustness_profile: Option<NumericRobustnessProfile>,
+    pub geometry_witness_digest: Option<[u8; 32]>,
```

## 2) Make every derived artifact receiptable, not just writes

**Why this makes it better**

The plan already has an excellent `InvalidationWitness` contract and a strong `WriteReceipt` / `InvariantCertificate` surface. The missing step is to treat *all* important derived artifacts as receiptable objects: page plans, resolved resources, extracted surfaces, semantic graphs, access plans, color transforms, 3D scene frames, and backend-produced render tiles. That closes the loop between “exact invalidation exists” and “I can prove exactly what was materialized, from what, under which policy and determinism class.”  

This matters for backend promotion, cache debugging, proof reproduction, and cross-run comparisons. It also makes the substrate feel more like one coherent machine.

```diff
diff --git a/SPEC.md b/SPEC.md
@@
+### Materialization receipt doctrine
+
+Every durable or reusable derived artifact MUST be able to emit a schema-versioned
+`MaterializationReceipt`, not only save/write outputs.
+
+pub enum ArtifactKind {
+    PagePlan,
+    ResolvedResources,
+    RasterTile,
+    ExtractSurface,
+    SemanticGraph,
+    AccessPlan,
+    ColorTransform,
+    SceneFrame,
+    WritePlan,
+    DiffArtifact,
+}
+
+pub struct MaterializationReceipt {
+    pub schema_version: String,
+    pub artifact_kind: ArtifactKind,
+    pub artifact_digest: [u8; 32],
+    pub snapshot_id: SnapshotId,
+    pub source_node_digests: Vec<NodeDigest>,
+    pub policy_digest: [u8; 32],
+    pub determinism_class: Option<RenderDeterminismClass>,
+    pub invalidation_witness: Option<InvalidationWitness>,
+    pub build_algorithm_id: String,
+    pub trace_digest: [u8; 32],
+}
+
+Rules:
+- any cache entry eligible for cross-snapshot reuse or persistent storage MUST be able to produce a `MaterializationReceipt`
+- experimental backends may produce artifacts, but their receipts MUST clearly identify non-canonical algorithm IDs
+- proof artifacts compare receipts first, bytes/pixels second, to keep backend changes explainable
@@
 pub struct RenderReport {
@@
     pub budget_events: Vec<BudgetEvent>,
+    pub materialization_receipts: Vec<MaterializationReceipt>,
 }
@@
 pub struct ExtractReport {
@@
     pub provenance_summary: Option<SurfaceProvenanceSummary>,
+    pub materialization_receipts: Vec<MaterializationReceipt>,
 }
```

## 3) Add metamorphic proof, reducer, and fixture genealogy lanes

**Why this makes it better**

The current proof doctrine leans heavily on pathological corpora, round trips, oracle comparison, blind-spot ledgers, and reproducibility manifests. That is excellent, but external renderers can share the same blind spot, and round trips do not explore enough semantically equivalent rewrites. A truly alien-artifact proof system should also include *metamorphic* tests: transforms that should preserve meaning, or preserve clearly bounded surfaces, even when representation changes.  

This is one of the best robustness multipliers available. It will catch whole classes of parser, writer, and extractor bugs that no single golden file or external oracle will catch.

```diff
diff --git a/SPEC.md b/SPEC.md
@@
+### Metamorphic proof doctrine
+
+Oracle comparison is necessary but not sufficient. Monkeybee MUST maintain a metamorphic proof lane
+that applies representation-changing transforms with known semantic expectations, then verifies
+which surfaces are preserved, which are allowed to drift, and which drifts are bugs.
+
+pub enum MetamorphicTransformKind {
+    ObjectRenumbering,
+    XrefTableToStreamReencoding,
+    XrefStreamToTableReencoding,
+    WhitespaceAndCommentPerturbation,
+    DictionaryKeyOrderPermutation,
+    StreamFilterRecomposition,
+    IncrementalChainSquash,
+    PageTreeRebalance,
+    FontSubsetPrefixRename,
+    OptionalContentConfigReorder,
+}
+
+pub struct MetamorphicExpectation {
+    pub preserved_surfaces: Vec<String>,
+    pub allowed_drift_surfaces: Vec<String>,
+    pub forbidden_drift_surfaces: Vec<String>,
+}
+
+pub struct MetamorphicWitness {
+    pub transform_kind: MetamorphicTransformKind,
+    pub before_digest: [u8; 32],
+    pub after_digest: [u8; 32],
+    pub expectation: MetamorphicExpectation,
+    pub observed_deltas: Vec<String>,
+    pub verdict: String,
+}
+
+Introduce `tests/metamorphic/` and `tests/reducers/`:
+- `tests/metamorphic/` for representation-preserving or bounded-drift transforms
+- `tests/reducers/` for deterministic minimization of crashes, disagreements, and proof failures
+
+Introduce `FixtureGenealogy`:
+pub struct FixtureGenealogy {
+    pub fixture_digest: [u8; 32],
+    pub parent_fixture: Option<[u8; 32]>,
+    pub derived_by: Option<MetamorphicTransformKind>,
+    pub reducer_chain: Vec<String>,
+}
```

## 4) Add a single machine-readable capability/support matrix

**Why this makes it better**

The spec already has a three-tier compatibility doctrine, explicit render determinism classes, baseline-vs-experimental classifications, and target-qualified notes for some features. But those truths are distributed. A project with this surface area needs a single generated support matrix keyed by feature, target, backend class, determinism class, required modules, and proof-gating status. Otherwise the docs, CLI, CI, website, and APR materials will drift.  

This is also how you keep ambition *while* making staging explicit.

```diff
diff --git a/SPEC.md b/SPEC.md
@@
+### Capability surface matrix doctrine
+
+Tier doctrine, target qualification, determinism class, and feature-module availability MUST be
+materialized into one schema-versioned source of truth.
+
+pub enum SupportClass {
+    ProofCanonical,
+    NativeCompatible,
+    NativeHardened,
+    WasmStrict,
+    WasmCompatible,
+    ExperimentalOnly,
+}
+
+pub struct CapabilitySurfaceMatrixEntry {
+    pub feature_code: FeatureCode,
+    pub support_class: SupportClass,
+    pub compatibility_tier: u8,
+    pub determinism_class: Option<RenderDeterminismClass>,
+    pub required_modules: Vec<String>,
+    pub required_providers: Vec<String>,
+    pub proof_gating: bool,
+    pub expected_degradations: Vec<FeatureCode>,
+}
+
+Rules:
+- README capability tables, website matrices, CLI capability output, and CI gates MUST be generated from `CapabilitySurfaceMatrix`
+- no public capability claim may be hand-maintained once the matrix exists
+- feature promotion from experimental to baseline-gating MUST update the matrix and attach proof references
+
+Add:
+- `docs/compatibility/capability_surface_matrix.yaml`
+- `monkeybee capability-matrix --json`
```

## 5) Turn save planning into a constraint graph with unsat-core evidence

**Why this makes it better**

The existing save-planning section is already one of the strongest parts of the plan: `WritePlan`, `BytePatchPlan`, signature impact analysis, preservation algebra, and policy composition are all there. The remaining gap is that the system still feels classification-first rather than solver-first. In a document engine this broad, save feasibility is a constraint problem: signatures, preserve regions, ownership, incremental closure, active-content stripping, downlevel/profile constraints, import aliasing, and transport continuity can interact in ways that are hard to reason about procedurally.  

A constraint graph plus an explicit unsat-core witness makes late save failure rarer and explanations much better.

```diff
diff --git a/SPEC.md b/SPEC.md
@@
+### Preservation constraint graph doctrine
+
+`WritePlan` selection MUST be backed by an explicit `PreservationConstraintGraph`.
+Save planning is a feasibility problem, not only an object-classification pass.
+
+pub enum PreservationConstraintKind {
+    SignedByteRangeIntegrity,
+    ForeignBytePreservation,
+    IncrementalAppendClosure,
+    ProfileConformance,
+    ActiveContentSanitization,
+    OutputIntentRetention,
+    CrossDocumentAliasUniqueness,
+    TransportContinuityDependency,
+    EncryptionHandlerConstraint,
+}
+
+pub struct PreservationConstraintNode {
+    pub node_id: String,
+    pub kind: PreservationConstraintKind,
+    pub subject_objects: Vec<ObjRef>,
+    pub summary: String,
+}
+
+pub struct PreservationConstraintEdge {
+    pub from: String,
+    pub to: String,
+    pub reason: String,
+}
+
+pub enum FeasibilityVerdict {
+    Feasible,
+    FeasibleWithEscalation,
+    Unsat,
+}
+
+pub struct FeasibilityWitness {
+    pub verdict: FeasibilityVerdict,
+    pub chosen_write_mode: WriteMode,
+    pub escalations: Vec<String>,
+    pub unsat_core: Vec<String>,
+    pub policy_digest: [u8; 32],
+    pub trace_digest: [u8; 32],
+}
+
+Rules:
+- `WritePlan::compute` MUST emit a `FeasibilityWitness`
+- any escalation from incremental append to full rewrite MUST cite the minimal blocking constraint set
+- save refusal due to incompatible constraints MUST return the unsat core, not a generic “cannot save in preserve mode”
@@
 pub struct WriteReceipt {
@@
     pub post_write_validation: Vec<ValidationFinding>,
+    pub feasibility_witness: Option<FeasibilityWitness>,
 }
```

## 6) Add a shared spatial evidence index

**Why this makes it better**

The plan already has exact invalidation, semantic anchors, prepress analysis, redaction audits, active-content inventories, and hidden-content detection. Those are all different faces of the same missing primitive: a page-space evidence index that knows which draw ops, text spans, resources, OCGs, and signed zones intersect which regions. Right now that is implied by page plans and semantic graphs; it should be explicit.  

This is one of the cleanest ways to make the engine both faster and more explainable: smaller invalidation cones, sharper redaction audits, region-local TAC analysis, better hit-testing, and more stable anchors.

```diff
diff --git a/SPEC.md b/SPEC.md
@@
+### Spatial evidence index doctrine
+
+Monkeybee MUST maintain a page-space `CoverageCellIndex` as a first-class derived index for
+invalidation, hit-testing, redaction auditing, semantic anchoring, and prepress region analysis.
+
+pub struct CoverageAtom {
+    pub bbox: Rect,
+    pub object_ref: Option<ObjRef>,
+    pub operator_span: Option<OperatorSpanRef>,
+    pub text_span: Option<TextSpanRef>,
+    pub resource_refs: Vec<ObjRef>,
+    pub ocg_membership: Vec<String>,
+    pub signed_overlap: bool,
+    pub hidden_content_flags: Vec<String>,
+    pub ink_coverage_estimate: Option<f32>,
+}
+
+pub struct CoverageCell {
+    pub cell_id: u64,
+    pub bbox: Rect,
+    pub atoms: Vec<CoverageAtom>,
+}
+
+pub struct CoverageCellIndex {
+    pub page_index: u32,
+    pub grid_policy: String,
+    pub cells: Vec<CoverageCell>,
+}
+
+Rules:
+- exact invalidation MAY descend to cell granularity when page-wide invalidation is provably unnecessary
+- redaction audits MUST query `CoverageCellIndex` before claiming visual or extractive erasure
+- semantic anchors MAY attach to cells as one stability layer beneath higher-level graph anchors
+- prepress region TAC and hidden-content detectors MUST reuse the same cell index rather than private region walkers
```

## 7) Strengthen remote/progressive with verified sparse blobs and range-Merkle manifests

**Why this makes it better**

The transport section is already much better than most PDF plans: it has `TransportIdentity`, `ByteAvailabilityMap`, `RangeConsistencyError`, and `TransportContinuityReceipt`. The next additive step is to make remote continuity not just session-local but resumable and cryptographically stronger when the upstream can provide more than weak validators. 

That matters for huge files, cache reuse across sessions, and any workflow where partial remote trust influences preserve claims or signatures.

```diff
diff --git a/SPEC.md b/SPEC.md
@@
+### Verified sparse blob doctrine
+
+Range-backed sessions MAY materialize a `VerifiedSparseBlob` so previously validated byte ranges can
+be reused across sessions without pretending the full file is locally complete.
+
+pub struct RangeMerkleLeaf {
+    pub range: (u64, u64),
+    pub digest: [u8; 32],
+}
+
+pub struct RangeMerkleManifest {
+    pub chunk_size: u64,
+    pub root_digest: [u8; 32],
+    pub leaves: Vec<RangeMerkleLeaf>,
+}
+
+pub struct VerifiedSparseBlob {
+    pub transport_identity: TransportIdentity,
+    pub merkle_manifest: Option<RangeMerkleManifest>,
+    pub verified_ranges: Vec<(u64, u64)>,
+    pub suspect_ranges: Vec<(u64, u64)>,
+    pub fetch_epoch: FetchEpoch,
+}
+
+pub struct ResumptionReceipt {
+    pub prior_blob_digest: [u8; 32],
+    pub reused_verified_ranges: Vec<(u64, u64)>,
+    pub revalidated_ranges: Vec<(u64, u64)>,
+    pub continuity_result: TransportContinuityReceipt,
+}
+
+Rules:
+- a resumed remote session MUST either adopt a prior `VerifiedSparseBlob` under matching transport identity or discard it
+- Merkle-qualified range attestations SHOULD outrank weak HTTP validators when both exist
+- preserve/signature-sensitive workflows MUST cite whether remote trust depended on weak validators, range digests, or Merkle manifests
```

## 8) Harden 3D from a flagship feature into a proof-canonical scene contract

**Why this makes it better**

The 3D lane is already ambitious and compelling: PRC/U3D parsing, a unified scene graph, wgpu rendering, named views, sections, and product structure navigation. Because it is so ambitious, it needs stronger additive proof surfaces so it does not degrade into “renders something cool” without the same evidentiary discipline as the 2D engine.  

The key upgrade is to make scene interpretation itself receiptable and comparable: topology digests, PMI inventories, named-view digests, section-plane provenance, and deterministic camera-path witnesses.

```diff
diff --git a/SPEC.md b/SPEC.md
@@
+### 3D scene receipt doctrine
+
+3D support MUST emit scene-level receipts, not only pixels. A successful 3D parse/render operation
+needs a stable semantic artifact describing what scene was interpreted.
+
+pub enum SceneFormat {
+    PRC,
+    U3D,
+    Mixed,
+}
+
+pub struct ViewStateDigest {
+    pub camera_digest: [u8; 32],
+    pub section_plane_digest: Option<[u8; 32]>,
+    pub render_mode: String,
+    pub visibility_mask_digest: [u8; 32],
+}
+
+pub struct SceneReceipt {
+    pub scene_format: SceneFormat,
+    pub scene_digest: [u8; 32],
+    pub mesh_count: u64,
+    pub part_count: u64,
+    pub material_count: u64,
+    pub pmi_item_count: u64,
+    pub named_view_digests: Vec<[u8; 32]>,
+    pub topology_witness_digest: Option<[u8; 32]>,
+    pub budget_class: String,
+}
+
+Rules:
+- 3D proof fixtures MUST compare scene receipts in addition to screenshots
+- deterministic camera-path witnesses MUST exist for named-view interpolation tests
+- section-plane operations MUST carry `ViewStateDigest` provenance so geometry and screenshot comparisons are explainable
+- PMI extraction and product-structure traversal MUST be exposed as inventory surfaces even when advanced interactive rendering degrades
@@
+Add `tests/3d/` with:
+- canonical named-view camera-path fixtures
+- large-scene budget fixtures
+- PMI inventory fixtures
+- section-plane determinism fixtures
```

## Highest-leverage ordering

If I were revising the plan immediately, I would prioritize them in this order:

1. **Numeric/geometry kernel**
2. **Constraint-graph save planning**
3. **Derived-artifact receipts**
4. **Metamorphic proof + reducer/genealogy**
5. **Spatial evidence index**
6. **Capability/support matrix**
7. **Verified sparse blobs**
8. **3D scene receipts**

That ordering preserves your existing baseline doctrine: substrate first, proof first, explainability first, then broader performance/usefulness surfaces.  

If you want, I can do a second pass that rewrites these into one consolidated mega-patch against `SPEC.md` with section placement chosen to match the current document structure exactly.
