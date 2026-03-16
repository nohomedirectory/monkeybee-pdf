I reviewed the README, the implementation master, and the SPEC together. The plan is already unusually strong: it is centered on the closed loop, preserves a strict byte/syntax/semantic/content layering, uses immutable snapshots and explicit write planning, routes risky behavior through security profiles, and insists on proof artifacts rather than roadmap rhetoric. My revisions below are therefore not simplifications; they are additive expansions aimed at the seams that are already implied by the current architecture but not yet fully specified.   

The biggest pattern I see is this: Monkeybee already has the right spine, but several of its most important promises are still represented as adjacent ideas rather than first-class contracts. In particular, `OpenProbe` is promising more than it formally guarantees; `TraceEventStream` is named but not yet specified; signature-safe editing is strong on invariants but weak on post-write attestation; remote/range-backed operation is functional but not yet transport-trust-complete; and catalog-level document semantics are scattered across render/extract/write instead of having a dedicated semantic home. Tightening those surfaces would make the project more robust, more explainable, and much easier to prove and evolve.    

## 1) Make `OpenProbe` an admission-and-risk contract, not just a preflight

Right now the spec already gives `OpenProbe` real weight: it can inspect tail regions, signatures, encryption, risky decoders, linearization hints, and it returns a preliminary `CapabilityReport`, complexity class, recommended profile, and access-plan material. That is excellent, but it still leaves one crucial gap: the engine has no formal, schema-level answer to “should this operation proceed, under what budget/profile, and with what expected pain surface?” Turning probe output into a first-class admission contract makes viewer/editor behavior more predictable, lets CLI and library callers surface consistent warnings, and gives proof runs a stable “expected operating envelope” artifact. It also meshes directly with the existing emphasis on `ExecutionContext`, profile selection, and explainability.  

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 ### Open probe contract
+
+### Complexity fingerprint and admission contract
+
+`OpenProbe` MUST emit a deterministic `ComplexityFingerprint` and an
+`AdmissionDecision` in addition to the preliminary `CapabilityReport`.
+
+The goal is not merely "can I open this file?" but "what class of file is this,
+what runtime envelope does it deserve, and what degradation/risk surface should
+the caller expect before committing to full open or expensive downstream work?"
+
+pub enum ComplexityClass {
+    Tiny,
+    Small,
+    Medium,
+    Large,
+    Huge,
+    Pathological,
+}
+
+pub struct ComplexityFingerprint {
+    pub object_count_estimate: Option<u64>,
+    pub page_count_estimate: Option<u32>,
+    pub incremental_depth_estimate: u32,
+    pub stream_density_score: u32,
+    pub font_complexity_score: u32,
+    pub transparency_complexity_score: u32,
+    pub structure_complexity_score: u32,
+    pub signed_coverage_ratio: Option<f32>,
+    pub remote_first_paint_bytes_estimate: Option<u64>,
+    pub risky_decoder_set: Vec<DecoderType>,
+    pub active_content_score: u32,
+}
+
+pub struct BudgetRecommendation {
+    pub parse_budget: ResourceBudgets,
+    pub render_budget: ResourceBudgets,
+    pub extraction_budget: ResourceBudgets,
+    pub preferred_security_profile: SecurityProfile,
+}
+
+pub enum AdmissionDecision {
+    Admit {
+        class: ComplexityClass,
+        recommended_profile: OperationProfile,
+        budget: BudgetRecommendation,
+    },
+    AdmitDegraded {
+        class: ComplexityClass,
+        recommended_profile: OperationProfile,
+        budget: BudgetRecommendation,
+        expected_degradations: Vec<FeatureCode>,
+    },
+    Reject {
+        reason: AdmissionReason,
+        safe_probe_artifacts: Vec<ProbeArtifactRef>,
+    },
+}
+
+pub enum AdmissionReason {
+    BudgetHopeless,
+    ActiveContentPolicyBlocked,
+    PasswordRequired,
+    AmbiguousRecoveryBlocked,
+    TransportIntegrityFailed,
+    UnsupportedCriticalFeature,
+}
```

## 2) Specify `TraceEventStream` as a causal surface, not just a schema name

The spec already declares `TraceEventStream` to be a schema-versioned external interface, and the implementation already has diagnostics, budgets, cache summaries, and operation reports. But there is still no concrete causal model that says which decisions must be traced, how they relate, and how a user or proof harness can reconstruct “why” the engine chose a repair, a fallback font, a degraded decoder path, or a full rewrite. Since the README explicitly promises operational explainability, this is one of the highest-value additive expansions you can make. It turns diagnostics from a bag of warnings into an auditable causal narrative.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 ### Diagnostic streaming model
+
+### Decision trace and causal explainability contract
+
+Diagnostics answer "what happened." `TraceEventStream` answers "why this path
+was chosen instead of the alternatives."
+
+Every repair choice, fallback chain branch, provider resolution, cache
+miss-to-recompute transition, security-profile denial/isolation decision,
+ownership escalation, and save-plan escalation MUST emit a causal trace event.
+
+pub struct OperationSpanId(pub u128);
+
+pub struct OperationSpan {
+    pub span_id: OperationSpanId,
+    pub parent: Option<OperationSpanId>,
+    pub operation_kind: OperationKind,
+    pub snapshot_id: Option<SnapshotId>,
+    pub page_index: Option<u32>,
+}
+
+pub enum DecisionKind {
+    RepairChoice,
+    FallbackChoice,
+    ProviderResolution,
+    SecurityGate,
+    CacheReuse,
+    CacheMiss,
+    OwnershipEscalation,
+    WritePlanEscalation,
+    RemoteFetchPriority,
+}
+
+pub struct DecisionRecord {
+    pub span_id: OperationSpanId,
+    pub decision_kind: DecisionKind,
+    pub subject: String,
+    pub chosen: String,
+    pub alternatives: Vec<String>,
+    pub confidence: Option<f64>,
+    pub reason: String,
+    pub causal_inputs: Vec<CausalRef>,
+}
+
+pub struct TraceEvent {
+    pub ts_monotonic_ns: u64,
+    pub span_id: OperationSpanId,
+    pub event: TraceEventKind,
+}
+
+pub enum TraceEventKind {
+    SpanStart(OperationSpan),
+    SpanEnd { outcome: TraceOutcome },
+    Decision(DecisionRecord),
+    Metric { key: String, value: f64 },
+    DiagnosticRef { diagnostic_code: String },
+}
+
+`TraceEventStream` is a required proof artifact for:
+- ambiguous recovery
+- signature-impacting save plans
+- remote progressive render sessions
+- any proof-harness failure capsule
```

