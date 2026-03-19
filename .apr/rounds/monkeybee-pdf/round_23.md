I read both the README and the full plan. The through-line is unusually strong already: the closed loop, invariant substrate, exact invalidation, policy-complete operation planning, proof-first release gates, and explicit expansion lanes for 3D, signatures, accessibility, prepress, and forensics are all real architectural commitments, not fluff.    

My main conclusion: the plan is already excellent at naming *what* Monkeybee should own. The highest-leverage additive revisions are about making several of those promises even more operational: parser artifacts, cache admissibility, signed-save reservation, transport convergence, adversarial complexity, font authority, 3D canonicalization, and prepress proof conditions. Those are the places where a formidable system becomes harder to fool, easier to prove, and much faster in real use. The spec already has the right substrate/query/receipt vocabulary to absorb these cleanly.    

## 1) Add a first-class parser artifact tape and salvage index

The current spec is strong on lexer specifics, repair, preserve mode, and zero-copy/performance discipline, but it still jumps a bit too quickly from bytes to parsed COS. Adding an explicit parser artifact layer would make tolerant repair, lazy open, cross-reference salvage, provenance, and performance all sharper at once. This also gives Kani/fuzz/proof a stable target between raw bytes and semantic objects. The plan already wants bounded parsing, structured repairs, zero-copy spans, vectorized search, and exact provenance; this change turns those into one coherent surface.   

```diff
@@ Part 3 — `monkeybee-parser`
+ ### Parser artifact-tape and salvage-index doctrine
+
+ Parsing SHOULD materialize a first-class immutable parser artifact layer
+ between raw bytes and `monkeybee-syntax`.
+
+ pub enum LexKernelClass {
+     ScalarAuditable,
+     SimdAccelerated,
+ }
+
+ pub struct TokenTape {
+     pub tape_digest: [u8; 32],
+     pub token_count: u64,
+     pub lex_kernel: LexKernelClass,
+     pub source_spans: Vec<ByteSpanRef>,
+ }
+
+ pub struct ObjectBoundaryIndex {
+     pub tape_digest: [u8; 32],
+     pub candidate_objects: Vec<(ObjRef, ByteSpanRef)>,
+     pub xref_independent_hits: Vec<ObjRef>,
+ }
+
+ pub struct SalvageScanReceipt {
+     pub tape_digest: [u8; 32],
+     pub scanned_regions: Vec<ByteSpanRef>,
+     pub recovered_objects: Vec<ObjRef>,
+     pub false_positive_count: u64,
+ }
+
+ Rules:
+ - tolerant recovery SHOULD prefer `ObjectBoundaryIndex` and `TokenTape`
+   evidence before falling back to repeated whole-file rescans
+ - preserve-mode provenance MAY point to token-tape spans as an intermediate
+   witness between raw bytes and syntax objects
+ - proof-canonical runs MUST be able to report whether recovery came from xref,
+   tape-guided salvage, or full-object scan
```

## 2) Add a cache namespace admissibility contract

The spec already has exact invalidation, determinism classes, policy digests, transport epochs, provider policies, feature modules, and materialization receipts. What it still needs is one explicit doctrine answering: **when is a cached artifact even eligible for reuse?** Without that, the most subtle future bugs will be cross-mode cache poisoning between proof-canonical, viewer-adaptive, remote, GPU, native, and different provider manifests.    

```diff
@@ Part 3 — Incremental query engine doctrine
+ ### Cache namespace and reuse-admissibility doctrine
+
+ pub struct CacheNamespaceDigest(pub [u8; 32]);
+
+ pub struct CacheNamespace {
+     pub namespace_digest: CacheNamespaceDigest,
+     pub policy_digest: [u8; 32],
+     pub support_class: SupportClass,
+     pub render_determinism: Option<RenderDeterminismClass>,
+     pub native_isolation: Option<NativeIsolationClass>,
+     pub provider_manifest_digest: [u8; 32],
+     pub feature_module_manifest_digest: [u8; 32],
+     pub fetch_epoch: Option<FetchEpoch>,
+ }
+
+ pub struct ReuseAdmissibilityReceipt {
+     pub artifact_digest: [u8; 32],
+     pub namespace_digest: CacheNamespaceDigest,
+     pub admissible: bool,
+     pub rejection_reason: Option<String>,
+ }
+
+ Rules:
+ - cross-snapshot reuse requires both dependency validity and namespace
+   admissibility
+ - proof-canonical artifacts MUST never reuse viewer-adaptive or ambient-provider
+   cache entries
+ - transport-epoch drift, provider-manifest drift, or native-isolation drift
+   MUST invalidate reuse even when source digests match
```

