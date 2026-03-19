I read both the README and the spec. The core is already unusually strong: the README anchors Monkeybee on a single persistent substrate and a proof-first closed loop, while the spec turns that into concrete contracts for operation profiles, exact invalidation, write planning, receipts, determinism classes, transport integrity, failure capsules, and scope-registry-driven release gates.          

My strongest recommendation is to keep expanding exactly where the current plan is already making extremely ambitious promises: provenance truthfulness, invalidation explainability, serializer replayability, cross-document import evidence, extraction truthfulness, transport trust, and benchmark reproducibility. Those are the places where an “alien artifact” stops being a very large spec and becomes a genuinely hard-to-copy engine.  

## 1) Add a first-class provenance trust lattice across every surfaced artifact

The spec already has provenance at the syntax layer, hypothesis tracking for ambiguous recovery, and strong preservation claims for writes; what it does **not** yet make explicit is a single typed trust model that follows text spans, semantic anchors, render regions, imported resources, and diagnostics all the way out to the public API. Right now, provenance exists, but it is still somewhat subsystem-local. A unified trust lattice would make extraction truthfulness, agent-safe edits, and forensic interpretation substantially stronger.    

```diff
@@ Part 0 — Non-negotiables
+12. Every caller-visible fact emitted by Monkeybee — text span, semantic anchor,
+    diff claim, render-region diagnosis, import alias, validation finding, or
+    save-impact explanation — MUST carry typed provenance and trust class, not
+    merely an unstructured confidence string.

@@ Part 3 — after "Decision trace and causal explainability contract"
+### Provenance trust lattice
+
+pub enum ProvenanceTrustClass {
+    SourceExact,
+    SourceRepaired,
+    SourceSynthesized,
+    ProviderSupplied,
+    OracleConsensusDerived,
+    HeuristicInferred,
+}
+
+pub struct ProvenanceAtom {
+    pub trust_class: ProvenanceTrustClass,
+    pub source_span: Option<ByteSpanRef>,
+    pub source_object: Option<ObjRef>,
+    pub hypothesis_set_id: Option<HypothesisSetId>,
+    pub evidence_refs: Vec<EvidenceRef>,
+    pub confidence: Option<f64>,
+}
+
+pub struct SurfaceProvenanceSummary {
+    pub exact_count: u64,
+    pub repaired_count: u64,
+    pub synthesized_count: u64,
+    pub inferred_count: u64,
+    pub provider_supplied_count: u64,
+}
+
+Rules:
+- `ExtractResult`, `RenderReport`, `DiffReport`, `CapabilityReport`, `WriteReceipt`,
+  and agent-facing query/edit surfaces MUST expose provenance summaries.
+- per-span / per-anchor provenance MUST be retrievable on demand
+- semantic claims derived from ambiguous recovery MUST retain the originating
+  hypothesis-set reference
+- preserve-mode workflows may only claim `SourceExact` or `SourceRepaired`,
+  never silently upgrade to stronger trust classes
```

Why this improves the project: it makes “truthfulness” auditable at the same granularity as the rest of the engine. It also gives you a principled way to say “this text is exact,” “this text came from a repaired CMap,” or “this anchor is heuristic.” That is a major upgrade for downstream automation and forensics.

## 2) Add invalidation witnesses so “exact invalidation” becomes provable, not just architectural doctrine

The spec’s incremental query engine is excellent, but it currently stops at dependency-aware reuse semantics. I would add an explicit `InvalidationWitness` artifact that records *why* a cached result was reused, partially reused, or recomputed. That would let the proof harness validate one of the engine’s most important claims directly.  