## 3) Add a dedicated `monkeybee-catalog` subsystem for document-level semantics

Today catalog-level semantics are present, but scattered. Optional content is treated mostly as a render concern; embedded files and outlines appear mainly in extraction; writer catalog requirements are minimal; page labels, named destinations, number trees, and viewer preferences do not have a dedicated home in the crate graph. That creates a long-term architecture risk: these structures will become “everybody’s little side model,” which is exactly how large document engines lose coherence. A dedicated catalog subsystem would keep navigation, naming, optional-content configuration, viewer/document preferences, and attachment topology explicit and shared across inspect/extract/edit/write/diff/validate.    

```diff
diff --git a/README.md b/README.md
@@
 | `monkeybee-document` | Semantic document graph built from syntax snapshots: page tree, inherited state, resource resolution, ownership classes, dependency graph contract, bounded cache management |
+| `monkeybee-catalog` | Catalog semantic subsystem: outlines, named destinations, page labels, name/number trees, viewer preferences, optional-content configurations, and embedded-file inventory |
 | `monkeybee-content` | Content-stream IR + event interpreter shared by render/extract/inspect/edit; consumer sink adapters (RenderSink, ExtractSink, InspectSink, EditSink) |
diff --git a/implementation_master.md b/implementation_master.md
@@
 │   ├── monkeybee-document/       # semantic document graph built from syntax snapshots
+│   ├── monkeybee-catalog/        # catalog semantics: outlines, destinations, name trees, page labels, viewer prefs, OCG configs, attachments
 │   ├── monkeybee-content/        # content-stream IR and event interpreter
@@
-monkeybee-content       (depends on: core, document)
+monkeybee-catalog       (depends on: core, syntax, document)
+monkeybee-content       (depends on: core, document)
@@
-monkeybee-write         (depends on: core, bytes, document, codec)
+monkeybee-write         (depends on: core, bytes, document, catalog, codec)
@@
-monkeybee-validate      (depends on: core, document)
+monkeybee-validate      (depends on: core, document, catalog)
diff --git a/SPEC.md b/SPEC.md
@@
 #### `monkeybee-document`
@@
+#### `monkeybee-catalog`
+
+Document-catalog semantics that are broader than any one page and more
+structured than raw COS preservation:
+- outline / bookmark trees
+- named destinations and destination arrays
+- page labels
+- name trees and number trees
+- viewer preferences, page mode, and page layout
+- optional content configurations (`/OCProperties`, default configs, print/export states)
+- embedded-file inventory and AF relationships
+
+The catalog subsystem is the authoritative semantic model for these structures.
+Render/extract/write/diff/validate consume it; they do not each grow ad hoc
+partial models.
+
+Catalog round-trip invariants:
+- preserve sibling/child order in outline trees unless explicitly edited
+- preserve page-label numbering semantics across page insertion/deletion
+- preserve named-destination identity across full rewrite and incremental append
+- preserve OCG/OCMD semantics unless the edit explicitly changes visibility policy
```