## 3) Add a page coverage atlas and paint-order witness

The README already names coverage-cell indexes, render-chunk graphs, text-paint correspondence, semantic anchors, and geometry witnesses. The next additive step is to unify them into a stable *coverage atlas* that can answer “what painted here, in what order, with what occlusion and provenance?” This would materially improve redaction proof, hit-testing, annotation snapping, disagreement localization, hidden-content forensics, and exact invalidation.   

```diff
@@ Part 3 — Content / Render / Extract shared surfaces
+ ### Coverage-atlas and paint-order witness doctrine
+
+ pub struct CoverageCellId(pub [u8; 32]);
+
+ pub struct PaintOrderInterval {
+     pub z_min: u32,
+     pub z_max: u32,
+     pub contributing_chunks: Vec<RenderChunkId>,
+ }
+
+ pub struct CoverageCell {
+     pub cell_id: CoverageCellId,
+     pub bbox: Rectangle,
+     pub paint_order: PaintOrderInterval,
+     pub text_spans: Vec<SpanId>,
+     pub annotations: Vec<AnnotationId>,
+ }
+
+ pub struct CoverageAtlas {
+     pub page_index: u32,
+     pub cells: Vec<CoverageCell>,
+     pub atlas_digest: [u8; 32],
+ }
+
+ Rules:
+ - redaction audit, hit-testing, annotation placement, hidden-content analysis,
+   and oracle-disagreement localization SHOULD consume `CoverageAtlas`
+ - chunk-level invalidation MAY reuse unaffected coverage cells directly
+ - `CoverageAtlas` MUST be receiptable as a derived artifact
```

## 4) Add signature reservation, append-budget, and overflow doctrine

The plan is already very strong on ByteRange integrity, incremental append, save planning, PAdES/DSS/VRI, and preserve-mode feasibility. The missing operational piece is **reservation economics**: placeholder sizing, DSS/VRI growth, timestamp growth, and deterministic overflow handling. Real signing systems often fail here, not in CMS parsing.   

```diff
@@ Part 5 / signature + write planning surfaces
+ ### Signature reservation and append-budget doctrine
+
+ pub enum ReservationOverflowPolicy {
+     RefuseAndExplain,
+     EscalateToLargerUnsignedPlaceholder,
+     EscalateToCounterfactualPlan,
+ }
+
+ pub struct SignatureReservationPlan {
+     pub field_ref: ObjRef,
+     pub reserved_contents_bytes: u64,
+     pub reserved_dss_bytes: u64,
+     pub reserved_vri_bytes: u64,
+     pub overflow_policy: ReservationOverflowPolicy,
+ }
+
+ pub struct AppendBudgetReceipt {
+     pub prior_revision_count: u32,
+     pub predicted_append_bytes: u64,
+     pub reserved_bytes: u64,
+     pub overflow_risk: String,
+ }
+
+ Rules:
+ - signing workflows MUST compute reservation plans before byte emission
+ - under-reserved placeholders MUST fail with an explicit receipt, never with a
+   late opaque serialization failure
+ - counterfactual planning SHOULD propose the nearest legal larger-reservation
+   signing plan when the requested reservation is too small
```

## 5) Add transport convergence and sparse-blob proof

The spec already has fetch epochs, range consistency errors, persistent range cache, and transport continuity receipts. That is strong. The next level is making remote correctness convergent: when does a sparse range-backed session become trusted as a whole, and under what proof artifact? This matters for remote-first open, persisted artifact reuse, and later save/verification workflows.  

```diff
@@ Part 3 — `monkeybee-bytes`
+ ### Sparse-convergence and whole-source verification doctrine
+
+ pub enum SparseConvergenceClass {
+     SparseUnverified,
+     SparseValidatorBound,
+     WholeFileDigestVerified,
+ }
+
+ pub struct RangeConflictWitness {
+     pub conflicting_ranges: Vec<(u64, u64)>,
+     pub prior_epoch: FetchEpoch,
+     pub new_epoch: FetchEpoch,
+     pub reason: RangeConsistencyError,
+ }
+
+ pub struct SourceConvergenceReceipt {
+     pub source_identity: TransportIdentity,
+     pub convergence_class: SparseConvergenceClass,
+     pub sparse_digest_map: SparseDigestMap,
+     pub whole_file_digest: Option<[u8; 32]>,
+ }
+
+ Rules:
+ - persisted remote artifacts SHOULD record whether they were built from sparse
+   validator-bound state or whole-file-verified state
+ - save, signature, and proof workflows MAY require `WholeFileDigestVerified`
+   unless an explicit degraded policy allows otherwise
+ - range conflicts MUST poison only the affected epoch, not silently downgrade
+   global trust
```