```diff
@@ Part 3 — after "Incremental query engine doctrine"
+### Invalidation witness contract
+
+pub enum ReuseVerdict {
+    FullReuse,
+    PartialReuse,
+    Recompute,
+}
+
+pub struct DependencyDelta {
+    pub changed_node: NodeDigest,
+    pub change_kind: String,
+    pub affected_queries: Vec<QueryKey>,
+}
+
+pub struct InvalidationWitness {
+    pub witness_id: String,
+    pub query_key: QueryKey,
+    pub snapshot_before: SnapshotId,
+    pub snapshot_after: SnapshotId,
+    pub reuse_verdict: ReuseVerdict,
+    pub dependency_deltas: Vec<DependencyDelta>,
+    pub trace_digest: [u8; 32],
+}
+
+Rules:
+- `PagePlan`, rendered tiles, extraction surfaces, semantic graphs, write plans,
+  and diff reports MUST be able to emit an `InvalidationWitness`
+- proof runs MUST include exact-invalidation expectation fixtures:
+  "changed annotation appearance invalidates page N tiles X/Y but not unrelated pages"
+- reuse without an admissible witness is a correctness bug in proof-canonical mode
```

Why this improves the project: the current architecture says “reuse is exact.” This addition lets you *show the chain of causality*. That is especially valuable for page-local edits, progressive rendering, and performance claims.

## 3) Add a serializer replay journal and byte-address map

Your `WriteReceipt` is already strong, but it still tells the story mostly at the range and object-classification level. For a project that cares this much about preserve-mode, signatures, determinism, and durable evidence, I would make serialization decisions themselves first-class. That means recording object emission order, compression choices, xref style, stream-packing decisions, and final byte offsets in a replayable journal.  

```diff
@@ Part 5 — after `WriteReceipt`
+pub struct SerializedByteAddressMap {
+    pub object_offsets: Vec<(ObjRef, u64)>,
+    pub xref_offset: u64,
+    pub trailer_offset: u64,
+    pub eof_offset: u64,
+}
+
+pub enum SerializationDecisionKind {
+    PreserveVerbatim,
+    RewriteCanonical,
+    AppendIncremental,
+    CompressStream,
+    LeaveUncompressed,
+    PackIntoObjectStream,
+    EmitPlainIndirectObject,
+}
+
+pub struct SerializationDecision {
+    pub object_ref: Option<ObjRef>,
+    pub decision: SerializationDecisionKind,
+    pub reason: String,
+}
+
+pub struct EmissionJournal {
+    pub object_order: Vec<ObjRef>,
+    pub decisions: Vec<SerializationDecision>,
+    pub byte_map: SerializedByteAddressMap,
+}
+
+Rules:
+- `WriteReceipt` SHOULD embed or reference an `EmissionJournal`
+- deterministic writes in proof-canonical mode MUST be replayable from
+  `WritePlan + EmissionJournal + policy_digest`
+- serializer regressions MUST diff the emission journal, not only the final bytes
```

Why this improves the project: it gives you a much sharper debugging and proof surface for “why did this save invalidate a signature,” “why did output size change,” and “why did identical semantics produce different bytes.”

## 4) Add an import-closure certificate for cross-document merge/copy/split operations

The README rightly elevates cross-document import integrity to a v1 proof obligation, and the document layer already has provenance remap, collision policy, and semantic normal forms. What is still missing is a crisp artifact proving that an import operation was closure-complete and collision-safe.    

```diff
@@ Part 3 — after cross-document import / semantic normal form sections
+### Import closure certificate
+
+pub enum AliasSafetyClass {
+    CollisionFree,
+    RemappedSafely,
+    PreservedOpaque,
+    Blocked,
+}
+
+pub struct AliasResolutionRecord {
+    pub source_object: ObjRef,
+    pub target_object: ObjRef,
+    pub safety_class: AliasSafetyClass,
+    pub reason: String,
+}
+
+pub struct ImportClosureCertificate {
+    pub certificate_id: String,
+    pub source_snapshot: SnapshotId,
+    pub target_snapshot: SnapshotId,
+    pub imported_roots: Vec<ObjRef>,
+    pub closure_size: u64,
+    pub alias_resolutions: Vec<AliasResolutionRecord>,
+    pub semantic_normal_form_digest: [u8; 32],
+    pub blocked_imports: Vec<BlockedMerge>,
+}
+
+Rules:
+- copy-page / merge / split / resource-import workflows MUST be able to emit
+  `ImportClosureCertificate`
+- proof fixtures MUST verify closure completeness, alias stability, and
+  no-silent-collision guarantees
```