## 4) Add a post-write signature attestation artifact: `WriteReceipt`

The current plan already has strong pre-write logic (`WritePlan`, `SignatureImpact`, byte-range invariants) and even names a preserve-mode proof obligation. What it lacks is a canonical post-write artifact that says, in one machine-readable object, exactly what bytes were preserved, what was appended, which signatures remain valid, and why. That artifact is the missing bridge between beautiful internal invariants and user-visible trust. It would also make CLI, proof harness, and regression triage dramatically better.    

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 ### Save planning invariant
@@
 `WritePlan` is surfaced to the API/CLI and to the compatibility ledger. Signature-safe workflows
 must be explainable before bytes are emitted, not inferred after the fact.
+
+### Signature safety proof artifact contract
+
+Every successful write MUST optionally produce a `WriteReceipt`. For incremental
+append workflows, `WriteReceipt` is not a convenience artifact; it is the
+machine-readable attestation that the save respected preserve constraints.
+
+pub struct BytePreservationMap {
+    pub immutable_prefix_end: u64,
+    pub preserved_ranges: Vec<(u64, u64)>,
+    pub touched_ranges: Vec<(u64, u64)>,
+    pub signed_ranges: Vec<(u64, u64)>,
+}
+
+pub struct SignedCoverageEntry {
+    pub signature_ref: ObjRef,
+    pub covered_ranges: Vec<(u64, u64)>,
+    pub affected_objects: Vec<ObjRef>,
+    pub invalidated: bool,
+    pub invalidation_reason: Option<String>,
+}
+
+pub struct WriteReceipt {
+    pub schema_version: String,
+    pub snapshot_id: SnapshotId,
+    pub write_mode: WriteMode,
+    pub write_plan_digest: [u8; 32],
+    pub bytes_appended: u64,
+    pub preservation: BytePreservationMap,
+    pub signature_coverage: Vec<SignedCoverageEntry>,
+    pub ownership_transitions: Vec<OwnershipTransitionRecord>,
+    pub post_write_validation: Vec<ValidationFinding>,
+}
+
+`WritePlan.execute()` SHOULD return:
+`OperationSuccess<WriteResult { bytes, receipt: Option<WriteReceipt> }>`
```

## 5) Make transaction lineage, rebase, and undo explicit

The immutable-snapshot model is already one of the best parts of the design, and the spec already says change tracking enables undo/audit and that conflicting transactions must be rebased manually. That is exactly why this needs to become a first-class contract: once you have snapshots, deltas, change reasons, conflict detection, and save-impact explanation, transaction lineage is no longer optional infrastructure; it is part of the public semantics of editing. Formalizing it will make the engine much friendlier to future editor workflows, automation, and proof reproduction.  

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 ### EditTransaction validation rules
@@
 **Conflict detection:** Transactions are optimistically concurrent.
@@
 The engine does not automatically merge conflicting transactions.
+
+### Transaction lineage, rebase, and undo contract
+
+pub struct TransactionIntent {
+    pub edit_intent: EditIntent,
+    pub human_reason: String,
+    pub expected_write_mode: Option<WriteMode>,
+    pub preserve_constraints: Vec<PreserveConstraint>,
+}
+
+pub struct ConflictSet {
+    pub conflicting_objects: Vec<ObjRef>,
+    pub affected_pages: Vec<u32>,
+    pub signature_impact: SignatureImpactReport,
+    pub structure_impact: Option<StructureEditRisk>,
+}
+
+pub struct RebasePlan {
+    pub base_snapshot: SnapshotId,
+    pub target_snapshot: SnapshotId,
+    pub replayed_changes: Vec<ChangeEntry>,
+    pub rejected_changes: Vec<RejectedChange>,
+    pub new_ownership_transitions: Vec<OwnershipTransitionRecord>,
+}
+
+pub struct UndoJournalEntry {
+    pub snapshot_before: SnapshotId,
+    pub snapshot_after: SnapshotId,
+    pub inverse_change_set: Vec<ChangeEntry>,
+}
+
+Required invariants:
+- every committed transaction has a stable lineage record
+- every user-visible conflict has an object-level `ConflictSet`
+- rebasing is explicit, deterministic under deterministic mode, and auditable
+- undo is implemented as ordinary forward movement to a new snapshot, never
+  mutation of an existing snapshot
```