## 6) Add an adversarial complexity-hazard doctrine

The current plan has budgets, complexity fingerprints, stress classes, risky decoders, strict/hardened profiles, and fuzzing. What it still needs is a typed vocabulary for *which asymptotic bomb* was detected. That becomes crucial when you start seeing PRC mesh explosions, Type 4 function bombs, giant name trees, pathological object streams, font explosions, and deliberately hostile incremental chains.   

```diff
@@ Part 7 — Performance and safety doctrine
+ ### Complexity-hazard doctrine
+
+ pub enum ComplexityHazardKind {
+     OperatorExplosion,
+     ObjectStreamFanout,
+     IncrementalChainExplosion,
+     FunctionStepExplosion,
+     NameTreeBreadthExplosion,
+     FontTableExplosion,
+     MeshPrimitiveExplosion,
+     DecodeExpansionExplosion,
+ }
+
+ pub struct ComplexityHazard {
+     pub kind: ComplexityHazardKind,
+     pub subject: String,
+     pub observed_scale: u64,
+     pub mitigation: String,
+ }
+
+ pub struct BudgetDerivationReceipt {
+     pub fingerprint: ComplexityFingerprint,
+     pub hazards: Vec<ComplexityHazard>,
+     pub chosen_budget: BudgetRecommendation,
+ }
+
+ Rules:
+ - `AdmissionDecision` SHOULD cite typed hazards, not only a coarse rejection
+   reason
+ - proof and corpus aggregation SHOULD report failures by hazard kind
+ - security profiles MAY tighten budgets hazard-by-hazard instead of only with
+   one global multiplier
```

## 7) Add a font authority graph and deterministic subset-closure receipts

You already have font truth classes, repair receipts, CMap fallback, glyph/render truth, and emitted-subset planning. The missing additive piece is a unified graph of *which surface is authoritative for which font fact*: Unicode mapping, metrics, outlines, variation data, vertical metrics, and subset closure. This will improve extraction truth, render/extract consistency, diff explainability, generated-doc determinism, and signature-safe reuse.  

```diff
@@ Part 2 — Font truth and repair witness doctrine
+ ### Font authority-graph and subset-closure doctrine
+
+ pub enum FontAuthoritySurface {
+     UnicodeMap,
+     AdvanceMetrics,
+     OutlineProgram,
+     VerticalMetrics,
+     VariationData,
+     SubsetClosure,
+ }
+
+ pub struct FontAuthorityEdge {
+     pub surface: FontAuthoritySurface,
+     pub authority: FontTruthClass,
+     pub source: String,
+ }
+
+ pub struct FontAuthorityGraph {
+     pub font_fingerprint: ResourceFingerprint,
+     pub edges: Vec<FontAuthorityEdge>,
+ }
+
+ pub struct SubsetClosureReceipt {
+     pub font_fingerprint: ResourceFingerprint,
+     pub glyph_ids: Vec<u32>,
+     pub subroutines_retained: Vec<u32>,
+     pub canonical_subset_digest: [u8; 32],
+ }
+
+ Rules:
+ - emitted font subsets MUST be reproducible under deterministic mode
+ - extraction/render/diff surfaces SHOULD be able to cite `FontAuthorityGraph`
+ - subset closure receipts SHOULD be stored whenever generated output or form
+   appearance regeneration embeds a new font subset
```

## 8) Add 3D scene normal forms and tessellation determinism

The 3D lane is already unusually ambitious: PRC/U3D, product structure, named views, cross-sections, OIT, and scene receipts. The next additive requirement is a *scene normal form* that lets you distinguish “same interpreted scene” from “same screenshot.” Otherwise 3D proof will remain too image-heavy and backend-sensitive.  