Why this improves the project: it makes import safety concrete in the same way `WriteReceipt` makes save safety concrete.

## 5) Add deterministic edit rebase algebra and receipts

You already have snapshots, edit transactions, agent-safe proposals, and future CRDT lanes. The missing middle layer is a deterministic, typed rebase model for ordinary concurrent or sequential edit composition *before* you need full collaboration machinery. That will matter for API ergonomics, testing, and future automation.   

```diff
@@ Part 3 — mutation model / EditTransaction section
+### Edit rebase algebra
+
+pub enum EditIntentKind {
+    MetadataSet,
+    AnnotationAdd,
+    AnnotationModify,
+    FieldValueSet,
+    PageInsert,
+    PageRemove,
+    PageReorder,
+    ResourceReplace,
+    RedactionApply,
+}
+
+pub enum RebaseConflictKind {
+    AnchorMoved,
+    OwnershipEscalation,
+    PreserveConstraintViolation,
+    DeletedTarget,
+    AliasCollision,
+    AppearanceStale,
+}
+
+pub struct RebaseReceipt {
+    pub base_snapshot: SnapshotId,
+    pub input_delta: DeltaDigest,
+    pub rebased_delta: DeltaDigest,
+    pub conflicts: Vec<RebaseConflictKind>,
+    pub applied_rewrites: Vec<String>,
+}
+
+Rules:
+- `EditTransaction::commit()` MAY emit `RebaseReceipt`
+- deterministic mode MUST fix rebase order and tie-break behavior
+- agent-facing edit APIs MUST consume this same algebra instead of inventing a
+  second mutation model later
```

Why this improves the project: it makes ordinary edit composition principled now, and it gives the later collaboration lane a much stronger base.

## 6) Add a cross-subsystem numeric robustness profile

The spec already names exact analytic coverage, robust predicates, adaptive subdivision, tetrahedral ICC interpolation, and other mathematically sharp techniques. I would unify those under a single `NumericRobustnessProfile` so the engine can explicitly say when it is using fast floating-point paths, interval guards, adaptive exact predicates, or full exact fallback.   

```diff
@@ Part 7 — Performance and safety doctrine
+### Numeric robustness profile
+
+pub enum NumericKernelClass {
+    FastFloat,
+    GuardedFloat,
+    AdaptiveExactPredicate,
+    ExactFallback,
+}
+
+pub struct NumericRobustnessProfile {
+    pub path_geometry: NumericKernelClass,
+    pub clipping: NumericKernelClass,
+    pub mesh_subdivision: NumericKernelClass,
+    pub hit_testing: NumericKernelClass,
+    pub color_interpolation: NumericKernelClass,
+    pub blend_boundary_logic: NumericKernelClass,
+}
+
+Rules:
+- `RenderReport`, `ExtractReport`, and benchmark witnesses SHOULD surface the
+  active `NumericRobustnessProfile`
+- proof-canonical mode MUST pin the profile
+- any downgrade from a stronger to weaker numeric kernel is a plan-selection event
```

Why this improves the project: it turns “we use great math here” into a reproducible, support-class-qualified contract. That is exactly the kind of disciplined ambition this project wants.

## 7) Add a render-chunk graph above tiles and below pages

Today the rendering model has tile/band scheduling, progressive placeholders, and a page-level shared interpreter. I would add a `RenderChunkGraph` that makes transparency groups, image XObjects, glyph runs, shadings, and annotation appearances first-class reusable chunks. That gives you a better unit for invalidation, progressive refinement, CPU/GPU parity, and oracle disagreement localization.   

```diff
@@ Part 3 — render architecture / progressive rendering
+### Render chunk graph
+
+pub enum RenderChunkKind {
+    GlyphRun,
+    ImageXObject,
+    FormXObject,
+    TransparencyGroup,
+    ShadingSpan,
+    AnnotationAppearance,
+}
+
+pub struct RenderChunk {
+    pub chunk_id: [u8; 32],
+    pub kind: RenderChunkKind,
+    pub bbox: Rect,
+    pub dependency_digests: Vec<NodeDigest>,
+}
+
+pub struct RenderChunkEdge {
+    pub parent: [u8; 32],
+    pub child: [u8; 32],
+    pub blend_mode: Option<BlendMode>,
+    pub ocg_state: Option<OcgStateRef>,
+}
+
+pub struct RenderChunkGraph {
+    pub chunks: Vec<RenderChunk>,
+    pub edges: Vec<RenderChunkEdge>,
+}
+
+Rules:
+- tile scheduling MAY consume `RenderChunkGraph` rather than raw page-wide streams
+- placeholder refinement and chunk reuse MUST be expressible at chunk granularity
+- oracle disagreement tooling SHOULD localize disagreements to chunk ids when possible
```