## 6) Add a transport-integrity contract for remote/range-backed sessions

The current remote story is good on performance semantics: linearization, prefetch plans, placeholder metadata, range caches, and validator reuse all exist. But the design still lacks a formal answer to “what if the remote object changes halfway through the session?” or “what if range responses are internally inconsistent?” Those are not edge niceties; they are core correctness issues for a document engine that wants to be trusted on hostile real-world inputs. A transport-integrity contract would harden Slice C substantially without shrinking anything else.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 ### Fetch scheduler contract
@@
 **Concurrency:** The scheduler limits concurrent HTTP range requests
@@
 than to issue a separate request).
+
+### Remote transport integrity and sparse-availability contract
+
+Remote sessions MUST bind to a `TransportIdentity` and maintain a
+`ByteAvailabilityMap`. Range-backed correctness is not defined solely by
+"did bytes arrive?" but by "did they arrive from the same logical artifact?"
+
+pub struct TransportIdentity {
+    pub source_fingerprint: [u8; 32],
+    pub etag: Option<String>,
+    pub last_modified: Option<String>,
+    pub content_length: Option<u64>,
+    pub digest_hint: Option<[u8; 32]>,
+}
+
+pub struct ByteAvailabilityMap {
+    pub epoch: FetchEpoch,
+    pub available_ranges: Vec<(u64, u64)>,
+    pub verified_ranges: Vec<(u64, u64)>,
+    pub suspect_ranges: Vec<(u64, u64)>,
+}
+
+pub struct FetchEpoch(pub u64);
+
+pub enum RangeConsistencyError {
+    ValidatorChanged,
+    ContentLengthChanged,
+    OverlappingConflict,
+    TruncatedBody,
+    Unsupported206Semantics,
+}
+
+Rules:
+- all range responses within a session MUST agree on validator identity
+- validator drift freezes the session into explicit degraded mode
+- previously verified ranges remain trusted only within the same `FetchEpoch`
+- `OpenProbe`, `CapabilityReport`, and `WriteReceipt` MUST surface transport
+  integrity failures when they influence correctness
```

## 7) Formalize resource canonicalization and dedup safety

The current plan already has the ingredients: change tracking, GC/dedup, dependency graphs, cache namespaces, object-stream packing, and explicit optimization operations. What it does not yet have is a formal model of when two resources are deduplicable, when they are only appearance-equivalent, and when reuse is unsafe because provenance, ownership, or decode parameters differ. That distinction matters a lot for correctness on ugly PDFs, especially when trying to make writes smaller and faster without accidentally collapsing semantically important differences.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 ### Cache management doctrine
@@
 Cache budgets are exposed in `ExecutionContext`
@@
 diagnostics report cache pressure events).
+
+### Resource canonicalization and deduplication contract
+
+pub struct ResourceCanonicalForm {
+    pub semantic_fingerprint: ResourceFingerprint,
+    pub byte_fingerprint: Option<[u8; 32]>,
+    pub resource_kind: ResourceKind,
+    pub decode_parameters_digest: Option<[u8; 32]>,
+    pub provider_manifest_id: Option<String>,
+}
+
+pub enum DedupSafetyClass {
+    ByteExact,
+    SemanticEquivalent,
+    AppearanceEquivalent,
+    NotDeduplicable,
+}
+
+pub struct MaterializationPlan {
+    pub reused_existing: Vec<ObjRef>,
+    pub regenerated: Vec<ObjRef>,
+    pub dedup_merged: Vec<(ObjRef, ObjRef)>,
+    pub blocked_merges: Vec<BlockedMerge>,
+}
+
+Dedup rules:
+- `ForeignPreserved` objects may not be semantically merged in preserve workflows
+- decoder choice, provider manifest, and decode params participate in canonical identity
+- appearance-equivalent merges are allowed only in explicit optimization transactions
+- `WritePlan` and `WriteReceipt` MUST record all dedup merges and blocked merges
```