```diff
@@ Part 3 — `monkeybee-3d`
+ ### Scene normal-form and tessellation-determinism doctrine
+
+ pub enum SceneNormalFormKind {
+     TopologyCanonical,
+     TessellationCanonical,
+     ViewStateCanonical,
+ }
+
+ pub struct SceneNormalForm {
+     pub kind: SceneNormalFormKind,
+     pub canonical_digest: [u8; 32],
+     pub omitted_surfaces: Vec<String>,
+ }
+
+ pub struct TessellationReceipt {
+     pub source_scene_digest: [u8; 32],
+     pub tessellation_policy: String,
+     pub vertex_count: u64,
+     pub index_count: u64,
+     pub deterministic: bool,
+ }
+
+ Rules:
+ - 3D proof SHOULD compare normal forms before screenshots whenever the claim is
+   semantic/topological rather than purely visual
+ - named-view interpolation and section-plane tests SHOULD cite both
+   `SceneNormalForm` and `TessellationReceipt`
+ - backend-specific triangle ordering MUST not silently perturb proof claims
```

## 9) Add a prepress proof-condition lattice

The prepress lane is already serious: output intents, TAC, overprint simulation, halftones, BG/UCR, trap handling, separation preview, and preflight. What would make it materially more enterprise-compelling is an explicit proof-condition object representing the press/viewing condition itself. That turns prepress from “we have features” into “we can reproduce this proof context.”  

```diff
@@ Part 2 / prepress expansion contract
+ ### Proof-condition lattice for prepress and soft-proofing
+
+ pub struct ProofCondition {
+     pub target_profile: OutputIntentRef,
+     pub rendering_intent: String,
+     pub black_point_compensation: bool,
+     pub tac_limit_percent: Option<f32>,
+     pub screening_mode: Option<String>,
+     pub trap_policy: Option<String>,
+ }
+
+ pub struct SeparationReceipt {
+     pub condition_digest: [u8; 32],
+     pub colorants: Vec<String>,
+     pub overprint_mode: Option<u8>,
+     pub simulated_on_rgb: bool,
+ }
+
+ pub struct InkHazardGraph {
+     pub page_index: u32,
+     pub hotspots: Vec<RegionRef>,
+     pub hazard_codes: Vec<String>,
+ }
+
+ Rules:
+ - soft-proof, separation preview, TAC reporting, and trap diagnostics SHOULD
+   all cite a shared `ProofCondition`
+ - benchmark and oracle artifacts for prepress MUST record the active proof
+   condition so comparisons stay meaningful
```

## 10) Add algorithm-variant manifests for hot-path tournaments

The plan already names strategy tournaments, experimental annex rules, proof metrics, and performance witnesses. I would make that even more concrete by turning every competing hot-path implementation into a first-class manifest entry. This lets Monkeybee pursue extreme optimization without letting “clever fast path” silently become “unexplained semantic drift.”  

```diff
@@ Part 7 — Performance doctrine
+ ### Algorithm-variant manifest doctrine
+
+ pub struct AlgorithmVariantId(pub [u8; 32]);
+
+ pub struct AlgorithmVariantManifest {
+     pub variant_id: AlgorithmVariantId,
+     pub subsystem: String,
+     pub baseline_variant: String,
+     pub active_variant: String,
+     pub proof_metric: String,
+     pub cost_metric: String,
+ }
+
+ pub struct VariantSelectionReceipt {
+     pub subsystem: String,
+     pub chosen_variant: AlgorithmVariantId,
+     pub rejected_variants: Vec<AlgorithmVariantId>,
+     pub reason: String,
+ }
+
+ Rules:
+ - SIMD, GPU, spectral color, robust-predicate, rasterizer, and compression
+   tournaments SHOULD emit variant manifests in canonical benchmarks
+ - no experimental winner may become default without receiptable evidence
+   against its baseline competitor
```

## What I would prioritize first

If I were revising the spec immediately, I would land these in this order:

First: **cache namespace**, **parser artifact tape**, and **signature reservation**. Those three most directly reduce future correctness bugs while also improving speed and explainability.

Second: **transport convergence**, **complexity hazards**, and **font authority graph**. Those are the difference between a strong engine and one that survives the ugliest real-world PDFs without hidden trust failures.

Third: **3D scene normal forms** and **prepress proof conditions**. Those make the project’s most differentiating lanes substantially more defensible in proof, demos, and enterprise adoption narratives.

Net: I would not shrink anything. I would make the existing alien-artifact thesis *more formal at the joins*—especially the joins between parse and repair, cache and proof, save planning and signing, remote open and trust, and rich features and canonical evidence. That is where the current plan is already strongest in spirit and where additive precision would compound the most.   

If you want, I can turn this into a full patch against `SPEC.md` section-by-section.