Why this improves the project: the page is too coarse, and the tile is too raster-specific. A chunk graph is the right middle layer for reuse and explanation.

## 8) Add extraction truth surfaces and anchor stability witnesses

The spec already promises useful extraction, semantic anchors, and agent-safe edits. I would strengthen that by distinguishing exact text, recovered text, layout-inferred groupings, and reading-order hypotheses explicitly. Otherwise “anchor stability” risks sounding stronger than the supporting extraction evidence.   

```diff
@@ Part 1 — Workflow 12 / extraction architecture
+### Extraction truth surface contract
+
+pub enum TextTruthClass {
+    UnicodeExact,
+    UnicodeRecovered,
+    GlyphOnly,
+    ReadingOrderInferred,
+    TableStructureInferred,
+    Unmappable,
+}
+
+pub struct TextTruthSpan {
+    pub page_index: u32,
+    pub bbox: Rect,
+    pub truth_class: TextTruthClass,
+    pub provenance: ProvenanceAtom,
+}
+
+pub struct AnchorStabilityWitness {
+    pub anchor_id: SemanticAnchorId,
+    pub before_snapshot: SnapshotId,
+    pub after_snapshot: SnapshotId,
+    pub preserved_exactly: bool,
+    pub alias_target: Option<SemanticAnchorId>,
+    pub failure_reason: Option<String>,
+}
+
+Rules:
+- `ExtractResult` SHOULD surface a page-level truth summary
+- anchor-stability proof lanes MUST distinguish exact preservation from alias-map
+  continuity from heuristic re-identification
```

Why this improves the project: it gives the engine a truthful vocabulary for extraction quality, which is essential if later automation is going to rely on it.

## 9) Add oracle-consensus records and a blind-spot ledger

The proof harness already uses multiple reference renderers, typed disagreements, witnesses, failure capsules, and strategy tournaments. I would add two more artifacts: one that records *how consensus was reached* on a disputed case, and another that records *where the corpus is still blind*. That turns the proof harness from a regression system into a map of epistemic coverage.  

```diff
@@ Part 6 — proof doctrine
+### Oracle consensus record
+
+pub enum OracleVerdictClass {
+    Unanimous,
+    MajorityConsensus,
+    SplitDecision,
+    NoConsensus,
+}
+
+pub struct OracleConsensusRecord {
+    pub fixture_id: String,
+    pub page_index: Option<u32>,
+    pub verdict_class: OracleVerdictClass,
+    pub participating_oracles: Vec<String>,
+    pub winning_interpretation: Option<String>,
+    pub disagreement_axes: Vec<String>,
+}
+
+### Blind-spot ledger
+
+pub struct BlindSpotLedgerEntry {
+    pub feature_id: String,
+    pub proof_class: String,
+    pub exercised_fixture_count: u32,
+    pub producer_diversity_count: u32,
+    pub support_classes_seen: Vec<String>,
+    pub gap_reason: String,
+}
+
+Rules:
+- canonical proof runs SHOULD emit `oracle-consensus/` and `blind-spots/`
+- release-facing capability claims SHOULD be suppressible when blind-spot
+  coverage is below threshold, even if isolated fixtures pass
```

Why this improves the project: it prevents false confidence from a thin fixture set and makes disagreements legible in a much more scientific way.

## 10) Strengthen remote mode with digest ladders, not just validator identity

The transport-integrity section is already much better than most systems because it refuses to define correctness as “bytes arrived.” I would push that one step further by adding optional range digests and continuity receipts, so hostile or sloppy remote storage cannot quietly feed mixed objects under a stable URL. `ETag` and `Last-Modified` are good but not always trustworthy enough. 

