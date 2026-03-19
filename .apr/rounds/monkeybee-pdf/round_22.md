I read both the README and the plan. The plan is already unusually strong on the parts that matter most: a single content-addressed substrate, exact invalidation, preservation-aware save planning, deterministic/public proof artifacts, and a clear baseline-vs-experimental doctrine. My revisions therefore do **not** trim or simplify anything. They only add deeper kernel ownership, stronger proof surfaces, and harder performance contracts on top of what is already there.   

The biggest opportunity is this: the spec already says Monkeybee should feel like one coherent engine, but a few high-value surfaces are still described as cross-cutting behavior rather than being ratified as first-class architectural kernels. Those are the places I would grow it.  

## 1) Add a dedicated `monkeybee-color` crate and a first-class color kernel doctrine

**Why this makes the project better**

Right now, color/prepress semantics are intentionally cross-cutting: render, validate, extract, and prepress all depend on them. The spec already treats ICC interpolation, transfer/BG/UCR math, overprint simulation, output intents, DeviceN, separations, and soft proofing as serious engine surfaces. That is exactly why they should not stay smeared across `render`, `validate`, `extract`, and ad hoc helpers. A dedicated color kernel gives you one place for ICC evaluation, DeviceN resolution, tint transforms, black-point compensation, output-intent cascading, separation preview, and proof receipts. It also makes performance work much cleaner because color transforms become reusable materialized artifacts with stable digests and cache keys instead of hidden subroutines inside rendering.   

It also strengthens the “alien artifact” story materially. A serious PDF engine that owns prepress and color should have a real color kernel, not just a long list of color features. This is one of the clearest places where adding a new crate increases both coherence and ambition. 

````diff
diff --git a/README.md b/README.md
@@ Workspace crates:
+| `monkeybee-color` | First-class color/prepress kernel: ICC/profile evaluation, rendering intents, black-point compensation, DeviceN/Separation resolution, tint-transform execution, output-intent cascade, separation preview, TAC accounting, and color witness emission |

diff --git a/SPEC.md b/SPEC.md
@@ Part 3 — System architecture / Workspace layout
+#### `monkeybee-color`
+
+Dedicated color and prepress kernel shared by render, extract, validate, forms,
+compose, and proof.
+
+Key responsibilities:
+- ICC profile parsing, linking, and transform evaluation
+- Rendering-intent resolution including black-point compensation
+- DeviceGray/RGB/CMYK/Lab/ICCBased/Separation/DeviceN/NChannel handling
+- Tint-transform execution and alternate-space fallback
+- Output-intent cascade (document-level and page-level)
+- Separation preview, ink accounting, TAC estimation, and soft-proof primitives
+- Color witness and hazard reporting surfaces reused by prepress, proof, and diff
+
+### Color kernel doctrine
+
+```rust
+pub enum ColorKernelClass {
+    ProofCanonical,
+    DeterministicCpu,
+    SimdAccelerated,
+    ExperimentalGpu,
+}
+
+pub struct OutputIntentCascade {
+    pub document_intents: Vec<OutputIntentRef>,
+    pub page_intents: Vec<(u32, OutputIntentRef)>,
+    pub chosen_per_page: Vec<(u32, OutputIntentRef)>,
+}
+
+pub struct DeviceNResolutionPlan {
+    pub colorant_names: Vec<String>,
+    pub alternate_space: ColorSpaceRef,
+    pub tint_transform_digest: [u8; 32],
+    pub proof_mode: Option<String>,
+}
+
+pub struct ColorWitness {
+    pub kernel_class: ColorKernelClass,
+    pub source_space: String,
+    pub target_space: String,
+    pub output_intent_digest: Option<[u8; 32]>,
+    pub bpc_enabled: bool,
+    pub device_n_plan: Option<DeviceNResolutionPlan>,
+    pub hazard_codes: Vec<String>,
+}
+```
+
+Rules:
+- all ICC interpolation, output-intent resolution, soft-proof, TAC, and
+  Separation/DeviceN math MUST route through `monkeybee-color`
+- `ColorTransformCache` and related receipts MUST identify the active
+  `ColorKernelClass`
+- prepress inspection, render, and validation reports SHOULD be able to cite a
+  `ColorWitness` digest
````