## 8) Upgrade active-content handling from booleans to object-level inventory and sanitization receipts

The current `ActiveContentPolicy` is directionally right, but the reporting surface is still boolean-heavy. For real-world forensic, validation, and editor workflows, booleans are not enough. You need object-level inventory: which action dictionaries exist, where they are attached, whether they are external, whether they are preserved or stripped, and what the write path actually did. Since the spec already treats JavaScript, actions, embedded files, and rich media as first-class policy concerns, this is the natural next step.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 ### Active content policy
@@
 pub struct ActiveContentReport {
@@
     pub has_rich_media: bool,
 }
+
+pub struct ActiveContentInventory {
+    pub actions: Vec<ActionNode>,
+    pub javascript_blocks: Vec<JsBlockRef>,
+    pub launch_actions: Vec<ActionNode>,
+    pub uri_actions: Vec<ActionNode>,
+    pub remote_goto_actions: Vec<ActionNode>,
+    pub submit_form_actions: Vec<ActionNode>,
+    pub embedded_file_refs: Vec<FileSpecRef>,
+    pub rich_media_refs: Vec<RichMediaRef>,
+}
+
+pub struct SanitizationPlan {
+    pub policy: ActiveContentPolicy,
+    pub objects_to_strip: Vec<ObjRef>,
+    pub objects_to_preserve: Vec<ObjRef>,
+    pub objects_to_stub: Vec<ObjRef>,
+}
+
+pub struct SanitizationReceipt {
+    pub plan_digest: [u8; 32],
+    pub stripped: Vec<ObjRef>,
+    pub preserved: Vec<ObjRef>,
+    pub stubbed: Vec<ObjRef>,
+    pub warnings: Vec<Diagnostic>,
+}
+
+`CapabilityReport` MUST grow an `active_content_inventory_digest` field, and
+the CLI MUST support `inspect --active-content` and `sanitize --receipt-json`.
```

## 9) Add an evidence graph for extraction and reading-order claims

The extraction design is already strong: three surfaces, confidence on logical text, tagged-text preference when structure exists, and even an experimental probabilistic annex for better reading order and table inference. The missing piece is evidence. Right now a downstream caller can get confidence, but not a reusable explanation of *why* the engine grouped blocks, chose a reading order, inferred columns, or treated a region as a table. Adding an evidence graph would make extraction much more useful for debugging, downstream ranking, and proof without pushing Monkeybee into OCR or broad “document understanding.”   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 #### `monkeybee-extract`
@@
 Key responsibilities:
 - multi-surface text extraction
@@
 - diagnostic report generation
+
+### Extraction evidence contract
+
+`LogicalText` and `TaggedText` SHOULD optionally carry an evidence graph that
+lets downstream callers inspect why reading-order and table hypotheses were made.
+
+pub struct ConfidenceBreakdown {
+    pub geometric_score: f32,
+    pub tagged_score: f32,
+    pub font_decode_score: f32,
+    pub column_detection_score: f32,
+    pub table_detection_score: f32,
+}
+
+pub struct SpanEvidence {
+    pub span_id: SpanId,
+    pub source_ops: Vec<ContentOpRef>,
+    pub source_mcid: Option<u32>,
+    pub confidence: ConfidenceBreakdown,
+}
+
+pub struct TableHypothesis {
+    pub bbox: Rectangle,
+    pub rows: u32,
+    pub cols: u32,
+    pub confidence: f32,
+    pub evidence_spans: Vec<SpanId>,
+}
+
+pub struct ReadingOrderEvidence {
+    pub spans: Vec<SpanEvidence>,
+    pub edges: Vec<ReadingOrderEdge>,
+    pub table_hypotheses: Vec<TableHypothesis>,
+}
+
+pub struct ReadingOrderEdge {
+    pub before: SpanId,
+    pub after: SpanId,
+    pub reason: String,
+    pub weight: f32,
+}
```