```diff
@@ Part 3 — Remote transport integrity and sparse-availability contract
+pub struct RangeDigestRecord {
+    pub range: (u64, u64),
+    pub digest: [u8; 32],
+    pub fetch_epoch: FetchEpoch,
+}
+
+pub struct SparseDigestMap {
+    pub verified: Vec<RangeDigestRecord>,
+}
+
+pub struct TransportContinuityReceipt {
+    pub transport_identity: TransportIdentity,
+    pub epochs_seen: Vec<FetchEpoch>,
+    pub digest_map: SparseDigestMap,
+    pub continuity_failures: Vec<RangeConsistencyError>,
+}
+
+Rules:
+- when upstream transport provides cryptographic digests, Monkeybee SHOULD bind
+  them into `TransportContinuityReceipt`
+- proof-canonical remote fixtures SHOULD prefer digest-backed transport identities
+- write receipts for range-backed sessions MAY reference the continuity receipt
+  when transport trust materially affects correctness claims
```

Why this improves the project: it makes remote mode feel like part of the same high-assurance engine, not a weaker side channel.

## 11) Expand benchmark witnesses with topology, allocator, and SIMD evidence

You already require witness-backed performance claims and canonical manifests. For a project pushing this hard on optimization, I would make hardware topology and low-level runtime choices explicit: SIMD class, allocator, NUMA policy, and storage class. Otherwise a lot of “world-class performance” evidence becomes hard to compare or replay.  

```diff
@@ Part 6 — `BenchmarkWitness`
 pub struct BenchmarkWitness {
     pub witness_id: String,
     pub reproducibility_manifest_id: String,
     pub benchmark_profile_id: String,
     pub support_class: String,
     pub render_determinism_class: RenderDeterminismClass,
     pub fixture_set_digest: [u8; 32],
     pub warm_cache: bool,
+    pub allocator: String,
+    pub simd_class: String,
+    pub numa_policy: String,
+    pub storage_class: String,
     pub metrics: Vec<MetricObservation>,
     pub threshold_verdicts: Vec<ThresholdVerdict>,
 }
```

Why this improves the project: it turns performance evidence into something that can actually survive scrutiny from systems people.

## 12) Extend the scope registry so proof surfaces are first-class named capabilities too

Your scope registry is already excellent. I would widen it one notch further so proof surfaces themselves are tracked as capabilities: invalidation witnesses, emission journals, oracle consensus, blind-spot ledgers, transport continuity receipts, anchor-stability witnesses, and import-closure certificates. That keeps the new evidence surfaces from becoming “helpful extras” rather than named contracts. 

```diff
@@ Part 8 — Scope registry doctrine
 Each registry entry includes:
 - `feature_id`
 - `scope_class`
 - `support_classes`
 - `owning_crate`
 - `proof_class`
 - `schema_surfaces`
 - `bead_ids`
 - `notes`
+ - `witness_surfaces`
+ - `evidence_gating`
+ - `adoption_leverage_rank`
+
+Rules:
+- any artifact that can block or substantiate a release-facing claim MUST appear
+  as a named witness surface in the registry
+- generated capability docs MUST distinguish feature support from evidence support
+  (for example: "feature implemented" vs "feature implemented + witness-backed")
```

Why this improves the project: it keeps the engine honest in exactly the way the README says it wants to be honest.

## Highest-priority subset

If I were choosing the **best immediate additive upgrades**, I would do these first:

1. **Provenance trust lattice**
2. **Invalidation witnesses**
3. **Serializer replay / emission journal**
4. **Import-closure certificate**
5. **Extraction truth surfaces**

That set most directly strengthens the project’s distinctive claims: one substrate, truthful ambiguity handling, exact invalidation, explainable save planning, and stable automation surfaces.     

One overarching judgment: the plan already has rare breadth. The next step is not “more categories” so much as “more witnessable internal surfaces” at the exact places where the current spec makes its boldest promises. That is how it becomes harder, stranger, and more defensible without reducing a single atom of scope.  