## 2) Add a transport continuity and fetch-epoch model for remote/progressive PDFs

**Why this makes the project better**

Workflow 8 already promises progressive open, partial rendering, placeholders, and access planning. The README also explicitly says the engine should explain “transport continuity.” But the current plan does not fully ratify what happens when the remote object changes mid-session, when ranges come from different entity versions, or when cached artifacts were built against an earlier fetch epoch. That is a correctness gap, not just a networking detail.  

A real remote consistency model prevents subtle corruption: mixed-epoch page plans, placeholder refinement against stale ETags, or save plans formed against bytes that are no longer the same document. It also makes proofs stronger because you can witness that a progressive render was transport-consistent, not merely “fast.” 

````diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 3 — System architecture / monkeybee-bytes
+### Transport continuity doctrine
+
+Remote and progressive sessions MUST reason about continuity of the underlying
+byte source across time, not only about which ranges have been fetched.
+
+```rust
+pub struct FetchEpochId(pub u128);
+
+pub enum TransportContinuityClass {
+    SingleEntityVerified,
+    MultiRangeVerified,
+    WeakValidatorOnly,
+    ContinuityBroken,
+}
+
+pub struct RangeWitness {
+    pub epoch: FetchEpochId,
+    pub start: u64,
+    pub end: u64,
+    pub etag: Option<String>,
+    pub content_length: Option<u64>,
+    pub validator_strength: String,
+}
+
+pub struct TransportContinuityReceipt {
+    pub continuity_class: TransportContinuityClass,
+    pub root_entity_id: Option<String>,
+    pub fetched_ranges: Vec<RangeWitness>,
+    pub invalidated_artifacts: Vec<ArtifactRef>,
+    pub trace_digest: [u8; 32],
+}
+```
+
+Rules:
+- every remote/progressive session MUST track a `FetchEpochId`
+- page plans, placeholder refinements, and access plans derived from remote
+  bytes MUST record the fetch epoch they observed
+- if remote validators drift mid-session, the engine MUST classify continuity as
+  broken, invalidate affected derived artifacts, and refuse preserve-sensitive
+  workflows until continuity is re-established
+- `RenderReport`, `AccessPlan`, and failure capsules for remote fixtures SHOULD
+  reference a `TransportContinuityReceipt`
````

## 3) Add an out-of-core segmented artifact store for huge documents

**Why this makes the project better**

The spec is already serious about huge files, spillable stores, exact invalidation, and persisted artifacts. But for truly large PDFs, the missing ratification is segmentation: page-local closures, object-neighborhood indexes, and segment-level receipts that keep peak memory bounded without degrading into whole-document churn.  

This is one of the best “extreme optimization doctrine” additions because it directly affects memory ceilings, reopen cost, large-fixture benchmarking, and progressive viewing. It gives the substrate a more explicit out-of-core story rather than treating spill as a generic cache behavior. 

````diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 3 — System architecture / substrate-store lifecycle doctrine
+### Segmented artifact-store doctrine
+
+Large documents SHOULD materialize reusable artifacts in page-closure or
+object-neighborhood segments rather than monolithic page- or document-wide blobs.
+
+```rust
+pub struct SegmentId(pub [u8; 32]);
+
+pub enum SegmentKind {
+    PageClosure,
+    ObjectNeighborhood,
+    RenderChunkRegion,
+    SemanticRegion,
+    RevisionFrame,
+}
+
+pub struct ArtifactSegment {
+    pub segment_id: SegmentId,
+    pub kind: SegmentKind,
+    pub source_nodes: Vec<NodeDigest>,
+    pub estimated_working_set_bytes: u64,
+}
+
+pub struct WorkingSetForecast {
+    pub hot_segments: Vec<SegmentId>,
+    pub cold_segments: Vec<SegmentId>,
+    pub predicted_peak_bytes: u64,
+}
+
+pub struct SpillReceipt {
+    pub segment_id: SegmentId,
+    pub reason: String,
+    pub persisted: bool,
+    pub policy_digest: [u8; 32],
+}
+```
+
+Rules:
+- huge-document paths SHOULD prefer segment materialization over monolithic
+  artifact materialization when the same correctness class can be preserved
+- benchmark witnesses for huge fixtures SHOULD include `WorkingSetForecast`
+  evidence
+- spill/persist decisions for segmented artifacts SHOULD emit `SpillReceipt`s
+  so peak-memory claims remain explainable
````

## 4) Add a text-paint correspondence graph

**Why this makes the project better**

The current spec already has extraction truth classes, font truth, semantic anchors, render chunk graphs, and geometry witnesses. What it does **not** yet own as explicitly as it should is the bridge between extracted spans and painted glyph instances. That bridge is gold for redaction, search highlighting, selection, diff explainability, and forensic claims like “this extractable text corresponds to these painted glyphs in these chunks.”  

Without that bridge, render and extract stay parallel truths. With it, they become one substrate-backed truth surface. That is exactly the kind of additive coherence the project wants. 

````diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 3 — extraction / render integration
+### Text-paint correspondence doctrine
+
+Monkeybee MUST be able to relate extracted spans, glyph paint operations, and
+render chunks through stable witnesses.
+
+```rust
+pub struct GlyphInstanceId(pub [u8; 32]);
+
+pub struct PaintedGlyphWitness {
+    pub glyph_instance_id: GlyphInstanceId,
+    pub font_fingerprint: ResourceFingerprint,
+    pub charcode: u32,
+    pub unicode: Option<char>,
+    pub bbox: Rectangle,
+    pub render_chunk_id: RenderChunkId,
+    pub provenance: ProvenanceAtom,
+}
+
+pub struct TextPaintLink {
+    pub span_id: SpanId,
+    pub glyph_instances: Vec<GlyphInstanceId>,
+    pub continuity_class: TextTruthClass,
+}
+
+pub struct TextPaintReceipt {
+    pub page_index: u32,
+    pub links: Vec<TextPaintLink>,
+    pub orphan_paint_glyphs: Vec<GlyphInstanceId>,
+    pub orphan_extract_spans: Vec<SpanId>,
+}
+```
+
+Rules:
+- selection, search hit-testing, highlight, and redaction workflows SHOULD be
+  able to request a `TextPaintReceipt`
+- proof fixtures for redaction and extraction SHOULD compare text/paint linkage
+  when the underlying fixture contains both extractable and painted text
+- text/paint disagreements MUST be surfaced explicitly rather than silently
+  normalized away
````

## 5) Add an experimental `BytePatch` write mode between incremental append and full rewrite

**Why this makes the project better**

The current write space has deterministic rewrite, incremental append, and downlevel output. That is strong, but there is still a missing world-class lane: a strictly constrained, proof-heavy **byte patch** mode for constant-length or length-neutral edits when the engine can prove exact offset safety and preservation legality. This is the natural extension of the existing preservation-constraint graph and feasibility witness model.  

This would be extremely compelling in practice for forensic or regulated workflows: metadata corrections, narrow appearance fixes, selected field updates, or other edits where full rewrite is too destructive and incremental append is too heavy or semantically undesirable. It should absolutely be experimental and non-gating, but it is exactly the sort of hard, differentiated capability this project should name. 

```diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 0 — Operational mode doctrine / Write modes
+- **Byte patch (experimental):** Apply a length-safe or explicitly
+  re-offsetted patch to a bounded owned region when the engine can prove that
+  all affected xref, stream-extent, signature, and preservation constraints
+  remain satisfied. This mode is non-gating until proof-backed.

@@ Part 3 — Preservation constraint graph doctrine
+pub enum PreservationConstraintKind {
+    ...
+    InPlacePatchClosure,
+    StableOffsetWindow,
+}
+
+pub struct BytePatchAtom {
+    pub target_span: ByteSpanRef,
+    pub original_digest: [u8; 32],
+    pub replacement_digest: [u8; 32],
+    pub constant_length: bool,
+}
+
+pub struct BytePatchPlan {
+    pub atoms: Vec<BytePatchAtom>,
+    pub affected_xref_entries: Vec<ObjRef>,
+    pub signed_range_intersections: Vec<ByteRangeRef>,
+    pub safety_class: String,
+    pub feasibility: FeasibilityWitness,
+}
+
+pub struct BytePatchReceipt {
+    pub patch_plan_digest: [u8; 32],
+    pub applied_atoms: Vec<BytePatchAtom>,
+    pub preserved_signed_ranges: Vec<ByteRangeRef>,
+}
+
+Rules:
+- `BytePatch` is allowed only for `Owned` regions that satisfy
+  `InPlacePatchClosure`
+- any overlap with signed ranges, unknown foreign spans, or unstable stream
+  extents MUST force rejection or escalation to another write mode
+- experimental byte-patch runs MUST emit both `BytePatchPlan` and
+  `BytePatchReceipt`
```

## 6) Add a decoder equivalence laboratory for risky native and experimental paths

**Why this makes the project better**

The spec already has feature modules, native isolation, decoder attestations, strategy tournaments, and failure capsules. What is still under-specified is equivalence: when a native JPX/JBIG2/ICC path or an experimental GPU path is “better” or merely “different.” Add a formal shadow-execution/equivalence lab so those modules compete not just on speed and crashes, but on typed semantic parity.  

This makes the native bridge much more trustworthy. It converts “quarantined native decoder” from a safety story into a measurable correctness story. 

````diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 6 — proof doctrine
+### Decoder equivalence laboratory
+
+Any risky native or experimental decode/render path SHOULD support shadow
+execution against a baseline path on representative fixtures.
+
+```rust
+pub enum EquivalenceSurface {
+    DecodedPixels,
+    DecodedMask,
+    DecodedText,
+    ICCTransform,
+    StreamBytes,
+}
+
+pub struct DecoderEquivalenceRecord {
+    pub module_id: String,
+    pub baseline_algorithm_id: String,
+    pub candidate_algorithm_id: String,
+    pub surface: EquivalenceSurface,
+    pub verdict: String,
+    pub metric_summary: Vec<(String, f64)>,
+    pub fixture_digest: [u8; 32],
+}
+```
+
+Rules:
+- feature modules such as `jpx_native`, `jbig2_isolated`, and GPU render paths
+  SHOULD participate in equivalence tournaments before promotion
+- promotion requires both safety attestations and equivalence evidence
+- disagreement records SHOULD be able to cite `DecoderEquivalenceRecord`s when
+  a divergence originates below the page-render level
````

## 7) Replace the prose-only producer quirk catalog with a machine-readable producer phenotype registry

**Why this makes the project better**

The plan already has an excellent prose quirk catalog: Acrobat, Word, LaTeX, Chrome, LibreOffice, InDesign, Quartz. That is strong as documentation, but it should become engine-visible data. A phenotype registry lets the proof harness cluster fixture behavior, lets quirk activation be receipted rather than implicit, and lets the project discover new producer families from corpus evidence instead of only hand-maintaining them. 

This is one of the best additions for long-term reliability because it turns “producer quirks” into something measurable, discoverable, and explainable. 

````diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 2 — Producer quirks
+### Producer phenotype registry
+
+Producer handling MUST grow from a prose catalog into a machine-readable
+registry of structural phenotypes and quirk activations.
+
+```rust
+pub struct ProducerPhenotypeId(pub [u8; 32]);
+
+pub struct ProducerPhenotype {
+    pub phenotype_id: ProducerPhenotypeId,
+    pub declared_producer: Option<String>,
+    pub structural_markers: Vec<String>,
+    pub typical_repairs: Vec<String>,
+    pub quirk_modules: Vec<String>,
+}
+
+pub struct PhenotypeEvidence {
+    pub matched_markers: Vec<String>,
+    pub confidence: f32,
+}
+
+pub struct QuirkActivationReceipt {
+    pub phenotype_id: Option<ProducerPhenotypeId>,
+    pub activated_quirks: Vec<String>,
+    pub suppressed_quirks: Vec<String>,
+    pub reason: String,
+}
+```
+
+Rules:
+- tolerant open SHOULD emit a `QuirkActivationReceipt` whenever producer-specific
+  shims materially influence recovery or interpretation
+- proof aggregation SHOULD report repair frequency by phenotype, not only by
+  free-form producer string
+- newly discovered phenotype clusters MAY be proposed by corpus tooling but MUST
+  be reviewable artifacts before becoming default
````

## 8) Add a formal coverage lattice and corpus acquisition planner

**Why this makes the project better**

The README and spec already have blind-spot ledgers, scope registries, capability matrices, and proof artifacts. The obvious next step is a formal **coverage lattice** over feature × producer × operation × mode × backend × support class. That turns “blind spots” into a navigable mathematical object rather than a collection of warnings.  

This is especially valuable because the project has explicit expansion waves and named inventory counts. A coverage lattice tells you which new capabilities are genuinely under-proven, not merely under-implemented. 

````diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 6 — proof doctrine
+### Coverage lattice doctrine
+
+Blind-spot ledgers SHOULD be backed by a formal coverage lattice so corpus
+acquisition, promotion, and release claims can be computed rather than narrated.
+
+```rust
+pub struct CoverageCell {
+    pub feature_code: String,
+    pub producer_phenotype: Option<ProducerPhenotypeId>,
+    pub operation: String,
+    pub mode: String,
+    pub backend_class: Option<RenderDeterminismClass>,
+    pub support_class: SupportClass,
+}
+
+pub struct CoverageObservation {
+    pub cell: CoverageCell,
+    pub fixture_count: u32,
+    pub last_verdict: String,
+    pub disagreement_rate: f32,
+}
+
+pub struct CoverageLattice {
+    pub cells: Vec<CoverageObservation>,
+}
+
+pub struct AcquisitionRecommendation {
+    pub target_cells: Vec<CoverageCell>,
+    pub reason: String,
+    pub expected_signal_gain: f32,
+}
+```
+
+Rules:
+- release-facing capability claims SHOULD be derivable from `CoverageLattice`
+  observations plus the scope registry
+- proof planning SHOULD be able to emit `AcquisitionRecommendation`s for
+  under-covered cells
+- promotion of experimental paths SHOULD cite lattice coverage, not only raw
+  benchmark wins
````

## 9) Add anchor volatility scores and repair receipts

**Why this makes the project better**

The spec is already unusually thoughtful about semantic anchors, alias maps, rebase, and anchor continuity classes. The missing next layer is **fragility**: how likely is this anchor to survive safe rewrites, incremental append, resource rebinding, or layout inference drift? That is crucial for agent-safe edits and operational explainability.   

Adding volatility scores makes the agent story much more honest. It lets the engine say not just “anchor found,” but “anchor found with exact continuity,” “alias mapped,” or “heuristically repaired with these fragility causes.” 

````diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 3 — semantic anchors
+### Anchor volatility doctrine
+
+Semantic anchors MUST expose not only continuity class but also fragility and
+repair evidence.
+
+```rust
+pub enum AnchorFragilityCause {
+    GlyphDecodeWeak,
+    ReadingOrderHeuristic,
+    TableHypothesisDependent,
+    PageReflowLikeEdit,
+    ResourceSubstitution,
+    AmbiguousRepairLineage,
+}
+
+pub struct AnchorVolatilityScore {
+    pub anchor_id: SemanticAnchorId,
+    pub score_0_to_1: f32,
+    pub causes: Vec<AnchorFragilityCause>,
+}
+
+pub struct AnchorRepairReceipt {
+    pub prior_anchor: SemanticAnchorId,
+    pub current_anchor: Option<SemanticAnchorId>,
+    pub continuity: AnchorContinuityClass,
+    pub volatility: AnchorVolatilityScore,
+    pub neighborhood_fingerprint: [u8; 32],
+}
+```
+
+Rules:
+- agent-facing edit proposals SHOULD be able to demand a maximum allowed
+  anchor-volatility threshold
+- safe rewrite and incremental-append proof fixtures SHOULD compare
+  `AnchorRepairReceipt`s, not only pass/fail continuity classes
+- `CapabilityReport` and query surfaces MAY summarize high-volatility regions
+  for caller guidance
````

## 10) Upgrade the preservation frontier from a graph to a solver-grade planning surface

**Why this makes the project better**

The spec already has the right conceptual move: preservation is a feasibility problem, minimal blocking sets matter, and counterfactual frontiers should be surfaced when cheap. The additive improvement is to explicitly promote this to a solver-grade subsystem with deterministic minimal-unsat-core extraction, frontier ranking, and Pareto receipts. That is the difference between “a smart save planner” and a genuinely formidable one. 

This is one of the most important revisions because save planning is central to the whole closed loop. It affects signatures, preserve mode, imports, sanitization, and user trust. 

````diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 3 — Preservation constraint graph doctrine
+### Preservation frontier solver doctrine
+
+`PreservationConstraintGraph` SHOULD have an explicit deterministic solver layer
+for minimal-unsat-core extraction and bounded Pareto-frontier enumeration.
+
+```rust
+pub struct FrontierPointId(pub [u8; 32]);
+
+pub struct PreservationFrontierPoint {
+    pub point_id: FrontierPointId,
+    pub write_mode: WriteMode,
+    pub preserved: Vec<PreservedProperty>,
+    pub sacrificed: Vec<PreservedProperty>,
+    pub cost: CostEnvelope,
+    pub feasibility: FeasibilityVerdict,
+}
+
+pub struct FrontierWitness {
+    pub policy_digest: [u8; 32],
+    pub minimal_unsat_cores: Vec<Vec<String>>,
+    pub pareto_points: Vec<PreservationFrontierPoint>,
+    pub chosen_point: Option<FrontierPointId>,
+}
+```
+
+Rules:
+- deterministic mode MUST fix minimal-core enumeration order and frontier tie
+  breaks
+- save-plan explanations SHOULD prefer the smallest materially distinct unsat
+  cores and the nearest Pareto-optimal legal plans
+- proof capsules for save-plan failures SHOULD include the `FrontierWitness`
````

## Overall judgment

The current plan is already strongest where most projects are weakest: it has a real substrate thesis, real proof doctrine, real save-planning doctrine, and a refusal to hide ambiguity or degradation. The revisions above aim to make the next layer equally explicit: color ownership, remote continuity, out-of-core scaling, text/render correspondence, surgical patching, decoder equivalence, phenotype-aware quirk handling, coverage mathematics, anchor fragility, and solver-grade preservation planning. That makes the system not merely broader, but more *closed-loop complete*.   

If I were updating the planning arithmetic, I would treat these as a new additive uplift family rather than burying them inside existing counts, because they materially expand the named surface area in exactly the way the README’s anti-reduction doctrine requires. 

For reference, I grounded this review in the uploaded [README.md](sandbox:/mnt/data/README.md) and [SPEC.md](sandbox:/mnt/data/SPEC.md).