## 10) Make every serious proof failure emit a self-contained `FailureCapsule`

The proof harness is already ambitious: ledgers, regressions, minimized crashers, oracle manifests, MS-SSIM diffs, expectation manifests, and triage state are all present. The additive move that would make this feel truly alien-artifact-grade is to package those surfaces into a single reproducible artifact whenever something materially fails or disagrees. Add decoder attestation to that capsule and you also get much better observability around native bridges and risky-decoder paths. This would hugely improve debugging, community triage, and public credibility.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 ### CI integration contract
@@
 **Artifacts produced per CI run:**
@@
 5. `timing.json` — per-test-class timing data for performance regression detection
+
+### Failure capsule doctrine
+
+Every proof-harness regression, oracle disagreement above threshold, ambiguous
+repair drift, or native-decoder crash MUST emit a `FailureCapsule`.
+
+pub struct DecoderAttestation {
+    pub decoder: DecoderType,
+    pub backend: String,
+    pub isolated: bool,
+    pub version: String,
+    pub verdict: String,
+    pub crash_fingerprint: Option<String>,
+}
+
+pub struct FailureCapsule {
+    pub input_sha256: String,
+    pub minimized_fixture: Option<String>,
+    pub oracle_manifest: OracleManifest,
+    pub compatibility_ledger: CompatibilityLedger,
+    pub trace_stream_ref: Option<String>,
+    pub write_receipt_ref: Option<String>,
+    pub decoder_attestations: Vec<DecoderAttestation>,
+    pub repro_command: String,
+    pub failure_kind: String,
+}
+
+pub struct RepairStabilityRecord {
+    pub fixture_id: String,
+    pub expected_candidate: RecoveryCandidateId,
+    pub actual_candidate: RecoveryCandidateId,
+    pub semantic_digest_changed: bool,
+}
+
+`monkeybee-proof` MUST emit `capsules/` alongside `ledger/`, `diffs/`, and
+`regressions.json`.
```

## What I would prioritize first

If I were sequencing this without slowing v1, I would do these in this order:

1. complexity/admission, 2) decision traces, 4) write receipts, and 10) failure capsules first, because they amplify every existing subsystem without forcing broad feature work; 3) catalog semantics and 5) transaction lineage next, because they prevent long-term architectural drift; 6) remote transport integrity when Slice C is active; and 7-9 as force multipliers for optimization, sanitization, and extraction credibility. That ordering stays faithful to the existing delivery spine instead of fighting it. 

Overall: the current plan already has the bones of a serious engine. The revisions above would make it feel less like “a very good spec” and more like “a system with sealed seams”: admission before work, causality after decisions, receipts after writes, stable lineage after edits, transport trust in remote mode, catalog semantics with a home, and failure artifacts that make proof portable. That is exactly the kind of additive growth your anti-reduction doctrine is asking for.  
