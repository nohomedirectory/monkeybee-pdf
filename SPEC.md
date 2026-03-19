# PLAN TO CREATE MONKEYBEE PDF

## Part 0 — Constitutional basis

### Imported North Star thesis

Monkeybee PDF is an open-source, memory-safe, high-performance Rust PDF engine for ugly real-world PDFs. Its purpose is to become a genuinely formidable bidirectional document engine: one that can ingest hostile and malformed files, accurately render them, surface useful structure from them, mutate them in disciplined ways, generate and emit valid documents, and survive repeated round trips without collapsing into corruption, hand-waving, or roadmap theater.

The central thesis is the closed loop:

```
open → understand → render → inspect/extract → annotate/edit → save/generate → reopen → validate
```

This loop is not a feature list. It is the proof that the engine owns enough of document reality to operate bidirectionally on hard, ugly, real-world documents. Every subsystem, every crate, every design decision must be measured against this loop.


### Invariant document substrate thesis

The closed loop is the user-visible proof surface. The deeper architectural thesis is stronger:
every document state in Monkeybee must be representable as an invariant-preserving projection of a
single persistent, versioned document substrate.

At minimum, the substrate must unify:
- source bytes and preserved byte spans
- parsed COS syntax with provenance
- semantic object graphs and ownership classes
- content interpretation and graphics-state transitions
- extraction/layout/semantic-anchor surfaces
- edit deltas, undo/rebase history, and write plans
- proof artifacts, receipts, and compatibility ledgers

The key architectural question for every subsystem is therefore not only "does this feature work?"
but also "what stable substrate object, digest, dependency edge, and preservation claim does it
correspond to?" If that mapping is hand-wavy, the feature is not yet architecturally real.

This substrate thesis is the most consequential refinement to the current plan because it turns
several promises that are already correct in spirit but under-specified in mechanism — structural
sharing, precise invalidation, cheap snapshots, explainable diffs, temporal replay, and
signature-safe save planning — into one coherent computational model.

### Non-negotiables

1. The closed loop must be real, not aspirational. Load-modify-save-reload-validate must work on representative ugly PDFs.
2. Memory safety is constitutional identity, not a convenience. Unsafe is minimal, explicit, justified, and aggressively tested.
3. Ugly real-world PDFs are the target, not a clean elegant subset. The long tail of malformed, quirky, hostile files is where the engine proves itself.
4. Annotation, editing, and extraction are part of the engine thesis, not optional stretch goals.
5. Proof is automated, pathological-corpus-backed, round-trip-grounded, and externally legible. No feature ships on rhetoric alone.
6. Architecture is natively owned, not donor-inherited. External implementations are behavioral references, not architectural authorities.
7. The compatibility doctrine (Tier 1/2/3) is explicit and unapologetic. Silent evasion is unacceptable.

8. Snapshots, undo, diff, rebase, and cache reuse must be grounded in a content-addressed persistent substrate rather than ad hoc cloning, mutable global state, or cache-local tricks.
9. Save planning, signature safety, and byte-preservation claims must be derivable from explicit preservation rules and surfaced as machine-readable receipts before bytes are emitted.
10. Ambiguous recovery must remain visible as a bounded hypothesis set with causal evidence. Monkeybee must never silently pick a materially different interpretation without explanation.
11. External-facing intelligence surfaces (queries, semantic anchors, agent-facing edit APIs, diff explanations) must be deterministic, typed, and auditable rather than free-form magic.

### Anti-goals

- Monkeybee is not trying to become a polished desktop PDF suite in v1.
- Monkeybee is not a giant layout-authoring DSL. It does, however, need correct shaping, bidi handling, and basic line breaking for generated text.
- Monkeybee is not trying to achieve perfect semantic recovery from every hostile file ever made.
- Monkeybee is not trying to finish the entire PDF category in one stroke.
- Monkeybee must not become incoherent because agents can generate enormous quantities of code. Deeper coherence, not decorative sprawl.
- Monkeybee is not adding OCR, document understanding, accessibility remediation, PDF/UA-2 generation/validation, or semantic format conversion to v1. Those are post-v1 lanes.

- Monkeybee is not making v1 depend on theorem provers, zero-knowledge proofs, CRDT collaboration, reactive execution, or AI agents. Those may be layered later, but the baseline must stand without them.
- Monkeybee is not exposing unconstrained LLM-edit surfaces that can mutate PDFs without stable anchors, typed proposals, policy checks, or receipts.
- Monkeybee is not using "future-proofing" as an excuse for vague substrate hand-waving. The core substrate decisions must be explicit before subsystem fan-out.

### Alien artifact doctrine

Monkeybee aspires to feel like an alien artifact in the only sense that matters: a disturbing combination of breadth, depth, coherence, and evidence. Not arbitrary exotic cleverness. Not decorative math. A system where the renderer, document model, extraction layer, mutation substrate, generation path, compatibility handling, and proof machinery all feel like manifestations of one underlying idea rather than a pile of features.

Monkeybee keeps the alien-artifact ambition, but v1 is gated on a correct baseline engine, not on landing every advanced algorithm before first release.

**Anti-reduction doctrine:** This specification is the soul of the project and MUST only grow in detail, precision, and comprehensiveness. Suggestions to "consolidate," "simplify," "streamline," "merge sections," "remove redundancy," or "condense" are categorically rejected. Every section, struct, enum, contract, and narrative exists because the domain demands it. Refinement means adding precision, not removing content. The specification targets exhaustive coverage of the PDF problem space; any proposal that would reduce its surface area, remove detail, or flatten its structure is antithetical to the project's goals. Reviewers and automated refinement rounds must ADD specificity, never subtract it.

**Delivery rule:**
1. Every gated subsystem ships with a simple, auditable baseline path.
2. Advanced algorithms land behind pluggable traits / feature flags.
3. Baseline v1 must prefer simple, auditable defaults over compact or exotic defaults.
4. An experimental path becomes default only after it beats the baseline on correctness or cost under the proof harness.


### Substrate delivery doctrine

Monkeybee's most important new architectural commitment is a concrete persistent incremental
substrate. That substrate is baseline architecture, not an optional research flourish.

The baseline substrate must provide:
- content-addressed roots for syntax, semantics, preserved bytes, and temporal lineage
- exact dependency-tracked recomputation for derived artifacts
- explicit preservation claims for every edit and write plan
- machine-readable certificates tying saves, diffs, and diagnostics back to substrate digests

More exotic extensions — theorem-prover-backed invariants, zk attestations, reactive bindings,
differential layout optimization, collaborative CRDT layers — are permitted only as layers on top
of the baseline substrate. They may enrich the evidence model, but they may not become hidden
prerequisites for correctness.

### Delivery spine

Monkeybee defines six release slices:

0. **Slice F — Foundation freeze**
   - identity model (`DocumentId`, `SnapshotId`, `ResourceFingerprint`)
   - `ExecutionContext` precedence and provider-policy rules
   - ownership lattice + mutation/writeback invariants
   - render/content/paint boundary freeze
   - scope registry bootstrap (see Part 8)
   - cache-key doctrine finalized before subsystem fan-out
   - content-addressed snapshot substrate + root-digest schema
   - incremental query engine + exact invalidation semantics
   - preservation algebra + invariant certificate schema
   - temporal revision model + semantic-anchor identity rules

1. **Slice A — Reader kernel**
   - open/parse/repair
   - inspect/extract
   - baseline raster render
   - baseline 3D content rendering (PRC/U3D parsing, static scene render)
   - deterministic full rewrite
   - strict parse-own-output validation

2. **Slice B — Bidirectional preserve loop**
   - immutable snapshots + EditTransaction
   - incremental append
   - annotation add/save/reopen
   - AcroForm read/fill/appearance regeneration
   - signature-safe save planning

3. **Slice C — Remote/progressive**
   - range-backed ByteSource
   - progressive page/region render
   - prefetch planning + refinement


4. **Slice D — External proof**
   - pathological corpus
   - compatibility ledger
   - multi-oracle render comparison
   - CI scorecards and regression gates
   - invariant certificates, hypothesis ledgers, and failure capsules

5. **Slice E — Intelligence / collaboration (post-v1)**
   - temporal revision replay and historical inspection surfaces
   - spatial-semantic graph + typed query interface
   - agent-safe edit proposals anchored by stable semantic IDs
   - collaborative delta merge / CRDT research lane
   - optional provenance attestations (Merkle-first, zk-later)

**v1 release requirement:** Slice F + Slice A + Slice B + Slice D.

No feature bead may begin implementation until its Slice F dependencies are ratified.

**v1 optional beta lane:** Slice C may ship behind a feature flag if it threatens v1 critical path.
**post-v1 expansion lane:** Slice E is intentionally non-blocking so Monkeybee can absorb higher-order
document-intelligence features without destabilizing the v1 kernel.

Every task in the roadmap must declare its owning slice.

In concrete terms, "alien artifact" means unusual coherence, breadth, and evidence.
The baseline contract specifies required behavior and proof thresholds.
Advanced algorithm candidates live in `docs/architecture/EXPERIMENTAL_ANNEX.md`.

### Operational mode doctrine

Monkeybee adopts explicit, named operational modes that encode mutually competing goals rather than hand-waving them. These modes are not hints — they are contracts that constrain parser, object model, writer, and editor behavior simultaneously.

**Parse modes:**

- **Strict mode:** Deterministic, pedantic, conformance-focused. Rejects input that violates the PDF specification. Useful for conformance validation, profile checking, and detecting producer bugs. Does not attempt repair. Every deviation from spec produces a structured diagnostic.
- **Tolerant mode:** Recovers from malformed real-world PDFs without undefined behavior, panics, or unbounded allocation. This is the default mode for real-world ingestion. Applies repair strategies, heuristic recovery, and producer-quirk shims. Every repair action is recorded in the compatibility ledger.
  **Ambiguity rule:** if multiple recovery strategies produce materially different semantic
  outcomes (page count, object graph, text decode, signature coverage, or write impact) and no
  deterministic tiebreaker exists, tolerant mode emits `parse.repair.ambiguous`.
  By default, `engine.open()` returns the highest-confidence candidate plus a `RepairDecision`
  that records every materially different `RecoveryCandidate`.
  `ForensicPreserve` may instead reject ambiguous recovery unless
  `allow_ambiguous_recovery=true`.

```
pub struct RecoveryCandidate {
    pub candidate_id: RecoveryCandidateId,
    pub confidence: f64,
    pub semantic_digest: [u8; 32],
    pub page_count: u32,
    pub write_impact: WriteImpactPreview,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct RepairDecision {
    pub chosen: RecoveryCandidateId,
    pub alternatives: Vec<RecoveryCandidateSummary>,
    pub reason: String,
}
```
- **Preserve mode:** Byte-preserving where possible. Does not rewrite, renumber, or reformat objects the engine does not semantically own. This mode exists to support signature-safe workflows and incremental-update integrity. The parser in preserve mode retains raw byte spans, whitespace, and formatting for objects it does not modify.

**Write modes:**

- **Deterministic write:** Full document rewrite with canonical formatting, rebuilt cross-references, and normalized structure. Produces the smallest, cleanest output. Breaks existing signatures.
- **Incremental append:** Append-only update section. Adds a new cross-reference section and trailer without rewriting existing bytes. Preserves existing signatures on unmodified content. Required for signature-safe preserve workflows.
- **Downlevel output:** Emit output constrained to an older PDF version or a specific profile (e.g., PDF/A-4, PDF/X-6). The writer validates output against the target profile's constraints and rejects or downgrades features that violate them.

The mode is not a global singleton. A single session can parse in tolerant mode, inspect the result, and then write in incremental-append mode. The modes compose.

### Operation profiles

Modes are low-level contracts. Most callers should start from an `OperationProfile` preset:

- `ViewerFast`
  - parse=tolerant, write=deterministic, security=compatible, open=eager|lazy,
    provider_policy=pinned_then_ambient
- `ForensicPreserve`
  - parse=preserve, write=incremental_append, security=hardened, open=eager
- `EditorSafe`
  - parse=tolerant, write=plan_selected, security=hardened, open=eager
- `BatchProof`
  - parse=tolerant, write=deterministic, security=strict_or_hardened, open=eager,
    determinism=on, provider_policy=pinned_only
- `BrowserWasm`
  - parse=tolerant, write=deterministic, security=strict, open=in_memory_remote,
    provider_policy=pinned_only

`ExecutionContext::from_profile(profile)` materializes budgets, cache policy,
provider policy, optional provider overrides, determinism, and default
write/open behavior from the preset.

### Execution context doctrine

Every top-level API accepts an operation-scoped `ExecutionContext` carrying:
- Resource budgets (objects, decompressed bytes, operators, recursion depth)
- Cooperative cancellation / deadline
- Determinism settings for CI and proof
- Provider policy and optional per-operation provider overrides
- Trace / metrics sink

```
pub struct DeterminismSettings {
    pub deterministic_output: bool,    // canonical serialization order, stable hashers
    pub pinned_fallback_fonts: bool,   // use pinned font pack instead of system fonts
    pub fixed_thread_count: Option<usize>,  // for reproducible benchmarks
    pub stable_task_order: bool,
    pub canonical_float_reductions: bool,
    pub deterministic_diagnostic_order: bool,
}
```

`ExecutionContext` is never stored on `OpenSession`.
Sessions are long-lived document handles; execution contexts are per-call control planes.

### ExecutionContext as runtime bridge

`ExecutionContext` is the contract between runtime-agnostic core crates and the
asupersync-native orchestration layer.

In asupersync-native callers (facade, CLI, proof), `ExecutionContext` is derived
from `&Cx`:
- `CancellationCheckpoint` delegates to `cx.checkpoint()` (budget-aware, trace-aware,
  scheduler-cooperative)
- `BudgetState` is derived from `cx.budget()` with monkeybee-specific field mapping:
  `Budget.deadline` → `ExecutionContext.deadline`, `Budget.cost_quota` → operator count
  and decompressed bytes budgets, `Budget.poll_quota` → checkpoint frequency,
  `Budget.priority` → render priority (viewport-visible > prefetch > background)
- `DiagnosticSink` emits to `cx.trace()` for LabRuntime observability

In runtime-agnostic callers (WASM, third-party integrations), `ExecutionContext`
uses simple implementations (AtomicBool cancellation, manual budget tracking).

The bridge is intentionally zero-cost: a single function pointer indirection for
checkpoint calls, which are already on the order of microseconds between operators.

### Diagnostic streaming model

All diagnostics flow through a `DiagnosticSink` carried by `ExecutionContext`. The sink is a trait
with a single method: `emit(diagnostic: Diagnostic)`. Implementations include:

- `VecSink`: collects all diagnostics into a `Vec<Diagnostic>` (default for library use).
- `CallbackSink`: invokes a user-provided closure per diagnostic (for real-time display).
- `FilteringSink`: wraps another sink and filters by severity, subsystem, or error code.
- `CountingSink`: wraps another sink and counts diagnostics by category (for budget enforcement:
  "abort after 1000 warnings" policies).

Every diagnostic carries:
- Error code (hierarchical string, e.g., `parse.xref.wrong_offset`)
- Severity (Fatal, Error, Warning, Info)
- Subsystem origin (parser, renderer, writer, etc.)
- Object context (ObjRef, page number, byte offset — whichever are applicable)
- Human-readable message
- Machine-readable payload (repair details, original/corrected values, feature classification)

The `DiagnosticSink` is the input side; the compatibility ledger (Part 6) is the aggregated output
side. All diagnostics emitted during a session are collected into the compatibility ledger at
session close.

Diagnostics are never silently dropped. If the `ExecutionContext` has no explicit sink configured,
a default `VecSink` collects them. The API always returns the diagnostic collection alongside the

### Decision trace and causal explainability contract

Diagnostics answer "what happened." `TraceEventStream` answers "why this path
was chosen instead of the alternatives."

Every repair choice, fallback chain branch, provider resolution, cache
miss-to-recompute transition, security-profile denial/isolation decision,
ownership escalation, and save-plan escalation MUST emit a causal trace event.

```
pub struct OperationSpanId(pub u128);

pub struct OperationSpan {
    pub span_id: OperationSpanId,
    pub parent: Option<OperationSpanId>,
    pub operation_kind: OperationKind,
    pub snapshot_id: Option<SnapshotId>,
    pub page_index: Option<u32>,
}

pub enum DecisionKind {
    RepairChoice,
    FallbackChoice,
    ProviderResolution,
    SecurityGate,
    CacheReuse,
    CacheMiss,
    OwnershipEscalation,
    WritePlanEscalation,
    RemoteFetchPriority,
}

pub struct DecisionRecord {
    pub span_id: OperationSpanId,
    pub decision_kind: DecisionKind,
    pub subject: String,
    pub chosen: String,
    pub alternatives: Vec<String>,
    pub confidence: Option<f64>,
    pub reason: String,
    pub causal_inputs: Vec<CausalRef>,
}

pub struct TraceEvent {
    pub ts_monotonic_ns: u64,
    pub span_id: OperationSpanId,
    pub event: TraceEventKind,
}

pub enum TraceEventKind {
    SpanStart(OperationSpan),
    SpanEnd { outcome: TraceOutcome },
    Decision(DecisionRecord),
    Metric { key: String, value: f64 },
    DiagnosticRef { diagnostic_code: String },
}
```

`TraceEventStream` is a required proof artifact for:
- ambiguous recovery
- signature-impacting save plans
- remote progressive render sessions
- any proof-harness failure capsule
operation result.

Provider interfaces include `FontProvider`, `ColorProfileProvider`, `CryptoProvider`, and `OracleProvider`.
Default provider instances live on `MonkeybeeEngine`.
`ExecutionContext` does not own the provider registry; it carries only the
policy and any per-call override layer used to resolve providers.

```
pub struct ProviderOverrides {
    pub font_provider: Option<Arc<dyn FontProvider>>,
    pub color_profile_provider: Option<Arc<dyn ColorProfileProvider>>,
    pub crypto_provider: Option<Arc<dyn CryptoProvider>>,
    pub oracle_provider: Option<Arc<dyn OracleProvider>>,
}
```

### Provider trait contracts

**CryptoProvider:**
```
trait CryptoProvider {
  /// Verify a CMS/PKCS#7 detached signature against the signed bytes.
  /// Returns the verification result including certificate chain, timestamps, and trust status.
  fn verify_cms_signature(
    &self,
    signed_bytes: &[u8],
    signature_der: &[u8],
  ) -> Result<SignatureVerification>;

  /// Verify a timestamp token (RFC 3161).
  fn verify_timestamp(
    &self,
    tst_der: &[u8],
  ) -> Result<TimestampVerification>;

  /// Compute a message digest (SHA-256, SHA-384, SHA-512) for byte-range integrity checks.
  fn digest(&self, algorithm: DigestAlgorithm, data: &[u8]) -> Vec<u8>;
}
```

The default `CryptoProvider` implementation provides digest computation only (using pure-Rust
crypto). Full PKI verification requires a configured provider (e.g., backed by OpenSSL, ring,
or a platform keystore). When no verification-capable provider is configured, signature
inspection reports the signature structure (issuer, serial, algorithm, timestamps) without
trust validation.

**OracleProvider:**
```
trait OracleProvider {
  /// Look up a resource by its oracle key (font name, ICC profile identifier, CMap name).
  /// Returns the resource bytes if found, None if not available.
  fn resolve(&self, key: &OracleKey) -> Option<Arc<[u8]>>;

  /// Report the oracle manifest (versions, sources, pinning info) for proof reproducibility.
  fn manifest(&self) -> OracleManifest;
}
```

The engine uses explicit resource-pack policy in all modes:
- `PinnedOnly`
- `PinnedThenAmbient`
- `AmbientAllowed`

Pinned resource packs include fallback fonts, Base 14 metrics, CJK fallbacks, standard CMaps,
and ICC defaults with provenance + license metadata.

Proof/CI mode must use `PinnedOnly`.
`ViewerFast` may use `PinnedThenAmbient`.
Every fallback resolution records provenance (`pack`, `ambient`, or `caller-supplied`) in the
diagnostic stream and compatibility ledger.
Deterministic mode fixes serialization order, stable hashers, fallback resources, and oracle manifests so CI evidence is reproducible across hosts.

### Feature module registry

Beyond providers, the engine supports optional `FeatureModule`s for capabilities that are:
- platform-specific
- safety-sensitive
- non-baseline
- externally versioned

Examples:
- `jpx_native`
- `jbig2_isolated`
- `pki_verify`
- `xfa_inspect`
- `postscript_subset_translate`

Each module declares:
- capability codes
- supported targets (native/wasm)
- determinism class
- safety class
- version/hash for manifesting

Canonical CI/proof runs record the active feature-module manifest alongside the oracle manifest.

### Active content policy

Decoder security and active-content handling are separate control planes.
JavaScript, action dictionaries, embedded files, rich media, and external
references are governed by `ActiveContentPolicy`.

```
pub enum ActiveContentPolicy {
    PreserveButDenyExecute,
    StripOnWrite,
    ErrorOnPresence,
    AllowTrustedHandlers,
}

pub struct ActiveContentReport {
    pub has_open_action: bool,
    pub has_additional_actions: bool,
    pub has_javascript: bool,
    pub has_launch: bool,
    pub has_uri: bool,
    pub has_submit_form: bool,
    pub has_remote_goto: bool,
    pub has_embedded_files: bool,
    pub has_rich_media: bool,
}
```

`CapabilityReport` MUST include `active_content: ActiveContentReport`.
The default v1 behavior is `PreserveButDenyExecute`.

```
pub struct ActiveContentInventory {
    pub actions: Vec<ActionNode>,
    pub javascript_blocks: Vec<JsBlockRef>,
    pub launch_actions: Vec<ActionNode>,
    pub uri_actions: Vec<ActionNode>,
    pub remote_goto_actions: Vec<ActionNode>,
    pub submit_form_actions: Vec<ActionNode>,
    pub embedded_file_refs: Vec<FileSpecRef>,
    pub rich_media_refs: Vec<RichMediaRef>,
}

pub struct SanitizationPlan {
    pub policy: ActiveContentPolicy,
    pub objects_to_strip: Vec<ObjRef>,
    pub objects_to_preserve: Vec<ObjRef>,
    pub objects_to_stub: Vec<ObjRef>,
}

pub struct SanitizationReceipt {
    pub plan_digest: [u8; 32],
    pub stripped: Vec<ObjRef>,
    pub preserved: Vec<ObjRef>,
    pub stubbed: Vec<ObjRef>,
    pub warnings: Vec<Diagnostic>,
}
```

`CapabilityReport` MUST grow an `active_content_inventory_digest` field, and
the CLI MUST support `inspect --active-content` and `sanitize --receipt-json`.

### Policy composition validity contract

Security profile, active-content policy, provider policy, determinism settings,
write/open mode constraints, and optional feature-module availability compose
into a single resolved policy contract before any externally visible operation
begins. Monkeybee MUST not discover an invalid policy combination halfway
through open, import, save, or proof execution.

```
pub enum ProviderPolicy {
    PinnedOnly,
    PinnedThenAmbient,
    AmbientAllowed,
}

pub struct ResolvedPolicySet {
    pub operation_profile: OperationProfile,
    pub security_profile: SecurityProfile,
    pub active_content_policy: ActiveContentPolicy,
    pub provider_policy: ProviderPolicy,
    pub determinism: DeterminismSettings,
    pub allowed_write_modes: Vec<WriteMode>,
    pub active_feature_modules: Vec<String>,
    pub policy_digest: [u8; 32],
}

pub enum PolicyConflictKind {
    SecurityVsFeature,
    ActiveContentVsWorkflow,
    ProviderPolicyVsDeterminism,
    PreserveConstraintVsWriteMode,
    ProfileVsFeatureModule,
    RemoteModeVsPersistence,
}

pub struct PolicyConflict {
    pub kind: PolicyConflictKind,
    pub summary: String,
    pub blocking_fields: Vec<String>,
}
```

Rules:
- a top-level operation resolves policy exactly once from engine defaults,
  `OperationProfile`, `ExecutionContext`, and explicit call arguments
- child spans may tighten the resolved policy for safety or budget reasons, but
  may never silently relax it
- invalid policy combinations fail before expensive work begins and emit a
  typed `PolicyConflict`, not a late generic error
- `AdmissionDecision`, `WritePlan`, `CrossDocumentImportPlan`, materialized
  acceleration indexes, and proof artifacts refer to the same `policy_digest`
  so downstream evidence can explain exactly which contract was in force
- proof/canonical CI runs require pinned provider/oracle/module manifests
  consistent with the resolved policy; otherwise the run is non-canonical by
  definition

---

## Part 1 — User workflows and visible proof

### Workflow 1: Render hostile PDFs

A user (or agent) hands Monkeybee a PDF that other open-source tools mishandle. Monkeybee parses it (tolerantly where necessary), resolves the document structure, and produces trustworthy visual output. The output is verified against reference renderers. If the file contains features that cannot yet be rendered, the engine surfaces explicit diagnostics rather than producing silently wrong output.

Proof surfaces: pathological corpus render comparisons, reference-guided differential testing, compatibility ledger entries.

### Workflow 2: Annotate existing ugly PDFs

A user loads an ugly real-world PDF, adds annotations (text notes, highlights, stamps, drawing markup), saves the annotated file, and reopens it. The annotations are geometrically correct, the original document is preserved, and the round trip does not corrupt the file.

Proof surfaces: annotation round-trip harness, geometry preservation checks, save-reopen validation.

### Workflow 3: Extract useful structure

A user loads a PDF and extracts text with positions, metadata, page structure, resource inventory, font information, and diagnostics about unsupported regions. The extraction is useful enough that downstream tools can build on it.

Proof surfaces: extraction correctness tests on representative docs, position accuracy validation, diagnostics completeness.

### Workflow 4: Edit and mutate documents

A user loads a PDF, performs structural edits (add/remove/reorder pages, copy pages between documents, merge/split documents, update metadata, modify resources), saves, and reopens. The modified document is structurally valid, preserves cross-document provenance where relevant, and renders correctly.

Proof surfaces: mutation round-trip harness, structural validity checks, render comparison pre/post edit.

### Workflow 5: Generate new documents

A user (or agent) creates a new PDF from scratch using Monkeybee's generation API. The generated document renders correctly under both Monkeybee and reference implementations. It is structurally valid, well-formed, and useful.

Proof surfaces: generation validation against reference renderers, structural conformance checks.

### Workflow 6: Inspect and diagnose

A user loads a PDF and uses Monkeybee's inspection tools to understand the document's internal structure: object graph, cross-references, page tree, resource dictionaries, font tables, content stream structure, incremental updates, and compatibility status. This is the "X-ray mode" that makes the engine useful for debugging and forensics.

Proof surfaces: inspection accuracy on known-structure documents, diagnostics completeness.

### Workflow 7: Signature-safe modification

A user loads a signed PDF, adds annotations or form field values using incremental-append save mode, and the existing digital signatures remain valid. The engine does not rewrite bytes it does not own. The user can verify that signed byte ranges were preserved and, when a `CryptoProvider` is configured, request full signature verification after the modification.

Proof surfaces: signature validity checks pre- and post-modification, byte-range preservation verification, incremental-update structural integrity.

### Workflow 8: Open huge or remote PDFs progressively

A user opens a very large or range-backed PDF, renders the first page or a region quickly, and lets the engine fetch additional bytes lazily as needed. Linearization is used when present, but not required.

Proof surfaces: first-page latency benchmarks, prefetch-plan traces, partial-open regression tests, byte-range accounting.

### Progressive rendering contract

When rendering a page from a lazily/partially-loaded document, the renderer operates in
progressive mode:

1. **Available resources render immediately.** Content stream operators that reference already-fetched
   resources (fonts, images, XObjects) are rendered normally.
2. **Unavailable resources produce placeholders.** An image XObject whose stream data hasn't been
   fetched renders as a gray placeholder rectangle with a loading indicator. A font that hasn't been
   fetched uses a substitute font with a diagnostic.
3. **Placeholder metadata.** Each placeholder carries the byte range needed to fetch the missing
   resource. The caller can use this to prioritize fetches for the visible region.
4. **Incremental refinement.** When a previously-missing resource becomes available, only the
   affected tiles in the tile/band scheduler are invalidated and re-rendered. The rest of the page
   is preserved.
5. **Prefetch planning.** Before rendering, the render pipeline reports the set of resources needed
   for the requested page/region. The byte source's fetch scheduler can use this to issue range
   requests proactively.

Progressive rendering is orthogonal to the tile/band scheduler: a tile may be partially rendered
(some resources available, some not) and refined later. The cache key for progressive tiles includes
a "completeness" flag so refined tiles replace partial ones.

### Workflow 9: Fill, regenerate, and preserve forms

A user loads an AcroForm-heavy PDF, reads or updates field values, regenerates widget appearances, saves, and reopens without breaking the field tree or signed byte ranges outside the edited scope.

Proof surfaces: field round trips, appearance regeneration tests, signature-preserving form fills.

### Workflow 10: Explain compatibility and diff revisions

A user opens a PDF and immediately receives a `CapabilityReport` summarizing: signatures,
encryption, tagged-structure presence, XFA/JS/risky-decoder presence, edit-safety class,
preserve-mode constraints, and expected degradation zones.

A user compares two PDFs or two snapshots and receives a unified diff: structural changes,
text changes, render deltas, signature impact, and compatibility-tier changes.


### Workflow 11: Time-travel and revision forensics

A user loads a PDF with multiple incremental updates, signatures, or suspicious late-stage edits and
asks not merely "what does the latest state look like?" but "how did it get here?" Monkeybee can
inspect the revision chain, materialize historical snapshots, diff any two revision frames,
re-render historical states, and explain which objects changed at each step.

Proof surfaces: revision-replay harness, historical render/extract equivalence checks, signed-range
impact verification, and per-frame change receipts.

### Workflow 12: Queryable semantic anchors and agent-safe edits

A user or agent loads a PDF and queries it through a typed, geometry-aware semantic graph rather
than by scraping raw text. The caller can locate a table cell, paragraph, form field, annotation,
or signature region by stable semantic anchor; then propose a constrained edit, highlight,
redaction, or extraction workflow against that anchor. Monkeybee validates the proposal against the
current snapshot, policy, ownership rules, and preservation constraints before applying anything.

Proof surfaces: anchor-stability tests across safe rewrites, query determinism checks, proposal
validation logs, and edit receipts that point back to anchor IDs and substrate digests.

### Workflow 13: Render and interact with 3D PDF content

A user opens a PDF containing 3D annotations (PRC or U3D data streams). Monkeybee parses the 3D
data, builds a scene graph, and renders via wgpu. The user can orbit, pan, zoom, switch named
views, toggle rendering modes (solid, wireframe, transparent, illustration), apply cross-sections,
and navigate the product structure tree. This works natively on desktop (Vulkan/Metal/DX12) and
in the browser (WebGPU).

Proof surfaces: 3D content detection and parsing on corpus, scene graph construction validation,
render comparison against Adobe Acrobat screenshots, named view interpolation tests, cross-section
geometry verification.

### Workflow 14: Document security forensics

A user loads a PDF and requests a security/forensics analysis. Monkeybee detects hidden content
(white-on-white text, off-page content, content clipped behind images), insufficient redactions
(content still extractable under opaque rectangles), post-signing modifications (classify changes
as permitted or suspicious), known CVE-pattern signatures in parsed structures, producer
fingerprinting from structural patterns (not just the `/Producer` string), and font fingerprinting
via glyph outline matching.

Proof surfaces: hidden content detection on known-planted corpus, redaction sufficiency audit on
intentionally bad redactions, post-signing modification classification accuracy.

### Workflow 15: Prepress inspection, soft proofing, and separation preview

A user opens a print-oriented PDF and asks not only "does it render?" but "is it ready for the
press condition I care about?" Monkeybee can simulate CMYK overprint on an RGB display, soft-proof
against the document's output intent or a caller-supplied ICC profile, preview individual process
and spot-color separations, estimate total area coverage (TAC), and run print-preflight checks for
image resolution, bleed/trim alignment, color-space suitability, and trap annotations. This
workflow explicitly includes halftone dictionaries (Types 1, 5, 6, 10, 16), spot-function and
threshold-screen inspection, `/TR` and `/TR2` transfer-function evaluation, `/BG`, `/BG2`, `/UCR`,
and `/UCR2` print-state reporting, document-level and page-level `/OutputIntents`, and TAC policy
checks against caller-configured press ceilings such as 300-340%.

Proof surfaces: overprint-simulation comparisons on CMYK corpus files, output-intent and
soft-proof reference renders, TAC expectation fixtures, separation-preview image checks, and
preflight regression suites for low-DPI images, missing bleed, and color-profile mismatches.

### Workflow 16: Sign, timestamp, and long-term validate PDFs

A user creates a signature field or reuses an existing one, signs the document in incremental
append mode, adds trusted timestamps and validation material, and later verifies the result
offline. Monkeybee can classify PAdES conformance level (B-B, B-T, B-LT, B-LTA), model DSS/VRI
state, surface certificate-chain and revocation evidence, and explain which save plans preserve or
invalidate signature guarantees. This includes explicit chain building from signing leaf to trust
anchor via SKI/AKI/AIA metadata, OCSP/CRL evidence capture or embedding, RFC 3161 TSA integration,
and per-signature VRI material keyed so each signature can be validated independently without a
network round trip.

Proof surfaces: signature-creation interoperability tests, timestamp-validation fixtures, DSS/VRI
round trips, offline long-term validation tests, and post-signing save-impact classification.

### Workflow 17: Audit tagged PDF and accessibility semantics

A user loads a tagged PDF and asks for a semantic audit rather than a raw tree dump. Monkeybee can
inspect structure roles, attribute/class-map state, `/ActualText`, `/Alt`, `/E`, `/Lang`,
pronunciation metadata, artifacts, and structure destinations; it can also visualize reading order
and emit a PDF/UA-style audit report that explains structural gaps without attempting remediation.
That audit must be rich enough to reason about standard structure types, heading hierarchy,
artifact-marked content, table header associations, figure alt text, and where tag-driven reading
order diverges from geometric order.

Proof surfaces: tagged corpus fixtures with expected role trees, ActualText-versus-decoded-text
preference checks, artifact-exclusion extraction tests, audit-rule suites, and reading-order
overlay regressions.

### Workflow 18: Exchange, flatten, and preserve form data

A user imports FDF/XFDF into an AcroForm document, exports the filled result back out, flattens the
form when required, and still expects appearance correctness, field-tree integrity, and
signature-safe preserve behavior. Monkeybee can also detect calculation/format/validate scripts,
classify submit-form targets, create new signature fields with correct placeholders, and handle
barcode fields or Tier 2 static-XFA flattening where safe. Form flattening in this workflow is not
generic annotation flattening; it must resolve field inheritance, value synchronization, and
calculation-order preservation before visual burn-in.

Proof surfaces: FDF/XFDF round trips, form-flatten visual comparisons, script-detection fixtures,
submit-target inventories, signature-field placeholder tests, and static-XFA flattening cases.

### Workflow 19: Inventory actions, links, and active content

A user opens a suspicious or highly interactive PDF and asks for a full action inventory. Monkeybee
enumerates the action graph, classifies every action type, extracts a document link map for
navigational actions, preserves all action dictionaries during round trip, and can produce a
sanitization plan without executing any active content. The inventory covers the full family of PDF
actions, including GoTo, GoToR, GoToE, GoTo3DView, Launch, Thread, URI, Sound, Movie, Hide, Named,
SetOCGState, Rendition, Transition, JavaScript, ImportData, ResetForm, SubmitForm, and
RichMediaExecute.

Proof surfaces: action-corpus inventories, link-map equivalence tests, sanitization-receipt
regressions, and round-trip preservation tests for action dictionaries across incremental append
and full rewrite.

### Workflow 20: Inspect portfolios, article threads, page transitions, and multimedia

A user opens a PDF portfolio, slideshow, magazine-style article-thread document, or multimedia-rich
file and wants the engine to expose what is there even when playback is denied. Monkeybee can
enumerate collections, embedded-document relationships, article beads, page transitions,
thumbnails, alternate presentations, page-piece dictionaries, web-capture structures, screen
annotations, sound/movie objects, rendition trees, and media clips. This includes `/Threads` bead
navigation, `/Trans` presentation dictionaries such as Dissolve/Wipe/Fly/Push/Cover/Uncover/Fade,
`/Thumb` page thumbnails, `/Collection` schemas and navigators, and legacy movie or sound objects
that must be cataloged even when execution is denied.

Proof surfaces: parse-and-preserve round trips for portfolios and threaded documents, inventory
fixtures for page transitions and thumbnails, and multimedia cataloging tests that verify detection
without execution.

---

## Part 2 — Compatibility target

### The ugly PDF tail

The target is not the clean subset that makes engineering feel elegant. The target is the long tail of real PDFs in the wild:

**Structural pathologies:**
- Malformed cross-reference tables and streams
- Broken or circular object graphs
- Missing or invalid trailers
- Incremental-update chains with conflicting state
- Linearization artifacts and damaged linearization headers
- Hybrid cross-reference files

**Font and encoding nightmares:**
- Missing or incomplete ToUnicode CMaps
- Type 1 fonts with broken metrics
- CIDFont subsetting errors
- Encoding conflicts between font dictionaries and content streams
- Embedded fonts with invalid tables
- CJK fonts with non-standard encodings

**Rendering edge cases:**
- Transparency groups with isolated/knockout combinations
- Soft masks from luminosity and alpha
- Overprint and overprint mode interactions
- Blend mode stacking
- Type 3 font glyphs with embedded graphics state
- Tiling patterns with unusual matrices
- Shading patterns across color spaces

**Producer quirks:**
- Files from hundreds of different PDF producers with varying spec compliance
- Quirks specific to major producers (Acrobat, Word, LaTeX, Chrome print, LibreOffice, etc.)
- Scanned documents with unusual page structures
- Print-pipeline artifacts

**Legacy and hostile categories:**
- XFA forms and hybrid XFA/AcroForm documents (Tier 2/3: detect, characterize, handle where safe)
- PostScript XObjects (Tier 2/3: detect, constrained handling where feasible)
- Flash/RichMedia legacy (Tier 3: detect and degrade explicitly)
- Encrypted files with various security handlers
- Adversarial inputs designed to exploit parser bugs

### Repair strategies and fallback chains

This section specifies the actual recovery mechanisms for each pathology class. These are not aspirational — they are the concrete strategies the tolerant parser must implement.

#### Cross-reference repair

A well-formed PDF ends with `%%EOF`, preceded by a `startxref` pointer to the last cross-reference section. In practice, this chain breaks constantly.

**Wrong `startxref` offset:** The `startxref` value points to a byte offset that does not contain `xref` (for table-style) or a valid cross-reference stream object. Recovery strategy:
1. Scan backward from `%%EOF` looking for the `xref` keyword or a stream object with `/Type /XRef`. Search in a window of ±4096 bytes from the declared offset first, then expand.
2. If backward scan fails, scan forward from the declared offset.
3. If both fail, fall back to full-file `xref` keyword scan (costly but necessary for severely damaged files).
4. Record the repair in the compatibility ledger with the original offset, the discovered offset, and the scan method used.

**Corrupted cross-reference table entries:** Individual xref entries are 20 bytes each (10-digit offset, 5-digit generation, `f`/`n` flag, and line endings). Common corruptions:
- Wrong offsets (object is not at the declared position). Recovery: when an object lookup fails at the declared offset, scan forward/backward from that position looking for `N 0 obj` where N is the expected object number. If found within a reasonable window (configurable, default 8192 bytes), use the discovered offset and record the repair.
- Missing entries. Recovery: if an object is referenced but has no xref entry, perform a full-file scan for `N 0 obj` patterns and build a supplementary xref from discovered objects.
- Entries pointing to freed objects that are still referenced. Recovery: treat as in-use if the object is actually present at the offset.

**Cross-reference stream recovery:** Cross-reference streams (PDF 1.5+) encode the xref as a compressed stream object. Failures include: corrupted stream data (decompression failure), invalid `/W` array (field widths), or missing `/Size`. Recovery:
1. If the stream decompresses but the `/W` array is invalid, try common field-width patterns: `[1 2 1]`, `[1 3 1]`, `[1 3 2]`, `[1 4 2]`.
2. If the stream itself is corrupted, fall back to a full-file object scan.
3. For hybrid files (both table and stream xrefs), cross-validate entries between the two and prefer the stream version for objects present in both.

**Missing or invalid trailer:** The trailer dictionary provides `/Root`, `/Info`, `/Size`, `/Encrypt`, and `/ID`. If the trailer is missing or damaged:
1. Scan for `/Root` references across all dictionaries. The catalog object is identifiable by containing `/Type /Catalog` and `/Pages`.
2. Reconstruct `/Size` from the highest object number discovered during xref repair.
3. If `/Encrypt` is missing but streams appear encrypted (decompression fails in patterns consistent with encryption), report the situation — do not guess encryption parameters.

**Incremental update chain conflicts:** Each incremental update appends a new xref section and trailer with `/Prev` pointing to the previous xref. Conflicts arise when:
- Multiple updates define the same object number with different content. Resolution: last-writer-wins (the most recent update in the chain takes precedence), per the PDF specification.
- `/Prev` chain is circular (points back to an earlier section already visited). Detection: maintain a visited-offsets set during chain traversal. If a cycle is detected, stop traversal and use the objects discovered so far.
- An intermediate update's xref is corrupted. Recovery: skip the corrupted section and continue traversal via its `/Prev` pointer if recoverable, or terminate the chain at the last valid section.

**Linearization damage:** Linearized files have a specific structure (hint tables, part 1/part 2 division) that enables page-at-a-time download. When linearization headers are damaged but the underlying objects are intact, the strategy is: ignore linearization hints entirely and fall back to standard xref-based access. Record the linearization damage in the compatibility ledger. Never attempt to repair linearization hints — only bypass them.

### Linearization detection and use

**Detection:** A linearized PDF has:
1. A linearization dictionary as the first indirect object after the header. It contains
   `/Linearized` (version number, typically 1.0), `/L` (file length), `/O` (first-page object
   number), `/E` (end of first-page cross-reference), `/N` (page count), `/T` (offset of main
   xref), and `/H` (hint stream offsets/lengths).
2. A first-page cross-reference section immediately after the linearization dictionary.
3. All objects needed for the first page appear before the first-page xref.
4. Hint tables (primary and optional overflow) that map pages to byte ranges.

**Use in eager/lazy mode:** When linearization is intact, the parser can use the linearization
dictionary to locate the first-page objects directly without reading the main xref at the end
of the file. This enables faster first-page display for large files.

**Use in remote mode:** The fetch scheduler uses linearization hints to issue range requests for
specific pages. The primary hint table maps page numbers to byte ranges. The parser requests the
first-page range, renders it, then requests subsequent pages on demand.

**Bypass:** When linearization is detected but damaged (hint tables corrupted, first-page xref
invalid, or file length doesn't match `/L`), the parser:
1. Records a `parse.linearization.damaged` diagnostic
2. Falls back to reading the standard xref at end-of-file (via `startxref`)
3. Proceeds as if the file were not linearized
4. The fetch scheduler falls back to heuristic prefetching (request the last 64KB first to get
   the xref, then request pages based on xref offsets)

Linearized output is explicitly deferred to post-v1 (writing linearized files requires careful
object ordering and hint table generation). For v1, the engine reads linearized files but always
writes non-linearized output.

#### Font and encoding recovery

Font handling in real-world PDFs is where the gap between spec and reality is widest. The engine must implement a multi-stage fallback chain for text decoding (character code → Unicode) and a separate chain for glyph rendering (character code → glyph outline).

**Text decoding fallback chain (character code → Unicode):**

1. **ToUnicode CMap:** If the font dictionary contains a `/ToUnicode` entry pointing to a CMap stream, use it. This is the most reliable path when present. Parse the CMap's `beginbfchar`/`beginbfrange` mappings. Handle common CMap errors: wrong range boundaries, overlapping ranges (last definition wins), and malformed hex strings (pad or truncate to expected length).

2. **Predefined CMap + CIDSystemInfo:** For CIDFonts (Type 0 composite fonts), if no ToUnicode is present, check the `/Encoding` entry for a named CMap (e.g., `UniJIS-UTF16-H`, `GBK-EUC-H`). Combine with the font's `/CIDSystemInfo` (Registry/Ordering/Supplement) to select the correct predefined CMap from the engine's built-in CMap database. The engine must ship with the standard Adobe CMaps for CJK encodings: Adobe-Japan1, Adobe-CNS1, Adobe-GB1, Adobe-Korea1.

3. **Encoding + Differences array:** For simple fonts (Type 1, TrueType with simple encoding), check the `/Encoding` entry. If it names a standard encoding (`WinAnsiEncoding`, `MacRomanEncoding`, `MacExpertEncoding`, `StandardEncoding`), use the predefined character-code-to-name mapping. If a `/Differences` array is present, apply its overrides: each entry is a starting code followed by glyph name replacements. Map glyph names to Unicode via the Adobe Glyph List (AGL) and its supplement.

4. **Glyph name → Unicode via AGL:** If the font uses named glyphs (common in Type 1 and TrueType with post tables), map glyph names to Unicode using the Adobe Glyph List. Handle `uniXXXX` and `uXXXX` naming conventions. Handle the common case where producers use nonstandard glyph names (e.g., `a1`, `a2` for Zapf Dingbats variants) by maintaining a supplementary mapping table.

5. **TrueType `cmap` table direct mapping:** For embedded TrueType fonts, if all CMap/Encoding paths fail, attempt to extract Unicode mappings directly from the font's `cmap` table. Prefer platform 3 encoding 1 (Windows Unicode BMP) or platform 0 (Unicode). This is a recovery-only path — the PDF spec says the font dictionary's encoding takes precedence, but when that encoding is broken or missing, the `cmap` table is often the only remaining source of truth.

6. **Character code identity mapping:** As a last resort for CIDFonts with no other mapping source, treat the character code as a Unicode code point (identity mapping). This is frequently correct for fonts with Identity-H or Identity-V encoding when the CIDs happen to correspond to Unicode values. Record this assumption in the compatibility ledger.

7. **Unmappable:** If no strategy produces a Unicode mapping, record the character as unmappable with its raw character code, font name, and encoding information. The extraction layer reports these. The renderer uses `.notdef` or a substitution glyph.

**Glyph rendering fallback chain (character code → outline):**

1. For embedded fonts: extract the glyph program from the embedded font data (Type 1 charstring, TrueType `glyf`/`gvar` table, CFF charstring, or Type 3 content stream). If the embedded font data is corrupt (invalid charstring, broken `glyf` offsets, truncated CFF), attempt partial recovery: render glyphs that are valid, use `.notdef` for broken individual glyphs.

2. For non-embedded fonts: resolve through the configured `FontProvider`. Ambient system fallback is opt-in and non-canonical; proof runs use a pinned fallback font pack. The default `FontProvider` maintains a substitution table mapping common PDF font names (`Helvetica`, `Times-Roman`, `Courier`, `Symbol`, `ZapfDingbats`, `Arial`, `TimesNewRoman`) to bundled equivalents, consulting font descriptor flags (serif, sans-serif, fixed-pitch, italic, bold) and Panose classification if present.

3. For the 14 standard fonts (the "Base 14"): the engine must ship with metrics for all 14 (Courier, Courier-Bold, Courier-Oblique, Courier-BoldOblique, Helvetica, Helvetica-Bold, Helvetica-Oblique, Helvetica-BoldOblique, Times-Roman, Times-Bold, Times-Italic, Times-BoldItalic, Symbol, ZapfDingbats). Optionally ship with outlines; at minimum, provide glyph widths so text positioning is correct even if the outlines must come from system substitution.

4. Width override: regardless of which glyph outlines are used, the font dictionary's `/Widths` array (or `/W` array for CIDFonts) takes precedence for glyph advance widths. A common producer bug is embedding a subsetted font whose internal metrics disagree with the `/Widths` array. Always trust the PDF-level widths for positioning; use the embedded font only for outlines.

**Specific font-type recovery notes:**

- **Type 1 with broken PFB:** Some producers embed Type 1 fonts with incorrect segment lengths in the PFB binary header. Recovery: ignore segment lengths and parse the ASCII/binary segments by looking for the `eexec` and `cleartomark` markers directly.
- **Type 1 with non-standard encryption keys:** When the standard Type 1 charstring key (`4330`)
  or normal eexec assumptions produce garbage, try the small known set of non-standard keys (with
  `0` as the most common alternate) before giving up, and record the winning key in the
  compatibility ledger.
- **CFF with wrong offsets:** CFF (Compact Font Format) data uses offset-based indexing. If the Top DICT's CharStrings offset is wrong, scan for the CharStrings INDEX structure (count followed by offsize followed by offset array) in likely positions.
- **CFF subroutine integrity during subsetting:** Subsetting must compute the full dependency
  closure over global and local subroutines, remove dead subroutines, renumber survivors, and
  recalculate the bias based on the surviving counts rather than the original counts.
- **TrueType with broken `loca` table:** The `loca` table maps glyph IDs to offsets in the `glyf` table. If `loca` entries point outside `glyf` bounds, clamp to the `glyf` table length and record the error. Individual broken glyphs use `.notdef`; the rest of the font remains usable.
- **CJK identity CIDFonts with no embedded data:** Very common in files from Asian producers. The font dictionary specifies a CIDFont with no embedded font program, relying on the viewer having the font installed. The engine must: (a) recognize common CIDFont names and map them to available CJK fonts, (b) provide a CJK fallback font or clearly report the missing font so the user can supply one.

#### Transparency edge case handling

The PDF transparency model (ISO 32000-1 §11.6, ISO 32000-2 §11.7) is a full Porter-Duff compositing system with additional complexity from isolation, knockout, soft masks, and blend modes. The edge cases that matter most:

**Isolated vs. non-isolated groups:** An isolated transparency group composites against a fully transparent backdrop. A non-isolated group composites against the group's own backdrop (the content underneath it). The difference is subtle but visually significant when the group contains semi-transparent elements. The engine must track the isolation flag per group and correctly initialize the backdrop for compositing.

**Knockout groups:** In a knockout group, each element composites directly against the group's initial backdrop rather than against the accumulated result of previous elements in the group. This means earlier elements in the group do not contribute to the backdrop of later elements. The engine must maintain both the "accumulated" buffer and the "initial backdrop" buffer for knockout groups, selecting the correct source per-element.

**Isolated knockout groups:** The combination of isolation and knockout is the hardest case. The group composites against a transparent backdrop (isolation), and each element composites against that transparent backdrop rather than against previously drawn elements (knockout). This requires careful buffer management to avoid double-compositing artifacts.

**Soft mask from luminosity:** A soft mask derived from a transparency group's luminosity uses the formula: `mask_value = luminosity(composite_result) * backdrop_alpha`. The luminosity calculation depends on the group's color space. The engine must composite the mask group to completion, convert each pixel to luminosity (using the color space's luminosity coefficients — for DeviceRGB: 0.2126·R + 0.7152·G + 0.0722·B), and then use the result as an alpha mask. A common bug is using simple averaging instead of proper luminosity weights.

**Soft mask from alpha:** Simpler than luminosity: the mask value is the alpha channel of the composited mask group. But note that the mask group itself may contain transparency, so it must be fully composited before the alpha is extracted.

**Blend mode interactions:** PDF defines 16 blend modes: Normal, Multiply, Screen, Overlay, Darken, Lighten, ColorDodge, ColorBurn, HardLight, SoftLight, Difference, Exclusion, Hue, Saturation, Color, Luminosity. The last four (Hue, Saturation, Color, Luminosity) are non-separable — they operate on the composite color value rather than per-channel. The engine must implement all 16. The hard cases: blend modes stacked inside nested transparency groups with different isolation/knockout settings; blend modes applied to elements that are themselves soft-masked; blend modes in different color spaces requiring conversion before blending.

**Overprint and overprint mode:** Overprint (`/OP`, `/op`) controls whether painting in one colorant erases other colorants in the same area. Overprint mode (`/OPM`) modifies the behavior for DeviceCMYK: OPM=1 means a zero component value does not overwrite the corresponding backdrop component (the "nonzero overprint" rule). This matters for CMYK-heavy print-oriented PDFs and is a common source of visual differences between renderers. Baseline v1 MUST track overprint state and emit explicit diagnostics when OPM=1 or Separation/DeviceN overprint semantics are not available on the active support class. Full OPM=1 nonzero-overprint behavior becomes Tier 1 only after scope-registry promotion and proof-harness coverage.

#### Producer quirk catalog

Different PDF producers generate files with consistent, predictable deviations from the specification. The engine should maintain an isolated quirk-shim layer that detects the producer (from the `/Producer` and `/Creator` metadata fields) and activates appropriate compensations.

**Adobe Acrobat quirks:**
- Often writes `%%EOF` followed by additional bytes (common in incremental saves). Tolerant parser must not choke on trailing garbage after `%%EOF`.
- Uses proprietary extensions in form fields (e.g., `/AA` additional-actions dictionaries with Acrobat-specific trigger types).
- Sometimes writes `/Length` values for streams that are off by one (missing the final newline before `endstream`). Recovery: if `endstream` is not found at exactly offset + length, scan forward up to 16 bytes for `endstream`.

**Microsoft Word/Print-to-PDF quirks:**
- Generates extremely deep page trees (one intermediate node per page instead of a flat list or balanced tree). The page tree walker must handle arbitrary depth without stack overflow.
- Often emits CIDFonts with Identity-H encoding and a ToUnicode CMap, but the ToUnicode CMap sometimes has incomplete coverage (particularly for ligatures and special characters).
- Uses named destinations with unusual characters that other parsers may reject. Tolerant parser must accept any byte sequence in a name object after the `/` prefix.

**LaTeX/pdfTeX/LuaTeX quirks:**
- Type 1 font subsetting with glyph names that do not conform to the Adobe Glyph List (e.g., custom glyph names for math symbols). The AGL lookup must fall back to the font's built-in encoding when AGL lookup fails.
- Content streams that use `BT`/`ET` blocks with unusual text positioning (e.g., resetting the text matrix mid-block with `Tm` for every glyph). This is legal but unusual and can confuse text extraction that assumes sequential `Tj`/`TJ` within a block.

**Chrome/Chromium print-to-PDF quirks:**
- Generates pages with large numbers of small path operations (one path per character when printing web content). This is legal but performance-sensitive; the content stream interpreter must handle hundreds of thousands of operators per page without degradation.
- Uses DeviceRGB exclusively, even for content that originated in CMYK workflows. No color management issues, but the files can be extremely large.
- Sometimes generates clipping paths that are never used (empty clip followed by immediate restore). The renderer should handle this gracefully (it is a no-op).

**LibreOffice quirks:**
- Occasionally generates cross-reference tables with incorrect entry counts (`/Size` is wrong). Recovery: count actual entries and use the real count.
- Font embedding sometimes omits the `/FontDescriptor` for standard fonts. The engine must not crash on a missing font descriptor — fall back to the Base 14 metrics if the `/BaseFont` name matches a standard font.

**InDesign quirks:**
- Heavy use of Separation and DeviceN color spaces with spot colors. The engine must handle DeviceN with many components (4+) and correctly apply the `/AlternateSpace` and `/TintTransform` when the actual colorants are unavailable.
- Uses nested transparency groups extensively. Performance and correctness are both at stake.

**Quartz (macOS Preview/Core Graphics) quirks:**
- Generates PDFs with unusual UserUnit values (the page is defined in a non-standard unit). The engine must scale correctly based on the `/UserUnit` page attribute (default is 1/72 inch per unit).
- Sometimes uses `/Filter` arrays with a single filter (e.g., `[/FlateDecode]` instead of `/FlateDecode`). Both forms are legal per spec, but some parsers only handle the name form. The engine must accept both.

#### XFA safe-contained handling (Tier 2)

XFA (XML Forms Architecture) is deprecated as of PDF 2.0 but persists in millions of existing government and enterprise forms. Full XFA support requires a complete XML layout engine — that is not a v1 goal. However, the engine must not simply blank-page on XFA documents.

**Detection:** Check for `/AcroForm` dictionary with an `/XFA` entry. The `/XFA` value is either a stream or an array of alternating name/stream pairs (packet-based XFA). Also check for the presence of a `/NeedsRendering` key set to `true` in the catalog, which signals that the XFA layer is authoritative over AcroForm.

**Safe subset handling:**
1. **Hybrid XFA/AcroForm documents:** Many XFA documents also contain an AcroForm fallback layer with pre-rendered appearance streams. If AcroForm field widgets have appearance streams (`/AP` entries), render from those. This gives usable visual output for the majority of hybrid documents without any XFA interpretation.
2. **XFA template extraction:** Parse the XFA XML packets to extract the template structure, dataset values, and locale information. Expose these through the extraction/inspection API as structured XML, even if the engine cannot render the XFA layout.
3. **Static XFA forms:** A subset of XFA forms are "static" (no dynamic layout recalculation needed). For these, the AcroForm appearance streams are usually sufficient.
4. **Dynamic XFA forms without AcroForm fallback:** These are the hard case. The engine reports them as Tier 3 (detected, cannot render, explicit diagnostic). The compatibility ledger records the XFA version, template complexity, and which packets are present.

**What the engine explicitly does not do for XFA in v1:** No XFA layout engine. No XFA scripting. No XFA-specific form filling. No XFA-to-AcroForm conversion.

#### PostScript XObject handling (Tier 2/3)

PostScript XObjects (`/Subtype /PS`) embed raw PostScript code within a PDF. They are deprecated since PDF 1.4 but still appear in old files generated by early Acrobat Distiller and some RIPs.

**Detection:** Identify XObjects with `/Subtype /PS`. These contain PostScript language code rather than PDF content stream operators.

**Tier 2 handling (where feasible):** For the limited subset of PostScript XObjects that consist only of path construction (moveto, lineto, curveto, fill, stroke) and simple graphics state manipulation, translate the PostScript operators to equivalent PDF content stream operators. This handles the common case of vector art embedded as PostScript.

**Tier 3 handling (default):** For PostScript XObjects that use control flow, stack manipulation, or PostScript-specific operators that have no PDF equivalent, record a diagnostic and render a placeholder (bounding-box rectangle with diagnostic text, or skip the XObject). Do not attempt to interpret arbitrary PostScript — that would require a full PostScript interpreter, which is a different engine.

#### Encryption and security handler recovery

**Encryption and security handler recovery**

**Standard security handlers (V1-V5):** Support all standard encryption revisions. V1/V2 use RC4; V4 adds AES-128; V5 uses AES-256. The engine must support all of these for decryption. Output encryption is a post-baseline, non-gating feature. It is disabled in the baseline v1 build. When the optional `write-encryption` feature is enabled and promoted, the default SHOULD be AES-256 (V5, R6).

**Encryption key derivation specifics:**

For V1-V4 (R2-R4), the encryption key is derived from:
1. Pad the user password to 32 bytes using the standard padding string (defined in ISO 32000-1 Table 3.19 — a fixed 32-byte sequence starting with `0x28 0xBF 0x6E 0x5E...`).
2. Hash with MD5: padded password + `/O` value + permissions integer (little-endian 4 bytes) + document `/ID` first element.
3. For R3+, iterate the MD5 hash 50 times, taking the first N bytes each time (where N is the key length / 8).
4. The result is the file encryption key.

For V5 (R5-R6), key derivation uses SHA-256/SHA-384/SHA-512:
1. Concatenate the UTF-8 password (truncated to 127 bytes, SASLprep-normalized for R6) with the validation salt and user key data from the `/U` entry.
2. Hash with SHA-256 (R5) or the extended hash algorithm (R6 — an iterative SHA-256/384/512 scheme with AES-CBC rounds, designed to resist GPU acceleration).
3. The intermediate hash decrypts the `/UE` (user encryption key) entry using AES-256-CBC to yield the file encryption key.

**Per-object key derivation:** For V1-V4, each object is encrypted with a per-object key derived from: file encryption key + object number (little-endian 3 bytes) + generation number (little-endian 2 bytes), hashed with MD5, truncated to min(key_length + 5, 16) bytes. For AES (V4), append the bytes `0x73 0x41 0x6C 0x54` ("sAlT") before the final MD5. For V5, the file encryption key is used directly (no per-object derivation).

**What gets encrypted:** All strings and streams are encrypted, with exceptions: the `/ID` array values in the trailer, the encryption dictionary itself, strings within the encryption dictionary, and cross-reference streams (their data is not encrypted, though objects referenced from them may be). String encryption uses the per-object key; stream encryption also uses the per-object key but may use a different algorithm (e.g., strings with RC4 and streams with AES in a V4 file, controlled by `/StrF` and `/StmF`).

**Crypt-filter identity handling:** If a string or stream uses `/Filter /Crypt` with
`/DecodeParms` naming the `/Identity` crypt filter, that crypt stage is a no-op and must not
trigger decryption attempts or "encrypted stream" confusion. Producers sometimes emit this
redundantly; the engine must preserve the declaration, treat it deterministically, and ledger it as
an identity/no-op case rather than an encryption failure.

**Password handling edge cases:**
- Empty owner/user passwords (very common). The engine must correctly handle the empty-string password case, which requires specific padding per the spec (28 bytes of the standard padding string).
- UTF-8 password normalization (SASLprep, per PDF 2.0). Passwords must be normalized before encryption key derivation.
- Files where the permissions integer is incorrect but the encryption is otherwise valid. Do not reject a file solely because its stated permissions seem wrong — decrypt and let the user decide.

**Non-standard security handlers:** The engine detects non-standard security handlers (any `/Filter` value other than `/Standard` in the encryption dictionary) and reports them as Tier 3. It does not attempt to implement proprietary DRM schemes (Adobe LiveCycle Policy Server, FileOpen, etc.).

### PDF 2.0 normative supplements

Monkeybee tracks the normative supplements that materially affect modern PDF 2.0 interoperability.
These are not decorative checkboxes; they affect decryption, integrity reporting, structure
preservation, color management, and extraction fidelity.

**AES-GCM authenticated encryption (ISO/TS 32003):** AES-GCM provides authenticated encryption
(integrity + confidentiality in one operation). The engine must support AES-GCM for decryption
when the `/SubFilter` indicates GCM mode. For output encryption (post-baseline), AES-GCM SHOULD be
the preferred algorithm over AES-CBC. The key derivation and per-object key computation follow the
same V5/R6 scheme but with GCM mode selection.

**Document integrity protection (ISO/TS 32004):** Document integrity dictionaries (`/DID`)
provide modification detection beyond signature `/ByteRange`. The engine must parse `/DID`
entries, verify integrity hashes when present, and surface integrity violations in
`CapabilityReport` and the compatibility ledger.

**Structure namespaces (ISO/TS 32005):** PDF 2.0 introduces namespace-qualified structure element
types. The engine must resolve role mapping chains through namespace declarations, handle standard
structure namespaces (PDF 2.0, MathML), and preserve namespace declarations during round-trip.

**Hash algorithm agility (ISO/TS 32001):** Beyond SHA-256/384/512, support SHA-3 and SHAKE
families when specified in signature or integrity dictionaries. The `CryptoProvider` trait must
accept algorithm identifiers from the ISO/TS 32001 registry.

**Associated files (AF relationships):** PDF 2.0 `/AF` dictionaries associate files with any
object, not only the document catalog. The catalog crate must parse `/AF` arrays, track
relationship types (`/AFRelationship`), and preserve AF linkages during round-trip. The extraction
crate must enumerate all AF-linked files.

**Page-level output intents:** PDF 2.0 allows `/OutputIntents` on individual pages, not just the
document catalog. The color pipeline must check for page-level output intents before falling back
to document-level intents.

**Black point compensation:** When `/BlackPointCompensation` is `true` in a rendering intent, the
color conversion pipeline must apply BPC (scale the source black point to the destination black
point rather than mapping to absolute zero). This affects ICC profile evaluation and perceptual
shadow detail.

**Geospatial features:** PDF 2.0 measure dictionaries define coordinate system transforms for
geospatial PDFs. The engine must parse `/Measure` dictionaries, expose coordinate system metadata
through extraction, and preserve measure dictionaries during round-trip. Rendering of geospatial
annotations is Tier 2.

**Requirement handlers:** The `/Requirements` array in the catalog declares viewer capabilities
required to process the document. The engine must parse requirement dictionaries, check handler
availability, and report unsatisfied requirements in `CapabilityReport`.

### Tiered compatibility doctrine

For every feature category, Monkeybee must classify its handling:

**Tier 1 — Full native support:** The feature is implemented directly, tested against the pathological corpus, and covered by round-trip validation. This is the default target for all core PDF 2.0 features.

**Tier 2 — Safe contained handling:** The feature cannot be fully supported natively, but partial, sandboxed, or constrained handling is possible. The engine provides useful functionality without violating safety or polluting architecture. The compatibility ledger records the handling mode.

**Tier 3 — Explicit detected degradation:** The feature is detected and reported. The engine produces diagnostics, records the situation in the compatibility ledger, and degrades in principled, instrumented ways. No silent failures. No blank pages without explanation.

### Compatibility ledger

Every document processed by Monkeybee produces a compatibility report: what was encountered, how it was handled, what tier applied, what degraded, what succeeded. This ledger is machine-readable and is the backbone of the proof infrastructure.

The compatibility ledger schema is specified in Part 6 (Proof Doctrine).

### Strategic expansion lanes beyond the baseline gate

The baseline v1 gate remains the closed-loop kernel. However, several adjacent domains are too
important to leave as vague future hand-waving. Monkeybee therefore names the following strategic
lanes explicitly. They are not all baseline-gating, but they MUST appear in the scope registry,
compatibility ledger, and implementation contracts so APR/proof work can fan out coherently.

**Enterprise print production:**
- Halftone dictionaries (Types 1, 5, 6, 10, 16), spot-function evaluation, threshold-based
  screening, and explicit detection when full raster-screen simulation is unavailable.
- Transfer functions (`/TR`, `/TR2`), black generation (`/BG`, `/BG2`), and undercolor removal
  (`/UCR`, `/UCR2`) as first-class print-pipeline concerns rather than dead graphics-state fields.
- RGB-display overprint simulation, soft proofing against document or caller-supplied output
  intents, separation preview, TAC analysis, print preflight, and trap-network inspection.

**Digital signature lifecycle:**
- PAdES profile classification (B-B, B-T, B-LT, B-LTA) as an engine-visible concept, not an
  external checklist.
- DSS and VRI modeling, certificate-chain construction, OCSP/CRL evidence ingestion, TSA/RFC 3161
  timestamp handling, and creation-side CMS/PAdES emission.
- Offline long-term validation as a proof surface when the necessary material is embedded.

**Tagged PDF and accessibility audit:**
- Full recognition of standard structure element types and namespace-qualified variants.
- Attribute objects, class maps, `/ActualText`, `/Alt`, `/E`, `/Lang`, pronunciation hints,
  artifact marking, and structure destinations as extractable semantic inputs.
- PDF/UA-style audit reporting and reading-order visualization as explicit post-baseline
  capabilities, even though remediation/generation remains outside the baseline gate.

**Forms and interchange:**
- FDF/XFDF import/export, form flattening, JavaScript/submit-action inventory, signature-field
  creation, barcode-field handling, and safe-contained static-XFA flattening.

**Actions, document structure, and multimedia:**
- Full typed action inventory for the complete PDF action family, plus document link-map extraction
  for navigational actions.
- Article threads, page transitions, thumbnails, collections/portfolios, alternate presentations,
  page-piece dictionaries, and web-capture structures as parse/expose/preserve surfaces.
- Screen/sound/movie/rendition/media structures as preserve-and-inventory features under
  `PreserveButDenyExecute` by default.

**Advanced rendering quality:**
- Higher-quality resampling kernels, N-dimensional sampled-function interpolation, shading-edge
  anti-aliasing, and matte un-premultiplication precision are promoted from scattered notes to
  named rendering contracts.

**Deep correctness and hardening surfaces:**
- Redaction canary scanning must verify that redacted strings do not survive anywhere in the saved
  file, including strings, names, XMP, bookmarks, annotations, forms, attachments, and font-side
  metadata.
- Font resilience must explicitly cover CFF subroutine dependency analysis and renumbering,
  alternate-key recovery for damaged Type 1 encryption, `/FontDescriptor` flag cross-validation,
  and CIDFont vertical metrics.
- Tagged/document-structure integrity must explicitly cover RoleMap-chain termination and circular
  detection, marked-content nesting repair, page/resource-level metadata streams, web-capture
  provenance, and structure destinations.
- Parser/render hardening must explicitly cover inline-image resource leakage, deep `/Limits`
  validation for name/number trees, blend-mode preference lists, Type 4 function complexity
  analysis, stream-extent cross-validation, and `/Identity` crypt-filter no-op handling.
- Prepress/color fidelity must explicitly cover `/Trapped`, ICC version hazards, alternate image
  representations, custom spot-function cataloging, DeviceN attributes/mixing hints, and output
  intent condition identifiers.
- OCG/action/signature detail must explicitly cover named optional-content configurations, cloudy
  annotation borders, JavaScript trigger timing graphs, and certification-vs-approval signature
  classification with MDP-chain validation.

APR sequencing for these expansion lanes is explicit:

- **Wave 1 — immediate differentiators:** digital-signature creation/LTV, accessibility audit,
  enterprise prepress, FDF/XFDF plus form flattening, and full action inventory/link-map
  extraction. These lanes most directly turn Monkeybee from a general-purpose engine thesis into a
  useful engine for regulated, enterprise, and forensic workflows.
- **Wave 2 — supporting document-reality inventory:** article threads, portfolios, transitions,
  thumbnails, alternate presentations, page-piece dictionaries, web capture, and multimedia
  catalogs. These are not optional decorations; they are the surrounding preservation and
  inspection surfaces that make the first wave materially more credible.
- **Wave 3 — rendering-quality uplift:** higher-order resampling, N-dimensional interpolation,
  shading-edge anti-aliasing, and matte precision. These remain named contracts now, but their
  backend work compounds best once the first two waves have fixture coverage and stable reporting.

For APR arithmetic, `143` means `104 + 39` priority uplift families. `155` means `143 + 12`
supporting catalog/inventory lanes. `169` means `104 + 39 + 26` once the second hardening uplift
is counted alongside the first priority uplift. `181` means `169 + 12` when the broader
catalog/inventory lanes are included too. All four counts are valid if labeled; none of them
license any reduction in ambition.

The `39`-item priority uplift is itself spelled out so APR rounds do not lose arithmetic fidelity:

- original spec inventory: `104`
- print production: `+9`
- digital signature lifecycle: `+8`
- tagged PDF / accessibility: `+10`
- advanced rendering quality: `+4`
- advanced forms and interchange: `+7`
- full action catalog and link-map extraction: `+1`

That yields the working priority total of **`104 + 39 = 143` named algorithms and techniques**.
Using the current APR comparison shorthand of **FrankenTUI at `30+`**, Monkeybee's `143`-item
named inventory is nearly **5x** larger on algorithmic breadth. That comparison is not a license
for rhetorical inflation or scope reduction; it is a concise explanation for why the scope
registry, proof harness, and implementation contracts now foreground these lanes explicitly.

The second hardening uplift is also spelled out explicitly so APR rounds do not collapse these
details back into generic subsystem names:

- redaction, signatures, and active-content forensics: `+3`
  - redaction canary scanner over the entire emitted file
  - JavaScript trigger timing graph
  - certification-vs-approval signature classification with MDP-chain validation
- font resilience and text correctness: `+4`
  - CFF subroutine dependency analysis, dead-code elimination, renumbering, and bias recalculation
  - Type 1 alternate-key recovery for damaged eexec/charstring encryption
  - `/FontDescriptor` flag cross-validation against embedded font data
  - CIDFont vertical metrics (`/W2`, `/DW2`)
- structure and metadata integrity: `+5`
  - RoleMap-chain termination and circular detection
  - marked-content nesting repair and audit
  - page/resource-level metadata stream enumeration and preservation
  - web-capture provenance (`/SourceInfo`)
  - structure destinations (`/SD`)
- parser/render hardening: `+6`
  - inline-image resource leakage tolerance
  - name/number-tree `/Limits` validation and repair
  - blend-mode preference-list handling
  - Type 4 function complexity analysis
  - stream-extent cross-validation
  - `/Identity` crypt-filter no-op handling
- prepress and color fidelity: `+6`
  - `/Trapped` semantics
  - ICC profile version detection and mixed-profile hazard reporting
  - alternate image representations
  - custom spot-function library/catalog
  - PDF 2.0 DeviceN attributes and mixing hints
  - output-intent condition-identifier lookup
- OCG and annotation rendering detail: `+2`
  - optional-content configuration sequences (`/Configs`)
  - cloudy annotation border effects (`/BE`)

That yields the second working APR total of **`104 + 39 + 26 = 169` named algorithms and
techniques** on the priority-plus-hardening track, or **`181`** when the broader
catalog/inventory lanes are counted too. Using the same **FrankenTUI at `30+`** shorthand,
Monkeybee's `169`-item framing is well beyond **5x** the named algorithmic breadth.

For APR rounds, demos, and public-proof compression, the highest-signal additions to foreground are:

1. **PAdES digital-signature creation plus long-term validation** because it turns the engine into
   an immediately useful legal/regulatory workflow tool rather than a passive signature-preserving
   reader.
2. **PDF/UA accessibility auditing** because the compliance surface is increasingly mandated and
   yields clear report artifacts for proof and press.
3. **Enterprise print-production coverage** because ink coverage, soft proofing, separation
   preview, preflight, and trap networks are where enterprise prepress value concentrates.
4. **FDF/XFDF round-trip and form flattening** because they create obvious utility for
   government-form, benefits, and regulated-document interchange.
5. **Full action catalog and link-map extraction** because they strengthen the forensics/security
   narrative by showing every action family a document can trigger or reference.

For the second hardening pass, the highest-signal additions to foreground are:

1. **Redaction canary scanning across the entire emitted file** because real redaction failures
   usually survive in metadata, names, attachment labels, form values, or font-side objects rather
   than in the visible content stream alone.
2. **CFF subroutine recompilation plus damaged-Type-1 recovery** because correct embedded-font
   repair/subsetting is rare and technically legible in proof artifacts.
3. **JavaScript trigger timing graphs** because they surface document-open/page/annotation/form/
   print/save action timing without executing hostile code.
4. **Certification-vs-approval signature classification with MDP-chain validation** because it
   turns vague DocMDP support into a clear trust-policy and post-signing-forensics story.
5. **DeviceN/ICC/output-intent/trapped hazard reporting** because it shows the engine understands
   real press semantics rather than generic RGB rendering only.

---

## Part 3 — System architecture

### Workspace layout

`monkeybee` is a dedicated facade crate at `crates/monkeybee/` and the only semver-stable public library crate.
`monkeybee-diff` is an implementation crate that owns structural/text/render/save-impact comparison; `monkeybee` re-exports the stable diff API.
`monkeybee-signature` is an implementation crate that owns signature dictionaries, byte-range maps,
DocMDP/FieldMDP policy, timestamp/trust metadata, verification plumbing, and write-impact classification.
All other workspace crates are implementation crates unless explicitly re-exported by `monkeybee`.
The workspace layout is not itself the public API contract.

The high-value expansion lanes are intentionally cross-cutting rather than siloed into one
"enterprise" crate. Prepress support spans `monkeybee-render`, `monkeybee-validate`, and
`monkeybee-extract` on top of shared color/page-state machinery. PAdES/DSS/VRI/signing spans
`monkeybee-signature`, `monkeybee-write`, and `monkeybee-forms`. Accessibility auditing spans
`monkeybee-extract` and `monkeybee-validate`. Action, portfolio, article-thread, and multimedia
inventory spans `monkeybee-catalog`, `monkeybee-extract`, and `monkeybee-forensics`. This is
deliberate: the thesis is one engine with reusable substrate and shared semantics, not a pile of
feature silos.

Monkeybee is a Cargo workspace with six explicit strata:
1. **Byte/revision layer** — immutable source bytes plus appended revisions.
2. **Persistent substrate/query layer (`monkeybee-substrate`)** — content-addressed roots,
   structural sharing, temporal lineage, invariant certificates, and dependency-tracked query
   recomputation. This is the computational kernel that makes snapshot/diff/undo/invalidation
   claims concrete.
3. **Syntax/COS layer (`monkeybee-syntax`)** — immutable parsed objects, token/span provenance,
   xref provenance, object-stream membership, raw formatting retention, and repair records.
   This is the preservation boundary.
4. **Semantic document layer (`monkeybee-document`)** — resolved page/resource/object graph built
   from syntax snapshots and substrate roots; it owns semantic meaning, not raw-byte fidelity.
5. **Content layer** — parsed content-stream IR and interpreter shared by render/extract/inspect/edit.
6. **Facade/report layer** — `monkeybee` (stable public API), `monkeybee-diff`, `monkeybee-signature`, and `monkeybee-cli`.

`monkeybee-core` is intentionally small; it provides shared primitives rather than becoming a god crate.
`monkeybee-syntax` is intentionally dumb but durable: it preserves what the parser saw and what the
repair engine inferred, without forcing the semantic layer to own raw syntax detail.
All open paths operate on a `ByteSource` trait so the architecture supports mmap files, in-memory buffers, and future range-backed sources.


`monkeybee-substrate` is intentionally not a god crate. It does not own PDF semantics, parsing,
rendering, or write policy; it owns the persistent computational substrate that those layers share.
`monkeybee-syntax` remains dumb but durable. `monkeybee-document` remains semantically rich. The
substrate exists so those crates do not each reinvent structural sharing, diffing, lineage, and
invalidation in slightly incompatible ways.

### Persistent incremental document substrate

Monkeybee's substrate is a content-addressed, structurally shared document store backed by
cryptographic digests (BLAKE3 in the baseline design, abstracted behind a digest trait if later
research warrants alternatives). The substrate is the concrete answer to the spec's current claim
that snapshots are "structurally shared by default."

```
pub struct NodeDigest(pub [u8; 32]);

pub enum SubstrateNode {
    RawSpan { source: ByteSpanRef },
    ParsedCos { kind: CosKind, children: Vec<NodeDigest> },
    SemanticObject { kind: SemanticKind, children: Vec<NodeDigest> },
    DerivedIndex { kind: DerivedIndexKind, children: Vec<NodeDigest> },
}

pub struct SnapshotRoot {
    pub syntax_root: NodeDigest,
    pub semantic_root: NodeDigest,
    pub preserve_root: NodeDigest,
    pub lineage_root: NodeDigest,
    pub hypothesis_set: HypothesisSetId,
    pub query_epoch: u64,
}
```

Rules of the substrate:
- leaves may refer zero-copy to original byte spans; preserve-mode raw bytes are first-class nodes,
  not side tables
- interior node digests include normalized local payload + ordered child digests
- snapshot creation is root creation, not document cloning
- diffing skips identical digests and descends only into changed subtrees
- undo is historical root selection; redo is ordinary forward movement to another root
- cross-snapshot reuse is keyed by digests and manifest-qualified view state, never by mutable
  object addresses
- every write, diff, and proof artifact may point back to substrate digests for causal explanation

This design makes several core properties natural rather than bespoke:
- O(1) historical snapshot handles
- O(changed subgraph) structural diffs and exact invalidation
- zero-copy preserve workflows for untouched spans
- digest-backed write receipts, semantic anchors, and temporal replay

### Substrate-store lifecycle doctrine

The substrate store is not an immortal heap and not a vague cache. Root
pinning, spill policy, persistence eligibility, and reclamation are explicit
contracts.

```
pub enum StoreResidency {
    MemoryOnly,
    Spillable,
    Persisted,
}

pub enum RootPinReason {
    LiveSnapshot,
    QueryMaterialization,
    WriteEvidence,
    FailureCapsule,
    ImportClosure,
}

pub enum PersistentEligibility {
    Eligible,
    SessionOnly,
    RequiresEncryptedStore,
    ForbiddenByPolicy,
}

pub struct SubstrateRootHandle {
    pub digest: NodeDigest,
    pub residency: StoreResidency,
    pub pin_reason: RootPinReason,
    pub reachable_snapshots: Vec<SnapshotId>,
    pub persistent_eligibility: PersistentEligibility,
}

pub struct StoreLifecycleStats {
    pub live_nodes: u64,
    pub pinned_nodes: u64,
    pub reclaimed_nodes: u64,
    pub spilled_nodes: u64,
    pub protected_nodes: u64,
}
```

Rules:
- live snapshots, active query materializations, write receipts/certificates,
  failure capsules, and cross-document import closures pin all reachable nodes
- reclamation is reachability-based from pinned roots and retained history, not
  best-effort eviction of still-referenced digests
- derived indexes may spill, but preserve roots and any raw spans needed for
  in-flight signature evidence or preserve-mode write planning may not be
  dropped early
- encrypted, permission-restricted, or otherwise sensitive substrate material
  may only persist beyond the session boundary if the resolved policy allows it
- store maintenance emits diagnostics and trace events, and deterministic mode
  fixes sweep/spill ordering so proof artifacts remain reproducible
- persisted substrate blobs are published via a manifest-last sequence:
  write blob -> verify digest/size -> fsync blob -> atomically publish the
  root-set or artifact manifest that references it
- crash recovery ignores or quarantines blobs that were not linked by a durable
  manifest/root-set publication; partially written persistence state may not be
  reused opportunistically

### Engine / session / snapshot model

- `MonkeybeeEngine` owns global policy: providers, caches, worker pools, oracle manifests, and security defaults.
- `OpenSession` binds a byte source and revision chain to that engine.
- `PdfSnapshot` is immutable, shareable across threads, and structurally shared by default.
  Snapshot creation must be copy-on-write / persistent-data-structure based; full-document cloning
  is a fallback of last resort, not the baseline design.
- `EditTransaction` consumes a snapshot and produces a new snapshot plus a serializable delta.


`PdfSnapshot` is therefore not a bag of mutable hash maps. It is an immutable semantic view over a
`SnapshotRoot` plus policy-qualified query handles. The user-facing `SnapshotId` remains the stable
API identity, but internally every snapshot also has a digestable substrate root that can back
replay, diff, receipts, and change attribution.

```
pub struct SnapshotLineage {
    pub snapshot_id: SnapshotId,
    pub root: SnapshotRoot,
    pub parent: Option<SnapshotId>,
    pub transaction_intent: Option<TransactionIntent>,
    pub change_digest: [u8; 32],
}

pub struct SnapshotDelta {
    pub changed_objects: Vec<ObjRef>,
    pub changed_node_digests: Vec<NodeDigest>,
    pub invalidated_queries: Vec<QueryKey>,
    pub preserved_properties: Vec<PreservationClaim>,
}
```

`CapabilityReport` is an early open artifact: it is produced during `OpenProbe` (before full open)
and refined after full parse. It is not merely a workflow promise.

```
pub struct CapabilityReport {
    pub signed: bool,
    pub signature_summary: SignatureSummary,
    pub encrypted: bool,
    pub tagged: bool,
    pub structure_complexity: Option<StructureComplexity>,
    pub structure_edit_risk: Option<StructureEditRisk>,
    pub has_xfa: bool,
    pub has_javascript: bool,
    pub risky_decoder_set: Vec<DecoderType>,
    pub edit_safety: EditSafetyClass,
    pub save_constraints: SaveConstraintReport,
    pub preserve_constraints: Vec<PreserveConstraint>,
    pub expected_degradations: Vec<FeatureCode>,
    pub recovery_confidence: RecoveryConfidence,
    pub ambiguity_count: u32,
    pub active_content: ActiveContentReport,
    pub substrate_digest: [u8; 32],
    pub temporal_revision_depth: u32,
    pub hypothesis_summary: Option<HypothesisSetSummary>,
    pub semantic_surface: Option<SemanticSurfaceSummary>,
    pub prepress_summary: Option<PrepressSummary>,
    pub accessibility_summary: Option<AccessibilitySummary>,
    pub forms_summary: Option<FormInterchangeSummary>,
    pub action_inventory_summary: Option<ActionInventorySummary>,
    pub rich_structure_summary: Option<RichStructureSummary>,
}

pub struct SaveConstraintReport {
    pub doc_mdp: Option<DocMdpPolicy>,
    pub field_mdp: Vec<FieldMdpPolicy>,
    pub encrypt_permissions: Option<PermissionBits>,
    pub allowed_incremental_ops: Vec<SaveOperationKind>,
    pub blocked_ops: Vec<BlockedSaveOperation>,
    pub signature_impact: SignatureImpactReport,
}

pub struct HypothesisSetSummary {
    pub set_id: HypothesisSetId,
    pub candidate_count: u32,
    pub chosen: Option<RecoveryCandidateId>,
    pub unresolved_material_ambiguities: u32,
}

pub struct SemanticSurfaceSummary {
    pub has_layout_graph: bool,
    pub has_semantic_anchors: bool,
    pub anchor_policy: AnchorStabilityPolicy,
}

pub enum PadesLevel {
    BB,
    BT,
    BLT,
    BLTA,
}

pub struct SignatureSummary {
    pub signature_count: u32,
    pub pades_levels_present: Vec<PadesLevel>,
    pub has_document_timestamp: bool,
    pub has_dss: bool,
    pub vri_entry_count: u32,
    pub offline_ltv_ready_count: u32,
}

pub struct PrepressSummary {
    pub output_intents: Vec<OutputIntentRef>,
    pub page_output_intent_count: u32,
    pub halftone_types: Vec<HalftoneType>,
    pub has_transfer_functions: bool,
    pub has_bg_ucr: bool,
    pub has_overprint_sensitive_content: bool,
    pub tac_risk: Option<TacRiskClass>,
    pub low_dpi_asset_count: u32,
    pub bleed_risk_count: u32,
    pub separation_names: Vec<String>,
    pub trap_network_count: u32,
}

pub struct AccessibilitySummary {
    pub standard_roles_seen: Vec<String>,
    pub has_actual_text: bool,
    pub has_alt_text: bool,
    pub has_expansion_text: bool,
    pub has_language_spans: bool,
    pub has_pronunciation_metadata: bool,
    pub artifact_span_count: u32,
    pub figure_without_alt_count: u32,
    pub heading_hierarchy_findings: u32,
    pub table_header_findings: u32,
    pub pdfua_findings: Vec<PdfUaFinding>,
}

pub struct FormInterchangeSummary {
    pub has_xfa: bool,
    pub can_import_fdf: bool,
    pub can_export_fdf: bool,
    pub can_import_xfdf: bool,
    pub can_export_xfdf: bool,
    pub flatten_ready: bool,
    pub javascript_hook_count: u32,
    pub submit_target_count: u32,
    pub barcode_field_count: u32,
    pub signature_field_count: u32,
}

pub struct ActionInventorySummary {
    pub total_actions: u32,
    pub actions_by_kind: Vec<ActionKindCount>,
    pub navigation_target_count: u32,
    pub link_map_edge_count: u32,
    pub execute_capable_action_count: u32,
    pub rich_media_action_count: u32,
    pub execute_deny_findings: u32,
}

pub struct RichStructureSummary {
    pub article_thread_count: u32,
    pub bead_count: u32,
    pub page_transition_count: u32,
    pub thumbnail_count: u32,
    pub portfolio_item_count: u32,
    pub alternate_presentation_count: u32,
    pub page_piece_entry_count: u32,
    pub web_capture_count: u32,
    pub multimedia_item_count: u32,
    pub rendition_tree_count: u32,
}
```

### Open probe contract

Before full open, the engine may perform an `OpenProbe`:

```
probe = engine.probe(byte_source, probe_opts, &exec_ctx)?;
```

`OpenProbe` is bounded, cheap, and the default preflight for viewer/editor/CLI flows.
`engine.open()` SHOULD accept a prior probe to avoid duplicate work.
It may inspect:
- header and declared version
- tail region (`startxref`, `%%EOF`, update depth estimate)
- linearization dictionary and first-page hint presence
- encryption dictionary presence
- signature field presence and `/ByteRange` inventory
- `/Catalog` feature hints (AcroForm, XFA, StructTreeRoot, OCGs, JavaScript, `/OutputIntents`,
  `/Collection`, `/Threads`, presentation metadata, and rich-media roots when cheaply knowable)
- likely risky decoder set
- approximate page count / object count when cheaply knowable

`OpenProbe` returns a preliminary `CapabilityReport`, an estimated complexity class,
a recommended `OperationProfile`, an optional `PreliminaryAccessPlan`, and any `RecoveryCandidateSummary` records that
can be determined cheaply.

Expansion-lane summaries in `CapabilityReport` are allowed to be partial at probe time. Their
contract is to surface useful early truth, not to pretend bounded probing can fully validate
PAdES/LTV readiness, PDF/UA findings, TAC risk, form interchange coverage, or multimedia topology
without the later full-open pass.

`AccessPlan` is a reusable artifact for lazy/remote sessions. It records:
- page -> object/resource dependency closure
- critical byte ranges for first paint
- linearization-derived page hints when available
- fallback xref-derived byte ranges when linearization is absent or damaged
- viewport-priority fetch groups for region rendering

The facade exposes:

```
engine.open_with_candidate(byte_source, open_options, candidate_id, &exec_ctx)
```

`engine.open(...)` may accept a prior probe result to avoid duplicate work.


### Hypothesis set and candidate collapse contract

Ambiguous tolerant parsing is promoted from a diagnostic side note to a first-class artifact.
Monkeybee maintains a bounded `HypothesisSet` whenever materially different repairs survive initial
probe/open analysis.

```
pub struct HypothesisSet {
    pub set_id: HypothesisSetId,
    pub candidates: Vec<HypothesisCandidate>,
    pub collapse_policy: HypothesisCollapsePolicy,
}

pub struct HypothesisCandidate {
    pub candidate_id: RecoveryCandidateId,
    pub snapshot_root: SnapshotRoot,
    pub confidence: f64,
    pub evidence: Vec<HypothesisEvidence>,
}

pub struct HypothesisEvidence {
    pub source: String,
    pub score_delta: f64,
    pub reason: String,
}
```

Baseline v1 behavior remains practical:
- `engine.open()` returns the highest-confidence candidate plus full alternative summaries
- `engine.open_with_candidate(...)` can force a specific candidate for forensic or debugging work
- all receipts and ledgers for ambiguous files record the originating hypothesis set ID
- if a caller selects a policy that forbids unresolved ambiguity, the open is rejected explicitly

Post-v1, deeper parts of the stack may choose to carry multiple live candidates longer, but the
current architectural requirement is simpler and stricter: ambiguity must remain observable,
serializable, and auditable all the way to the proof artifacts.

### Complexity fingerprint and admission contract

`OpenProbe` MUST emit a deterministic `ComplexityFingerprint` and an
`AdmissionDecision` in addition to the preliminary `CapabilityReport`.

The goal is not merely "can I open this file?" but "what class of file is this,
what runtime envelope does it deserve, and what degradation/risk surface should
the caller expect before committing to full open or expensive downstream work?"

```
pub enum ComplexityClass {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Pathological,
}

pub struct ComplexityFingerprint {
    pub object_count_estimate: Option<u64>,
    pub page_count_estimate: Option<u32>,
    pub incremental_depth_estimate: u32,
    pub stream_density_score: u32,
    pub font_complexity_score: u32,
    pub transparency_complexity_score: u32,
    pub structure_complexity_score: u32,
    pub signed_coverage_ratio: Option<f32>,
    pub remote_first_paint_bytes_estimate: Option<u64>,
    pub risky_decoder_set: Vec<DecoderType>,
    pub active_content_score: u32,
}

pub struct BudgetRecommendation {
    pub parse_budget: ResourceBudgets,
    pub render_budget: ResourceBudgets,
    pub extraction_budget: ResourceBudgets,
    pub preferred_security_profile: SecurityProfile,
}

pub enum AdmissionDecision {
    Admit {
        class: ComplexityClass,
        recommended_profile: OperationProfile,
        budget: BudgetRecommendation,
    },
    AdmitDegraded {
        class: ComplexityClass,
        recommended_profile: OperationProfile,
        budget: BudgetRecommendation,
        expected_degradations: Vec<FeatureCode>,
    },
    Reject {
        reason: AdmissionReason,
        safe_probe_artifacts: Vec<ProbeArtifactRef>,
    },
}

pub enum AdmissionReason {
    BudgetHopeless,
    ActiveContentPolicyBlocked,
    PasswordRequired,
    AmbiguousRecoveryBlocked,
    TransportIntegrityFailed,
    UnsupportedCriticalFeature,
}
```

### Policy-aware plan selection contract

Admission is not execution. Whenever Monkeybee has multiple legal strategies
for the same operation, it computes a bounded candidate set, filters it through
the resolved policy, and records why one plan was selected.

```
pub enum PlanKind {
    OpenStrategy,
    RenderBackend,
    SaveStrategy,
    CrossDocumentImport,
    QueryAcceleration,
    RecoveryCollapse,
}

pub struct CostEnvelope {
    pub estimated_bytes_read: u64,
    pub estimated_peak_memory: u64,
    pub estimated_cpu_units: u64,
    pub estimated_latency_ms: u64,
    pub expected_degradations: Vec<FeatureCode>,
}

pub struct RejectedPlan {
    pub label: String,
    pub reason: String,
}

pub struct PlanCandidate {
    pub kind: PlanKind,
    pub label: String,
    pub policy_digest: [u8; 32],
    pub required_capabilities: Vec<FeatureCode>,
    pub predicted_preservation: Vec<PreservedProperty>,
    pub cost: CostEnvelope,
}

pub struct PlanSelectionRecord {
    pub plan_kind: PlanKind,
    pub chosen: String,
    pub rejected: Vec<RejectedPlan>,
    pub policy_digest: [u8; 32],
    pub reason: String,
    pub trace_digest: [u8; 32],
}
```

Rules:
- only policy-valid candidates are allowed to enter scoring; candidates rejected
  by composition rules are recorded but never half-executed
- correctness and preservation obligations outrank cost; cost breaks ties only
  among candidates that satisfy the same safety and fidelity class
- deterministic mode fixes candidate ordering and tie-break behavior so the same
  inputs yield the same `PlanSelectionRecord`
- `AdmissionDecision`, `WritePlan`, cross-document import, backend selection,
  and acceleration-index materialization may cite the selected plan record by
  digest in receipts, ledgers, and failure capsules
- if no candidate survives policy filtering, the operation fails explicitly
  before mutation or byte emission

### Render determinism class contract

Render planning is not only about backend choice; it is also about what kind of
determinism the caller is buying.

```
pub enum RenderDeterminismClass {
    ProofCanonical,
    BackendDeterministic,
    ViewAdaptive,
    Experimental,
}
```

Rules:
- every render backend selection emits a `RenderDeterminismClass` into
  `RenderReport`, plan-selection evidence, benchmark witnesses, and oracle
  disagreement artifacts
- `ProofCanonical` is the only class allowed for canonical proof/render-hash
  evidence; it uses pinned providers, the baseline auditable render path,
  stable tile ordering, and disables display-adaptive behavior such as ambient
  font lookup or subpixel LCD variation
- `BackendDeterministic` means output is stable for a pinned backend/provider/
  module manifest tuple, but cross-host byte-identical output is not claimed
- `ViewAdaptive` is allowed to use GPU selection, display-aware subpixel
  policies, or other host-adaptive behavior; it is judged by perceptual witness
  metrics rather than raw hash equality
- `Experimental` is never release-gating and must always retain an auditable
  downgrade path to a non-experimental class
- cache namespaces and benchmark evidence must distinguish render determinism
  classes so proof-canonical artifacts are never silently mixed with
  viewer-adaptive ones

### Fault containment doctrine

Execution failures are contained to explicit fault domains rather than being
allowed to poison the whole engine:
- operator / content-span failures
- tile / page failures
- query materialization and acceleration-index failures
- native bridge invocations
- transport sessions and fetch epochs
- save/commit publication
- proof-fixture execution

Rules:
- a contained fault may degrade the local result, but it may not silently taint
  a committed snapshot, a clean cache entry, or a durable artifact manifest
- failed query/index materializations may be retried or remain dirty, but they
  must never be surfaced as clean reusable artifacts with incomplete provenance
- native crashes, timeouts, or quarantine kills are contained to their own
  invocation/region and surface as typed outcomes plus attested diagnostics;
  they do not invalidate sibling pages, sessions, or already-durable evidence
- transport-consistency failures degrade or freeze only the affected remote
  session/fetch epoch; they do not contaminate local snapshots or unrelated
  sessions
- save failures may not mutate the source snapshot or destination path, and may
  not publish receipts/manifests that reference uncommitted bytes
- proof runs may classify a fixture as failed, panicked, or quarantined, but
  that fixture's failure must not suppress artifact generation or accounting for
  the other fixtures in the run unless an explicit global fail-fast policy says
  otherwise

**API surface:**

```
engine = MonkeybeeEngine::new(config)?;
session = engine.open(byte_source, open_options)?;         // parses, produces first snapshot
snapshot = session.current_snapshot();                      // Arc<PdfSnapshot>, cheap clone

// Read operations (parallel-safe on snapshot):
rendered = snapshot.render_page(page_index, render_opts, &exec_ctx)?;
text = snapshot.extract_text(page_index, extract_opts, &exec_ctx)?;
info = snapshot.inspect_object(obj_ref)?;

// Mutation:
tx = EditTransaction::new(snapshot.clone());
tx.add_annotation(page_index, annotation)?;
tx.set_metadata("Title", "New Title")?;
new_snapshot = tx.commit()?;                                // produces new snapshot, delta

// Write:
plan = WritePlan::compute(&new_snapshot, write_mode)?;      // classify, check signatures
bytes = plan.execute(&engine, &exec_ctx)?;                  // serialize to bytes
```

This API ensures:
- No mutable access to live snapshots (all mutation goes through EditTransaction)
- Read operations are parallelizable
- Write planning is inspectable before committing bytes
- The engine's caches survive across snapshots (keyed by snapshot_id)

### Outcome discipline

Public operations return `OperationOutcome<T>` rather than raw `Result<T, E>`:

```
pub type OperationOutcome<T> = Outcome<OperationSuccess<T>, MonkeybeeError>;
```

Operations that can be cancelled return `Outcome<T, E>` rather than `Result<T, E>`.
The four-valued Outcome distinguishes:
- `Ok(T)` — operation succeeded with full result
- `Err(E)` — domain error (malformed PDF, unsupported feature, validation failure)
- `Cancelled(CancelReason)` — operation was cancelled (viewport change, user abort,
  budget exhaustion, shutdown). Partial results may be available.
- `Panicked(PanicPayload)` — unrecoverable failure (native decoder crash, bug).
  Must be surfaced to supervision/diagnostics, never silently swallowed.

The severity lattice is `Ok < Err < Cancelled < Panicked`. Aggregation across joins,
races, retries, and supervision is monotone: the aggregate outcome is the most severe
child outcome.

`CancelReason` carries structured kind: `User`, `Timeout`, `FailFast`, `ParentCancelled`,
`Shutdown`, `BudgetExhausted`. These map to different retry, diagnostic, and supervision
policies.

At FFI boundaries (C API, WASM bindings), `OperationOutcome<T>` may be collapsed
to a `Result`-shaped representation with explicit cancellation/panic tags.
Within the Rust API, `OperationOutcome<T>` is preserved.

### Session lifecycle regions

The monkeybee facade models session lifecycle as asupersync regions:

| Lifecycle concept | Region model |
|---|---|
| `engine.open()` | Creates a session region (child of caller's region) |
| `snapshot.render_page()` | Creates a render region (child of session, deadline budget) |
| `snapshot.extract_text()` | Creates an extract region (child of session) |
| `EditTransaction` scope | Creates a transaction region (child of session, tighter budget) |
| `WritePlan.execute()` | Creates a write region (child of session) |
| Native decoder invocation | Creates a quarantine region (FailFast policy, tight budget) |
| Progressive tile batch | Creates a tile region (CollectAll policy — partial results acceptable) |

Region ownership guarantees:
- Closing a session cancels all child operations and waits for quiescence.
- Cancelling a render cancels only that render's tiles, not sibling operations.
- Native decoder panics are contained in the quarantine region and surface as
  `Outcome::Panicked` to the parent, not as a process crash.
- Budget tightening is automatic: a child region inherits the tighter of its own
  budget and its parent's remaining budget via `Budget.combine()` (meet semiring).

```
pub struct RenderResult {
    pub pixels: RasterSurface,
    pub report: RenderReport,
}

pub struct RenderReport {
    pub render_determinism_class: RenderDeterminismClass,
    pub degraded_regions: Vec<RegionRef>,
    pub placeholder_regions: Vec<PlaceholderRef>,
    pub missing_resources: Vec<ResourceKey>,
    pub substituted_fonts: Vec<FontSubstitution>,
    pub budget_events: Vec<BudgetEvent>,
}

pub struct ExtractResult {
    pub surface: PhysicalText | LogicalText | TaggedText,
    pub report: ExtractReport,
}

pub struct ExtractReport {
    pub unmappable_spans: Vec<TextGap>,
    pub substituted_fonts: Vec<FontSubstitution>,
    pub degraded_regions: Vec<RegionRef>,
}
```

### Library API result contract

Every public API that processes PDF data returns `OperationOutcome<T>` where:

- `Err(MonkeybeeError)` indicates a fatal failure — the operation did not produce a usable result.
  Examples: file cannot be opened, decryption fails with wrong password, no valid xref found
  even after repair.
- `Ok(result)` indicates the operation completed. The result may include degradations.

Successful operations carry diagnostics and an operation-specific report alongside
the primary value:

```
pub struct OperationSuccess<T> {
    pub value: T,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,      // true if any Error-severity diagnostics were emitted
    pub has_warnings: bool,    // true if any Warning-severity diagnostics were emitted
    pub report: Option<OperationReport>,
    pub budget_summary: BudgetSummary,
    pub cache_summary: CacheSummary,
}

pub enum OperationReport {
    Probe(CapabilityReport),
    Render(RenderReport),
    Extract(ExtractReport),
    Write(WriteReport),
    Diff(DiffReport),
}
```

API methods return `OperationOutcome<T>`. The caller can:
1. Check `result.has_errors` to detect degraded results
2. Inspect `result.diagnostics` for specific degradation details
3. Ignore diagnostics entirely if they only care about the primary value

The `DiagnosticSink` on `ExecutionContext` receives diagnostics in real time during processing.
The `OperationSuccess` collection is the post-hoc summary. Both exist because different callers
have different needs: a viewer wants real-time progress; a batch tool wants a summary.

**Error coarsening rule:** Subsystem-specific errors (ParseError, RenderError, WriteError) are
wrapped in `MonkeybeeError` at API boundaries. Within a crate, functions may use crate-specific
error types. At the public API surface, everything is `MonkeybeeError`. This prevents leaking
internal error types across crate boundaries.

All caches, proofs, and invalidation logic key off `snapshot_id`, never mutable in-place document state. This lifecycle model makes page-parallel render/extract, stable PagePlan caches, preserve/incremental save correctness, reproducible proof artifacts, and future viewer/editor use all straightforward because nothing mutates in place.



### Incremental query engine doctrine

The current spec already names the major caches. The refinement here is that those caches are not
conceptually seven unrelated hash maps; they are materialized query families over the persistent
substrate.

Every derived artifact in Monkeybee — resolved resources, page plans, rendered tiles, extracted
surfaces, semantic graphs, write plans, diff reports, and proof receipts — is modeled as a query
with declared dependencies and memoized results.

```
trait QueryEngine {
    fn get<Q: QuerySpec>(&self, key: Q::Key, ctx: &ExecutionContext) -> Q::Value;
}

fn resolved_resources(page_index: u32) -> ResolvedResources;
fn page_plan(page_index: u32, mode: PagePlanMode) -> PagePlan;
fn rendered_tile(page_index: u32, tile: TileId, profile: RenderProfile) -> RasterTile;
fn extract_surface(page_index: u32, surface: ExtractSurfaceKind) -> ExtractSurface;
fn semantic_graph(page_index: u32, profile: ExtractProfile) -> SpatialSemanticGraph;
fn write_plan(snapshot_id: SnapshotId, mode: WriteMode) -> WritePlan;
fn diff(a: SnapshotId, b: SnapshotId, mode: DiffMode) -> DiffReport;
```

Required properties:
- queries record the substrate digests and dependent query keys they observed
- if none of a query's transitive input digests changed, recomputation is forbidden; the cached
  result must be reused
- if any transitive input digest changed, reuse is forbidden unless the query spec explicitly
  defines a partial-reuse rule
- invalidation is driven by changed digests and graph edges, not by broad subsystem-wide flushes
- query traces are explainable through `TraceEventStream`

This is a major improvement over the current plan because it converts several ambitious promises —
"exact invalidation," "cache reuse across snapshots," "only rerender the touched page," "diff only
changed structure," and "history replay without reparsing the world" — into one reusable
architectural mechanism.

### Cache management doctrine

All query materializations and caches are governed by a single `CachePolicy`.

`CachePolicy` defines:
- in-memory byte budget
- spill-store byte budget
- optional persistent derived-artifact store policy
- per-cache admission rules
- pinning rules
- eviction rules
- deterministic mode behavior
- wasm/native default profiles

Every cache key belongs to a `CacheNamespace`:

```
CacheNamespace = (
  snapshot_id,
  security_profile,
  provider_manifest_id,
  determinism_class,
  view_state_hash
)
```

`view_state_hash` covers any setting that can change visible or extracted output
without changing document bytes (for example optional-content configuration,
substitute-font policy, and active-content policy).

Canonical caches:
- `ParsedObjectCache`      key=(document_id, revision_id, objref)
- `DecodedStreamCache`     key=(resource_fingerprint, filter_chain_hash)
- `ParsedFontCache`        key=(font_fingerprint)
- `PagePlanCache`          key=(cache_namespace, page_index, dependency_fingerprint, pageplan_mode_hash)
- `RasterTileCache`        key=(cache_namespace, page_index, tile_id, dpi, completeness, render_profile_hash)
- `ColorTransformCache`    key=(icc_fingerprint, intent, target_space)

- `ResolvedResourceCache`  key=(snapshot_id, page_index, inheritance_fingerprint)
- `SemanticGraphCache`     key=(cache_namespace, page_index, extract_profile_hash)
- `InvariantCertificateCache` key=(before_snapshot_id, after_snapshot_id, write_mode, proof_profile_hash)
- `TemporalReplayCache`    key=(document_id, revision_frame_id, view_state_hash)

- No cache is unbounded in any runtime, including native and WASM.
- WASM uses smaller default budgets, not different cache semantics.
- Cross-snapshot reuse is allowed only for immutable artifacts identified by fingerprints.

**Scratch spill store:** bounded local store for oversized decoded streams, raster tiles,
  isolated-decoder outputs, and other large intermediate artifacts.

**Persistent derived-artifact store:** optional disk-backed cache keyed by:
`(input_sha256, engine_version, provider_manifest, security_profile, artifact_kind)`.
Eligible artifacts:
- repaired xref index
- parsed object-stream index
- parsed font / CMap / ICC metadata
- page dependency graph
- progressive prefetch plans

Ineligible artifacts:
- raw decrypted streams
- caller-sensitive extracted text
- artifacts derived from ambiguous recovery unless explicitly allowed

The persistent store is disabled by default for encrypted inputs and for restricted corpus tiers.

When memory pressure exceeds in-memory cache budgets, eligible artifacts may spill to the
scratch store instead of being dropped outright.

Cache budgets are exposed in `ExecutionContext` so proof runs can use smaller budgets to stress
eviction paths. The cache reports hit/miss/eviction statistics through the trace/metrics sink.

When memory pressure exceeds all cache budgets, the engine degrades gracefully: re-decoding streams
on demand rather than caching, re-interpreting content streams instead of reusing PagePlans. This
degradation is instrumented (diagnostics report cache pressure events).

### Resource canonicalization and deduplication contract

```
pub struct ResourceCanonicalForm {
    pub semantic_fingerprint: ResourceFingerprint,
    pub byte_fingerprint: Option<[u8; 32]>,
    pub resource_kind: ResourceKind,
    pub decode_parameters_digest: Option<[u8; 32]>,
    pub provider_manifest_id: Option<String>,
}

pub enum DedupSafetyClass {
    ByteExact,
    SemanticEquivalent,
    AppearanceEquivalent,
    NotDeduplicable,
}

pub struct MaterializationPlan {
    pub reused_existing: Vec<ObjRef>,
    pub regenerated: Vec<ObjRef>,
    pub dedup_merged: Vec<(ObjRef, ObjRef)>,
    pub blocked_merges: Vec<BlockedMerge>,
}
```

Dedup rules:
- `ForeignPreserved` objects may not be semantically merged in preserve workflows
- decoder choice, provider manifest, and decode params participate in canonical identity
- appearance-equivalent merges are allowed only in explicit optimization transactions
- `WritePlan` and `WriteReceipt` MUST record all dedup merges and blocked merges

### Crate boundaries

#### `monkeybee-core`

Shared primitives: object IDs, geometry, errors, diagnostics, execution budgets. Intentionally minimal — does not own the full document graph or byte storage.

Key responsibilities:
- PDF object type definitions (booleans, integers, reals, strings, names, arrays, dictionaries, streams, references)
- Object identity (ObjRef: object number + generation)
- Shared coordinate geometry and transformation pipeline
- Shared error taxonomy and diagnostic types
- `ExecutionContext` definition (budgets, cancellation, providers, tracing)
- Trait definitions shared across strata (`ByteSource`, `FontProvider`, `ColorProfileProvider`, `CryptoProvider`)

The core must remain small enough that every other crate can depend on it without pulling in subsystem weight.

**PDF object type specifics:**

The object model must faithfully represent the nine fundamental PDF object types:

- **Boolean:** `true` or `false`. Trivial but must be preserved exactly for round-trip.
- **Integer:** Signed integers. PDF does not define a specific integer size; in practice, 64-bit signed covers all real-world cases. Must not silently truncate.
- **Real:** Floating-point numbers. PDF reals are expressed as fixed-point or floating-point decimal strings. The internal representation must preserve enough precision to round-trip without drift (at minimum, f64). When serializing, emit the minimum number of decimal places that preserves the value, capped at 6 decimal places for coordinates and 10 for color values.
- **String:** Two forms — literal strings (parenthesis-delimited, with escape sequences) and hexadecimal strings (angle-bracket-delimited). The internal representation stores raw bytes. Text interpretation (PDFDocEncoding vs. UTF-16BE, detected by BOM) is layered above.
- **Name:** `/`-prefixed atoms. Name objects use `#XX` hex encoding for bytes outside the printable ASCII range. The internal representation is the decoded byte sequence; the serializer re-encodes as needed.
- **Array:** Ordered heterogeneous collections. Arrays may contain any object type, including other arrays and indirect references. No inherent size limit, but the engine should enforce a configurable maximum nesting depth to prevent stack overflow on adversarial input.
- **Dictionary:** Key-value maps where keys are names and values are any object type. Duplicate
  keys are technically malformed. Handling depends on the parse mode:
  - **Tolerant mode:** Last-definition-wins. A diagnostic (`parse.object.duplicate_key`) is
    emitted with both the kept and discarded values.
  - **Strict mode:** Duplicate keys are a validation error. The parser still produces a result
    (using last-definition-wins) but the diagnostic is Error severity, which will cause
    Arlington validation to fail.
  - **Preserve mode:** Both entries are retained in the raw syntax layer
    (`monkeybee-syntax`), preserving their byte spans and ordering. The semantic layer
    (`monkeybee-document`) applies last-definition-wins when resolving the dictionary. This
    ensures that preserve-mode round-trip emits the same bytes even if duplicates exist.
- **Stream:** A dictionary plus a byte sequence. The dictionary contains at least `/Length`. The raw bytes are the encoded form; the decoded form is obtained by applying the filter chain. The core stores both the raw bytes (for preserve-mode round-trip) and provides lazy-decoded access.
- **Null:** The null object. Represents absence. References to free objects resolve to null.
- **Indirect reference:** A pair of (object number, generation number) pointing to an indirect object elsewhere in the file. The object graph resolves these on demand.

**Resource dictionary resolution chain:**

Page resources are inherited down the page tree hierarchy. A page's effective resources are determined by:
1. Check the page object's own `/Resources` dictionary.
2. If absent (or for missing individual resource categories), walk up the page tree to ancestors.
3. The first ancestor with a `/Resources` dictionary for the needed category provides it.
4. Resource categories (`/Font`, `/XObject`, `/ExtGState`, `/ColorSpace`, `/Pattern`, `/Shading`, `/Properties`) are resolved independently — a page might inherit fonts from one ancestor and XObjects from another.

The core must cache resolved resources per page to avoid repeated tree traversal.

**Coordinate geometry and transformation pipeline:**

PDF uses a bottom-left coordinate system with units of 1/72 inch (by default). The transformation pipeline is:
1. **User space → Device space:** The CTM (Current Transformation Matrix) maps from user space to device space. It is a 3×2 affine matrix [a b c d e f] applied as: x' = a·x + c·y + e, y' = b·x + d·y + f.
2. **Page boxes:** MediaBox defines the physical medium boundary. CropBox (defaults to MediaBox) defines the visible region. BleedBox, TrimBox, and ArtBox define printing-related regions. All boxes are in user space coordinates.
3. **Page rotation:** The `/Rotate` attribute (0, 90, 180, 270 degrees) is applied after the page's own coordinate system. Rendering, annotation placement, and extraction must all account for rotation.
4. **UserUnit:** PDF 1.6+ allows a `/UserUnit` value that scales the default user-space unit. A UserUnit of 2.0 means each unit is 2/72 inch instead of 1/72.
5. **Form XObject transformations:** Form XObjects have their own `/Matrix` that maps from the form's coordinate space to the enclosing content stream's space. This is composed with the current CTM.

The shared geometry module must provide: matrix multiplication, matrix inversion, point transformation, rectangle transformation (with proper handling of rotated/skewed rectangles), and rectangle intersection/union.

#### `monkeybee-bytes`

Byte sources, mmap/in-memory/range-backed access, revision chain, and raw span ownership. This is the byte/revision layer.

Key responsibilities:
- `ByteSource` trait implementations (mmap, in-memory, range-backed)
- Fetch scheduler and prefetch planning for remote/lazy byte sources
- Persistent range cache keyed by source URL + validator (ETag/Last-Modified/content hash)

### Fetch scheduler contract

The fetch scheduler mediates byte-range requests for remote or lazy byte sources:

```
trait FetchScheduler {
  /// Request bytes in the given range. Returns a future that resolves when the bytes are
  /// available in the byte source's local buffer.
  fn request_range(&self, offset: u64, length: u64) -> FetchHandle;

  /// Submit a prefetch plan (ordered list of ranges by priority). The scheduler issues
  /// requests in priority order, subject to concurrency limits. Returns immediately.
  fn submit_prefetch(&self, plan: PrefetchPlan);

  /// Cancel all outstanding requests. In-flight requests may or may not complete.
  fn cancel_all(&self);

  /// Report fetch statistics (requests issued, bytes fetched, latencies).
  fn statistics(&self) -> FetchStatistics;
}
```

**PrefetchPlan:** An ordered list of `(offset, length, priority)` tuples. The render pipeline
generates a prefetch plan by inspecting the page's resource dependencies: font programs, image
streams, form XObject streams. Resources visible in the current viewport get higher priority.

**Concurrency:** The scheduler limits concurrent HTTP range requests (default: 6, matching
browser connection limits). Requests are coalesced when ranges overlap or are adjacent within
a configurable gap threshold (default: 4096 bytes — it's cheaper to fetch a few extra bytes
than to issue a separate request).

### Remote transport integrity and sparse-availability contract

Remote sessions MUST bind to a `TransportIdentity` and maintain a
`ByteAvailabilityMap`. Range-backed correctness is not defined solely by
"did bytes arrive?" but by "did they arrive from the same logical artifact?"

```
pub struct TransportIdentity {
    pub source_fingerprint: [u8; 32],
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub content_length: Option<u64>,
    pub digest_hint: Option<[u8; 32]>,
}

pub struct ByteAvailabilityMap {
    pub epoch: FetchEpoch,
    pub available_ranges: Vec<(u64, u64)>,
    pub verified_ranges: Vec<(u64, u64)>,
    pub suspect_ranges: Vec<(u64, u64)>,
}

pub struct FetchEpoch(pub u64);

pub enum RangeConsistencyError {
    ValidatorChanged,
    ContentLengthChanged,
    OverlappingConflict,
    TruncatedBody,
    Unsupported206Semantics,
}
```

Rules:
- all range responses within a session MUST agree on validator identity
- validator drift freezes the session into explicit degraded mode
- previously verified ranges remain trusted only within the same `FetchEpoch`
- `OpenProbe`, `CapabilityReport`, and `WriteReceipt` MUST surface transport
  integrity failures when they influence correctness

**Integration with progressive rendering:** When the render pipeline encounters a stream whose
bytes are not yet available, it:
1. Records a placeholder in the tile output.
2. Returns the needed byte range to the caller.
3. The caller submits the range to the fetch scheduler.
4. When the fetch completes, the caller invalidates the affected tiles and re-renders.

- Revision chain tracking (original file + appended incremental updates)
- Raw span ownership for preserve-mode byte-perfect write-back
- No PDF-semantic understanding — purely byte-level


#### `monkeybee-substrate`

Persistent computational substrate shared across syntax, document, diff, proof, and writeback.

Key responsibilities:
- content-addressed node store for raw spans, parsed COS nodes, semantic nodes, and derived indexes
- snapshot root creation, structural sharing, and subtree-delta computation
- dependency-tracked incremental query engine
- temporal revision graph and historical snapshot materialization
- invariant certificate generation and digest-backed provenance plumbing
- bounded hypothesis-set storage and candidate lineage

`monkeybee-substrate` owns no PDF feature semantics by itself. It owns the persistent machinery
that lets higher layers express those semantics without giving up exact invalidation, cheap
snapshots, or proof-quality receipts.

#### `monkeybee-syntax`

First-class syntax/COS layer between the parser and the semantic document model. This is the preservation boundary.

Key responsibilities:
- Immutable parsed COS object representation (dictionaries, arrays, streams, names, strings, numbers, booleans, null)
- Token/span provenance: every parsed token retains its source byte range
- Xref provenance: original vs. repaired cross-reference entries
- Object-stream membership tracking (which objects lived in which object streams)
- Raw formatting retention for preserve-mode byte-perfect writeback (whitespace, comment preservation)
- Repair records: what the parser inferred, what strategy was used, and confidence scores
- Preservation boundary contract: the syntax layer preserves what the parser saw; the semantic layer above builds meaning from it

The syntax layer is intentionally "dumb but durable." It does not interpret page trees, resolve resources, or understand content streams. It holds the faithful COS-level representation that the semantic document layer builds upon and that the preserve-mode write path can emit byte-for-byte.

#### `monkeybee-document`

Semantic document graph: page tree, inherited state, resource resolution, ownership classes. This is the semantic document layer built on top of syntax snapshots.

Key responsibilities:
- Document-level model (PdfDocument, ObjectStore, PageTree)
- Cross-reference management and resolution
- Page tree with inherited attribute materialization
- Resource dictionary resolution chain
- Object ownership classification (`Owned`, `ForeignPreserved`, `OpaqueUnsupported`)
- Incremental update tracking and merge
- Change tracking and mutation model
- Reference integrity index (forward and reverse lookups)
- Dependency graph (page -> resources -> forms/patterns/xobjects/fonts/images/annotations)

### Dependency graph contract

Monkeybee maintains three related graph views:
1. `ReferenceGraph`: the raw directed object-reference graph; cycles are legal.
2. `OwnershipGraph`: semantic ownership/inheritance edges with stronger invariants.
3. `CondensedDependencyGraph`: a strongly-connected-components condensation DAG used for
   invalidation, page-impact, and resource-GC planning.

The graph is computed lazily and cached per snapshot.

**Nodes:** Every indirect object is a potential node. In practice, only objects reachable from
the page tree, catalog, or AcroForm root are tracked.

**Edges:** An edge from A to B means "A references B directly." Edge types are classified:
- `ContentRef`: page/form XObject content stream references a resource by name
- `DictRef`: dictionary value is an indirect reference
- `ArrayRef`: array element is an indirect reference
- `InheritedRef`: page inherits an attribute from an ancestor

**Queries:**
1. `dependents_of(obj_id) -> Set<ObjRef>`: all objects that transitively depend on `obj_id`
   (reverse reachability). Used for cache invalidation — editing `obj_id` invalidates all
   dependents.
2. `dependencies_of(obj_id) -> Set<ObjRef>`: all objects that `obj_id` transitively depends on
   (forward reachability). Used for resource GC — an object is garbage if no root reaches it.
3. `page_dependencies(page_index) -> Set<ObjRef>`: the full transitive closure of objects needed
   to render/extract page N. Used for PagePlan cache keying and write planning.
4. `edit_impact(changed_ids: Set<ObjRef>) -> EditImpact`: given a set of changed objects, report
   which pages are affected, which caches must be invalidated, and which other objects may need
   regeneration (e.g., widget appearances after field value change).
5. `scc_of(obj_id) -> SccId`
6. `ownership_violations() -> Vec<OwnershipViolation>`

**Computation:** The graph is built by walking the object store from known roots (catalog, page
tree nodes, AcroForm fields). Dictionary values and array elements are scanned for indirect
references. Content stream resource names are resolved against the page's resource dictionary to
find the referenced objects. The walk is bounded by the object count budget in `ExecutionContext`.

**Storage:** The graph is stored as adjacency lists (forward and reverse) in a `DashMap` for
concurrent read access. The graph is immutable per snapshot — edits produce a new snapshot with
an incrementally updated graph (only the changed subgraph is recomputed).

- Derived-artifact invalidation (PagePlan, resolved resources, decoded streams, widget appearances)


### Temporal revision graph contract

Incremental-update chains are not just a parser concern. They form a first-class temporal graph that
can be queried, replayed, and diffed.

```
pub struct RevisionFrame {
    pub frame_id: RevisionFrameId,
    pub byte_range: (u64, u64),
    pub snapshot_id: SnapshotId,
    pub parent: Option<RevisionFrameId>,
    pub appended_objects: Vec<ObjRef>,
    pub signature_state: SignatureSummary,
}

pub struct TemporalRevisionGraph {
    pub frames: Vec<RevisionFrame>,
    pub latest: RevisionFrameId,
}
```

Required behavior:
- `OpenSession` can enumerate all recoverable revision frames in order
- any historical frame can be materialized as a read-only `PdfSnapshot`
- rendering, extraction, inspection, and diffing must all be definable against historical
  snapshots, not only the latest state
- unchanged substrate nodes are reused across frames; replay is not permitted to devolve into
  blind full-document reparsing of each revision unless the file is so damaged that no stronger
  strategy survives
- proof artifacts for signed or litigated documents must be able to cite the exact frame used

### Snapshot query interface

Monkeybee should expose a typed snapshot query surface rather than forcing every advanced workflow
through raw object traversal.

```
let fonts = snapshot.query().pages(3..4).resources().fonts().collect::<Vec<_>>();

let hits = snapshot
    .query()
    .all_pages()
    .text_runs()
    .filter(|run| run.font_size > 14.0)
    .filter(|run| run.text.contains("CONFIDENTIAL"))
    .with_geometry()
    .collect::<Vec<_>>();

let history = session
    .history()
    .between(frame_a, frame_b)
    .structural()
    .semantic()
    .visual();
```

The query API is not ornamental sugar. It is the stable, typed boundary that lets viewers,
forensics tools, automation, and future agent adapters ask meaningful questions without duplicating
internal traversal logic.

### Acceleration index doctrine

The query surface is backed by explicit materialized acceleration indexes, not
incidental caches with undocumented semantics.

```
pub enum IndexKind {
    ObjectRefLookup,
    PageDependencyClosure,
    NameTreeLookup,
    TextSearch,
    AnchorSpatial,
    ActionGraph,
    ImportClosure,
    RevisionFrameLookup,
}

pub enum IndexFreshness {
    Fresh { snapshot_id: SnapshotId },
    StaleButUsable { reason: String },
    PartialRemote { missing_ranges: Vec<(u64, u64)> },
}

pub struct MaterializedIndexRef {
    pub kind: IndexKind,
    pub snapshot_id: SnapshotId,
    pub root_digest: NodeDigest,
    pub materialization_digest: [u8; 32],
    pub policy_digest: [u8; 32],
    pub freshness: IndexFreshness,
}
```

Required behavior:
- canonical index families include object lookup, page-dependency closure, name
  tree lookup, full-text search, spatial-anchor lookup, action topology,
  cross-document import closure, and revision-frame lookup
- indexes are keyed by snapshot root, query family, and `policy_digest`; they
  may not be reused across incompatible policies or provider manifests
- lazy/remote sessions may expose partial indexes, but partiality and freshness
  MUST be surfaced to the API, trace stream, and proof artifacts
- if a fresh index is unavailable, the engine may fall back to a bounded scan
  only with an explicit diagnostic and traceable reason
- invalidation remains exact: changed digests dirty only dependent indexes and
  queries, while clean indexes remain reusable

#### `monkeybee-catalog`

Document-catalog semantics that are broader than any one page and more
structured than raw COS preservation:
- outline / bookmark trees
- named destinations and destination arrays
- page labels
- name trees and number trees, including deep `/Limits` validation and repair
- viewer preferences, page mode, and page layout
- optional content configurations (`/OCProperties`, default configs, print/export states)
- embedded-file inventory and AF relationships
- article threads and bead chains
- page transitions, thumbnail images, and alternate presentations
- collection / portfolio schema, navigator state, and embedded-document relationships
- page-piece dictionaries (`/PieceInfo`) and web-capture structures
- document-level multimedia and rendition inventory rooted in the catalog or name trees

The catalog subsystem is the authoritative semantic model for these structures.
Render/extract/write/diff/validate consume it; they do not each grow ad hoc
partial models.

Catalog round-trip invariants:
- preserve sibling/child order in outline trees unless explicitly edited
- preserve page-label numbering semantics across page insertion/deletion
- preserve named-destination identity across full rewrite and incremental append
- preserve OCG/OCMD semantics unless the edit explicitly changes visibility policy

#### `monkeybee-content`

Content-stream IR and event interpreter shared by render, extract, inspect, and edit subsystems. This is the content layer.

Key responsibilities:
- Content stream parsing and operator dispatch
- Graphics state machine (single implementation shared by all consumers)
- Streaming event model for one-shot execution
- `PagePlan` IR: immutable page-scoped display list for cached/region-aware workflows
- Marked content span tracking
- Source-span provenance for content-stream-level debugging
- Consumer adapters (`RenderSink`, `ExtractSink`, `InspectSink`, `EditSink`) so downstream crates
  do not reimplement operator semantics

#### `monkeybee-codec`

Bounded byte transformations shared across parse, render, extract, and write:

Key responsibilities:
- Stream filters and predictors (FlateDecode, LZWDecode, ASCII85Decode, ASCIIHexDecode, RunLengthDecode, CCITTFaxDecode, DCTDecode, JPXDecode, JBIG2Decode, Crypt)
- Image decode/encode adapters
- Predictor logic (PNG predictors, TIFF predictor)
- Native/isolated decoder shims
- Bounded decode pipelines with explicit memory/time budgets
- Decode telemetry for proof and diagnostics

No crate outside `monkeybee-codec` may invoke risky native decoders (JBIG2, JPEG 2000, etc.) directly.

#### `monkeybee-security`

Execution-safety policy and enforcement:

Key responsibilities:
- Security profiles (`Compatible`, `Hardened`, `Strict`)
- Budget broker (memory/time/operator budgets)
- Worker isolation / kill-on-overrun for risky decoders
- Risky-decoder allow/deny policy
- Hostile-input policy enforcement
- All high-risk decode jobs and all optional native bridges execute through `monkeybee-security`
  and `monkeybee-native`, with explicit memory/time budgets and optional worker isolation

**Security-gated decoder invocation flow:**

```
Consumer (render/extract/parse) requests stream decode
  → monkeybee-codec public API
  → Checks ExecutionContext.security_profile.policy_for(decoder_type)
  → Allow: invoke decoder directly with budget
  → Isolate: spawn bounded worker, invoke decoder in worker, collect result or timeout
  → Deny: return Err with Tier 3 diagnostic, consumer handles degradation
```

The security gate is not bypassable. The internal decoder functions in `monkeybee-codec` are
`pub(crate)` — external crates cannot invoke JBIG2, JPEG 2000, Type 4 calculator, or XFA
packet handlers directly. They must go through the security-gated public API.

#### `monkeybee-native`

All optional FFI/native bridges live behind a narrow quarantine/broker crate.

Key responsibilities:
- JPX/JBIG2/ICC/FreeType and similar adapter boundaries
- brokered subprocess or isolated-worker execution for risky native paths
- native-module manifesting, version/hash attestation, and capability reporting
- translation between owned byte buffers / typed descriptors and native APIs
- crash, timeout, and resource-verdict reporting for diagnostics, witnesses, and
  failure capsules

Rules:
- no domain crate links risky native libraries directly or passes borrowed
  engine memory, live object references, or callbacks across the FFI boundary
- native jobs accept immutable typed inputs and produce size-bounded outputs plus
  typed attestations; partial writes into engine-owned state are forbidden
- risky modules default to isolated worker or brokered subprocess execution
  under `native-hardened` and `proof-canonical`; in-process use is explicit,
  audited, support-class-qualified, and never silent
- cache keys, benchmark witnesses, and failure capsules include native-module
  manifest identity whenever native code influenced the result
- a native bridge is an implementation choice, not an architectural authority:
  Monkeybee owns the public semantics, diagnostics, and proof surfaces

#### `monkeybee-parser`

Reads PDF bytes into the document model via syntax-preserving parsing with repair provenance. The parser is a structural machine: it delegates to `monkeybee-codec` for filter-chain decode/encode work and to `monkeybee-security` for risky-decoder policy and budget enforcement.

Key responsibilities:
- Lexing and tokenization
- Object parsing (all PDF object types)
- Cross-reference parsing (tables and streams, including repair)

**Lexer specifics:**

The PDF lexer must handle:

1. **Whitespace:** PDF defines whitespace as: NUL (0x00), TAB (0x09), LF (0x0A), FF (0x0C), CR (0x0D), SPACE (0x20). CR+LF is treated as a single end-of-line. The lexer must consume whitespace between tokens but preserve it for preserve-mode byte fidelity.

2. **Comments:** `%` introduces a comment that runs to end-of-line. Comments are not semantically significant but must be preserved in preserve mode.

3. **Delimiter characters:** `(`, `)`, `<`, `>`, `[`, `]`, `{`, `}`, `/`, `%`. These terminate any preceding token and begin a new syntactic element.

4. **Numeric tokens:** Integers (optional sign, digits) and reals (optional sign, digits with decimal point). The lexer must distinguish between integers and reals because some PDF operations treat them differently. Edge cases: `+0`, `-0`, `.5` (no leading zero), `5.` (no trailing digit after decimal), and very large integers that exceed i64 range (fall back to f64 representation with a diagnostic).

5. **String tokens:** Literal strings `(...)` with balanced parenthesis counting and escape sequences (`\n`, `\r`, `\t`, `\b`, `\f`, `\\`, `\(`, `\)`, `\ddd` for octal, and line continuation with `\` at end of line). Hex strings `<...>` with pairs of hex digits (a trailing odd digit is padded with 0). The lexer must handle multi-line literal strings and nested parentheses correctly.

6. **Name tokens:** `/` followed by the name bytes. `#XX` hex escape sequences within names (e.g., `/My#20Name` for the name "My Name"). The name's decoded form is a byte sequence, not necessarily valid UTF-8 (though it usually is in practice).

7. **Keyword tokens:** `true`, `false`, `null`, `obj`, `endobj`, `stream`, `endstream`, `xref`, `trailer`, `startxref`, `R` (indirect reference marker). The lexer recognizes these as distinct from general name tokens.

8. **Stream data:** Between `stream` (followed by a single EOL: LF or CR+LF, not just CR) and `endstream`. The lexer must extract exactly `/Length` bytes of stream data. The `endstream` keyword must follow immediately (possibly preceded by an EOL that is not counted in the length — this is a spec ambiguity that causes real-world issues). In tolerant mode, if `endstream` is not found at the expected position, scan forward up to 32 bytes.
- Delegation to `monkeybee-codec` for stream decompression and filter chains
- Delegation to `monkeybee-security` for risky-decoder policy and budget enforcement
- Incremental update parsing
- Encryption/decryption support (standard security handlers)
- Linearization detection and handling
- Tolerant mode: repair malformed structures, recover from common producer errors
- Strict mode: validate against spec, reject non-conforming input
- Preserve mode: retain raw byte spans for signature-safe workflows
- Content stream parsing (operators and operands)
- Parser diagnostics and error recovery metadata

**Filter chain specifics:**

Stream data may pass through multiple filters in sequence. The `/Filter` entry specifies either a single filter name or an array of filter names applied in order. The `/DecodeParms` entry (or array thereof) supplies filter-specific parameters.

- **FlateDecode:** zlib/deflate decompression. The most common filter. Parameters: `/Predictor` (PNG predictors 10-15, TIFF predictor 2), `/Colors`, `/BitsPerComponent`, `/Columns`. The predictor undoing is applied *after* deflate decompression. This is a frequent source of bugs — the predictor parameters must be correctly propagated. For PNG predictors, each row is preceded by a filter-type byte (0=None, 1=Sub, 2=Up, 3=Average, 4=Paeth).
- **LZWDecode:** LZW decompression. Less common than Flate. Parameters include `/EarlyChange` (default 1). The LZW implementation must handle the early-change variant that Acrobat uses.
- **ASCII85Decode / ASCIIHexDecode:** Text-to-binary decodings. These appear as outer wrappers when the PDF was generated for environments that cannot handle binary data. ASCII85 uses the `~>` end-of-data marker; ASCIIHex uses `>`. Both must tolerate whitespace.
- **RunLengthDecode:** Simple RLE decompression. Byte-based: 0-127 means copy next N+1 bytes; 129-255 means repeat next byte 257-N times; 128 is EOD.
- **CCITTFaxDecode:** Group 3 or Group 4 fax decompression. Parameters: `/K` (-1 for Group 4, 0 for Group 3 one-dimensional, >0 for mixed), `/Columns`, `/Rows`, `/BlackIs1`, `/EncodedByteAlign`. Common in scanned document images.
- **JBIG2Decode:** JBIG2 decompression. May reference global segments via `/DecodeParms` → `/JBIG2Globals`. The globals stream must be resolved and prepended. In hardened security mode this decoder must be isolated or disabled with explicit Tier 3 reporting.
- **DCTDecode:** JPEG decompression. The stream data is a complete JPEG file (with SOI/EOI markers). The engine must handle both baseline and progressive JPEG. The `/ColorTransform` parameter controls whether a YCbCr→RGB conversion is applied (default: yes for 3-component, no for others).
- **JPXDecode:** JPEG 2000 decompression. The stream data is a complete JP2 or J2K codestream. Color space information may come from the image's own headers or be overridden by the PDF's `/ColorSpace` entry. In hardened security mode this decoder must be isolated or disabled with explicit Tier 3 reporting.
- **Crypt:** Decryption filter for per-object encryption. Used when only specific streams are encrypted differently from the document default.

**Filter chain edge cases and failure modes:**

- **Cascaded filters:** A stream with `/Filter [/ASCII85Decode /FlateDecode]` means: first decode ASCII85 (outer), then decompress Flate (inner). The order in the array is the decoding order. A common producer bug is listing the filters in encoding order instead of decoding order — the tolerant parser should detect this (if the first filter fails, try reversing the order) and record the repair.
- **Predictor parameter mismatch:** PNG predictors require `/Colors`, `/BitsPerComponent`, and `/Columns` to be correct. If they are wrong, the unpredicted data will be garbled. The tolerant parser should: (a) try the declared parameters first, (b) if the result looks wrong (e.g., image dimensions don't match expected), try inferring parameters from the image XObject's own `/Width`, `/Height`, `/BitsPerComponent`.
- **Truncated streams:** If decompression produces fewer bytes than expected (based on the image or content dimensions), pad with zeros and record a diagnostic. Do not crash or reject the document.
- **JBIG2 global segments:** JBIG2 streams can reference global segment data stored in a separate stream (pointed to by `/DecodeParms` → `/JBIG2Globals`). The engine must resolve this reference before decompression. If the globals stream is missing, the JBIG2 decode fails; report as Tier 3.
- **DCTDecode color transform ambiguity:** When a JPEG image is decoded via DCTDecode, the `/ColorTransform` parameter controls YCbCr→RGB conversion. If absent, the default is 1 (yes) for 3-component images and 0 (no) for 1- or 4-component. Some producers set it incorrectly, leading to images with inverted or wrong colors. The tolerant parser can detect this heuristically: if the decoded image looks predominantly blue/yellow (a hallmark of missing YCbCr→RGB conversion), try toggling the color transform.
- **JPXDecode color space conflict:** A JPEG 2000 image may have an embedded ICC profile that conflicts with the PDF-level `/ColorSpace`. Per the spec, the PDF-level color space takes precedence. The engine must extract the raw component data from the JP2 and reinterpret it in the PDF-declared color space.

**Content stream parsing specifics:**

Content streams contain a sequence of operands followed by operators. The parser must:
1. Tokenize the content stream, recognizing the same object types as the main document parser (numbers, strings, names, arrays, dictionaries) plus the special inline-image syntax (`BI`...`ID`...`EI`).
2. Build operand stacks and pair them with operators. The operator determines how many preceding operands to consume.
3. Handle nested content streams: a page's content may be a single stream or an array of streams that are logically concatenated.
4. Handle `BX`/`EX` compatibility regions, where unknown operators between these markers are skipped without error.

#### `monkeybee-text`

Shared text subsystem used by render, extract, write, annotate, and inspect. This is the single source of truth for text handling across the engine.

Key responsibilities:
- Font program parsing and caching (Type 1, TrueType, OpenType/CFF, CIDFont, Type 3)
- CMap / ToUnicode handling and resolution
- Unicode fallback chain (ToUnicode -> predefined CMap -> encoding/differences -> AGL -> cmap table -> identity -> unmappable)
- PDF text decode pipeline for existing documents:
  character code -> font/CMap -> CID/glyph -> Unicode/metrics/selection primitives
- Authoring layout pipeline for emitted text:
  Unicode -> shaping/bidi/line breaking/font fallback -> positioned glyph runs
- Subsetting and ToUnicode generation for emitted PDFs
- Search, hit-testing, and selection primitives for viewer/editor workflows
- Text search index construction

All crates that need font resolution, text decoding, subsetting, layout, or search delegate to
`monkeybee-text`, but they do so through explicit decode-vs-layout APIs so existing PDF text is not
accidentally "re-shaped" during rendering or extraction.

#### `monkeybee-render`

Produces visual output from the document model.

Key responsibilities:
- Consumption of `monkeybee-content` events or `PagePlan` IR through backend adapters
- Reuse of `monkeybee-paint` primitives where page rendering and appearance composition overlap
- Text rendering via `monkeybee-text`: font lookup, encoding/CMap resolution, glyph dispatch,
  positioned-glyph realization, and Unicode-aware diagnostics
- Image rendering: inline and XObject images, color space conversion, interpolation
- Vector graphics: path construction, stroking, filling, clipping, winding rules
- Color management: DeviceRGB, DeviceCMYK, DeviceGray, CalRGB, CalGray, Lab, ICCBased, Indexed, Separation, DeviceN, Pattern
- Transparency: groups, soft masks, blend modes, isolated/knockout, alpha compositing
- Patterns: tiling patterns plus function/axial/radial shadings in the baseline;
  mesh shadings are target-qualified and may degrade explicitly until promoted by
  the scope registry.
- Graphics state: CTM, clipping, line properties, rendering intent, and overprint
  state tracking; full OPM=1 semantics follow the support-class/scope-registry table.
- Prepress-oriented render modes: RGB-display overprint simulation, soft proof against
  output intents or caller-supplied ICC profiles, process/spot separation preview, and
  TAC accumulation hooks shared with validation and diagnostics.
- Print-oriented color hooks: transfer-function application, halftone/spot-function
  evaluation hooks, black-generation/undercolor-removal evaluation hooks, and explicit
  diagnostics when the active backend cannot realize the full print pipeline natively.
- Rendering-quality uplift points: higher-quality resampling kernels (Lanczos /
  Mitchell-Netravali), N-dimensional sampled-function interpolation, shading-edge
  anti-aliasing, and robust matte un-premultiplication.
- Page rendering: media box, crop box, bleed/trim/art boxes, rotation, user unit
- Optional content (layers): OCG visibility, OCMD membership, default/print/export states
- Output targets: raster (PNG/JPEG), vector (SVG), region render, thumbnail render, and extensible backend interface
- Render backend selections always advertise a `RenderDeterminismClass`; proof-canonical output,
  backend-deterministic output, and viewer-adaptive output are distinct evidence classes

**Output backend architecture:**

The renderer is backend-agnostic: it consumes the shared content interpreter's events or `PagePlan`
and emits drawing commands to an abstract backend trait. This enables multiple output formats from a single interpretation pass.

*Raster backend (PNG/JPEG):* Renders through a tile/band surface abstraction.
Full-page RGBA output is one sink, not the only working set.
Region render, thumbnail render, and remote-first first-paint reuse the same tile scheduler,
dependency tracking, and caches.
Anti-aliasing is applied during rasterization (not as a post-process). JPEG output converts from RGBA to RGB (discarding alpha — JPEG does not support transparency) and encodes with configurable quality. PNG output preserves alpha and uses maximum compression.

*SVG backend:* Translates PDF drawing operations to SVG elements. Text is emitted as `<text>` elements with explicit positioning (not as paths, unless the font is not embeddable or the text uses unusual rendering modes). Images are embedded as base64 data URIs. Clipping paths use SVG `<clipPath>` elements. Transparency uses SVG opacity and filter elements. The SVG output is useful for web embedding and as a debugging tool (SVG structure mirrors PDF content stream structure).

*Extensible backend trait:* The backend trait defines methods for: begin/end page, draw path (with fill/stroke/clip mode), draw text run (positioned glyphs), draw image (positioned pixel data), push/pop graphics state, set clipping path, begin/end transparency group. Third-party backends (e.g., a GPU-accelerated backend, a print spooler backend, or a PDF-to-PDF transformation backend) can implement this trait without modifying the core renderer.

**Page rendering specifics:**

A page is rendered by:
1. Determine the effective page boxes (MediaBox, CropBox after inheritance resolution).
2. Apply the page rotation (`/Rotate`). Rotation is applied as a coordinate transform: for 90° rotation, the rendering origin shifts and axes swap. The content stream itself is not rotated — the rendering pipeline applies the rotation.
3. Apply UserUnit scaling if present (multiply all dimensions by the UserUnit value).
4. Initialize the graphics state: CTM to the identity (plus any rotation/scaling from steps 2-3), clip to CropBox, color to DeviceGray black, line width 1.0, and all other state parameters to their defaults (per ISO 32000-2 Table 52).
5. If the page has multiple content streams (an array of stream references), concatenate them
   logically with a space separator. Specifically:
   - Decode each stream independently through its filter chain.
   - Concatenate the decoded bytes with a single SPACE (0x20) byte between each stream.
   - The space separator prevents token merging: without it, the last token of stream N and the
     first token of stream N+1 could merge into a single malformed token.
   - The graphics state is NOT reset between streams. A `q` in stream 1 can be matched by `Q`
     in stream 2. Fonts, colors, and all other state persist across stream boundaries.
   - The content stream array may contain null references (some producers leave gaps). Null
     entries are skipped without inserting a separator.
   - An empty content stream array means the page has no content (blank page). This is legal.
   - A single stream reference (not in an array) is treated identically to a one-element array.
6. Interpret the concatenated content stream through the shared graphics state machine and emit
   commands into a tile/band scheduler that can materialize either a full page or only the requested region.
7. Render annotations on top of the page content (annotations are painted after the page's content stream, in the order they appear in the page's `/Annots` array).
8. Apply optional content visibility: if the document has optional content groups (OCGs / layers), evaluate the visibility of each marked content span based on the current OCG state (default, print, or export configuration). Invisible content is skipped during rendering.

### Cooperative cancellation in rendering

The render pipeline checks `exec_ctx.checkpoint.check()` at the following checkpoints. In
asupersync-native callers, this delegates to `cx.checkpoint()` which is budget-aware,
trace-aware, and scheduler-cooperative in a single call. In WASM or standalone callers, it
checks a simple AtomicBool cancellation token.

1. **Per-operator:** After each content stream operator dispatch. This is the finest granularity and
   ensures that even a single pathological operator (e.g., a huge mesh shading) can be interrupted.
2. **Per-tile/band:** Before materializing each tile in the tile/band scheduler. A cancelled tile
   produces a placeholder (transparent or diagnostic-colored region).
3. **Per-page:** Before starting each page in a multi-page render. Already-completed pages are
   retained; the cancelled page and subsequent pages are skipped.
4. **Per-resource:** Before decoding each image or font resource. Large JPEG 2000 or JBIG2 decodes
   are interruptible at the codec level (the decode pipeline checks cancellation between data blocks).

When cancellation fires, the render pipeline returns `Outcome::Cancelled(reason)` with partial
result metadata indicating which pages/tiles completed and which were cancelled. The partial result
is usable (not corrupted). The `CancelReason` carries the specific cause: `User`, `Timeout`,
`BudgetExhausted`, `ParentCancelled`, or `Shutdown`.

Budget enforcement uses the same checkpoints: if the operator count, memory, or time budget is
exceeded, the effect is identical to cancellation with `CancelReason::BudgetExhausted`. The
diagnostic carries the specific budget that was exhausted.

asupersync's three-lane scheduler (cancel > timed > ready) ensures that cancellation cleanup
gets scheduler priority over new work. This is critical for viewport-change cancel storms in
progressive rendering: cancelled tiles drain before new viewport tiles begin.

**Optional content (layers) handling:**

PDF optional content allows content to be organized into groups that can be shown or hidden. This is used for: language variants, CAD drawing layers, watermarks, print-only or screen-only content.

- `/OCProperties` in the document catalog defines the optional content configuration.
- Individual content streams use `BDC /OC /OptionalContentGroupName` ... `EMC` to mark content belonging to a group.
- The `/OCGs` array lists all optional content groups.
- The `/D` (default) configuration specifies: `/ON` (visible groups), `/OFF` (hidden groups), `/Order` (UI display order), `/AS` (auto-state rules for print/export).
- The `/Configs` array may define additional named viewing configurations (for example screen,
  print, redacted, or reviewer views). These configurations must be parsed, preserved, surfaced to
  callers, and switchable without flattening them into the default configuration.
- Optional Content Membership Dictionaries (OCMDs) combine multiple OCGs with Boolean logic (`/AllOn`, `/AnyOn`, `/AllOff`, `/AnyOff`).
- The renderer must evaluate OCG/OCMD visibility for each marked content span and skip invisible content.

**Content stream operator set:**

The PDF content stream uses approximately 73 operators organized into the following categories. The renderer's dispatch table must handle all of them:

*Path construction operators:*
- `m` (moveto), `l` (lineto), `c` (curveto — cubic Bézier, 3 control points), `v` (curveto — initial point is current point), `y` (curveto — final point is third control point), `h` (closepath), `re` (rectangle — appends a closed rectangular subpath)

*Path painting operators:*
- `S` (stroke), `s` (close and stroke), `f` (fill, nonzero winding), `F` (fill, nonzero winding — same as `f`, PDF 1.0 compatibility), `f*` (fill, even-odd), `B` (fill and stroke, nonzero winding), `B*` (fill and stroke, even-odd), `b` (close, fill, and stroke, nonzero winding), `b*` (close, fill, and stroke, even-odd), `n` (end path without fill or stroke — used for clipping-only paths)

*Clipping operators:*
- `W` (clip, nonzero winding), `W*` (clip, even-odd). Note: these modify the clipping path only in combination with a subsequent path-painting operator. The clipping takes effect after the paint. This is a common source of implementation bugs.

*Text object operators:*
- `BT` (begin text object), `ET` (end text object)

*Text state operators:*
- `Tc` (character spacing), `Tw` (word spacing), `Tz` (horizontal scaling), `TL` (leading), `Tf` (font and size), `Tr` (rendering mode — 0=fill, 1=stroke, 2=fill+stroke, 3=invisible, 4-7=same with clipping), `Ts` (rise)

*Text positioning operators:*
- `Td` (move to start of next line, offset by tx/ty), `TD` (same as `Td` but also sets leading to -ty), `Tm` (set text matrix and text line matrix), `T*` (move to start of next line using current leading)

*Text showing operators:*
- `Tj` (show string), `TJ` (show string array — alternates strings and numeric kerning adjustments), `'` (move to next line and show string), `"` (set word spacing, character spacing, move to next line, show string)

*Color operators:*
- `CS` (set stroke color space), `cs` (set fill color space), `SC` (set stroke color — up to 4 components), `SCN` (set stroke color — supports Pattern and additional components), `sc` (set fill color), `scn` (set fill color — supports Pattern), `G` (set stroke gray), `g` (set fill gray), `RG` (set stroke RGB), `rg` (set fill RGB), `K` (set stroke CMYK), `k` (set fill CMYK)

*Graphics state operators:*
- `q` (save graphics state — push), `Q` (restore graphics state — pop), `cm` (concatenate matrix to CTM), `w` (line width), `J` (line cap style — 0=butt, 1=round, 2=square), `j` (line join style — 0=miter, 1=round, 2=bevel), `M` (miter limit), `d` (dash pattern — array + phase), `ri` (rendering intent), `i` (flatness tolerance), `gs` (set parameters from ExtGState dictionary)

*XObject and shading operators:*
- `Do` (invoke named XObject — image, form, or PostScript), `sh` (paint area with shading pattern)

*Inline image operators:*
- `BI` (begin inline image), `ID` (begin image data — followed by raw bytes), `EI` (end inline image). The inline image dictionary uses abbreviated key names (`/W` for `/Width`, `/H` for `/Height`, `/CS` for `/ColorSpace`, etc.).

*Marked content operators:*
- `BMC` (begin marked content), `BDC` (begin marked content with properties), `EMC` (end marked content), `MP` (marked content point), `DP` (marked content point with properties)

*Compatibility operators:*
- `BX` (begin compatibility section), `EX` (end compatibility section). Unknown operators between `BX`/`EX` are silently ignored.

**Font type differences and rendering requirements:**

Each font type places different demands on the engine:

*Type 1 fonts:*
- Glyph outlines encoded as PostScript Type 1 charstrings (a stack-based bytecode). The engine must interpret hinting instructions (hstem, vstem, flex) and charstring operators (rmoveto, rlineto, rrcurveto, etc.).
- Font data format: PFB (binary) or PFA (ASCII hex). Encrypted with the well-known `eexec` and charstring encryption.
- Encoding: specified by the font dictionary's `/Encoding` entry. For subsetted Type 1 fonts, the encoding often uses a `/Differences` array that maps character codes to custom glyph names.
- Metrics: glyph widths from the font dictionary's `/Widths` array. For non-embedded Type 1, use the built-in metrics from the font program or the Base 14 tables.
- Multiple Master Type 1 fonts: rare in PDFs but technically possible. The engine should detect and handle the default instance; full Multiple Master interpolation is a Tier 2 feature.

*TrueType fonts:*
- Glyph outlines encoded as quadratic Bézier splines in the `glyf` table (or as CFF data in the `CFF ` table for OpenType/CFF).
- Font data: a complete TrueType/OpenType font file (or a subset thereof). The engine must parse the font directory (`sfnt` header), `cmap`, `glyf`, `loca`, `head`, `hhea`, `hmtx`, `maxp`, `name`, `OS/2`, and `post` tables at minimum.
- Encoding: TrueType fonts in PDFs can use either "simple" encoding (via the `/Encoding` entry, similar to Type 1) or "composite" encoding (via a CMap, when used as a CIDFont descendant). For simple TrueType, the character code typically indexes via the font's `cmap` table using platform 3 encoding 1 (Windows Unicode BMP) or platform 1 encoding 0 (Mac Roman), depending on the `/Encoding` value.
- Hinting: TrueType instructions (bytecode) in the `fpgm`, `prep`, and individual glyph programs. Full hinting is not required for v1 — unhinted outlines at document resolution are acceptable. Hinting primarily matters at low DPI (screen rendering).

*CFF (Compact Font Format) / OpenType-CFF:*
- Glyph outlines encoded as CFF charstrings (Type 2 charstring format — cubic Bézier). The CFF format uses: Header, Name INDEX, Top DICT INDEX, String INDEX, Global Subr INDEX, and per-font CharStrings INDEX, Private DICT, and Local Subr INDEX.
- Subroutines: CFF charstrings use both global and local subroutine calls (operators 10 and 29). The engine must implement subroutine flattening for rendering and must handle the biased numbering scheme (subr index = operand + bias, where bias depends on the number of subroutines).
- CFF2 (used in newer OpenType/CFF2 variable fonts): not expected in most PDFs but should be detected and flagged if present.

*CIDFont (Type 0 composite fonts):*
- The "Type 0" font in PDF is a composite font: a top-level Type 0 font dictionary references a CIDFont descendant (either CIDFontType0 for CFF-based or CIDFontType2 for TrueType-based). A CMap maps character codes to CIDs (Character IDs), and the CIDFont maps CIDs to glyph outlines.
- CMap chain: The `/Encoding` of the Type 0 font names a CMap. Predefined CMap names (e.g., `Identity-H`, `Identity-V`, `UniJIS-UTF16-H`) reference built-in CMap definitions. Custom CMaps are embedded as stream objects. The CMap may reference a `usecmap` parent for hierarchical definitions.
- The `/W` and `/W2` arrays in the CIDFont dictionary define glyph widths (horizontal and vertical). These arrays use a compressed format: `[cid [w1 w2 ...]]` or `[cid_start cid_end w]`.
- `/DW` (default width) and `/DW2` (default vertical metrics) provide defaults for CIDs not in the `/W` arrays.
- `/CIDToGIDMap`: for CIDFontType2 (TrueType-based CIDFont), this maps CIDs to glyph IDs in the TrueType font. It can be `/Identity` (CID = GID) or a stream of 2-byte big-endian GID values.

*Type 3 fonts:*
- Glyph outlines defined as content streams (PDF operators, not font-specific bytecode). Each glyph is a small PDF page, essentially.
- The font dictionary's `/CharProcs` maps glyph names to content streams. The `/Encoding` maps character codes to glyph names.
- Type 3 glyph content streams can use most PDF operators including color, images, and even other fonts. The engine must recursively interpret these content streams.
- The font matrix (`/FontMatrix`) transforms from glyph space to text space. Common values are `[0.001 0 0 0.001 0 0]` (1000 units per em, like Type 1) or `[1 0 0 1 0 0]` (glyph coordinates are directly in text space units).
- Type 3 rendering must establish a new graphics state with the font matrix composed into the text rendering matrix. Color operators in the glyph stream apply to the glyph (unlike Type 1/TrueType where the glyph inherits the current color state based on the rendering mode).

#### Subpixel text rendering

For screen-resolution rendering (≤150 DPI), subpixel positioning materially improves text
clarity:

- **LCD subpixel geometry:** Detect display RGB stripe orientation (horizontal RGB, horizontal
  BGR, vertical RGB, vertical BGR). Render glyphs at 3x horizontal resolution with per-subpixel
  coverage.
- **ClearType-style filtering:** Apply a 3-tap or 5-tap low-pass filter kernel to suppress color
  fringing while preserving subpixel sharpness. The filter weights are configurable (default:
  `[1/4, 1/2, 1/4]` for the 3-tap path).
- **Gamma-correct blending:** Perform alpha blending in linear light rather than sRGB space to
  avoid darkening artifacts at glyph edges. Convert sRGB → linear before blending, blend, then
  convert linear → sRGB.
- **Subpixel positioning:** Position glyphs at 1/4 pixel granularity. For each quantized subpixel
  position already represented in the glyph cache key, rasterize a distinct glyph bitmap that
  accounts for the fractional pixel offset in the coverage computation.

Subpixel rendering is optional. It is disabled by default for print-quality DPI and proof runs.
When enabled, it is the default for the `ViewerFast` profile at ≤150 DPI.

**Color space resolution chain:**

When a color operator sets a color, the engine must resolve the color space and convert to the output color space. The resolution chain:

1. **Device color spaces** (`DeviceRGB`, `DeviceCMYK`, `DeviceGray`): Direct color values in the named device space. Conversion to the output space depends on rendering intent. If a default color space is registered (e.g., `/DefaultRGB` in the page's color space resources), that default applies.

2. **CIE-based color spaces** (`CalRGB`, `CalGray`, `Lab`): Device-independent color spaces with defined white points, gamma, and matrix parameters. The engine converts from the CIE-based space to the profile connection space (PCS) and then to the output device space.

3. **ICCBased:** The color space references an ICC profile stream. The engine must parse the ICC profile to extract the color conversion (A2B and B2A tables or matrix/TRC). For v1, support ICC v2 and v4 profiles for the common classes: input, display, output, and color space conversion. Use the absolute colorimetric, relative colorimetric, perceptual, and saturation rendering intents.

4. **Indexed:** Maps integer indices (0-N, typically 0-255) to colors in a base color space. The lookup table is a byte string where each entry has as many components as the base space. Resolution: decode the index, look up the base color, then resolve the base color space.

5. **Separation:** A single-component space representing a named colorant (e.g., a spot color). Contains an `/AlternateSpace` and `/TintTransform` (a PDF function) for fallback rendering. The engine evaluates the tint transform to convert the separation tint value to the alternate space, then resolves the alternate space.

6. **DeviceN:** Multi-component named colorants. Similar to Separation but with N named components, an alternate space, and a tint transform. DeviceN with `/Process` and `/Spot` sub-categories (PDF 2.0) must be parsed. The tint transform converts the N-component input to the alternate space.

7. **Pattern:** Either a tiling pattern (repeating graphical content) or a shading pattern (smooth color gradients). Tiling patterns contain a content stream rendered at the pattern cell's scale and tiled across the painted area. Shading patterns are defined by shading dictionaries with function-based, axial, radial, free-form mesh, lattice mesh, Coons patch mesh, or tensor-product patch mesh types.

### Color space conversion paths

The renderer must implement the following conversion paths for the output color space (typically
sRGB for screen, CMYK for print):

| Source | Target | Path |
|---|---|---|
| DeviceRGB | sRGB | Identity (assume sRGB) or via DefaultRGB if defined |
| DeviceCMYK | sRGB | Via ICC profile (FOGRA39 default) or naive inversion |
| DeviceGray | sRGB | G → (G, G, G) |
| CalRGB | sRGB | CalRGB → CIEXYZ (via gamma + matrix) → sRGB |
| CalGray | sRGB | CalGray → CIEXYZ (via gamma + white point) → sRGB |
| Lab | sRGB | Lab → CIEXYZ → sRGB |
| ICCBased | sRGB | Via ICC profile A2B/B2A tables or matrix/TRC |
| Indexed | sRGB | Lookup base color → convert base to sRGB |
| Separation | sRGB | Evaluate tint transform → convert alternate space to sRGB |
| DeviceN | sRGB | Evaluate tint transform → convert alternate space to sRGB |

**Naive CMYK→RGB inversion** (used when no ICC profile is available):
```
R = 1 - min(1, C × (1 - K) + K)
G = 1 - min(1, M × (1 - K) + K)
B = 1 - min(1, Y × (1 - K) + K)
```
This is visually acceptable for screen display but not color-accurate. The proof harness must
track which documents use naive CMYK inversion versus ICC-profiled conversion.

**DefaultRGB/DefaultCMYK/DefaultGray override:** When a page defines these in its resource
dictionary, all device color space references on that page are implicitly redirected to the
corresponding CIE-based space. The override is per-page (not document-global) and applies only
to device color spaces set by color operators, not to image color spaces specified in image
XObject dictionaries.

**Shading types:**
- Type 1 (function-based): color = f(x, y) for each point in the shading domain
- Type 2 (axial): linear gradient between two points, parameterized by t ∈ [t0, t1]
- Type 3 (radial): gradient between two circles, parameterized by t ∈ [t0, t1]
- Type 4 (free-form Gouraud-shaded triangle mesh): vertices with colors, interpolated across triangles
- Type 5 (lattice-form Gouraud-shaded triangle mesh): grid of vertices with colors
- Type 6 (Coons patch mesh): 4-sided patches defined by 12 control points and 4 corner colors
- Type 7 (tensor-product patch mesh): 4-sided patches defined by 16 control points and 4 corner colors

**Tiling pattern rendering specifics:**

Tiling patterns repeat a small content stream over an area. The rendering is complex because:

1. **Pattern space:** The pattern has its own coordinate system defined by the pattern matrix. The pattern cell is defined by `/BBox` in pattern space. The cell is tiled at intervals of `/XStep` (horizontal) and `/YStep` (vertical) in pattern space.

2. **Paint type:** PaintType 1 (colored): the pattern's content stream defines its own colors. PaintType 2 (uncolored): the pattern's content stream uses the current color from the invoking context. For PaintType 2, the renderer must pass the invoking color state into the pattern's rendering context.

3. **Tiling type:** TilingType 1 (constant spacing): pattern cells are spaced exactly by XStep/YStep, with no distortion. TilingType 2 (no distortion): spacing may be adjusted slightly to fit an integral number of cells. TilingType 3 (faster): allows distortion for performance.

4. **Infinite tiling:** The pattern conceptually tiles infinitely. The renderer must tile only the visible region (the intersection of the clipping path and the current drawing area). Compute the range of tile indices that overlap the visible region and render only those. For very small XStep/YStep values (a common stress test), this can mean thousands of tiles — enforce a configurable maximum tile count.

5. **Pattern within pattern:** Tiling patterns can reference other patterns (including themselves, creating infinite recursion). The renderer must detect recursive patterns (via a visited set during pattern rendering) and abort with a diagnostic.

**Shading rendering specifics for mesh types (Types 4-7):**

The mesh-based shading types define smooth color gradients over complex geometric regions. They are the most demanding rendering task in the PDF specification.

For Type 6 (Coons patch mesh) and Type 7 (tensor-product patch mesh):
1. Each patch is defined by control points and corner colors. Type 6 uses 12 control points (4 boundary curves × 3 cubic Bézier control points, minus shared corners); Type 7 uses 16 (4×4 grid of control points, enabling interior shape control).
2. The renderer subdivides each patch into triangles fine enough that linear color interpolation within each triangle produces visually smooth results. Adaptive subdivision based on patch curvature and screen-space size is preferred over fixed subdivision.
3. Color interpolation: corner colors are bilinearly interpolated across the patch's parameter space (s, t) ∈ [0,1]². The patch geometry maps parameter space to page space.
4. Adjacent patches share edges (the last edge of one patch becomes the first edge of the next). The renderer must handle the edge-sharing protocol for both row-continuous and column-continuous meshes.
5. For Types 4 and 5 (triangle meshes), Gouraud shading is used: colors are linearly interpolated across each triangle's area based on barycentric coordinates.

Several PDF features (shading patterns, tint transforms for Separation/DeviceN, transfer functions, halftone spot functions) use PDF function objects. The engine must implement all four function types:

- **Type 0 (sampled function):** A lookup table with interpolation. The function maps input values to output values using a grid of samples. Between grid points, the engine interpolates (linear interpolation is required; higher-order is optional). Parameters: `/Domain`, `/Range`, `/Size` (grid dimensions), `/BitsPerSample`, `/Encode`, `/Decode`. The decode array maps sample indices to the function's domain; the encode array maps input values to sample indices.

- **Type 2 (exponential interpolation function):** Computes `C0 + x^N · (C1 - C0)` where `x` is the input, `N` is the exponent, and `C0`/`C1` are the boundary values. Simple but used frequently for linear and power-curve color transitions. Note: when N=1, this is a linear interpolation; when N≠1, it produces gamma-like curves.

- **Type 3 (stitching function):** Combines multiple sub-functions defined on adjacent intervals of the input domain. Parameters: `/Functions` (array of sub-functions), `/Bounds` (boundary points between intervals), `/Encode` (maps each interval to the sub-function's domain). Used to create piecewise-defined color gradients with different behaviors in different regions.

- **Type 4 (PostScript calculator function):** A small PostScript-like stack language with arithmetic, comparison, and conditional operators. The engine must implement a bounded interpreter for this language. Operators include: `add`, `sub`, `mul`, `div`, `idiv`, `mod`, `neg`, `abs`, `ceiling`, `floor`, `round`, `truncate`, `sqrt`, `sin`, `cos`, `atan`, `exp`, `ln`, `log`, `cvi`, `cvr`, `eq`, `ne`, `gt`, `ge`, `lt`, `le`, `and`, `or`, `xor`, `not`, `bitshift`, `true`, `false`, `if`, `ifelse`, `copy`, `exch`, `pop`, `dup`, `roll`, `index`. Type 4 functions must be resource-bounded: enforce a maximum stack depth (100) and maximum instruction count (10,000) to prevent hostile inputs from causing unbounded computation. Beyond raw instruction limits, the engine must analyze the function's effective complexity class and flag pathological branching/stack-manipulation patterns such as deeply nested `ifelse` trees or `roll`-heavy programs that imply super-linear or exponential behavior.

**Metadata extraction specifics:**

PDF documents carry metadata in two locations:

1. **Document Info dictionary** (`/Info` in the trailer): Contains `/Title`, `/Author`, `/Subject`, `/Keywords`, `/Creator`, `/Producer`, `/CreationDate`, `/ModDate`, and optionally `/Trapped`. Dates are in PDF date format: `D:YYYYMMDDHHmmSSOHH'mm'` where `O` is the UTC offset indicator. The parser must handle: missing timezone offsets (assume local), two-digit years (rare but exists), and invalid date strings (report as-is rather than crashing). `/Trapped` has three legal values (`True`, `False`, `Unknown`) and must be surfaced as prepress-significant state rather than collapsed into a generic string field.

2. **XMP metadata** (`/Metadata` stream on the catalog, page objects, or individual resource objects): An XML packet using the Extensible Metadata Platform format. The engine must parse enough XMP to extract: Dublin Core properties (dc:title, dc:creator, dc:description, dc:subject), XMP basic properties (xmp:CreateDate, xmp:ModifyDate, xmp:CreatorTool), PDF-specific properties (pdf:Producer, pdf:Keywords), and PDF/A identification (pdfaid:part, pdfaid:conformance). Metadata enumeration is not catalog-only: page objects, image XObjects, font objects, and form XObjects may each carry their own `/Metadata` stream and all of them must be preserved and reportable independently. Full XMP schema validation is not required for v1, but the engine must preserve XMP metadata byte-perfectly during round-trip operations (XMP packets often contain padding whitespace that must be preserved).

**XMP stream preservation rules:**

1. **Byte-perfect preservation:** The XMP metadata stream must be preserved byte-for-byte during
   round-trip operations unless the user explicitly modifies metadata. This includes the XML
   declaration, processing instructions, padding whitespace, and the packet wrapper
   (`<?xpacket begin="..." id="..."?>` ... `<?xpacket end="w"?>`).

2. **Padding preservation:** XMP packets typically include trailing whitespace padding (often
   2048+ bytes of spaces) to allow in-place metadata updates without rewriting the stream. The
   writer must preserve this padding. In incremental-append mode, the XMP stream is not touched
   unless metadata was modified.

3. **XMP modification:** When the user modifies metadata (title, author, etc.), the engine
   updates the XMP packet in-place if there is sufficient padding. If not, a new XMP stream is
   generated with fresh padding. The new stream must maintain the packet wrapper and include at
   least 2048 bytes of padding.

4. **Info dictionary synchronization:** When XMP is modified, the engine should also update the
   corresponding Info dictionary entries to maintain consistency. When Info dictionary entries are
   modified, the engine should also update XMP. The engine logs a diagnostic when the two
   sources are already inconsistent on input.

When Info dictionary and XMP metadata disagree (common — many producers update one but not the other), the engine reports both and lets the consumer decide which to trust. For PDF/A conformance, XMP is authoritative.

**Transparency compositing model:**

The rendering pipeline must implement the full PDF transparency model as specified in ISO 32000-1 §11.6:

The compositing formula for each pixel is:
```
Cr = (1 - αs/αr) · Cb + (αs/αr) · ((1 - αb) · Cs + αb · B(Cb, Cs))
αr = αs + αb · (1 - αs)
```
Where: Cr = result color, Cb = backdrop color, Cs = source color, αs = source alpha (including shape and opacity), αb = backdrop alpha, αr = result alpha, B = blend function.

For soft masks, the source alpha is modified: `αs = q · f_s · f_j · f_k`, where q = object opacity, f_s = shape (from the geometry), f_j = soft mask value, f_k = if in knockout group, uses the initial backdrop alpha rather than accumulated alpha.

The renderer must maintain a compositing stack: each transparency group pushes a new compositing buffer. Group results are composited back into the parent when the group ends. The buffer management must handle:
- Isolated groups (transparent initial backdrop)
- Knockout groups (per-element compositing against initial backdrop)
- Nested groups (arbitrary nesting depth)
- Different blend modes at each nesting level
- Soft masks applied to groups

#### `monkeybee-3d`

Interactive 3D content parsing and rendering. This is a flagship differentiator: native handling
for PRC/U3D-backed 3D annotations rather than detect-and-ignore behavior.

Key responsibilities:
- PRC format parsing (ISO 14739-1:2014): compressed B-rep, tessellated mesh extraction, product
  structure trees, PMI (Product Manufacturing Information)
- U3D format parsing (ECMA-363 4th Edition): mesh sets, point sets, line sets, CLOD
  (Continuous Level of Detail) progressive mesh decoding
- Unified scene graph: both PRC and U3D are mapped to a common scene representation with meshes,
  materials, lights, cameras, and transforms
- Rendering via wgpu (Vulkan/Metal/DX12 native, WebGPU in browser):
  - PBR lighting model (Cook-Torrance BRDF with metallic-roughness workflow)
  - Shadow mapping (cascaded shadow maps for large scenes)
  - Screen-space ambient occlusion (SSAO)
  - Transparency sorting (order-independent transparency via weighted blended OIT)
  - Mesh decimation / LOD selection (quadric error metrics, Garland-Heckbert)
- Named view interpolation (smooth camera transitions between predefined viewpoints)
- Rendering modes: solid, wireframe, transparent, illustration, hidden line
- Cross-section plane computation (real-time CSG intersection with arbitrary planes)
- Product structure navigation (part tree traversal, visibility toggling per part)
- 3D annotation integration: parse 3D and RichMedia annotation dictionaries, extract
  activation/deactivation behaviors, handle JavaScript triggers (detect and report, not execute)
- 2D/3D compositing: 3D content rendered to texture, composited into the 2D page at the
  annotation rect

The 3D crate shares the wgpu device/queue with `monkeybee-gpu` when both are active. The scene
graph is immutable per snapshot. 3D rendering respects `ExecutionContext` budgets including vertex
count, texture memory, and shader workload limits.

#### `monkeybee-gpu`

Optional GPU-accelerated 2D rendering backend via wgpu.

Key responsibilities:
- Compute shader path rasterization (GPU-native exact area coverage)
- Parallel tile compositing on GPU
- Hardware-accelerated transparency group blending
- GPU texture atlas for glyph caching
- Shared wgpu device/queue with `monkeybee-3d`
- Implements the render backend trait as a drop-in replacement for `tiny-skia` on capable hardware
- Fallback to CPU when GPU is unavailable or when document complexity exceeds GPU memory budget

The GPU backend is experimental and non-gating. It competes with the CPU baseline under the
strategy-tournament framework and becomes default only if it wins on proof metrics.

#### `monkeybee-compose`

High-level authoring and appearance composition.

Key responsibilities:
- Document/page/content builders for new documents
- Resource naming and assembly
- Annotation appearance stream generation helpers
- Form/widget appearance composition
- Font embedding planning and subsetting requests
- Content stream emission from high-level drawing/text operations

#### `monkeybee-write`

Serializes the document model back to valid PDF bytes.

`monkeybee-write` serializes a semantically complete document and never owns high-level content authoring.
Authoring, page assembly, appearance generation, and builder-style APIs live in `monkeybee-compose`.

Key responsibilities:
- Deterministic rewrite and incremental append
- Object serialization (all PDF object types)
- Cross-reference table/stream generation and xref/trailer emission
- Stream compression and filter chain application
- Structural validity enforcement
- Final compression, encryption, and output assembly
- Linearization for output files (future)

**Content stream generation specifics:**

The write path must generate content streams for:
1. **New pages in generated documents.** The generation API accepts high-level drawing commands (draw text, draw image, draw rectangle, set color, etc.) and emits a valid content stream. The emitter must: track graphics state to minimize redundant state-setting operators (don't emit `rg` if the color hasn't changed), properly balance `q`/`Q` pairs, emit `BT`/`ET` around text operations, and generate efficient `TJ` arrays with kerning adjustments rather than positioning each glyph individually.
2. **Annotation appearance streams.** Per the annotation contract in Part 5, each annotation type requires a generated appearance stream (form XObject). The appearance stream must set its own graphics state cleanly (it inherits the default state, not the page's state at the point of rendering).
3. **Annotation flattening.** When burning annotations into page content, the engine appends operators to the page's content stream. The appended operators must be wrapped in a `q`/`Q` pair to avoid contaminating the existing graphics state, and must correctly transform from the annotation's coordinate space to the page's coordinate space.
4. **Redaction application.** As described in Part 5, applying redactions requires rewriting the page content stream with the redacted operators removed and the overlay content inserted.

**Font embedding for generated content:**

When generating new pages with text, the write path must embed the fonts used:
1. Determine which glyphs are actually used on the page.
2. Subset the font to include only those glyphs. For TrueType: rebuild `cmap`, `glyf`, `loca`, `hmtx` tables with only the used glyphs; renumber glyph IDs starting from 0 (with .notdef at 0); update `maxp`, `head`, `hhea`. For CFF: rebuild the CharStrings INDEX, Private DICT, and Local/Global Subr INDEXes with only the used glyphs. For Type 1: subset the CharStrings dictionary and re-encrypt.
3. Generate the font dictionary, font descriptor, widths array, and ToUnicode CMap for the subsetted font.
4. Embed the subsetted font program as a stream object with appropriate compression.
5. For CIDFonts: generate the CIDFont descriptor with `/DW` and `/W` arrays, the CMap (typically Identity-H for Unicode-mapped fonts), and the CIDToGIDMap (typically Identity for subsetted TrueType CIDFonts).
6. Tag the font name with a 6-letter random prefix followed by `+` (e.g., `ABCDEF+ArialMT`) to indicate subsetting. This is a convention, not a requirement, but it aids debugging and conformance checking.

Generated text must not assume a one-codepoint-to-one-glyph mapping. Complex-script shaping and bidi reordering occur before subsetting and ToUnicode generation.

**Document generation API:**

The generation API is the public surface for creating new PDFs from scratch. It must be high-level enough to be useful without forcing the caller to understand PDF internals, but low-level enough to express the full range of PDF content.

API layers:
1. **Document builder:** Create a new document, set metadata (title, author, producer), configure output options (PDF version, encryption, compression).
2. **Page builder:** Add pages with specified dimensions and rotation. Set page-level resources.
3. **Content builder:** A fluent API for building page content:
   - Text operations:
     - Low-level: `begin_text()`, `set_font(name, size)`, `move_to(x, y)`, `show_text_raw(bytes)`, `show_glyphs(positioned_glyphs)`, `end_text()`
     - Shaped text: `show_text(string)` and `layout_text(paragraph, options)` route through a `TextShaper` that handles bidi, shaping, line breaking, and font fallback before emitting positioned glyphs
   - Graphics operations: `move_to(x, y)`, `line_to(x, y)`, `curve_to(cp1, cp2, end)`, `close_path()`, `stroke()`, `fill()`, `clip()`, `set_line_width(w)`, `set_dash_pattern(array, phase)`.
   - Image operations: `draw_image(image_data, format, rect)` — accepts JPEG, PNG, or raw pixel data and embeds appropriately (JPEG is passed through as DCTDecode; PNG is re-encoded as FlateDecode with PNG predictors; raw pixels are compressed with FlateDecode).
   - State operations: `save_state()`, `restore_state()`, `transform(matrix)`, `set_blend_mode(mode)`, `set_opacity(alpha)`.
   - Annotation operations: `add_annotation(type, rect, properties)` — delegates to the annotate crate.
4. **Resource management:** The builder automatically tracks which fonts, images, and ExtGState dictionaries are used and generates the appropriate resource dictionaries. The caller does not manually manage resource names.

**Resource naming convention for generated content:**

When the content builder assigns names to resources in the resource dictionary:

- **Fonts:** `/F1`, `/F2`, `/F3`, ... (incrementing integer suffix). For pages that reference
  existing resources (e.g., annotation flattening), the builder checks existing font names and
  continues from the highest existing number to avoid collisions.
- **Images:** `/Im1`, `/Im2`, `/Im3`, ...
- **Form XObjects:** `/Fm1`, `/Fm2`, `/Fm3`, ...
- **ExtGState:** `/GS1`, `/GS2`, `/GS3`, ...
- **Color spaces:** `/CS1`, `/CS2`, `/CS3`, ...
- **Patterns:** `/P1`, `/P2`, `/P3`, ...
- **Shadings:** `/Sh1`, `/Sh2`, `/Sh3`, ...

The naming convention follows the common PDF producer practice. Names must be unique within a
single resource dictionary. The builder maintains a name-to-object map and deduplicates: if the
same font/image/ExtGState is used multiple times on the same page, it gets one resource name.

The content builder emits content stream operators directly, tracking the graphics state to avoid redundant operator emission. The builder validates state consistency: `stroke()` without a preceding path produces an error, `end_text()` without `begin_text()` produces an error, unbalanced `save_state()`/`restore_state()` produces a warning.

**Structural validity requirements for well-formed PDF output:**

The write path must enforce the following invariants. These are not optional — a PDF that violates these will be rejected by conforming readers:

1. **Header:** Must begin with `%PDF-1.N` or `%PDF-2.0`. If the file contains binary data (almost always), the second line should be a comment containing at least four bytes with values ≥ 128 (e.g., `%âãÏÓ`). This signals to transport layers that the file is binary.

2. **Cross-reference integrity:** Every indirect object referenced anywhere in the document must have a cross-reference entry. The entry's byte offset must be exactly correct (pointing to the first digit of the object number in the `N G obj` line). The `/Size` entry in the trailer must be one greater than the highest object number used.

3. **Stream length accuracy:** Every stream's `/Length` entry must exactly match the number of
   bytes between `stream\n` (or `stream\r\n`) and `endstream`. Off-by-one errors here are one of
   the most common causes of PDF corruption. The write path should compute lengths after all
   content is finalized, not before. Validation must cross-check three surfaces independently when
   available: declared `/Length`, actual raw byte extent, and decoded size implied by the stream's
   semantic consumer (for example image dimensions / bits-per-component). When these disagree, the
   report must say which values disagree and what assumption the engine actually used.

4. **Page tree validity:** The page tree must have a root node (the `/Pages` object referenced by `/Root` → `/Pages`). Every `/Pages` node must have a `/Kids` array and a `/Count` that accurately reflects the total number of leaf pages in its subtree. Every page object must have `/Type /Page` and a `/Parent` back-reference.

5. **Catalog completeness:** The document catalog (`/Type /Catalog`) must contain at minimum `/Pages`. Recommended: `/MarkInfo`, `/Lang`, `/ViewerPreferences` for profile-compliant output.

6. **Font validity for generated content:** Embedded fonts must include: the font program stream with correct `/Length`, `/Length1` (for Type 1), `/Length2`, `/Length3` fields; a `/FontDescriptor` with `/FontBBox`, `/Flags`, `/ItalicAngle`, `/Ascent`, `/Descent`, `/CapHeight`, `/StemV`; and a `/Widths` array (or `/W` for CIDFonts) that matches the embedded glyph set.

7. **Encryption consistency:** If the output is encrypted, every string and stream must be encrypted with the correct per-object key (derived from the object number and generation number). The encryption dictionary must correctly specify the algorithm, key length, and permissions.

8. **Incremental save structure:** An incremental save appends: all new/modified objects, a new cross-reference section covering those objects, and a new trailer with `/Prev` pointing to the previous cross-reference section's byte offset. The appended section must not duplicate unchanged objects. The `/Size` in the new trailer must cover the full object number space (not just the new objects).

**Signature-safe write path:**

For preserve-mode / incremental-append workflows that must not invalidate existing digital signatures:

- The writer must not modify any bytes in the existing file. New content is strictly appended.
- The new cross-reference section references only new or modified objects. Unchanged objects retain their original byte offsets.
- Byte ranges covered by existing signatures (the `/ByteRange` values in signature dictionaries) must remain untouched.
- The writer must track which objects were modified and ensure the incremental update correctly supersedes only those objects.

**Incremental save byte-range accounting:**

When writing an incremental update to a file with existing digital signatures:

1. **Determine the append point:** The new content starts at the current end-of-file offset.
   All previous bytes (0 to EOF-1) are immutable. The writer must not modify or rewrite any
   byte before the append point.

2. **Write new/modified objects:** Each object is serialized and its byte offset (relative to
   file start) is recorded for the new xref. The offset = append_point + bytes_written_so_far.

3. **Write the new cross-reference:** The new xref section covers only the new/modified objects.
   Each entry's offset is computed as described above. Objects not in the new xref retain their
   original offsets from the previous xref chain.

4. **Write the new trailer:** The trailer's `/Prev` entry points to the byte offset of the
   previous xref section (which is before the append point and therefore immutable). The
   `/Size` covers the full object number space.

5. **Signature byte-range verification post-write:** After writing, the engine verifies that
   all existing signature byte ranges (`/ByteRange` arrays) are entirely within the immutable
   region (before the append point). If any byte range extends to or beyond the append point,
   this is an error — it means the original file was malformed or truncated.

6. **`startxref` at end:** The new `startxref` value points to the byte offset of the new
   xref section (within the appended region).

This accounting ensures that no existing byte is modified, all new byte offsets are correct,
and the incremental update chain is structurally valid.

#### `monkeybee-edit`

Transactional structural edits, resource management, and high-assurance operations.

Key responsibilities:
- `EditTransaction` framework: stage edits, compute closure, validate, commit/rollback
- Resource GC: detect and remove unreachable objects after edits
- Resource deduplication: identify and merge identical objects
- Redaction application: high-assurance rewrite with `RedactionPlan` (SemanticExact / SecureRasterizeRegion / SecureRasterizePage)
- Optimization operations: compaction, recompression, object stream repacking
- All optimization and cleanup operations are explicit user-triggered actions, not incidental writer side effects

### Content stream rewrite model

Content stream edits (redaction, annotation flattening, content removal) use a filter-and-rewrite
pipeline:

1. **Parse** the existing content stream into an operator sequence with provenance spans.
2. **Filter** the operator sequence through an `EditSink` that decides per-operator: keep, drop,
   or replace. The `EditSink` receives full graphics state context for each operator (not just the
   raw operator and operands).
3. **Inject** new operators at specified insertion points (e.g., annotation flattening appends
   operators wrapped in `q`/`Q`).
4. **Re-emit** the filtered/modified operator sequence as a new content stream.
5. **Update** the page's content stream reference(s) to point to the new stream object.

For redaction specifically:
- The `EditSink` identifies all operators that produce output within the redaction region by
  evaluating each operator's bounding box against the redaction rectangles.
- Text operators are split if a `TJ` array partially overlaps a redaction region (individual glyph
  positions are checked).
- Image operators that partially overlap are handled per the `RedactionPlan` mode: `SemanticExact`
  removes fully contained images; `SecureRasterizeRegion` replaces the region; `SecureRasterizePage`
  replaces the entire page.
- After rewrite, the old content stream object is marked as deleted in the change journal.

For annotation flattening:
- The annotation's appearance stream content is extracted, transformed by the annotation's
  position matrix, wrapped in `q`/`Q`, and appended to the page content.
- The annotation object is removed from the page's `/Annots` array.

**Flattening coordinate transform chain:**

When flattening an annotation into page content, the following transforms are composed:

1. The annotation's appearance stream is a form XObject with its own `/BBox` and optional
   `/Matrix`. The appearance content is drawn in the form's coordinate space.
2. The annotation's `/Rect` [llx lly urx ury] defines where the appearance is placed in the
   page's default user space (before page rotation).
3. The transform maps the appearance's `/BBox` to the annotation's `/Rect`:
   ```
   scale_x = (rect_urx - rect_llx) / (bbox_urx - bbox_llx)
   scale_y = (rect_ury - rect_lly) / (bbox_ury - bbox_lly)
   translate_x = rect_llx - bbox_llx * scale_x
   translate_y = rect_lly - bbox_lly * scale_y
   ```
4. If the appearance has a `/Matrix`, it is applied before the BBox-to-Rect mapping:
   `final_matrix = bbox_to_rect_matrix × appearance_matrix`
5. The flattened content is: `q [final_matrix components] cm [appearance stream operators] Q`
6. Page rotation does NOT affect the flattening transform — both the page content and the
   annotation are in the same unrotated coordinate space. The viewer applies rotation uniformly.
7. CropBox offset is similarly irrelevant to flattening — the annotation Rect is already in the
   page's coordinate system, which includes the CropBox position.

#### `monkeybee-forms`

AcroForm field tree, value model, appearance regeneration, calculation order, widget bridge, and signature-field helpers.

Key responsibilities:
- Field tree parsing and inheritance resolution
- Field value model (text, button, choice, signature)
- Appearance regeneration for widget annotations when field values change
- Calculation order execution model (detection and preservation; evaluation is post-v1)
- Widget/annotation bridge: connecting form field semantics to annotation visual representation
- Signature-field helpers: byte-range tracking, CMS envelope inspection, incremental-append preservation
- Form data import/export
- FDF/XFDF import and export as first-class interchange operations
- Form flattening after field-inheritance and appearance resolution
- JavaScript action inventory for calculate / format / validate / keystroke handlers
- Submit-form and reset-form analysis, including target classification and preservation
- Signature-field creation with placeholder sizing and save-plan integration
- Barcode-field parse/render support
- Tier 2 static-XFA inspection and flattening where the authoritative visual layer is recoverable

AcroForm handling is distinct from general annotation handling. Widgets are annotations visually, but the field tree semantics, inherited field properties, appearance regeneration rules, value synchronization, and signature handling justify a dedicated subsystem.

#### `monkeybee-paint`

Shared page-independent painting and appearance primitives:
- path stroking/filling helpers
- text run realization for appearance generation
- color and ExtGState emission helpers
- form XObject appearance composition primitives
- paint-side geometry utilities reused by render/compose/annotate

`monkeybee-paint` does not rasterize pages and does not own content-stream interpretation.
It is the shared kernel for emitted appearance content.

#### `monkeybee-annotate`

Non-form annotation creation, modification, and management.

Key responsibilities:
- Annotation types: Text, Link, FreeText, Line, Square, Circle, Polygon, PolyLine, Highlight, Underline, Squiggly, StrikeOut, Stamp, Caret, Ink, Popup, FileAttachment, Sound, Redact
- Geometry-aware placement using shared page-state understanding
- Appearance stream generation and management
- Annotation flattening (burn into page content)
- Annotation property modification
- Reply chains and markup relationships
- Bridge generic annotation handling to `monkeybee-forms` for Widget annotations
- Depends on `monkeybee-paint` for appearance stream content realization
  (glyph positioning, path construction, color setting within form XObjects)
- Round-trip preservation: add annotation → save → reopen → verify

**Appearance stream generation requirements per annotation type** are detailed in Part 5 (Subsystem Contracts).

#### `monkeybee-extract`

Structured extraction and inspection.

Key responsibilities:
- Internal `LayoutGraph` IR: spans, lines, blocks, reading-order edges, tag links, table/column hypotheses, and confidence scores
- `SpatialSemanticGraph`: a richer graph of semantic nodes (paragraphs, cells, figures, fields,
  annotations, signatures, regions) with reading-order, containment, geometric-adjacency,
  dependency, and revision-lineage edges
- Stable semantic anchors derived from snapshot-aware hashes so downstream tooling can refer to
  document regions without scraping ad hoc coordinates from raw text output
- Multi-surface text extraction built on `monkeybee-text`:
  - `PhysicalText` (glyphs, quads, exact geometry)
  - `LogicalText` (reading-order text with per-block confidence)
  - `TaggedText` (structure-tree-driven when available)
  - `SearchIndex`, `SelectionQuads`, and `HitTest` primitives for viewers and editors
- Metadata extraction (document info, XMP)
- Page structure inspection
- Resource inventory (fonts, images, color spaces)
- Object graph inspection and querying
- Image extraction
- Full tagged-semantic extraction: standard structure roles, namespace-aware roles,
  attribute/class-map state, `/ActualText`, `/Alt`, `/E`, `/Lang`, pronunciation hints,
  artifact marking, and structure destinations
- Action inventory and document link-map extraction for GoTo/GoToR/GoToE/Thread/URI and
  related navigational actions
- Article-thread, thumbnail, collection/portfolio, alternate-presentation, `/PieceInfo`,
  and web-capture inspection surfaces
- Multimedia inventory: screen annotations, sound/movie objects, media clips, players,
  rendition trees, and related active-content metadata
- Print-oriented inspection surfaces: output-intent inventory, separation names, TAC
  summaries, and preflight-facing image-resolution metadata

```
pub struct ExtractResult {
    pub layout_graph: Arc<LayoutGraph>,
    pub semantic_graph: Option<Arc<SpatialSemanticGraph>>,
    pub surface: ExtractSurface,
    pub report: ExtractReport,
}

pub enum ExtractSurface {
    Physical(PhysicalText),
    Logical(LogicalText),
    Tagged(TaggedText),
}
```

### Extraction evidence contract

`LogicalText` and `TaggedText` SHOULD optionally carry an evidence graph that
lets downstream callers inspect why reading-order and table hypotheses were made.

```
pub struct ConfidenceBreakdown {
    pub geometric_score: f32,
    pub tagged_score: f32,
    pub font_decode_score: f32,
    pub column_detection_score: f32,
    pub table_detection_score: f32,
}

pub struct SpanEvidence {
    pub span_id: SpanId,
    pub source_ops: Vec<ContentOpRef>,
    pub source_mcid: Option<u32>,
    pub confidence: ConfidenceBreakdown,
}

pub struct TableHypothesis {
    pub bbox: Rectangle,
    pub rows: u32,
    pub cols: u32,
    pub confidence: f32,
    pub evidence_spans: Vec<SpanId>,
}

pub struct ReadingOrderEvidence {
    pub spans: Vec<SpanEvidence>,
    pub edges: Vec<ReadingOrderEdge>,
    pub table_hypotheses: Vec<TableHypothesis>,
}

pub struct ReadingOrderEdge {
    pub before: SpanId,
    pub after: SpanId,
    pub reason: String,
    pub weight: f32,
}
```

**Object graph inspection:**

The inspection API provides programmatic access to the document's internal structure:

1. **Object-level access:** Look up any indirect object by number. Return its parsed representation (type, value, dictionary keys, stream metadata). For streams, report: raw length, decoded length, filter chain, and dictionary contents — without requiring full decompression.

2. **Reference graph:** Given an object, list all objects it references (forward references) and all objects that reference it (back references). This enables dependency analysis: "what happens if I delete this object?" — check its back references.

3. **Page tree visualization:** Render the page tree hierarchy showing: node types (Pages vs. Page), child counts, inherited attributes at each level, and which pages resolve to which leaf nodes.

4. **Cross-reference dump:** Show the cross-reference table/stream with repair annotations: which entries were repaired, what the original values were, which objects are in object streams, and the incremental update chain with per-update object inventories.

5. **Stream analysis:** For content streams, show the operator sequence with operand types and values without full interpretation. This is "disassembly mode" — useful for understanding what a content stream does without rendering it. Include byte offsets for each operator to correlate with hex dumps.

6. **Resource dependency map:** Given a page, trace all transitive resource dependencies: fonts (with their encoding and embedded data info), images (with dimensions, color space, compression), form XObjects (with their own resource dependencies), patterns, color spaces, and ExtGState dictionaries. This answers "what does this page need to render correctly?"
- Embedded file extraction
- Bookmark/outline extraction
- Form field data extraction
- Diagnostics: unsupported features, degraded regions, compatibility notes
- Machine-readable extraction output (JSON, structured text)

**Embedded file extraction:**

PDF documents can contain embedded files (attachments) in several locations:
1. Document-level: `/Root` → `/Names` → `/EmbeddedFiles` name tree. Each entry maps a filename to a file specification dictionary containing the embedded file stream.
2. Page-level: FileAttachment annotations contain embedded file streams.
3. Form-level: Widget annotations on push-button fields can reference file attachments.

The extraction pipeline must: enumerate all embedded files from all locations, extract the file data (decompressing through the filter chain), preserve the original filename and MIME type (from the `/UF` and `/Subtype` entries), and report file sizes and checksums.

**Bookmark/outline extraction:**

The document outline (bookmarks) is a tree rooted at `/Root` → `/Outlines`. Each outline item has: `/Title` (display text), `/Dest` or `/A` (destination — a page reference with view parameters, or an action dictionary), `/First` and `/Last` (child items), `/Next` and `/Prev` (sibling items), `/Count` (number of descendants), and `/C` (color) and `/F` (flags: italic, bold).

Edge cases: circular outline trees (detect via visited-set), outline items with actions instead of destinations (actions may reference external URIs, JavaScript, or other documents — extract what is safe, report the rest), very deep outline trees (hundreds of nesting levels in programmatically generated documents).

**Form field data extraction:**

Extract all AcroForm field values as structured data: field name (full hierarchical path, e.g., `form1.section2.field3`), field type, current value, default value, options (for choice fields), flags (required, read-only, etc.). This enables form data round-trip: extract values → modify → fill back in → save.

**Text extraction pipeline:**

Text extraction reuses the same content stream interpretation pipeline as the renderer but operates in "analysis" mode instead of "render" mode. The pipeline:

1. Interpret the content stream, tracking the text matrix and text rendering matrix exactly as the renderer would.
2. For each `Tj`, `TJ`, `'`, or `"` operator, decode the string using the font's encoding (same fallback chain as described in Part 2).
3. Record each character's Unicode value, position (in page coordinates, after applying the text matrix, CTM, and page rotation), font name, font size, and rendering mode.
4. `PhysicalText` is emitted before any reading-order heuristics are applied. This surface provides exactly what was painted, with glyphs/quads and geometry.
5. `LogicalText` is produced as a separate derived surface; each line/block carries a confidence score and the heuristic or tagged path that produced it. The heuristic: group characters into lines (characters with similar Y positions within a tolerance of ~font-size/3), then sort lines top-to-bottom, then sort characters within each line left-to-right (or right-to-left for RTL scripts, detected by Unicode bidi category).
6. Detect word boundaries by comparing horizontal gaps between consecutive characters to a threshold (typically 30-50% of the average character width in the current font).
7. Detect paragraph boundaries by analyzing vertical gaps between lines.
8. `TaggedText` is produced when a structure tree is present, using the tagged reading order rather than geometric heuristics.

**Text extraction edge cases:**

- **Text rendering mode 3 (invisible text):** Characters rendered with Tr=3 are invisible but still carry positional and Unicode information. Extraction must include them (they are the text layer in scanned PDFs with OCR overlays). The extraction output should flag invisible text distinctly.
- **Overlapping text:** Some producers render text multiple times at the same position (e.g., once for fill, once for stroke, or as a shadow effect). The extraction pipeline must deduplicate: characters at the same position (within a tolerance of 0.5pt) with the same Unicode value are merged into one.
- **Reversed text matrices:** Some producers position text right-to-left even for LTR scripts by using negative horizontal scaling or a mirrored text matrix. The extraction pipeline must detect this and reverse the character order to produce the correct logical reading order.
- **Vertical text:** CJK documents may use vertical writing mode (indicated by a `-V` suffix on the CMap name, e.g., `UniJIS-UTF16-V`). Vertical text flows top-to-bottom, right-to-left. The extraction pipeline must detect vertical writing mode and adjust line/column grouping accordingly.
- **Artificial spacing:** Some producers insert explicit spaces between every character for tracking/justification. The extraction pipeline should detect artificially spaced text (uniform gaps significantly smaller than a normal word space) and collapse the spacing.
- **Text in patterns and form XObjects:** Text appearing inside tiling patterns or form XObjects must be extracted with the correct transformation applied (the pattern matrix or form matrix composed with the invoking CTM).
- **Marked content for reading order:** Tagged PDFs use marked content sequences (`BMC`/`BDC`/`EMC`) with structure tags that define logical reading order. When available, the extraction pipeline should prefer the tagged reading order over geometric heuristics. However, most real-world PDFs are not tagged; geometric heuristics are the primary path.

#### `monkeybee-forensics`

Document security analysis and forensic inspection.

Key responsibilities:
- Hidden content detection: white-on-white text, off-page content, content behind opaque images,
  invisible text rendering-mode abuse, content outside CropBox but inside MediaBox
- Redaction sufficiency audit: verify that existing redaction annotations or overlays actually
  removed underlying content rather than merely obscuring it visually
- Post-signing modification forensics: classify modifications in incremental updates after the last
  signature as permitted (per DocMDP policy) or suspicious
- Known CVE pattern detection: structural patterns matching historical PDF exploit signatures such
  as malformed streams, recursive objects, and crafted JavaScript trigger structures
- Full action-graph and active-content inventory: all action types, trigger locations,
  target destinations, external references, and sanitize-preserve-stub planning
- Producer fingerprinting: identify actual producing software from structural patterns beyond the
  `/Producer` string
- Font fingerprinting: match glyph outlines against known font databases to identify real font
  identity when metadata is missing or wrong
- Steganographic channel detection: LSB analysis on embedded images and inspection of unusual data
  in padding or whitespace regions
- Metadata consistency analysis: cross-validate Info dictionary, XMP, font metadata, producer
  signatures, and structural patterns for inconsistencies
- Print-risk diagnostics: TAC threshold exceedance, suspicious overprint usage, missing output
  intents, low-resolution placed imagery, and trap-network anomalies

Forensics is read-only by default: it consumes the same document/content/signature surfaces as the
rest of the engine but never mutates the document model as part of analysis.

#### `monkeybee-diff`

Semantic and multi-surface comparison between documents or snapshots.

`monkeybee-diff` is a required implementation crate, not a report-only concept.
Its outputs are re-exported by the public `monkeybee` facade.

Key responsibilities:
- structural object/page/resource deltas
- text deltas (physical/logical/tagged surfaces)
- render deltas (reusing proof metrics and region maps)
- signature impact deltas
- capability/compatibility deltas
- save-plan deltas (`why would this become full rewrite?`)

Canonical output:
```
DiffReport {
  schema_version,
  left_document_id,
  right_document_id,
  structural_delta,
  text_delta,
  render_delta,
  signature_delta,
  capability_delta,
  write_impact_delta,
  diagnostics,
}
```

#### `monkeybee-validate`

Conformance validation, profile checking, and structural verification.

Key responsibilities:
- Arlington-model conformance validation (codegen from TSV, strict/tolerant integration)
- Profile-specific validation (PDF/A-4, PDF/X-6)
- Write preflight checks (structural validity before serialization)
- Signature byte-range verification
- Print preflight: image-resolution-at-print-size checks, bleed/TrimBox/BleedBox checks,
  output-intent and color-space suitability checks, TAC thresholds, and trap-network validation
- Accessibility audit: structure-tree completeness, figure alt-text presence, heading hierarchy,
  table-header association checks, artifact labeling, and reading-order consistency diagnostics
- Signature-lifecycle policy checks: PAdES profile classification, DSS/VRI completeness, and
  offline long-term validation readiness
- Consume validation results from the proof harness and turn them into evidence/regression artifacts

#### `monkeybee-proof`

Automated validation and evidence generation. `monkeybee-proof` is asupersync-native
and uses `LabRuntime` as its execution substrate.

Key responsibilities:
- Pathological corpus management (acquisition, indexing, categorization)
- Render comparison harness (Monkeybee vs. reference renderers)
- Round-trip validation harness (load → modify → save → reload → compare)
- Annotation round-trip validation
- Generation validation (create → render under Monkeybee and references)
- Compatibility ledger aggregation and reporting
- Performance benchmarking harness
- Evidence artifact generation (diffs, reports, manifests, traces)
- Regression detection
- Coverage tracking across corpus categories
- Conformance validation integration (Arlington model, profile checks)
- Prepress proof lanes: soft-proof regression sets, separation-preview fixtures, TAC baselines,
  and print-preflight expectation suites
- Signature-lifecycle proof lanes: PAdES profile fixtures, DSS/VRI/OCSP/CRL/TSA expectations,
  signature-creation interoperability, and offline validation runs
- Accessibility proof lanes: tagged-PDF semantic fixtures, ActualText/Alt/Lang/artifact
  expectations, PDF/UA audit-rule suites, and reading-order overlay baselines
- Rich-structure proof lanes: action-corpus inventories, article-thread fixtures, portfolios,
  thumbnails, transitions, PieceInfo/web-capture samples, and multimedia inventory fixtures
- Deterministic concurrency testing via LabRuntime
- Cancellation chaos testing and crashpack generation

**LabRuntime proof integration:**

The proof harness uses asupersync `LabRuntime` for all concurrency-sensitive testing:

- **Deterministic concurrent testing:** All multi-page parallel render/extract tests
  run under LabRuntime with fixed seeds. Same seed = same scheduling = reproducible
  results. This catches cache races, shared font corruption, and DashMap contention
  bugs that random testing misses.

- **DPOR exploration:** Critical concurrency tests use DPOR (Dynamic Partial Order
  Reduction) schedule exploration to systematically cover scheduling interleavings.

- **Oracle suite:** Every proof run asserts:
  - Quiescence oracle: no orphan tasks after session close
  - Obligation leak oracle: no leaked permits/channels/resources
  - Loser drain oracle: cancelled operations fully drained
  - Cancellation protocol oracle: request → drain → finalize sequence observed

- **Chaos injection:** Robustness tests use chaos presets:
  - `with_light_chaos()` for CI regression (fast, reliable signal)
  - `with_heavy_chaos()` for release-gate shakeout (deeper coverage)
  - Focused cancellation campaigns for crash-safe save, progressive render, and
    native decoder quarantine paths

- **Crashpacks:** Concurrency failures automatically produce crashpacks with seed,
  trace fingerprint, oracle failures, and replay command. These are CI artifacts
  linked to the compatibility ledger.

- **Futurelock detection:** Tests panic on futurelock (tasks stuck holding obligations
  without making progress). This catches shutdown wedges and leaked cleanup
  responsibility.

- **Virtual time:** LabRuntime's virtual time wheel completes sleeps instantly. Proof
  runs involving timeouts, deadlines, and progressive rendering waits execute at
  scheduler speed with no wall-clock delays, dramatically accelerating CI proof runs.

**Multi-oracle rendering arbitration:**

The proof harness does not trust any single external renderer as ground truth. Instead, it uses consensus-style arbitration:

1. Render each test page with Monkeybee, PDFium, MuPDF, pdf.js, and Ghostscript.
2. Compute perceptual difference metrics (SSIM, DSSIM, or perceptual hash distance) between each pair of renderers.
3. Identify consensus: if 3+ renderers agree (within a perceptual tolerance), that rendering is treated as the expected output. Monkeybee is measured against the consensus.
4. Where renderers disagree: record the disagreement with per-renderer output, investigate the cause (spec ambiguity, renderer bug, or genuine implementation choice), and document the resolution.
5. Where Monkeybee matches consensus but one renderer disagrees, that is evidence of the other renderer's bug — useful for the project's credibility narrative.

```
pub enum OracleResolutionKind {
    Consensus,
    MajorityWithOutlier,
    FixtureOverride,
    SpecAmbiguity,
    ExternalOracleBug,
    MonkeybeeBug,
    NeedsHumanTriage,
}

pub struct OracleDisagreementRecord {
    pub disagreement_id: String,
    pub page_index: u32,
    pub oracle_manifest_id: String,
    pub monkeybee_digest: [u8; 32],
    pub oracle_digests: Vec<(String, [u8; 32])>,
    pub metric_summary: Vec<(String, f64)>,
    pub resolution: OracleResolutionKind,
    pub blocking: bool,
}
```

Rules:
- any disagreement above the pinned proof threshold emits a typed
  `OracleDisagreementRecord`, even if Monkeybee ultimately wins consensus
- the record includes manifest-qualified oracle identities, metrics, and the
  exact resolution class used for gating
- unresolved disagreements remain blocking for promotion until they are resolved
  or triaged with a manifest-qualified waiver

**Conformance infrastructure (Arlington model):**

The Arlington PDF Model is a machine-readable description of every dictionary, array, and value constraint in the PDF specification. The engine should use it for:

1. **Validation:** Given a parsed PDF object, validate its keys, value types, required entries, and value constraints against the Arlington model. This replaces ad-hoc validation with systematic, spec-derived checking. Example: the Arlington model specifies that a Page dictionary must have `/Type` (value: `Page`), `/Parent` (indirect reference to Pages node), and must inherit or define `/MediaBox` (array of 4 numbers). The validator checks all of these mechanically.

2. **Code generation:** Generate Rust type definitions, validation functions, and serialization helpers from the Arlington model. This ensures the engine's understanding of PDF dictionaries stays synchronized with the specification. The generated types provide compile-time guarantees that required fields are present and correctly typed.

3. **Conformance testing:** For profile-specific output (PDF/A-4, PDF/X-6), the Arlington model's profile annotations indicate which features are required, optional, or prohibited. The write path uses these annotations to validate output conformance. For PDF/A-4 specifically: all fonts must be embedded, all color spaces must be device-independent or have an output intent, `/Metadata` must be present as XMP, and transparency must use only the blend modes permitted by the profile.

4. **Diagnostic enrichment:** When validation fails, the Arlington model provides the spec reference (section number, table number) for the violated constraint. This makes diagnostics directly actionable: instead of "missing key in dictionary," the error says "Page dictionary missing required /MediaBox (ISO 32000-2 Table 30)."

**Visual and structural PDF comparison:**

The proof harness includes a comparison system that goes beyond pixel-diffing:

1. **Perceptual image comparison:** Render both versions at the same DPI, compute SSIM (Structural Similarity Index) per page. SSIM > 0.98 is "visually identical." SSIM between 0.95-0.98 is "visually similar, minor differences." SSIM < 0.95 requires investigation. Additionally, compute a perceptual hash and DSSIM (Structural Dissimilarity) for a second opinion.

2. **Structural comparison:** Compare the document object trees (page count, resource sets, font dictionaries, annotation lists, metadata) independently of rendering. This catches cases where rendering looks identical but the underlying structure has changed (e.g., font names rewritten, resources duplicated, metadata lost).

3. **Text comparison:** Extract text from both versions and diff. This catches text extraction regressions separately from rendering regressions.

4. **Reconciliation:** When structural and visual comparisons disagree (structure changed but visual is identical, or structure unchanged but visual differs), flag for manual investigation. These discrepancies often indicate subtle bugs.

#### `monkeybee-cli`

Command-line interface for all engine capabilities.

Key responsibilities:
- `monkeybee render <file> [--pages] [--format png|svg] [--dpi]`
- `monkeybee extract <file> [--text|--meta|--images|--fonts|--structure]` `[--mode physical|logical|tagged]`
- `monkeybee search <file> <query>` — document text search with page/quad results
- `monkeybee render <file> --region <x,y,w,h>` — region render for viewer-like workflows
- `monkeybee inspect <file> [--objects|--xref|--pages|--resources|--updates]`
- `monkeybee annotate <file> --add <type> [--position] [--content] -o <o>`
- `monkeybee edit <file> [--add-page|--remove-page|--reorder|--metadata] -o <o>`
- `monkeybee generate [--template] -o <o>`
- `monkeybee validate <file> [--roundtrip|--structure|--render-compare|--conformance]`
- `monkeybee diagnose <file>` — full compatibility report
 - `monkeybee diagnose <file> [--html]` — JSON or self-contained HTML dossier
 - `monkeybee diff <before> <after> [--render|--text|--structure|--save-impact]`
   emits a schema-versioned `DiffReport`
- `monkeybee plan-save <file> [--incremental|--rewrite]` — preview ownership, rewritten regions,
  signature impact, and fallback reasons before saving; emits a schema-versioned `WritePlanReport`
- `monkeybee plan-save <file> --patches` — emit `BytePatchPlan` with preserved ranges,
  append ranges, and signed-range audit
- `monkeybee proof <corpus-dir>` — run the full proof harness
- `monkeybee conformance <file> [--profile pdf-a4|pdf-x6]` — profile-specific validation
- `monkeybee optimize <file> [--dedup|--gc|--recompress] -o <o>` — full-rewrite compaction and cleanup as an explicit user operation
- `monkeybee trace <file>` — emit page/subsystem spans, repair decisions, cache statistics, and budget consumption as JSON
- `monkeybee signatures <file>` — inspect signature dictionaries, byte ranges, CMS envelope metadata, and provider-backed verification results
- `monkeybee signatures <file> --pades --dss --vri --ocsp --crl --timestamps` — long-term validation inventory and readiness classification
- `monkeybee sign <file> --field <name> --cert <p12|pem> --tsa <url> -o <o>` — create CMS/PAdES signatures in incremental-append mode
- `monkeybee forms import-fdf <pdf> --fdf <data.fdf> -o <o>` / `monkeybee forms export-fdf <pdf>` / `monkeybee forms flatten <pdf> -o <o>`
- `monkeybee inspect <file> --actions --link-map --threads --portfolio --piece-info --web-capture --multimedia`
- `monkeybee render <file> --simulate-overprint [--soft-proof <icc>] [--separations <all|plate>]`
- `monkeybee validate <file> --print-preflight` / `monkeybee validate <file> --pdf-ua-audit`



### Semantic anchor and agent-facing contract

Monkeybee should expose stable semantic anchors so that viewers, automation, and future AI/native
integrations can refer to document meaning without depending on fragile page-local heuristics.

```
pub struct SemanticAnchorId(pub [u8; 32]);

pub enum SemanticNodeKind {
    Page,
    Paragraph,
    Line,
    Span,
    Table,
    TableCell,
    Figure,
    FormField,
    Annotation,
    Signature,
    Region,
}

pub struct SemanticNode {
    pub anchor_id: SemanticAnchorId,
    pub kind: SemanticNodeKind,
    pub page_index: u32,
    pub bbox: Rectangle,
    pub text_excerpt: Option<String>,
    pub depends_on: Vec<ObjRef>,
    pub source_span_ids: Vec<SpanId>,
}

pub struct EditProposal {
    pub proposal_id: String,
    pub anchors: Vec<SemanticAnchorId>,
    pub op: SemanticEditOp,
    pub precondition_snapshot: SnapshotId,
    pub precondition_digest: [u8; 32],
    pub intent: EditIntent,
}
```

Required properties:
- anchor IDs are deterministic for a given snapshot + extraction profile
- safe rewrites and incremental appends SHOULD preserve anchor IDs for semantically unchanged
  regions; when they cannot, Monkeybee emits an alias map rather than silently drifting
- agent-facing or external edit APIs operate on typed `EditProposal`s and yield ordinary
  `EditTransaction` receipts; they do not bypass ownership, policy, preservation, or signature
  planning rules
- a future JSON-RPC/WASM adapter may expose this surface, but the core guarantee lives in the
  engine proper, not in one transport

This absorbs the best part of the "AI-native PDF" plans without sacrificing discipline: the engine
becomes queryable and automatable because it exposes typed semantic anchors, not because it hands an
LLM a bag of text and hopes for the best.

### CLI output discipline

All CLI commands follow a consistent output contract:

**Stdout:** Primary output (rendered files, extracted text, JSON reports). Machine-parseable.
**Stderr:** Diagnostics, progress, and error messages. Human-readable.

**Exit codes:**
- 0: success, no errors or degradations
- 1: operation completed but with errors or degradations (check stderr or `--json` output)
- 2: operation failed (fatal error)
- 3: invalid arguments or usage error

**JSON mode:** Every command supports `--json` which formats all output (including diagnostics
and errors) as JSON to stdout. In JSON mode, stderr receives only fatal errors. The JSON output
wraps the primary result in an envelope:

```json
{
  "schema_version": "1.0",
  "status": "success" | "degraded" | "failed",
  "result": { /* command-specific */ },
  "diagnostics": [ /* array of Diagnostic objects */ ],
  "timing": { "wall_ms": 1234, "parse_ms": 500, "render_ms": 700 }
}
```

**Progress reporting:** Long-running operations (multi-page render, corpus proof) report
progress to stderr as `[page N/M]` or `[file N/M]` lines. In JSON mode, progress is suppressed.

**Quiet mode:** `--quiet` suppresses all diagnostics and progress on stderr. Only fatal errors
are reported. Exit code still reflects the operation status.

**Command details:**

*render:* Renders specified pages (default: all) to the specified format at the given DPI (default: 150). Supports page range syntax (`1-5`, `1,3,7`, `1-3,5-`). For SVG output, emits one SVG file per page. For PNG/JPEG, emits one image file per page. Output files are named `<basename>-page-<N>.<ext>`. If `--compare-ref` is specified, also renders with PDFium/MuPDF and outputs a visual diff image alongside. Returns a non-zero exit code if any page fails to render, with diagnostics on stderr.

*extract:* Extracts structured data from the document. `--text` outputs UTF-8 text with optional position information (`--text --positions` outputs JSON with per-character coordinates). `--meta` outputs document metadata as JSON (Info dictionary + XMP). `--images` extracts all embedded images to individual files. `--fonts` outputs a font inventory: name, type, encoding, embedded status, glyph count, and ToUnicode coverage. `--structure` outputs the document structure: page tree, resource map, cross-reference summary, incremental update chain, and annotation inventory.

*inspect:* Low-level document structure inspection. `--objects N` dumps the raw parsed representation of object N. `--xref` dumps the cross-reference table with repair annotations. `--pages` dumps the page tree with resolved inherited attributes. `--resources N` dumps the resolved resource dictionary for page N. `--updates` dumps the incremental update chain with per-update object lists. Output is JSON by default, with `--format pretty` for human-readable indented output.

*diagnose:* Produces the full compatibility ledger for the document as specified in Part 6. Output is JSON. This is the "doctor's report" command: what features are present, what tier each is classified as, what repairs were applied, and what the overall compatibility assessment is. Useful for triaging problem PDFs.

*validate:* Runs validation checks. `--structure` checks structural validity (xref integrity, page tree, required entries). `--roundtrip` performs load-save-reload and compares. `--render-compare` renders and compares against reference renderers. `--conformance` checks against the specified profile's requirements. Returns non-zero exit code on validation failure with structured diagnostic output.

*proof:* Runs the full proof harness against a corpus directory. Produces the corpus-level compatibility report, regression summary, and evidence artifacts. Designed for CI integration: returns non-zero exit code if any regression is detected or if the pass rate drops below the configured threshold.

---

## Part 4 — Shared invariants

These invariants apply across all crates and all operations. They are the architectural commitments that make the closed loop possible.

### PDF version awareness

The engine tracks the PDF version at three levels:

1. **Input version:** The version declared in the file header (`%PDF-1.N` or `%PDF-2.0`) and
   optionally overridden by the catalog's `/Version` entry (which takes precedence when present and
   higher). The parser uses the input version to select appropriate parsing behaviors:
   - Pre-1.5: no object streams, no cross-reference streams
   - Pre-1.4: no transparency model
   - Pre-1.6: no AES encryption, no OpenType/CFF embedding
   - 2.0: additional encryption revisions (R6), new annotation types, updated color semantics

2. **Feature version:** Each parsed feature is tagged with the minimum PDF version that defines it.
   In strict mode, features that exceed the declared version produce a diagnostic. In tolerant mode,
   they are accepted (many producers declare an older version but use newer features).

3. **Output version:** The writer emits the minimum version that covers all features present in the
   output document. If the user requests a specific output version (e.g., for downlevel
   compatibility), features incompatible with that version produce a preflight error.

Version tracking feeds into the compatibility ledger: the ledger records the declared version, the
effective version (minimum version needed for all features actually used), and any version
mismatches detected.

### Thread-safety model

`PdfSnapshot` is `Send + Sync`. Multiple threads may read from the same snapshot concurrently.
The following operations are safe to run in parallel on the same snapshot:

- Rendering different pages
- Extracting text from different pages
- Inspecting different objects
- Decoding different streams (via the decode cache, which uses concurrent-safe access)

The following require exclusive access and cannot run in parallel with reads on the same snapshot:

- `EditTransaction::commit()` (produces a new snapshot; does not mutate the source snapshot)

Engine-level caches use lock-free or sharded concurrent data structures:
- Decoded stream cache: `DashMap<CacheKey, Arc<[u8]>>` or equivalent sharded concurrent map
- Font cache: `DashMap<ObjRef, Arc<ParsedFont>>` with interior read-through
- PagePlan cache: `DashMap<(SnapshotId, usize), Arc<PagePlan>>`

The `ExecutionContext` is cloneable per-task (each parallel render task gets its own copy with shared
budget counters using atomic operations). Cancellation propagates to all clones.

### Rayon ↔ asupersync bridge contract

The bridge between asupersync (async orchestration) and Rayon (CPU parallelism)
follows these invariants:

1. **Lifecycle ownership:** asupersync regions own the lifecycle of all work,
   including Rayon-dispatched compute. A Rayon job is always spawned from within
   an asupersync scope and its result is always collected back into that scope.

2. **Cancellation propagation:** ExecutionContext (derived from Cx) is passed into
   Rayon closures. Rayon work checks `exec_ctx.checkpoint.check()` at every
   content-stream operator, tile boundary, and resource decode point.

3. **No async in Rayon:** Rayon closures are purely synchronous. They never call
   `block_on()`, never create async runtimes, never hold async locks. The "async
   Rayon sandwich" (async → rayon → async → rayon) is forbidden.

4. **Oneshot bridge:** Results flow from Rayon to asupersync via oneshot channels.
   The asupersync task awaits the oneshot (cancellable); the Rayon closure sends
   the result when compute completes.

5. **Budget respecting:** Rayon work respects the same budget as the enclosing
   asupersync region. Budget exhaustion in Rayon triggers the same early-return
   as cancellation.

6. **Panic containment:** Rayon panics (from native decoders, malformed input) are
   caught at the Rayon scope boundary and converted to `Outcome::Panicked` in the
   asupersync region. They do not propagate across the bridge.

### Object identity

Every PDF object in the document model has a stable identity (object number + generation number for indirect objects). Object identity is preserved across parse → manipulate → serialize cycles. Reference resolution is explicit and traceable.

Object identity is layered:
- `ObjRef` is document-local identity only.
- `DocumentId + ObjRef` is cross-session object identity.
- `SnapshotId` identifies semantic state.
- `ResourceFingerprint` identifies immutable reusable artifacts across snapshots/documents.

No cache may key cross-document data by `ObjRef` alone.


### Root digest and substrate identity invariant

Every snapshot carries multiple digest surfaces:
- `syntax_digest` — normalized digest of the preserved/parsed COS layer
- `semantic_digest` — digest of the semantic object graph and ownership state
- `preserve_digest` — digest of all byte-preserved spans that remain relevant to writeback
- `lineage_digest` — digest of temporal ancestry and change-set linkage

Digest equality is stronger than object-number equality and weaker than byte-for-byte file identity.
It is the right granularity for structural sharing, cache reuse, semantic anchor stability,
certificate generation, and diff short-circuiting.

Required rule: no subsystem may claim two states are "the same" purely because the same `ObjRef`s
exist. Equivalence must always be stated in terms of explicit digests or explicit preservation
claims.

### Provenance preservation

The core document model preserves enough information about the original document structure that round-trip operations do not silently destroy structure. Canonicalization normalizes for sanity but does not erase provenance, geometry, ordering, or writeback-relevant metadata.

In preserve mode, the model additionally retains:
- Raw byte spans for unmodified objects (enabling byte-perfect write-back)
- Original whitespace and formatting
- Original cross-reference entry formats (table vs. stream)
- Object stream membership (which objects were packed into which object streams)

### Geometry guarantees

All subsystems that deal with spatial operations (rendering, annotation, extraction, editing) share the same coordinate geometry and transformation pipeline from `monkeybee-core`. No subsystem maintains its own private geometry stack that can drift.

### Mutation safety

The document model supports disciplined mutation: adding, modifying, and removing objects with explicit change tracking. Mutations produce a well-defined delta that the write path can serialize. Mutations do not corrupt the object graph or violate reference integrity.

Mutations occur inside an `EditTransaction` against an immutable `PdfSnapshot`; commit produces a new snapshot, never a partially mutated live document.
Each touched object is classified as:
- `Owned` — semantically understood and eligible for rewrite/canonicalization
- `ForeignPreserved` — carried forward byte-preservingly unless an edit explicitly takes ownership
- `OpaqueUnsupported` — detected but not safely transformable; incompatible edits must fail explicitly

Transaction flow:
1. Stage semantic edits
2. Compute the affected page/resource/reference closure
3. Run validation preflight
4. Either commit atomically as a delta or roll back

### Cross-document import invariant

Copy-page, merge, split, and resource-import workflows are first-class document
mutations over a source snapshot and a target snapshot. They are not treated as
an ad hoc serializer trick.

```
pub struct CrossDocumentImportPlan {
    pub source_snapshot: SnapshotId,
    pub target_snapshot: SnapshotId,
    pub source_objects: Vec<ObjRef>,
    pub imported_object_map: Vec<(ObjRef, ObjRef)>,
    pub collision_kinds: Vec<ImportCollisionKind>,
    pub policy_digest: [u8; 32],
    pub plan_selection_digest: Option<[u8; 32]>,
}

pub struct ImportedObjectProvenance {
    pub source_document_id: DocumentId,
    pub source_snapshot: SnapshotId,
    pub source_object: ObjRef,
    pub source_digest: NodeDigest,
    pub target_object: ObjRef,
    pub imported_closure_root: NodeDigest,
}

pub enum ImportCollisionKind {
    ResourceNameCollision,
    NamedDestinationCollision,
    FormFieldCollision,
    AnnotationNameCollision,
    EmbeddedFileCollision,
    SignatureFieldCollision,
    ActiveContentConflict,
}
```

Rules:
- imported objects receive fresh target-side `ObjRef`s; source object numbers are
  provenance, never identity inside the target document
- import closure analysis includes every dependent page-tree node, resource,
  annotation, form field, named destination, and catalog edge required for
  semantic validity in the target document
- the committed delta retains an auditable provenance map from source document
  state to target document state and exposes it through the compatibility ledger
- byte-preservation and object-identity claims do not cross document boundaries;
  only semantic/provenance claims may survive import
- collision handling is explicit and policy-aware; resource/name/field conflicts
  may be renamed, rejected, or caller-resolved, but never silently merged
- imported signatures, execute-capable actions, and restricted content do not
  silently gain authority in the target document; they are reclassified under
  the target policy and may be degraded or rejected explicitly

### Edit intent contract

Every `EditTransaction` declares an `EditIntent`:

```
enum EditIntent {
  ForensicPreserve,
  SafeIncremental,
  SemanticRewrite,
  CanonicalizeOwned,
  Optimize,
}
```

`EditIntent` constrains ownership escalation and write planning.

### EditTransaction validation rules

At `commit()` time, the transaction performs the following validations:

1. **Reference integrity:** Every indirect reference created or modified by the transaction must
   point to an existing object (either pre-existing in the snapshot or newly created in the
   transaction). Dangling references are rejected.

2. **Page tree validity:** If the transaction modified the page tree (add/remove/reorder pages),
   the resulting tree must have correct `/Count` values, valid `/Parent` back-references, and at
   least one leaf page. An empty page tree is rejected.

3. **Resource completeness:** For newly created or modified content streams, the resources
   referenced by the content stream operators must exist in the page's resource dictionary (or
   the resource dictionary must be updated in the same transaction). Missing resources produce a
   warning (not a rejection — missing resources are handled at render time), unless the
   transaction is for a generated document (where missing resources are an error).

4. **Ownership constraints:** Edits to `OpaqueUnsupported` objects are rejected unless the edit
   explicitly takes ownership (transitions to `Owned`).
   - In `ForensicPreserve` and `SafeIncremental`, edits to `ForeignPreserved` objects do **not**
     auto-escalate. The caller must explicitly call `take_ownership(objref, reason)`.
   - In `SemanticRewrite` and `CanonicalizeOwned`, ownership escalation is permitted and recorded.
   - Every ownership escalation produces an `OwnershipTransitionRecord` that is surfaced in the
     `WritePlan` and compatibility ledger.

5. **Structural cycle detection:** The dependency graph is checked for cycles introduced by the
   transaction (e.g., a form XObject that references itself). Cycles are rejected.

**Conflict detection:** Transactions are optimistically concurrent. Two transactions based on the
same source snapshot can both commit — the second commit detects conflicts by checking whether
any object it modified was also modified by the first (via snapshot_id comparison). Conflicts
are reported to the caller, who must resolve them (typically by rebasing the second transaction
on the new snapshot). The engine does not automatically merge conflicting transactions.

### Transaction lineage, rebase, and undo contract

```
pub struct TransactionIntent {
    pub edit_intent: EditIntent,
    pub human_reason: String,
    pub expected_write_mode: Option<WriteMode>,
    pub preserve_constraints: Vec<PreserveConstraint>,
}

pub struct ConflictSet {
    pub conflicting_objects: Vec<ObjRef>,
    pub affected_pages: Vec<u32>,
    pub signature_impact: SignatureImpactReport,
    pub structure_impact: Option<StructureEditRisk>,
}

pub struct RebasePlan {
    pub base_snapshot: SnapshotId,
    pub target_snapshot: SnapshotId,
    pub replayed_changes: Vec<ChangeEntry>,
    pub rejected_changes: Vec<RejectedChange>,
    pub new_ownership_transitions: Vec<OwnershipTransitionRecord>,
}

pub struct UndoJournalEntry {
    pub snapshot_before: SnapshotId,
    pub snapshot_after: SnapshotId,
    pub inverse_change_set: Vec<ChangeEntry>,
}
```

Required invariants:
- every committed transaction has a stable lineage record
- every user-visible conflict has an object-level `ConflictSet`
- rebasing is explicit, deterministic under deterministic mode, and auditable
- undo is implemented as ordinary forward movement to a new snapshot, never
  mutation of an existing snapshot

Resource GC, deduplication, unreachable-object pruning, and rewrite-time compaction are explicit edit operations, not incidental writer side effects.

Full-rewrite mode may canonicalize only `Owned` objects. Preserve-mode output must not silently take ownership of foreign or opaque structures.

The underlying change tracking model:
- Every mutation is recorded as a `ChangeEntry { object_id, old_fingerprint, new_value, reason,
  ownership_before, ownership_after, dependency_delta }`. This enables incremental save, undo,
  precise cache invalidation, and save-impact explanation.
- New objects are assigned object numbers from a monotonically increasing allocator.
  The allocator starts at `max_existing_object_number + 1` for the current snapshot.
  In incremental-append mode, new object numbers must not collide with any existing object
  (including objects in the free list). In full-rewrite mode, object numbers may be reassigned
  (compacted) starting from 1, since the entire xref is rebuilt. However, compaction changes
  all internal references and must update every indirect reference in the document. The
  baseline v1 writer does not compact object numbers in full-rewrite mode — it preserves
  existing numbers and adds new objects at higher numbers. Compaction is an explicit
  optimization operation in `monkeybee-edit`. Generation numbers for new objects are 0.
- Deleted objects are recorded in the free list. Their generation number is incremented.
- Reference integrity is maintained by a reference index: the model knows which objects reference which other objects, enabling orphan detection and referential integrity checks before save.


### Collaboration-ready delta invariant

Monkeybee v1 does **not** promise automatic multi-writer merge. The current refinement is more
foundational: transaction deltas must be serializable, causally ordered, and rich enough that a
future collaboration layer can be added without replacing the core mutation model.

This means every committed delta records:
- causal parent snapshot
- touched object digests before and after
- semantic intent and ownership transitions
- conflict set boundaries precise enough for deterministic rebase

A post-v1 CRDT or multiplayer layer may build on this. It is explicitly not allowed to force v1 to
adopt weaker local guarantees in exchange for speculative future collaboration.

### Ambiguity and hypothesis invariant

Any artifact derived from an ambiguous parse or repair path must carry hypothesis lineage until the
caller or engine policy has legitimately collapsed it. Render reports, extraction results,
diff reports, write plans, and proof ledgers for ambiguous files must all be traceable back to the
originating hypothesis set.

### Dependency and invalidation invariant

Every cached or derived artifact must be traceable to the objects and inherited state that produced it.
Invalidation is exact and versioned by snapshot:
- Edit an object -> invalidate only dependents reachable in the dependency graph
- Commit a new snapshot -> preserve cache entries for untouched subgraphs
- Traces report why each cache entry was reused or invalidated

This invariant ensures that the engine never serves stale caches and never grossly over-invalidates after small edits.



### Preservation algebra

Preservation is not a vague aspiration. Monkeybee models what each transform preserves,
invalidates, or intentionally destroys.

```
pub enum PreservedProperty {
    ByteRange { start: u64, end: u64 },
    ObjectIdentity(ObjRef),
    StructuralValidity,
    VisualEquivalence { page_index: u32 },
    SemanticEquivalence,
    TaggedStructureIntegrity,
    FormFieldSemantics,
    AnnotationGeometry,
    SignatureValidity(ObjRef),
}

pub enum TransformClass {
    AddAnnotation,
    FillField,
    SetMetadata,
    ReorderPages,
    FlattenAnnotation,
    Redact,
    IncrementalAppend,
    FullRewrite,
    Optimize,
}

pub struct PreservationClaim {
    pub property: PreservedProperty,
    pub verdict: PreservationVerdict,
    pub reason: String,
    pub evidence: Vec<CausalRef>,
}
```

Composition rules:
- if two transforms each preserve property `P` and their touch sets do not conflict, the composed
  transform preserves `P`
- signature safety is derived, not guessed: a save is signature-safe only if all signed byte ranges
  and policy-relevant structural claims remain preserved
- redaction is modeled explicitly as anti-preservation: the engine must prove what was removed and
  what surrounding properties still hold
- `WritePlan` classification is derived from preservation claims + ownership + write mode, not from
  ad hoc heuristics alone

Baseline v1 implements this with an auditable rule engine and receipts. Stronger external proof
mechanisms may be layered later, but the preservation algebra itself is baseline architecture.

### Semantic-equivalence normal form contract

`SemanticEquivalence` is only meaningful relative to a declared normal form and
explicit tolerance budget. Monkeybee MUST not claim semantic equivalence based on
intuition or raw object traversal.

```
pub enum SemanticNormalFormKind {
    DocumentStructure,
    PageVisualSemantics,
    TextExtraction,
    TaggedStructure,
    FormState,
    ImportProvenance,
}

pub struct NormalFormTolerance {
    pub geometry_epsilon_pt: f32,
    pub color_delta_e00: Option<f32>,
    pub allow_order_insensitive_sets: bool,
    pub allow_alias_map: bool,
}

pub struct SemanticNormalForm {
    pub kind: SemanticNormalFormKind,
    pub canonical_digest: [u8; 32],
    pub relevant_pages: Vec<u32>,
    pub tolerance: NormalFormTolerance,
    pub omitted_surfaces: Vec<String>,
}
```

Rules:
- every preservation claim that cites `SemanticEquivalence` MUST cite a
  `SemanticNormalForm` in its evidence or degrade to a narrower property claim
- normal forms intentionally ignore object numbers, xref layout, stream packing,
  serialization order, compression choices, and other byte-level artifacts that
  do not change meaning
- normal forms may include alias maps when safe rewrites or cross-document
  imports legitimately remap identities; such aliasing must be explicit
- proof mode pins tolerance defaults; looser tolerances must be visible in
  receipts, ledgers, and test expectations
- if a relevant semantic surface is unavailable or materially degraded, the
  engine must not claim semantic equivalence for that surface

### Save planning invariant

Before any write, Monkeybee computes a `WritePlan` that classifies each touched object as one of:
`PreserveBytes`, `AppendOnly`, `RewriteOwned`, `RegenerateAppearance`, `RequiresFullRewrite`, or
`Unsupported`.

`WritePlan` is surfaced to the API/CLI and to the compatibility ledger. Signature-safe workflows
must be explainable before bytes are emitted, not inferred after the fact.


`WritePlan` is therefore the executable frontier of the preservation algebra. For every touched
object and every caller-visible preservation claim, the plan records whether the engine will:
- preserve bytes verbatim
- append a superseding object without mutating old bytes
- rewrite owned semantics while preserving higher-level properties
- invalidate specific properties with an explicit, cited reason

### Signature safety proof artifact contract

Every successful write MUST optionally produce a `WriteReceipt`. For incremental
append workflows, `WriteReceipt` is not a convenience artifact; it is the
machine-readable attestation that the save respected preserve constraints.

```
pub struct BytePreservationMap {
    pub immutable_prefix_end: u64,
    pub preserved_ranges: Vec<(u64, u64)>,
    pub touched_ranges: Vec<(u64, u64)>,
    pub signed_ranges: Vec<(u64, u64)>,
}

pub struct SignedCoverageEntry {
    pub signature_ref: ObjRef,
    pub covered_ranges: Vec<(u64, u64)>,
    pub affected_objects: Vec<ObjRef>,
    pub invalidated: bool,
    pub invalidation_reason: Option<String>,
}

pub struct WriteReceipt {
    pub schema_version: String,
    pub snapshot_id: SnapshotId,
    pub write_mode: WriteMode,
    pub write_plan_digest: [u8; 32],
    pub policy_digest: [u8; 32],
    pub plan_selection_digest: Option<[u8; 32]>,
    pub pre_snapshot_digest: [u8; 32],
    pub post_snapshot_digest: [u8; 32],
    pub delta_digest: [u8; 32],
    pub bytes_appended: u64,
    pub preservation: BytePreservationMap,
    pub signature_coverage: Vec<SignedCoverageEntry>,
    pub ownership_transitions: Vec<OwnershipTransitionRecord>,
    pub invariant_certificate: Option<InvariantCertificate>,
    pub hypothesis_set: Option<HypothesisSetSummary>,
    pub post_write_validation: Vec<ValidationFinding>,
}
```

`WritePlan.execute()` SHOULD return:
`OperationSuccess<WriteResult { bytes, receipt: Option<WriteReceipt> }>`


### Invariant certificate contract

Every write, diff, redaction application, and history-replay export MAY additionally emit an
`InvariantCertificate`. In baseline form this is a Merkle/digest-backed receipt, not a theorem
prover artifact and not a zk dependency.

```
pub struct InvariantCertificate {
    pub certificate_version: String,
    pub before_digest: [u8; 32],
    pub after_digest: [u8; 32],
    pub delta_digest: [u8; 32],
    pub preserved: Vec<PreservationClaim>,
    pub invalidated: Vec<PreservedProperty>,
    pub trace_digest: [u8; 32],
    pub external_attestations: Vec<ExternalAttestationRef>,
}
```

Rules:
- baseline certificates are generated entirely inside Monkeybee with pinned digest algorithms
- optional external attestations (PKI timestamps, notarization, future zk proofs) attach to the
  certificate; they do not replace the baseline receipt schema
- certificate production is deterministic under deterministic mode
- proof harnesses can independently recompute certificate digests for validation

**WritePlan classification rules:**

Each object in the document is classified based on its ownership status and edit history:

1. **PreserveBytes:** The object was not modified in the current transaction, its ownership is
   `ForeignPreserved`, and the write mode is incremental-append. The object's original bytes are
   emitted verbatim. This is the default for incremental saves on unmodified objects.

2. **AppendOnly:** The object is new (created in the current transaction). It is serialized and
   appended in the incremental update section. Applies only in incremental-append mode.

3. **RewriteOwned:** The object was modified in the current transaction and its ownership is
   `Owned`. The object is re-serialized from its semantic representation. In incremental mode,
   the new version supersedes the old via the updated cross-reference. In full-rewrite mode,
   all `Owned` objects use this classification.

4. **RegenerateAppearance:** The object is a widget annotation whose parent field value changed.
   The appearance stream must be regenerated from the new field value before serialization.
   This is a specialized form of `RewriteOwned` that triggers appearance generation.

5. **RequiresFullRewrite:** The object cannot be safely emitted in incremental mode. Triggers
   include: object was deleted (must be recorded in free list in full rewrite, or in incremental
   xref), object's ownership is `OpaqueUnsupported` and an edit attempted to modify it, or the
   object participates in a structural change (page tree reorganization) that cannot be expressed
   as incremental updates.

6. **Unsupported:** The object's structure is not understood by the engine (ownership
   `OpaqueUnsupported`) and it was not modified. In full-rewrite mode, it is copied byte-for-byte
   from the original. In incremental mode, it is left untouched.

**Signature impact analysis:** The `WritePlan` computes which existing signatures (if any) would
be invalidated by the planned write. For each signature's `/ByteRange`, the plan checks whether
any classified object's original byte span overlaps. If yes, the signature is flagged as
invalidated. In incremental-append mode with only `PreserveBytes` and `AppendOnly` objects, no
existing signatures should be invalidated — if the plan detects otherwise, it reports an error
before any bytes are written.

`WritePlan` additionally records:
- `edit_intent`
- `ownership_transitions`
- `blocked_preserve_regions`
- `full_rewrite_reasons`
- `structure_impact`
- `accessibility_impact`
- `permission_impact`
- `policy_digest`
- `plan_selection_digest`
- `byte_patch_plan`

After `WritePlan`, the writer must compile a concrete `BytePatchPlan`:

```
BytePatchPlan {
  immutable_prefix_end: u64,
  preserved_spans: Vec<ByteSpan>,
  appended_segments: Vec<AppendedSegment>,
  signed_range_audit: Vec<SignedRangeCheck>,
  planned_startxref: u64,
  patch_sha256: [u8; 32],
}
```

`BytePatchPlan` is the last inspectable artifact before byte emission and MUST
be computable in dry-run mode.
Preserve-mode and signature-safe guarantees are made against `BytePatchPlan`, not only against
object-level classifications.

### Error taxonomy

All errors across the engine use a shared error taxonomy:
- **Parse errors**: malformed syntax, invalid structure, unrecoverable corruption
- **Semantic errors**: spec violations, unsupported features, incompatible combinations
- **Render errors**: missing resources, unsupported operators, visual degradation
- **Write errors**: serialization failures, structural invalidity
- **Round-trip errors**: post-save validation failures
- **Compatibility notes**: detected features handled at Tier 2 or Tier 3

Every error carries enough context for diagnostics, compatibility accounting, and debugging. Context includes: the object number (if applicable), byte offset (if applicable), the subsystem that generated the error, a human-readable description, and a machine-readable error code.

**Error code structure:**

Error codes are hierarchical strings: `{subsystem}.{category}.{specific}`. Examples:

- `parse.xref.wrong_offset` — cross-reference entry points to wrong byte offset
- `parse.xref.missing_entry` — referenced object has no cross-reference entry
- `parse.xref.circular_prev` — /Prev chain forms a cycle
- `parse.stream.decompression_failed` — filter chain decompression produced an error
- `parse.stream.wrong_length` — /Length value does not match actual stream data length
- `parse.stream.truncated` — stream data shorter than expected
- `parse.object.circular_reference` — object reference chain forms a cycle
- `parse.object.duplicate_key` — dictionary contains duplicate key (last-wins applied)
- `parse.encrypt.wrong_password` — provided password does not decrypt the file
- `parse.encrypt.unsupported_handler` — non-standard security handler detected
- `font.missing_tounicode` — font has no ToUnicode CMap; text extraction will use fallback
- `font.broken_embedded` — embedded font data is corrupt; using substitution
- `font.missing_glyph` — requested glyph not found in font; using .notdef
- `font.no_embedded_data` — font is not embedded and no system substitute found
- `font.encoding_conflict` — font dictionary encoding conflicts with embedded font encoding
- `render.transparency.unsupported_blend` — unknown blend mode encountered
- `render.color.missing_profile` — ICCBased color space references missing profile stream
- `render.color.profile_parse_failed` — ICC profile data is invalid
- `render.pattern.recursive` — tiling pattern references itself
- `render.image.decode_failed` — image data could not be decoded
- `render.xobject.postscript` — PostScript XObject detected (Tier 2/3)
- `write.xref.offset_mismatch` — self-consistency check: written offset doesn't match xref entry
- `write.stream.length_mismatch` — self-consistency check: stream length doesn't match /Length
- `compat.xfa.detected` — XFA form data detected
- `compat.xfa.dynamic_no_fallback` — dynamic XFA with no AcroForm fallback
- `compat.flash.detected` — Flash/RichMedia content detected
- `compat.postscript_xobject` — PostScript XObject detected

**Error severity levels:**

- **Fatal:** The operation cannot continue (e.g., cannot decrypt file, no valid xref found even after repair). The engine returns an error result.
- **Error:** A significant problem was encountered but the operation can continue with degraded results (e.g., a font is missing and substituted, an image cannot be decoded). The result includes degradation markers.
- **Warning:** A minor deviation from spec was detected and automatically handled (e.g., wrong stream length corrected, duplicate dictionary key resolved). The result is correct but the input was technically malformed.
- **Info:** A notable feature was detected that may be relevant for compatibility reporting (e.g., XFA detected, encryption present, optional content layers present).

All errors at all severity levels are collected into the compatibility ledger. Fatal errors terminate the current operation but do not crash the process.

### Content stream contract

Content stream parsing and interpretation follow a shared contract:
- The graphics state machine is implemented once in `monkeybee-content`.
- Rendering, extraction, and inspection all consume content streams through the same interpretation pipeline.
- The pipeline supports both "execute for rendering" and "analyze for extraction" modes without duplicated logic.
- The pipeline emits events (operator dispatched, state changed, text shown, path painted, etc.) that downstream consumers can subscribe to selectively.

**Content stream error recovery:**

When the content interpreter encounters an error in tolerant mode, recovery follows a defined protocol:

1. **Operator-level isolation:** A failing operator does not abort the page. The interpreter emits an
   Error event, discards the current operator's effects, and advances to the next operator.
2. **State rollback on failure:** If an operator partially modified the graphics state before failing
   (e.g., `gs` applied some ExtGState entries before hitting an invalid one), the interpreter rolls
   back to the state before that operator. This prevents half-applied state from corrupting
   subsequent rendering.
3. **Resource resolution failures:** If a `Do`, `Tf`, or `sh` operator references a resource that
   cannot be resolved, the interpreter skips the operator, emits an Error event with the resource
   name and type, and continues. For `Tf` (font), a fallback font is substituted so subsequent text
   operators don't crash.
4. **Inline image recovery:** If `BI`/`ID`/`EI` parsing fails (corrupted image data, wrong
   dimensions), the interpreter attempts to find the `EI` marker by scanning forward, skips the
   inline image, and continues. If `EI` cannot be found within a bounded scan (4096 bytes), the
   rest of the content stream is abandoned with a diagnostic.
5. **Stack underflow:** If `Q` is called with an empty graphics state stack, the interpreter resets
   to the page's initial graphics state and emits a warning. This is common in real-world PDFs with
   mismatched `q`/`Q` across concatenated content streams.
6. **Recursion limit:** Form XObject and tiling pattern nesting is bounded (default: 28 levels,
   matching Acrobat's limit). Exceeding the limit produces an Error event and the nested content
   is skipped.

The content subsystem exposes two surfaces built from the same interpreter:
1. **Streaming events** for low-memory, one-shot execution.
2. **PagePlan IR** for repeated, cached, or region-aware workflows.

`PagePlan` is an immutable page-scoped display list containing normalized draw ops, text runs/quads, resource dependencies, marked-content spans, degradation annotations, and source-span provenance.
Render, extract, inspect, diff, and edit subsystems may consume either surface; the interpreter remains the single source of truth.

**PagePlan structure:**
- `ops: Vec<DrawOp>` — normalized draw operations (FillPath, StrokePath, ClipPath, DrawImage,
  BeginGroup/EndGroup, BeginMarkedContent/EndMarkedContent) in page painting order.
- `text_runs: Vec<TextRun>` — positioned text with resolved Unicode, glyph IDs, per-glyph
  bounding quads, font reference, size, render mode, and color.
- `resource_deps: Set<ObjRef>` — all objects this page depends on, for cache invalidation.
- `marked_spans: Vec<MarkedSpan>` — marked content regions with tags and property references.
- `degradations: Vec<DegradationNote>` — operator-level errors or degradations encountered
  during interpretation.
- `provenance: Vec<SourceSpan>` — maps each op back to its byte offset in the content stream.

The PagePlan is the shared currency between subsystems: render consumes DrawOps and TextRuns
to produce visual output; extract consumes TextRuns for text extraction; inspect consumes
everything for structure analysis; diff consumes the full plan for page comparison.

**Event model:**

The content stream interpreter is structured as an event emitter. Each operator in the content stream produces one or more events. Consumers register for the event categories they need:

- **StateChange events:** Emitted whenever the graphics state changes. Includes: matrix change (from `cm`), color change (from color operators), text state change (from `Tc`, `Tw`, `Tz`, `TL`, `Tf`, `Tr`, `Ts`), line property change, ExtGState application, save/restore. Consumers: renderer (to update drawing state), extractor (to track text state).

- **PathPaint events:** Emitted when a path is stroked, filled, or used for clipping (`S`, `f`, `B`, `W`, etc.). Includes the fully constructed path (list of subpaths with segments), the paint mode (stroke/fill/clip), and the relevant graphics state snapshot (color, line properties, CTM, blend mode). Consumers: renderer (to rasterize), extractor (to analyze page layout geometry).

- **TextShow events:** Emitted for `Tj`, `TJ`, `'`, `"` operators. Includes: the raw byte string(s), decoded Unicode characters, per-character positions (in both text space and page space), font reference, font size, rendering mode, character spacing, and word spacing adjustments. Consumers: renderer (to draw glyphs), extractor (to collect text with positions).

- **ImagePaint events:** Emitted for `Do` (image XObject) and `BI`/`ID`/`EI` (inline image). Includes: decoded image data, color space, dimensions, the CTM mapping the image to page space, interpolation flag, and any soft mask. Consumers: renderer (to composite the image), extractor (to catalog images).

- **FormXObject events:** Emitted when `Do` invokes a form XObject. Includes: the form's resource dictionary, matrix, bounding box, and content stream reference. The interpreter recursively enters the form's content stream with the appropriate state setup. Consumers receive a "begin form" / "end form" bracket around the nested events.

- **MarkedContent events:** Emitted for `BMC`, `BDC`, `EMC`, `MP`, `DP`. Includes: tag name, properties dictionary (if present), and optional content group reference (for layer visibility). Consumers: extractor (for tagged PDF structure), renderer (for optional content visibility).

- **Error events:** Emitted when the interpreter encounters an invalid operator, wrong operand count, missing resource, or other error. In tolerant mode, the interpreter continues after the error. The error event includes the byte offset, the problematic operator, and a description. Consumers: all (for diagnostics and compatibility ledger).

The event model ensures single-pass interpretation: the content stream is parsed and interpreted exactly once, and all consumers receive the events they need simultaneously. This avoids the performance cost of re-interpreting the same content stream for different purposes (rendering + extraction + inspection).

---

## Part 5 — Subsystem contracts

### Object model contract

The PDF object model in `monkeybee-core` must faithfully represent all PDF 2.0 object types. Indirect objects carry object number + generation number. Streams carry the dictionary plus a byte-backed stream handle; decoded bytes live in engine-managed caches keyed by snapshot and filter chain, not inline in the object graph. The object graph supports forward and reverse reference lookups. Object access is zero-copy where practical, lazy by default for large/remote inputs, and always safe.

### StreamHandle contract

PDF streams are the engine's primary data-carrying objects. The `StreamHandle` type mediates
between the raw byte source and consumers that need decoded data:

```
StreamHandle {
  object_id: ObjRef,
  raw_span: ByteSpan,              // offset + length in the byte source
  filter_chain: Vec<FilterSpec>,   // ordered decode filters with parameters
  expected_decoded_length: Option<u64>,  // from content dimensions, when known
}
```

**Access patterns:**

1. **Raw bytes:** `handle.raw_bytes(byte_source) -> &[u8]` — returns the undecoded stream data
   directly from the byte source. Used by preserve-mode write, stream analysis, and hex dump.
2. **Decoded bytes:** `handle.decoded_bytes(engine_caches, exec_ctx) -> Arc<[u8]>` — returns
   decoded data through the engine's decode cache. The cache key is
   `(snapshot_id, object_id, filter_chain_hash)`. If the cache misses, the handle orchestrates
   decode through `monkeybee-codec`, respecting the `exec_ctx` security profile and budgets.
3. **Streaming decode:** `handle.decode_stream(byte_source, exec_ctx) -> impl Read` — returns a
   streaming reader for large streams where materializing the full decoded output is unnecessary
   or too expensive. Used by image rendering (progressive JPEG) and large content stream parsing.

**Invariants:**
- `StreamHandle` is `Clone + Send + Sync`. It carries no decoded data — only the metadata needed
  to locate and decode the stream.
- Decoded data is always `Arc<[u8]>` (shared, immutable). Multiple consumers can hold references
  to the same decoded data without coordination.
- The filter chain is validated at parse time. Invalid filter names produce a diagnostic and the
  handle records the failure. Attempting to decode a handle with an invalid filter chain returns
  an error without panicking.
- For object streams, the `StreamHandle` of the container stream is resolved first; individual
  objects within the stream are extracted by offset after decode.

**Invariants:**
- Every indirect object has a unique (object_number, generation_number) pair.
- Resolving an indirect reference always terminates (no infinite loops). The resolver maintains a visited set per resolution chain and produces an error for circular references.
- Dictionary key order is preserved (insertion order) for provenance preservation, but semantics are order-independent.
- Stream data is lazily decoded: raw bytes are always available through byte spans or range-backed
  sources; decoded bytes are produced on demand and may be cached outside the semantic object graph.

`PdfSnapshot` must use structural sharing. Opening a new snapshot or saving a delta must not imply
cloning the full object store.
- The object model is thread-safe for read access. Write access requires exclusive ownership (enforced by Rust's ownership model).
- Null objects and free-list objects are distinguishable: a reference to a free object resolves to null, but a null value stored directly in a dictionary is a different semantic state.

**Data flow:** Raw bytes → parser → object model (in core) → consumers (renderer, extractor, writer, annotator). The object model is the single shared representation.

**Edge cases:**
- Object streams (PDF 1.5+): multiple objects packed into a single compressed stream object. The parser must extract individual objects from the stream. Object numbers within object streams do not have independent generation numbers (always 0).

**Object stream extraction contract:**

An object stream is a stream object with `/Type /ObjStm`. It contains:
- `/N`: the number of objects in the stream
- `/First`: the byte offset within the decoded stream data where the first object begins
  (everything before `/First` is the index)
- The decoded stream data contains: first an index of N pairs of (object_number, byte_offset)
  as space-separated integers, then the objects themselves starting at byte offset `/First`

**Extraction algorithm:**
1. Decode the stream through its filter chain (typically FlateDecode).
2. Parse the first `/First` bytes as the index: read N pairs of (object_number, byte_offset).
   Each byte_offset is relative to `/First` (not to the start of the stream data).
3. For each object, seek to `/First` + byte_offset and parse the object value. Objects in
   object streams do NOT have the `N G obj ... endobj` wrapper — they are bare values.
4. Register each extracted object in the cross-reference with its object number and generation
   number 0 (objects in object streams always have generation 0).

**Edge cases:**
- Index corruption: if the index cannot be parsed (wrong number of entries, non-numeric values),
  the entire object stream is skipped and all its objects are treated as missing. Record
  diagnostic `parse.objstream.corrupt_index`.
- Offset out of range: if a byte_offset points beyond the stream data, that individual object
  is skipped. Other objects in the same stream are still extracted.
- Nested object streams: an object stream containing another object stream is malformed per
  spec. Detect and report; do not attempt recursive extraction.
- The xref stream itself cannot be in an object stream.
- Self-referencing objects: a dictionary that references itself (directly or indirectly). Must not cause infinite loops during traversal, serialization, or garbage collection.
- Extremely large arrays or dictionaries (100,000+ entries): the model must handle these without O(n²) behavior on lookup. Use efficient data structures (hash maps for dictionaries, vectors for arrays).

### Cross-reference and update contract

Cross-reference tables and streams must be parsed, repaired when malformed, and regenerated on save. Incremental updates must be parsed as a chain and merged into the object graph with correct precedence. The write path must support both full-rewrite (rebuild xref from scratch) and incremental-append (add new xref section) save modes.

**Invariants:**
- After parsing, the merged cross-reference provides a single authoritative mapping from object number to byte offset (or object stream location). Later incremental updates override earlier ones.
- In tolerant mode, every repair action is recorded: which entries were repaired, what the original (wrong) value was, what the discovered (correct) value is, and what repair strategy found it.
- In preserve mode, the original cross-reference structure (table vs. stream, entry order, formatting) is recorded so that incremental saves can produce a structurally compatible append section.
- The write path's full-rewrite mode produces a single cross-reference section covering all live objects. No free entries except the mandatory head-of-free-list entry (object 0).
- The write path's incremental-append mode produces a cross-reference section covering only new/modified/deleted objects, with a `/Prev` pointer to the existing cross-reference.

**Failure modes:**
- Cross-reference pointing to middle of an object (wrong offset): recovered by scanning for `N 0 obj`.
- Cross-reference entry count mismatch (declared N entries, actual M entries): use the actual count, record diagnostic.
- Hybrid cross-reference (both table and stream in the same file): cross-validate, prefer stream entries for objects present in both.
- Object stream containing objects that are also listed as non-stream objects in the xref: prefer the xref table's byte-offset entry (it is more likely to be the latest version).

### Page inheritance contract

Page attributes (MediaBox, CropBox, Resources, Rotate, etc.) are inherited down the page tree. The core must resolve inherited attributes explicitly so that consumers (renderer, extractor, annotator) always see the effective values, not raw tree fragments.

**Inheritable attributes** (per ISO 32000-2 §7.7.3.4): `Resources`, `MediaBox`, `CropBox`, `Rotate`. Note: `BleedBox`, `TrimBox`, and `ArtBox` are NOT inheritable — they must be specified on the page object itself. A common producer bug is placing these on a parent node; the engine should handle this gracefully in tolerant mode (accept the inheritance) while flagging it as non-conforming in strict mode.

**Resolution rules:**
1. Look at the page object for the attribute.
2. If not present, walk up the page tree via `/Parent` references.
3. Stop at the first ancestor that defines the attribute.
4. If no ancestor defines it, use the default: for `Resources`, an empty dictionary; for `MediaBox`, error (required); for `CropBox`, defaults to `MediaBox`; for `Rotate`, 0.

**Page box relationships:** `CropBox` ≤ `MediaBox`. `BleedBox` defaults to `CropBox`. `TrimBox` defaults to `CropBox`. `ArtBox` defaults to `CropBox`. The renderer clips to `CropBox` for normal viewing. The write path must ensure these relationships hold for generated output.

### Font and encoding contract

Font handling must resolve: font dictionary → encoding → ToUnicode → glyph selection → metrics. The engine must handle Type 1, TrueType, OpenType/CFF, CIDFont (Type 0), and Type 3 fonts. Missing or broken ToUnicode maps must be detected and reported. Fallback glyph selection must be principled, not silently wrong. Font subsetting must be supported for the write path.

**Detailed resolution chain:**

1. **Font dictionary lookup:** The text operator `Tf /FontName size` selects a font. `/FontName` is looked up in the page's (resolved) resource dictionary under `/Font`. The result is a font dictionary.

2. **Font type dispatch:** Examine `/Subtype` in the font dictionary:
   - `/Type1` → Type 1 path
   - `/TrueType` → TrueType path
   - `/Type3` → Type 3 path
   - `/Type0` → Composite font path (CIDFont)
   - `/MMType1` → Multiple Master Type 1 (rare, Tier 2)
   - `/CIDFontType0` or `/CIDFontType2` → These appear only as descendants of Type 0; direct use is malformed.

3. **Encoding resolution (simple fonts — Type 1, TrueType, Type 3):**
   - Check `/Encoding` in the font dictionary.
   - If `/Encoding` is a name (`WinAnsiEncoding`, `MacRomanEncoding`, `MacExpertEncoding`): use the named encoding's code-to-name mapping.
   - If `/Encoding` is a dictionary: check `/BaseEncoding` for the base mapping, then apply `/Differences` array overrides.
   - If `/Encoding` is absent: for Type 1, use the font's built-in encoding; for TrueType, use the `cmap` table (platform 1 encoding 0 or platform 3 encoding 1, depending on the symbolic flag in `/FontDescriptor`).
   - **Edge case:** TrueType fonts with the symbolic flag set but no built-in encoding. Fall back to the `cmap` table's first available encoding.
   - **Font-descriptor flag cross-check:** The `/FontDescriptor` `/Flags` bits (FixedPitch,
     Serif, Symbolic, Script, NonSymbolic, Italic, AllCap, SmallCap, ForceBold) must be checked
     against the embedded font program and reported when they disagree. This is especially
     important for Symbolic vs NonSymbolic because it changes which `cmap` subtables are valid for
     extraction and glyph lookup.

4. **Encoding resolution (composite fonts — Type 0 → CIDFont):**
   - The Type 0 font's `/Encoding` names a CMap (predefined name or embedded stream).
   - The CMap maps character codes (1-4 bytes) to CIDs.
   - The CIDFont's `/CIDToGIDMap` (for CIDFontType2) maps CIDs to glyph IDs.
   - For CIDFontType0 (CFF-based), CIDs directly index the CFF charstring INDEX.
   - Vertical writing metrics from `/W2` and `/DW2` must be honored when present, including the
     vertical origin offsets and vertical advances needed for correct CJK vertical layout.

5. **ToUnicode mapping:** Applied regardless of font type. If `/ToUnicode` is present, it overrides all other Unicode mapping paths. The ToUnicode CMap's `beginbfchar`/`beginbfrange` entries map character codes (in the content stream encoding) to Unicode values. **The ToUnicode CMap uses the original character codes from the content stream, not the CIDs.** This is a common point of confusion.

6. **Glyph outline retrieval:** Based on font type and the resolved glyph ID / glyph name / CID, extract the glyph outline from the font program (embedded or substituted).

7. **Width retrieval:** The font dictionary's `/Widths` array (simple fonts) or `/W` array (CIDFonts) provides glyph advance widths in glyph space units (typically 1/1000 of the font size for 1000-unit-per-em fonts). These widths are authoritative for text positioning, regardless of what the embedded font program says.

**Invariants:**
- Text positioning is always correct if the font dictionary's widths are used, even if glyph outlines are wrong or substituted.
- Unicode extraction succeeds as far as the available mapping allows. Unmappable characters are flagged, not silently dropped.
- The font cache maps (font_dictionary_object_id) → parsed font data. Font data is parsed once and reused across pages.
- Font subsetting for the write path: when embedding fonts in generated PDFs, include only the glyphs actually used. Update the `/Widths` or `/W` array to cover only the used glyph set. For TrueType, regenerate the `cmap`, `glyf`, `loca`, `hmtx` tables with only the used glyphs. For CFF, regenerate the CharStrings INDEX.

### Graphics state contract

The graphics state machine (CTM, color state, text state, line properties, clipping path, blend mode, transparency, overprint, rendering intent) must be implemented as a single shared stack machine. Push/pop via `q`/`Q` operators must be precise. State inheritance across content streams (page → form XObject → tiling pattern) must follow the spec.


The refinement here is that the graphics state is specified as a transition algebra, not merely as a
mutable bag of fields.

```
pub struct GraphicsTransition {
    pub before: GraphicsStateDigest,
    pub after: GraphicsStateDigest,
    pub emitted_effects: Vec<InterpreterEffect>,
}

fn transition(state: &GraphicsState, op: &Operator) -> TransitionResult;
```

Required algebraic properties:
- `q`/`Q` form a well-bracketed stack discipline; imbalance is diagnosable and recoverable only via
  explicit error-recovery rules
- CTM composition is associative in operator order; derived geometry consumers must all consume the
  same composed transform
- clipping is monotone within a save/restore frame
- render, extract, inspect, and edit consumers all observe the same operator transition stream and
  differ only in their effect handlers

This is one of the best ideas from the competing plans because it turns the current excellent but
primarily descriptive graphics-state section into something derivable, property-testable, and safe
for optimizer work.

**The graphics state contains:**

*Device-independent state:*
- **CTM** (Current Transformation Matrix): 3×2 affine matrix. Modified by `cm`. Saved/restored by `q`/`Q`.
- **Clipping path**: the intersection of all clipping paths established by `W`/`W*` operators. Modified by `W`/`W*` followed by a path painting operator. Saved/restored by `q`/`Q`. Note: the clipping path can only shrink, never grow, within a single save/restore level.
- **Color space and color** (stroke and fill separately): set by `CS`/`cs` and the various color operators. Default: DeviceGray, 0.0 (black).
- **Text state**: character spacing (`Tc`), word spacing (`Tw`), horizontal scaling (`Tz`), leading (`TL`), font and size (`Tf`), rendering mode (`Tr`), text rise (`Ts`). These persist across `BT`/`ET` boundaries but are saved/restored by `q`/`Q`.
- **Text matrix and text line matrix**: set by `Tm`, modified by `Td`/`TD`/`T*`/`Tj`/`TJ`/`'`/`"`. These are reset to identity at each `BT`.
- **Line width** (`w`), **line cap** (`J`), **line join** (`j`), **miter limit** (`M`), **dash pattern** (`d`)
- **Rendering intent** (`ri`): `AbsoluteColorimetric`, `RelativeColorimetric`, `Perceptual`, `Saturation`
- **Flatness** (`i`): controls curve-to-line-segment conversion tolerance
- **Smoothness**: controls shading interpolation accuracy (set via ExtGState)

*Device-dependent state (from ExtGState dictionary via `gs` operator):*
- **Overprint** (`/OP` for stroke, `/op` for fill) and **overprint mode** (`/OPM`)
- **Blend mode** (`/BM`): one of the 16 standard blend modes, or an array for specifying a preference list
- **Soft mask** (`/SMask`): a soft-mask dictionary referencing a transparency group
- **Stroke alpha** (`/CA`) and **fill alpha** (`/ca`)
- **Alpha source flag** (`/AIS`)
- **Transfer function** (`/TR`, `/TR2`): adjusts the relationship between device color components and output
- **Halftone** (`/HT`): controls halftone screening for raster output
- **Black generation** (`/BG`, `/BG2`) and **undercolor removal** (`/UCR`, `/UCR2`): CMYK-specific adjustments
- **Font** (`/Font`): array of [font size] — rarely used in ExtGState, usually set by `Tf`

When `/BM` is an array, the renderer must treat it as a preference list and select the first blend
mode it actually supports under the active backend/profile. Falling through the array without a
match is an explicit degradation and must be reported rather than silently assuming `Normal`.

**Operator dispatch categories for the shared graphics state machine:**

The interpreter receives each operator and dispatches to the appropriate handler. The categories are:

1. **Graphics state operators** (`q`, `Q`, `cm`, `w`, `J`, `j`, `M`, `d`, `ri`, `i`, `gs`): modify the current graphics state. The `gs` operator requires resolving the named ExtGState from the page resources and applying all its entries to the current state.

2. **Path construction operators** (`m`, `l`, `c`, `v`, `y`, `h`, `re`): build the current path. These do not produce any visible output. The path exists only in the current path register.

3. **Path painting operators** (`S`, `s`, `f`, `F`, `f*`, `B`, `B*`, `b`, `b*`, `n`): consume the current path and produce visible output (or, for `n`, just clear the path). In render mode, these rasterize/stroke/fill the path. In extraction mode, these are typically ignored (except for clipping analysis).

4. **Clipping operators** (`W`, `W*`): modify the clipping path. These must be followed by a path painting operator; the clipping takes effect after the paint. Common misunderstanding: `W n` (clip with the current path, then discard the path without painting) is the idiomatic way to set a clipping path without visible output.

**Path rendering specifics:**

The PDF path model is built on cubic Bézier curves and straight-line segments. Key rendering considerations:

*Winding rules:* The nonzero winding number rule (`f`, `B`, `W`) counts the direction of path crossings: a point is inside if the total winding count is nonzero. The even-odd rule (`f*`, `B*`, `W*`) simply counts crossings: a point is inside if the crossing count is odd. Self-intersecting paths and paths with holes behave differently under these two rules. Both must be correctly implemented.

*Stroke geometry:* Stroking a path generates a new shape by expanding the path by half the line width on each side. At path joins, the join style (miter, round, bevel) determines the shape. At path endpoints (open subpaths), the cap style (butt, round, square) determines the termination shape. Miter joins that exceed the miter limit are automatically converted to bevel joins. Dash patterns convert a continuous stroke into a series of dashes; the dash phase specifies the starting offset into the dash pattern. Stroke expansion is computed analytically (offset curves for Bézier segments, with exact arc segments for round joins/caps) rather than by polygon approximation — this feeds directly into the exact analytic area coverage rasterizer for artifact-free thin-line rendering at any scale.

*Degenerate paths:* Zero-length subpaths (moveto followed immediately by another moveto or closepath) produce output only if the line cap is round or square (per the spec, they produce a dot). Zero-width strokes (`w 0`) are drawn as the thinnest possible line (1 device pixel). These edge cases are common in real-world PDFs and must be handled correctly. The robust geometric predicates (Part 7) ensure that degenerate and near-degenerate path configurations are handled without floating-point artifacts.

*Rectangle shortcut:* The `re` operator (x y w h re) creates a closed rectangular subpath. This is by far the most common path construction in PDFs (used for backgrounds, table cells, form fields, clipping regions). The renderer should fast-path rectangle detection for both rasterization and clipping.

#### Advanced path geometry operations

- **Exact offset curves for stroke expansion:** Stroke expansion is computed using algebraic
  offset curves for cubic Bézier segments, not polygon approximation. For a cubic `P(t)` with
  normal `N(t)` and half-width `w`, the offset curve is `P(t) + w·N(t)`. This is a degree-5
  rational curve; it is approximated by a sequence of cubics with error bounded by the flatness
  tolerance.
- **Minkowski sum for round joins/caps:** Round line joins and round line caps are computed as
  exact circular arcs (quadratic rational Bézier segments) rather than polygon approximations.
  This produces mathematically perfect round geometry at any zoom level.
- **Path boolean operations:** Union, intersection, and difference of arbitrary path regions,
  needed for advanced clipping composition and content-stream optimization. Uses the
  Weiler-Atherton or Greiner-Hormann algorithm with Shewchuk predicates for robustness.
- **Arc-length parameterization for dash patterns:** Dash patterns require computing arc length
  along Bézier curves. Use Gauss-Legendre quadrature (5-point, exact for degree ≤9 polynomials)
  for arc length estimation, with adaptive subdivision when the integrand varies too rapidly.

5. **Text operators** (`BT`, `ET`, `Tc`, `Tw`, `Tz`, `TL`, `Tf`, `Tr`, `Ts`, `Td`, `TD`, `Tm`, `T*`, `Tj`, `TJ`, `'`, `"`): text state manipulation and text rendering. In render mode, these resolve fonts, decode strings, position glyphs, and draw them. In extraction mode, these produce character/position records.

6. **Color operators** (`CS`, `cs`, `SC`, `SCN`, `sc`, `scn`, `G`, `g`, `RG`, `rg`, `K`, `k`): set the current color state for subsequent painting operations.

7. **XObject and shading operators** (`Do`, `sh`): invoke external resources. `Do` requires resolving the named XObject from resources and dispatching based on its subtype (Image, Form, or PS). Form XObjects require recursive content stream interpretation with the form's own matrix and resources. `sh` paints the current area with a shading pattern.

8. **Inline image operators** (`BI`, `ID`, `EI`): define and render an inline image. The image dictionary is between `BI` and `ID`; the raw image data is between `ID` and `EI`. Inline image dictionaries use abbreviated key names: `/W` for `/Width`, `/H` for `/Height`, `/BPC` for `/BitsPerComponent`, `/CS` for `/ColorSpace` (with abbreviations: `/G` = DeviceGray, `/RGB` = DeviceRGB, `/CMYK` = DeviceCMYK, `/I` = Indexed), `/F` for `/Filter` (with abbreviations: `/AHx` = ASCIIHexDecode, `/A85` = ASCII85Decode, `/LZW` = LZWDecode, `/Fl` = FlateDecode, `/RL` = RunLengthDecode, `/CCF` = CCITTFaxDecode, `/DCT` = DCTDecode), `/D` for `/DecodeParms`, `/DP` for `/DecodeParms`. Finding the `EI` marker is non-trivial because the image data itself may contain the bytes `EI` — the parser must track the expected data length (from Width × Height × BPC × components, accounting for filters) to know where the data ends.
   - Tolerant mode must also detect the producer bug where an inline image references a full
     colorspace name from the page/resource dictionary instead of only the abbreviated inline-image
     forms. When that happens, resolve through the resource dictionary, render if safe, and emit an
     explicit diagnostic rather than failing silently.

**EI detection algorithm:**

Finding the `EI` operator that terminates inline image data is non-trivial because the image
data can contain the bytes `E`, `I` in sequence. The algorithm:

1. **Compute expected data length** from the image parameters: `ceil(W × BPC × components / 8)
   × H`, where W = width, H = height, BPC = bits per component, components = number of color
   components. For filtered images, this is the decoded length; the encoded length may differ.

2. **For unfiltered images:** Skip exactly the computed number of bytes after `ID` (plus the
   mandatory single whitespace byte after `ID`). Verify that the bytes at that position are
   `EI` preceded by whitespace. If so, accept.

3. **For filtered images (encoded length unknown):** Scan forward from the `ID` data start,
   looking for the pattern: whitespace + `E` + `I` + (whitespace or end-of-stream). At each
   candidate position, verify:
   a. The byte before `E` is whitespace (SP, LF, CR, TAB, NUL, FF)
   b. The byte after `I` is whitespace or triggers the next operator parse
   c. The data between `ID` and the candidate `EI` is a valid encoded stream (attempt a trial
      decode — if decompression succeeds and produces a plausible image, accept this position)

4. **Fallback:** If no valid `EI` is found within a bounded scan (default: 1 MiB of data after
   `ID`), the inline image is abandoned. The interpreter emits an error diagnostic and attempts
   to resynchronize by scanning for the next known operator keyword.

5. **Edge case — empty inline images:** An inline image with W=0 or H=0 has zero data bytes.
   The `EI` immediately follows the whitespace after `ID`.

**Image rendering specifics:**

Image XObjects (`/Subtype /Image`) require:
1. Decode the image data through the filter chain.
2. Apply the `/Decode` array: this maps the raw sample values to a range in the color space. For example, a `/Decode [1 0]` on a DeviceGray image inverts it.
3. Apply the color space conversion: the image's `/ColorSpace` determines how sample values become colors. For Indexed images, look up each sample in the lookup table, then resolve the base color space.
4. Apply the image matrix: the image is rendered into a 1×1 unit square in user space. The CTM at the time of the `Do` operator maps this to the desired position and size. Common transformation: `width 0 0 height x y cm` before `Do` to place an image at position (x,y) with size (width,height).
5. Apply interpolation: the `/Interpolate` flag requests smooth interpolation when the image is scaled. When true, use bilinear or bicubic interpolation. When false (default), use nearest-neighbor.
6. Apply the image mask: if `/ImageMask` is true, the image is a stencil mask (1-bit, no color space). Painted pixels use the current fill color; unpainted pixels are transparent.
7. Apply soft mask from `/SMask`: the soft mask image (another image XObject, typically DeviceGray) provides per-pixel alpha values.
8. Apply the Matte array: if the image has been pre-multiplied with a matte color (indicated by the `/Matte` entry in the soft mask), the renderer must un-premultiply before compositing.

9. **Marked content operators** (`BMC`, `BDC`, `EMC`, `MP`, `DP`): semantic markers for tagged PDF and optional content. In render mode, these control optional content visibility (layers). In extraction mode, these provide structure hints.

10. **Compatibility operators** (`BX`, `EX`): allow skipping unknown operators. Between `BX` and `EX`, any operator not recognized by the interpreter is silently skipped along with its operands (operand count inferred from the stack).

**State inheritance across content streams:**

When a form XObject is invoked via `Do`, a new sub-interpretation context is created:
1. The graphics state is saved (implicit `q`).
2. The form's `/Matrix` is concatenated to the CTM.
3. The form's `/Resources` override the page's resources for the duration of the form.
4. The form's content stream is interpreted.
5. The graphics state is restored (implicit `Q`).

For tiling patterns, the process is similar but the content stream is interpreted repeatedly (once per tile), with the pattern matrix applied. Color state within a tiling pattern depends on the pattern's paint type: PaintType 1 (colored) uses its own colors; PaintType 2 (uncolored) inherits the current color from the invoking context.

### Annotation contract

Annotations must be resolved with their appearance streams. If an appearance stream exists, use it. If not, generate one. Annotation geometry is in the annotation's coordinate space, mapped to page space via the annotation's Rect and optional matrix. The annotate crate must use the shared geometry pipeline from core. Round-trip preservation means: the original annotation survives save/reload without geometry drift or content loss.

**Appearance stream resolution:**

1. Check the annotation's `/AP` (appearance) dictionary.
2. `/AP` contains up to three entries: `/N` (normal), `/R` (rollover), `/D` (down). Each can be either a single stream (form XObject) or a dictionary mapping state names to streams (for annotations with multiple states, like checkbox widgets).
3. For rendering, use `/N` for the normal appearance. If `/N` is a dictionary, use the entry matching the annotation's `/AS` (appearance state) value.
4. If `/AP` is absent or the needed entry is missing, generate an appearance stream. The generation requirements are annotation-type-specific.

**Appearance stream generation requirements per annotation type:**

*Text (sticky note):* Generate a small icon at the annotation's `/Rect` position. The icon style depends on the `/Name` entry (Comment, Key, Note, Help, NewParagraph, Paragraph, Insert). The appearance is a form XObject containing vector art for the icon, colored with the annotation's `/C` (color) entry.

*FreeText:* Generate a text block within the annotation's `/Rect`. The text content is from `/Contents` (or `/RC` for rich text). The `/DA` (default appearance) string provides the font and size (parsed as a content stream fragment, typically `"/FontName size Tf color rg"`). The `/Q` entry controls alignment (0=left, 1=center, 2=right). Border style from `/BS` or `/Border`.

*Line:* Generate a line from `/L` [x1 y1 x2 y2] with optional line endings (`/LE` — e.g., OpenArrow, ClosedArrow, Circle, Diamond, Square, Butt, None). The appearance stream must draw the line and any ending shapes, using the annotation's color and border width.

*Square / Circle:* Generate a rectangle/ellipse within `/Rect`. Apply `/IC` (interior color) for
fill and `/C` for stroke. Border from `/BS`. If the annotation carries a `/BE` border-effect
dictionary with the cloudy style, generate a cloudy border geometry rather than a plain stroke;
cloud intensity must affect the arc/radius sequence along the boundary and be preserved through
round trip even when appearance regeneration is deferred.

*Polygon / PolyLine:* Generate a path from the `/Vertices` array. For Polygon, close the path. Apply interior and border colors.

*Highlight / Underline / Squiggly / StrikeOut (text markup annotations):* The `/QuadPoints` array defines the quadrilaterals covering the marked text. Each group of 8 numbers defines 4 points (x1 y1 x2 y2 x3 y3 x4 y4). The appearance stream must draw the appropriate markup within each quadrilateral: highlight = semi-transparent fill; underline = line along the bottom edge; squiggly = wavy line along the bottom; strikeout = line through the vertical center.

*Stamp:* Generate the stamp content. For standard stamps (`/Name` = Approved, Experimental, NotApproved, AsIs, Expired, NotForPublicRelease, Confidential, Final, Sold, Departmental, ForComment, TopSecret, Draft, ForPublicRelease), render the stamp text with appropriate styling. Custom stamps may include an appearance stream; if provided, use it.

*Ink:* Generate paths from the `/InkList` array. Each entry in the array is a series of coordinate pairs defining a freehand stroke. The appearance stream draws each stroke using the annotation's color and width.

*Redact:* The redaction annotation has two visual states: before application (shows the region to be redacted, typically with a red outline or crosshatch) and after application (fills the region with the overlay color, removes underlying content). Before-application appearance: draw the region outline. After-application appearance: fill the `/RO` (redaction overlay) area with `/IC` and optional `/OverlayText`.

Redaction annotation creation lives in `monkeybee-annotate`; redaction application lives in `monkeybee-edit` and is treated as a high-assurance rewrite.
When a redaction is applied, the engine first constructs a `RedactionPlan` and selects one of:
- `SemanticExact` — only when complete semantic removal can be proven (all content operators within the redacted region are fully identified and removable without side effects)
- `SecureRasterizeRegion` — replace the affected region with a safe raster surrogate when exact semantic removal cannot be proven (e.g., reused XObjects, partial image overlap, complex transparency)
- `SecureRasterizePage` — rebuild the entire page as a safe raster surrogate when region-level proof is insufficient

Post-apply verification is mandatory: extraction must not recover redacted text, and surviving
canary bytes/resources must be scanned aggressively rather than only where convenient.

The redaction assurance path must include a **redaction canary scanner** that accepts the original
redacted text (or caller-supplied canary fragments) and searches the entire emitted file for
surviving fragments. This scan is not limited to visible page content. It must inspect raw bytes
and parsed structures including string objects, name objects, metadata streams, XMP packets,
outline/bookmark titles, annotation contents, form field values, font CMaps, ToUnicode maps, file
attachment names, and other preserved metadata-bearing surfaces. The report must say whether the
match was found in visible content, hidden structure, metadata, or opaque byte ranges so callers
can distinguish "safe by proof" from "probably removed."

`apply_redactions()` returns a `RedactionAssuranceReport`:
- selected apply mode
- text-extraction verification result
- resource/canary leakage verification result
- unresolved risks
- proof artifact references

Default policy is fail-closed: if the achieved assurance level is below caller policy, the save is rejected.

The applied redaction must be irreversible — the original content must not be recoverable from the saved file. This means: do not merely cover the content with an opaque rectangle (the content would still be extractable). The selected apply mode guarantees actual removal or destruction of underlying content.

*Widget (form field):* Appearance depends on the field type. Text fields: render the field value text within the widget rect using `/DA`. Checkboxes: render a check mark or empty box. Radio buttons: render a filled or empty circle. Choice fields: render the selected value or a dropdown indicator. Button fields: render the button label. The `/MK` (appearance characteristics) dictionary provides rotation, border color, background color, caption, and icon information.

**Tagged PDF awareness:**

Tagged PDFs contain a structure tree (`/StructTreeRoot` in the catalog) that maps content to logical elements (paragraphs, headings, tables, figures, lists). Tagged-structure preservation is a gated sub-feature of preserve mode: any edit that claims not to damage existing semantic structure must preserve the structure tree. While Monkeybee v1 does not perform accessibility remediation (generating tags for untagged PDFs), it must:

1. **Preserve existing tags** during round-trip operations. The structure tree, its elements, and the marked content references in content streams must survive load-modify-save cycles.
2. **Extract tagged structure** when present. The extraction pipeline should report the structure tree as part of the document inspection output: element types, nesting, associated content, and alt text.
3. **Use tagged structure for extraction hints.** When a structure tree is present, use it to improve reading order accuracy and table detection (table elements have `TR`/`TH`/`TD` structure elements).
4. **Not corrupt tags during annotation.** When adding annotations to tagged PDFs, the annotations should be added outside the existing structure tree (annotations have their own structure element type). The existing tagged content must not be disrupted.

### Structure tree preservation contract

The structure tree (`/StructTreeRoot`) contains structure elements that reference page content
via Marked Content IDs (MCIDs) and Marked Content References (MCR). Preserving this linkage
during round-trips requires:

1. **MCID stability:** When a content stream is re-serialized without modification, the MCIDs
   within it must be preserved exactly (same numeric values, same positions relative to the
   content they mark). The content stream rewriter in `monkeybee-edit` must preserve BMC/BDC/EMC
   operators and their MCID properties for any content that is not being edited.

2. **Structure element preservation:** Structure elements in the tree carry `/K` (kids) arrays
   that reference MCIDs, other structure elements, or object references. During incremental save,
   unmodified structure elements are preserved byte-for-byte. During full rewrite, structure
   elements are re-serialized but their semantic content (tag types, MCID references, attributes)
   is preserved.

3. **Content stream editing impact:** When content is removed (e.g., page deletion, redaction),
   the corresponding structure elements become orphaned. The edit transaction must:
   - Remove structure elements whose content was deleted
   - Update parent element `/K` arrays to remove deleted children
   - Update `/ParentTree` number tree entries for affected pages
   - Preserve the `/IDTree` name tree for structure element IDs

4. **Annotation structure:** When adding annotations to tagged PDFs, the annotation gets an
   `/StructParent` entry that links it into the parent tree. The engine must allocate a new
   parent tree index and add the annotation's structure element.

5. **Detection without full interpretation:** The engine detects the presence and complexity of
   the structure tree (number of elements, nesting depth, MCID coverage) and reports it in the
   compatibility ledger. Full structure tree validation (verifying that every MCID in every
   content stream has a corresponding structure element) is a proof-harness check, not a
   runtime check.

**Annotation geometry invariants:**

- The annotation's `/Rect` is in default user space (same as the page's coordinate system before any page rotation).
- For text markup annotations, `/QuadPoints` are also in default user space.
- When a page has a `/Rotate` value, the annotation's visual position must be consistent with the rotated page. This means: the annotation's rect and quad points are in the unrotated coordinate system, but the rendering must rotate them to match the page rotation.
- Annotations added by Monkeybee must correctly account for the page's CropBox offset (if CropBox origin is not at [0,0]) and Rotate value.

### Serialization contract

The write path must produce bytes that are valid PDF. Cross-references must be correct. Object offsets must be accurate. Stream lengths must match. The output must be parseable by Monkeybee's own parser (self-consistency) and by reference implementations. Incremental saves must produce a valid append that does not corrupt the existing data.

### Save commit contract

For file-backed saves, Monkeybee uses staged commit semantics:
1. Serialize to a temp file in the target directory.
2. `fsync` the temp file.
3. Re-open and validate according to the requested save policy.
4. `fsync` the parent directory when supported.
5. Atomically rename the temp file over the destination.
6. On failure, preserve the original destination unchanged.

Library APIs expose:
- `save_to_bytes()`
- `save_atomic(path, SaveCommitOptions)`
- `save_atomic_with_backup(path, SaveCommitOptions)`

`monkeybee-write` remains a serializer; staged commit is owned by the public facade / CLI adapter.

### Persisted artifact durability contract

Durability is a separate contract from semantic write correctness. `save_to_bytes()`
proves the bytes are valid; file-backed save and artifact publication prove those
bytes or evidence objects were durably published.

Rules:
- any persisted artifact written beyond the current call boundary — output PDFs,
  write receipts, invariant certificates, ledgers, disagreement records,
  failure capsules, benchmark witnesses, and persistent derived artifacts — is
  staged privately first
- the staged artifact's digest, size, and schema/version metadata are computed
  and verified before publication
- data blobs are fsynced before any manifest, pointer file, or directory rename
  that makes them authoritative; grouped artifact sets publish their manifest
  last
- manifests, ledgers, and receipts may reference only already-durable child
  artifacts; a reference to a not-yet-durable blob is a correctness bug
- crash recovery treats private temp objects and unreferenced blobs as
  unpublished; they may be quarantined or garbage-collected, but never surfaced
  as valid durable state
- reclamation of persisted artifacts is reachability-based from durable
  manifests/receipts plus retention policy, not from best-effort age heuristics

**Self-consistency invariant:** Monkeybee must be able to parse its own output without errors in strict mode. This is a hard test requirement, not a soft aspiration. Every generated PDF is round-tripped through Monkeybee's strict-mode parser as part of the write-path test suite.

**Serialization ordering:** Objects may be serialized in any order, but the cross-reference must correctly point to each object's starting offset. For human-readability and debugging, prefer: header → body objects (in object-number order) → cross-reference → trailer → `%%EOF`.

**String encoding:** PDF strings in output should use the most compact representation: literal strings for ASCII-safe content (with appropriate escaping of `(`, `)`, `\`, and non-printable bytes), hex strings when the content is mostly non-printable. UTF-16BE with BOM for Unicode text strings.

**Object stream packing:** Compact rewrite mode may pack objects into object streams once it is independently proven under the proof harness. The baseline deterministic writer should emit plain indirect objects by default because that path is simpler to audit, diff, fuzz, and recover from. Object streams must not contain: stream objects (streams cannot be nested inside object streams), the encryption dictionary, the document catalog, or the cross-reference stream itself.

**Cross-reference stream output:** Support both cross-reference tables and cross-reference streams. The baseline deterministic writer prefers classic cross-reference tables; compact mode may prefer cross-reference streams after proof stability is established. Cross-reference streams are more compact and support object streams
(which require cross-reference streams).

**Decision rules for xref format selection:**

1. If the output uses object stream packing → cross-reference streams are mandatory.
2. If the output version is < PDF 1.5 → classic cross-reference tables are mandatory
   (xref streams were introduced in PDF 1.5).
3. If incremental-append mode and the existing file uses xref tables → prefer appending
   an xref table for structural consistency (avoids hybrid files).
4. If incremental-append mode and the existing file uses xref streams → append an xref stream.
5. For full-rewrite mode with baseline settings → classic xref table (simpler to audit/debug).
6. For full-rewrite mode with compact/optimized settings → xref stream with PNG Up predictor.

The baseline v1 writer does not use object stream packing by default. The combination of
object streams + xref streams is an optimization that lands only after the baseline writer's
proof gates are satisfied.

### AcroForm contract

The engine must handle AcroForm (interactive form) fields for both reading and basic writing. AcroForm is distinct from XFA; it is the standard PDF form mechanism and is fully Tier 1.

**Field hierarchy:** Form fields form a tree rooted at the document catalog's `/AcroForm` → `/Fields` array. Fields can be organized hierarchically (parent fields with child fields). Inheritable field attributes: `/FT` (field type), `/V` (value), `/DV` (default value), `/Ff` (field flags), `/DA` (default appearance), `/Q` (quadding/alignment).

**Field inheritance resolution algorithm:**

Field attributes propagate from parent to child in the field hierarchy. Unlike page inheritance
(where the first ancestor with the attribute provides it), field inheritance has additional
complexities:

1. **Walk from field to root:** For each inheritable attribute, walk up the field hierarchy
   via `/Parent` references. The first ancestor that defines the attribute provides its value.

2. **`/FT` (field type):** Required on every terminal field. If a terminal field lacks `/FT`,
   inherit from the nearest ancestor. If no ancestor defines it, the field is malformed —
   report a diagnostic and skip the field.

3. **`/Ff` (field flags):** Flags are inherited as a complete bitmask, not merged. A child's
   `/Ff` replaces (not ORs with) the parent's flags. If a child has no `/Ff`, use the parent's.

4. **`/V` (value) and `/DV` (default value):** Inherited normally. A common pattern: the parent
   field defines `/DV` and each child inherits it unless overridden.

5. **`/DA` (default appearance):** The appearance string (e.g., "/Helv 12 Tf 0 g") is inherited
   as a complete string. The AcroForm dictionary's `/DA` entry serves as the document-level
   default, applied when no field in the hierarchy defines `/DA`.

6. **Partial field names:** A field's full name is constructed by concatenating ancestor names
   with `.` separators. For example, if a field named `zip` has a parent named `address` which
   has a parent named `form`, the full field name is `form.address.zip`. This is critical for
   form data import/export — the full name is the unique identifier.

7. **Widget-field relationship:** A widget annotation is the visual representation of a field.
   A field can have multiple widgets (e.g., a radio button group). Widgets inherit all field
   attributes from their parent field but can override visual properties via `/MK`.

**Field types:**
- **Text fields** (`/FT /Tx`): single-line or multi-line text. The value (`/V`) is a text string. The appearance stream must render the text using the font and size from `/DA`, within the widget's rect, respecting `/Q` alignment and `/MaxLen` limits.
- **Button fields** (`/FT /Btn`): push buttons, checkboxes, and radio buttons. Checkboxes have two states: the "on" state (named in `/AS`) and `/Off`. Radio buttons share a parent field; exactly one child can be "on" at a time.
- **Choice fields** (`/FT /Ch`): combo boxes (dropdown) and list boxes. The `/Opt` array lists available choices. Each choice can be a simple string or a two-element array `[export_value display_value]`.
- **Signature fields** (`/FT /Sig`): contain digital signature dictionaries. The signature dictionary (`/V` in the signature field) contains:
  - `/Filter` and `/SubFilter`: identify the signature format. Common: `/adbe.pkcs7.detached` (CMS/PKCS#7 detached signature), `/adbe.pkcs7.sha1` (legacy), `/ETSI.CAdES.detached` (CAdES — used by PAdES), `/ETSI.RFC3161` (timestamp).
  - `/ByteRange`: an array of 4 integers `[offset1 length1 offset2 length2]` defining the two byte ranges of the file that are signed (everything except the signature value hex string itself). The engine must understand this structure to verify that preserve-mode saves do not modify signed bytes.
  - `/Contents`: the actual signature value (a hex string containing the DER-encoded CMS/PKCS#7 signature).
  - `/Cert`: the signing certificate (for some signature types).
  - `/M`: signing date. `/Reason`, `/Location`, `/ContactInfo`: human-readable metadata.

  v1 guarantees:
  - Byte-range integrity checking
  - Signature dictionary parsing and CMS envelope inspection
  - Incremental-append preservation of signed bytes
  Full PKI trust validation is delegated to an optional `CryptoProvider` and is not claimed by the baseline engine alone. The engine must preserve signature fields and their byte ranges during incremental-save operations. Specifically:
  - The engine must track the exact byte offsets specified in `/ByteRange` and guarantee that incremental-append operations do not modify any bytes within those ranges.
  - When inspecting a document, the engine should report: number of signatures, signing dates, signature coverage (what percentage of the document is signed), and whether the document has been modified after signing (by checking if objects outside the signed byte range differ from those within it).

**Form appearance regeneration:** When a form field's value changes (e.g., the user fills in a text field), the widget annotation's appearance stream must be regenerated to reflect the new value. The `/NeedAppearances` flag in the AcroForm dictionary signals whether appearances need regeneration. If true, the engine must generate appearances for all widgets. For round-trip safety, the engine should always regenerate appearances when field values are modified, regardless of the flag.

**Field calculations and actions:** PDF forms support JavaScript-based calculations and trigger actions (e.g., calculate the sum of other fields). Full JavaScript execution is not a v1 goal. The engine should: (a) detect and report the presence of calculation scripts in the compatibility ledger, (b) preserve them during round-trip operations, (c) not evaluate them.

### Enterprise print-production and prepress expansion contract

Monkeybee is not just a screen renderer. It must grow a serious print-oriented inspection and proof
lane because enterprise print workflows are one of the largest PDF consumer categories. This lane
shares the normal render/content/color machinery; it must not fork into a separate ad hoc pipeline.

1. **Halftone modeling and screening:** Parse and preserve halftone dictionaries, including Types
   1, 5, 6, 10, and 16. Spot-function evaluation for dot-shape generation and threshold-based
   screening must be inspectable even when the active backend does not perform full raster-screen
   simulation. Tier 1 screening behavior is tied to a backend/support-class contract; lower tiers
   still report screen parameters, screen frequency/angle state, and degradation explicitly.
2. **Transfer, BG/UCR, and print-state evaluation:** `/TR`, `/TR2`, `/BG`, `/BG2`, `/UCR`, and
   `/UCR2` are not dead metadata. The engine must preserve them, expose them through inspection,
   and provide evaluation hooks for print-preview and proof modes. These hooks must accept PDF
   function Types 0, 2, 3, and 4 so transfer curves can be evaluated per component and BG/UCR
   transforms can participate in proof-mode color analysis. When these transforms are not applied
   on the active backend, the compatibility ledger must say so directly.
3. **RGB overprint simulation:** The renderer must provide an explicit overprint-simulation mode
   for RGB displays so CMYK overprint-heavy documents can be previewed without flattening away the
   semantics that matter in prepress.
4. **Soft proofing and separations:** Render against the document's `/OutputIntents` or a
   caller-supplied ICC profile; expose individual process and spot separations as grayscale plate
   previews; honor page-level output intents when present. Soft-proofing must be able to answer
   both "show me the intended press condition" and "show me this target device/paper pair" without
   creating a parallel renderer.
5. **Ink coverage analysis:** Compute TAC per page/region and emit diagnostics when configured
   thresholds are exceeded. TAC accounting must include spot/process separations consistently with
   the selected proof mode and support shop-style limits such as 300-340% without hard-coding a
   single threshold into the engine.
6. **Print preflight:** Validate image resolution at final print size, bleed/TrimBox/BleedBox
   relationships, output-intent presence, color-space suitability, font completeness, and other
   press-facing issues such as images falling below 300 DPI at print size.
7. **Trap networks:** Parse trap-related structures, expose them via inspection, and render them
   when the active backend supports it. Trap annotations and trap-network metadata must be
   discoverable even on execute-deny or non-prepress profiles. Lack of trap support is an explicit
   degradation, never a silent omission.
8. **`/Trapped` semantics:** Surface the document's `True` / `False` / `Unknown` trap status in
   preflight and treat it as operational guidance rather than decorative metadata. A document
   already marked trapped must not be silently routed into auto-trapping logic without an explicit
   override.
9. **ICC profile version awareness:** Detect ICC v2 versus v4 profile versions, use the correct
   interpretation rules for each, and emit hazard diagnostics when a document mixes versions in
   ways that suggest producer bugs or profile misuse.
10. **Alternate image representations:** Parse `/Alternates` arrays on image XObjects, preserve the
    alternate set, expose the available representations in extraction, and prefer the best screen
    or print variant based on the active render/prepress profile.
11. **Spot-function library:** Catalog standard and custom spot functions, including Type 4
    calculator-based screens such as diamond, cross, rhomboid, and double-dot variants, so print
    inspection can explain the actual halftone program rather than only report that a halftone
    exists.
12. **DeviceN attributes and mixing hints:** Parse PDF 2.0 DeviceN `/Subtype`, `/Process`,
    `/Colorants`, and `/MixingHints` dictionaries and surface the printing order, dot-gain hints,
    and process/spot relationships needed for credible separation preview.
13. **Output-intent condition identifiers:** Resolve common `/OutputConditionIdentifier` values
    such as SWOP/CGATS and FOGRA families to recognizable press-condition narratives so preflight
    can say not merely "an output intent exists" but what production condition it implies.

### Digital signature lifecycle and PAdES expansion contract

Preserving byte ranges is necessary but not sufficient. A serious engine must model the signature
workflow end to end.

1. **PAdES profile classification:** Treat B-B, B-T, B-LT, and B-LTA as first-class states in
   inspection, validation, and write planning rather than as prose attached to a generic CMS blob.
2. **DSS and VRI modeling:** Parse, preserve, emit, and inspect the Document Security Store and
   per-signature Validation Related Information structures. VRI indexing must remain stable per
   signature digest so later offline validation can explain exactly which evidence applies to which
   signature.
3. **Certificate-path construction:** Build signing chains from leaf to trust anchor using
   certificate extensions such as SKI, AKI, AIA, and CRL distribution points. Chain construction is
   a deterministic engine-visible operation, not a black box hidden behind provider callbacks.
4. **Revocation evidence:** OCSP responses and CRLs may be embedded, supplied by providers, or
   absent. The engine must classify which evidence is available, whether it is embedded in DSS, and
   whether offline LTV is possible for each signature independently.
5. **Timestamp support:** RFC 3161 timestamps must be inspectable and verifiable; creation-side TSA
   integration is required for B-T and above, including append-safe placeholder sizing and save-plan
   explanations.
6. **Signature creation:** Create CMS/PAdES signatures, allocate placeholder space correctly,
   integrate with incremental append, and explain write-plan consequences before bytes are emitted.
   Signature creation is a first-class write-side capability, not merely a preserve-mode side
   effect.
7. **Offline long-term validation:** When DSS/VRI/revocation/timestamp material is present, the
   engine must be able to validate without network access and report why a document does or does not
   meet LT/LTA expectations.
8. **Certification versus approval signatures:** Classify each signature explicitly as
   certification or approval, validate the DocMDP/FieldMDP modification chain, and expose when a
   later approval signature is inconsistent with the certification policy established by the first
   signer.

### Tagged PDF and accessibility-audit expansion contract

Preserving tags and using them as extraction hints is only the baseline. The semantic model is much
larger and must be owned explicitly.

1. **Structure-role coverage:** Recognize the full family of standard structure element types,
   namespace-qualified variants, and role-map chains. Role maps must be resolved transitively until
   they terminate at a standard role, and circular or broken chains must be flagged explicitly
   rather than silently truncated. This includes at minimum `Document`, `Part`, `Art`, `Sect`,
   `Div`, `BlockQuote`, `Caption`, `TOC`, `TOCI`, `Index`, `NonStruct`, `Private`, `P`, `H`,
   `H1`-`H6`, `L`, `LI`, `Lbl`, `LBody`, `Table`, `TR`, `TH`, `TD`, `THead`, `TBody`, `TFoot`,
   `Span`, `Quote`, `Note`, `Reference`, `BibEntry`, `Code`, `Link`, `Annot`, `Ruby`, `Warichu`,
   `Figure`, `Formula`, `Form`, plus PDF 2.0 additions and standards-based namespace extensions.
2. **Attribute/class-map parsing:** Parse layout, list, table, print-field, and user-property
   attributes together with class maps and expose them through extraction/inspection APIs.
3. **Semantic text overrides:** Prefer `/ActualText` for extraction when present; expose `/Alt`,
   `/E`, `/Lang`, and pronunciation metadata as part of semantic extraction output. `/ActualText`
   must win over raw decoded glyph content when the two disagree.
4. **Artifacts and destinations:** Detect artifact-marked content, allow artifact-aware extraction
   modes, and expose structure destinations and semantic links. Artifact handling must cover common
   headers, footers, page numbers, and watermark-style content.
5. **Marked-content integrity:** Track `BMC` / `BDC` / `EMC` nesting precisely, auto-close
   unclosed spans in tolerant mode at content-stream end, flag overlapping/impossible nesting as
   tagged-PDF audit findings, and preserve repaired intent in diagnostics rather than silently
   flattening it away.
6. **Audit without remediation:** PDF/UA-style validation belongs in Monkeybee as an audit/report
   surface even though accessibility remediation and tag generation remain outside the baseline gate.
   Audit rules must cover structure-tree completeness, figure alt-text presence, heading hierarchy,
   table header associations, and reading-order plausibility.
7. **Reading-order visualization:** Provide a debug/inspection overlay that shows structure order,
   artifacts, and the mapping from marked content to structure elements.

### Form-data interchange and flattening expansion contract

AcroForm handling becomes materially more useful once data interchange and flattening are first-class.

1. **FDF/XFDF interchange:** Import/export field values by fully qualified field name, preserve
   field-tree semantics, and round-trip cleanly with appearance regeneration.
2. **Form flattening:** Burn resolved widget appearances into page content after inheritance,
   value resolution, and calculation-order preservation. Flattening is distinct from generic
   annotation flattening and must respect field semantics and field-tree ownership.
3. **Script inventory:** Detect calculate, format, validate, and keystroke JavaScript hooks;
   preserve them; never execute them in baseline v1.
4. **Submit/reset analysis:** Classify submit targets and payload modes (FDF, XFDF, PDF, HTML,
   XML, email, URI) and surface them in active-content reports.
5. **Signature-field creation:** Create empty signature fields with correct widget state and
   placeholder sizing for later signing workflows.
6. **Barcode fields:** Parse and render barcode-field appearances and preserve their semantics,
   including common symbologies such as Code 128, QR, DataMatrix, and PDF417 where the document
   model exposes them.
7. **Static XFA flattening:** Tier 2 handling may flatten static XFA presentations into page
   content when the authoritative visual layer is safely recoverable. Dynamic-XFA layout engines
   remain outside the baseline, but detection and explicit ledgering are mandatory because these
   documents remain common in government and enterprise workflows.

### Action, document-structure, and multimedia catalog contract

The engine must inventory these surfaces comprehensively even when execution/playback is denied.

1. **Full action catalog:** Parse and preserve the entire PDF action family, including GoTo, GoToR,
   GoToE, GoTo3DView, Launch, Thread, URI, Sound, Movie, Hide, Named, SetOCGState, Rendition,
   Transition, JavaScript, ImportData, ResetForm, SubmitForm, and RichMediaExecute. Named actions
   such as `NextPage`, `PrevPage`, `FirstPage`, `LastPage`, and `Print` must remain typed rather
   than collapsed into opaque strings.
2. **Document link map:** Extract a typed link map from navigational actions and related
   annotations so downstream tooling can reason about document navigation without replaying UI
   behavior. Link maps must distinguish page destinations, named destinations, remote targets, and
   structure destinations (`/SD`) rather than collapsing them into a single opaque target field.
3. **JavaScript timing graph:** Inventory document-open, page-open/page-close, annotation
   focus/blur/mouse events, form keystroke/validate/calculate/format hooks, and save/print timing
   triggers as a queryable graph even when JavaScript execution is denied.
4. **Document-structure extras:** Parse and expose article threads and beads, page transitions,
   thumbnails, collections/portfolios, alternate presentations, `/PieceInfo`, named
   optional-content configurations (`/Configs`), and web-capture structures. Collection support
   includes schema and navigator extraction; web-capture support includes content sets,
   `SourceInfo`, and capture-command dictionaries.
5. **Multimedia inventory:** Parse and expose screen annotations, sound objects, movie annotations,
   media clips, rendition trees, and media-player parameters. Preserve them during round trip while
   default policy remains execute-deny; legacy movie and sound objects must still be cataloged even
   when no playback implementation is active.

### Rendering-quality expansion contract

Some renderer details are not optional polish; they materially affect correctness on hard files.

1. **Higher-quality resampling:** Support Lanczos-class downscaling and
   Mitchell-Netravali-class upscaling as pluggable kernels beyond the simple baseline filters.
   Lanczos-3 is the intended sharp downscale reference; Mitchell-Netravali is the intended
   low-ringing upscale reference.
2. **N-dimensional sampled-function interpolation:** Type 0 sampled functions with multiple input
   dimensions must use a defined multilinear interpolation strategy rather than hand-wavy lookup.
   Multi-input tint transforms such as CMYK DeviceN/Separation cases are the motivating examples.
3. **Shading-edge anti-aliasing:** Smooth shadings clipped against geometry must share the same
   exact-coverage discipline as the rest of the rasterizer.
4. **Matte un-premultiplication:** Soft-masked images with `/Matte` require a numerically stable
   un-premultiplication algorithm, including explicit handling of fully transparent and
   near-transparent pixels to avoid divide-by-zero and ringing artifacts.

---

## Part 6 — Proof doctrine

### Pathological corpus

The project must maintain a curated corpus of ugly, hard, and pathological PDFs. This corpus must span:

- Scanned documents (various qualities, DPI, preprocessing states)
- Form-heavy documents (AcroForm, XFA detection)
- Encrypted documents (various security handlers, permission levels)
- Embedded fonts with broken metadata
- Missing or invalid ToUnicode maps
- Transparency and blend edge cases
- CJK documents (Chinese, Japanese, Korean)
- RTL documents (Arabic, Hebrew)
- Very large files (100+ pages, large embedded resources)
- Malformed cross-references (missing entries, wrong offsets, hybrid tables)
- Print-production files with output intents, spot colors, overprint-heavy artwork,
  trap annotations, and TAC-sensitive separations
- Signed documents spanning bare CMS, timestamped signatures, DSS/VRI-embedded LTV cases,
  and post-signing modifications
- Tagged PDFs with rich structure trees, ActualText/Alt/Lang/artifact markup, and known
  PDF/UA audit expectations
- Portfolios, article-thread documents, page-transition/slideshow files, and multimedia-rich PDFs
- Adversarial inputs (fuzzed, hand-crafted to trigger parser bugs)
- Complex vector art (intricate path constructions, gradient meshes)
- Files from many different producers (Acrobat, Word, LaTeX, Chrome, LibreOffice, InDesign, Quartz, etc.)
- Incremental-update chains with complex histories
- Linearized files (intact and damaged)
- Files with unusual page structures (non-standard boxes, rotations, user units)

The corpus is split into `public/`, `restricted/`, `generated/`, and `minimized/` tiers.
Every fixture carries provenance, license/sensitivity metadata, expected failure/repair tags, and redistribution status.
Every fixture also carries an expectation manifest: expected tier assignments, allowed degradations, render-score thresholds, extraction goldens or invariants, signature expectations, and triage status (`approved`, `pending`, `known_bad`, etc.).
Expectation manifests may also freeze tolerant-repair behavior:
- expected chosen `RecoveryCandidateId`
- expected `semantic_digest`
- allowed alternative candidate ids
- whether `write_impact` equivalence is required

Changing the chosen repair candidate for an existing fixture is a proof regression unless explicitly triaged.

Crashers and regressions must be minimized into the `minimized/` tier whenever feasible.
`monkeybee-proof` includes an automated reducer that preserves one or more target signatures:
- panic fingerprint
- render-diff signature
- extraction mismatch signature
- repair-decision semantic digest
- signature-impact classification
The corpus must be indexed, categorized, and continuously exercised by CI.

**Specific test case classes and what each proves:**

*Class: xref-repair*
Test cases: wrong startxref offset, corrupted xref table entries, missing entries, hybrid xref with conflicts, circular /Prev chains, truncated xref streams, wrong /Size values.
Proves: tolerant parser can recover document structure from corrupted cross-references. Verifies every repair strategy in Part 2.

*Class: font-fallback*
Test cases: missing ToUnicode CMap, broken Type 1 PFB headers, TrueType with invalid loca table, CIDFont with no embedded font data, non-standard glyph names not in AGL, Identity-H CIDFont with incomplete ToUnicode, Type 3 font with complex content streams.
Proves: the font/encoding fallback chain produces correct text extraction and acceptable glyph rendering across all failure modes.

*Class: transparency-compositing*
Test cases: isolated group over opaque background, non-isolated group over semi-transparent background, knockout group with multiple elements, isolated knockout group, luminosity soft mask, alpha soft mask, nested groups with different blend modes, overprint mode 1 with CMYK, stacked blend modes (e.g., Multiply inside a ColorDodge group).
Proves: the transparency compositing pipeline handles all combinations of isolation, knockout, soft masks, and blend modes. Each test has a known-correct reference rendering from at least two external renderers.

*Class: producer-quirks*
Test cases: at least one representative file from each major producer (Acrobat, Word, Chrome, LibreOffice, InDesign, LaTeX/pdfTeX, Quartz/Preview, Ghostscript, Foxit, Nitro). Each file should exercise a known quirk of that producer.
Proves: the quirk-shim layer correctly compensates for producer-specific deviations.

*Class: incremental-update*
Test cases: single incremental update, multiple chained updates, update that deletes objects, update that modifies the page tree, update that adds annotations, update that changes encryption, conflicting object definitions across updates.
Proves: incremental update chain parsing correctly resolves to the latest state, and incremental save produces a valid appended section that other readers accept.

*Class: encryption-read*
Test cases: V1/R2 (40-bit RC4), V2/R3 (128-bit RC4), V4/R4 (AES-128), V5/R5 (AES-256), V5/R6 (AES-256 with revised password handling), empty passwords, non-ASCII passwords, permission restrictions.
Proves: decryption works for all standard security handler versions.

*Class: encryption-write* [post-v1 unless explicitly promoted]
Test cases: output files encrypted by Monkeybee can be opened by reference renderers.
Proves: output-encryption interoperability.

*Class: annotation-roundtrip*
Test cases: each annotation type created by Monkeybee, saved, and reopened. Annotations on pages with non-identity rotation. Annotations on pages with CropBox offset. Annotations added to files from different producers. Annotations with rich text content. Reply chains. Annotations that reference page resources.
Proves: annotation creation, serialization, and deserialization are geometrically and semantically correct.

*Class: page-mutation*
Test cases: add a blank page, remove a page from the middle, reorder pages, copy pages between documents, merge documents, split documents, change page rotation, change page MediaBox.
Proves: page tree manipulation produces valid structure, and the resulting document renders correctly.

*Class: generation*
Test cases: generate documents with text, images, vector graphics, annotations, multiple pages, embedded fonts, transparency, color spaces (at least DeviceRGB, DeviceCMYK, ICCBased).
Proves: generated output is valid PDF that renders correctly under Monkeybee and at least two reference renderers.

*Class: adversarial*
Test cases: fuzz-generated inputs, inputs with extreme nesting depth, inputs with very large objects, inputs with many cross-references, inputs designed to trigger integer overflow in offset calculations, inputs with contradictory metadata, zip-bomb-style stream compression.
Proves: the parser does not panic, does not allocate unbounded memory, does not enter infinite loops, and produces structured error diagnostics for all adversarial inputs.

*Class: color-space*
Test cases: documents using each color space type (DeviceRGB, DeviceCMYK, DeviceGray, CalRGB, CalGray, Lab, ICCBased with v2 and v4 profiles, Indexed, Separation, DeviceN, Pattern with tiling and shading). Documents with color space nesting (Indexed over ICCBased, DeviceN with ICCBased alternate). Documents with default color space overrides.
Proves: color space resolution produces correct color values at every stage of the chain.

*Class: content-stream-stress*
Test cases: pages with 500,000+ operators (Chrome print-to-PDF of complex web pages), deeply nested form XObjects (10+ levels), content streams with invalid operators inside BX/EX compatibility regions, content streams with interleaved path construction and text operations.
Proves: the content stream interpreter handles scale and edge cases without performance degradation or incorrect state.

*Class: signature-preserve*
Test cases: signed documents modified with incremental-append save, verification that byte ranges are preserved, verification that existing signatures validate after Monkeybee's modifications.
Proves: preserve-mode write path does not corrupt existing signatures.

*Class: redaction-safety*
Test cases: text-only, image-only, mixed vector/text, reused XObjects, form XObjects, transparency, and canary-text leakage checks.
Proves: no recoverable redacted content survives under the selected apply mode.

### External references

Monkeybee does not treat any single external renderer as ground truth. Instead, it uses consensus-style reference testing:

- **PDFium** as primary reference
- **MuPDF** as secondary reference
- **pdf.js** as tertiary reference (browser-native perspective)
- **Ghostscript** as strict canary

Where references disagree, the disagreement itself is recorded and investigated.

Every proof run records an oracle manifest: renderer name, exact version, build hash/container digest, invocation flags, and platform. Pinned oracle manifests are required in canonical CI runs for reproducibility.

### Arlington-model conformance validation

The Arlington PDF Model is a machine-readable description of the entire PDF specification's object structure: which dictionary keys are required, optional, or deprecated for each object type; what types and value ranges are legal for each key; which keys are mutually exclusive or conditionally required. Monkeybee integrates the Arlington model as a structural validation oracle:

1. **Code generation from Arlington:** At build time, parse the Arlington TSV data files and generate Rust validation functions for each PDF object type. Each function checks: required keys present, no unknown keys (in strict mode), value types correct, value ranges valid, conditional requirements satisfied (e.g., "/Subtype required when /Type is /Font").
2. **Integration with the tolerant parser:** After parsing each object, run the Arlington validator. In strict mode, violations are errors. In tolerant mode, violations are diagnostics recorded in the compatibility ledger with the specific Arlington rule that was violated.
3. **Integration with the write path:** Before serializing any object, validate it against the Arlington model. The write path must not emit objects that violate the spec — this is a hard invariant, not a best-effort check.
4. **Profile-specific validation:** When the target output is a supported v1 profile (PDF/A-4, PDF/X-6), the Arlington model encodes the additional constraints of that profile. The validator checks both base PDF conformance and profile-specific requirements. This is the foundation for the downlevel write mode.
5. **Coverage metric:** Track which Arlington rules are exercised by the pathological corpus. Rules that are never exercised represent document structures the engine has never encountered — these are blind spots that should drive corpus acquisition.


### Round-trip requirements

The following round-trip chains must pass on representative documents from the pathological corpus:

1. **Load → render → save → reload → render → compare** (visual stability)
2. **Load → annotate → save → reload → verify annotations** (annotation integrity)
3. **Load → edit (page ops, metadata) → save → reload → validate structure** (mutation integrity)
4. **Generate → render (Monkeybee) → render (reference) → compare** (generation correctness)
5. **Load → extract → verify against known ground truth** (extraction accuracy)
6. **Load (preserve mode) → annotate → save (incremental) → reload → verify signatures** (signature preservation)
7. **Load → save (full rewrite) → reload → render → compare to original render** (rewrite fidelity)
8. **Load → enumerate revision frames → materialize historical snapshot → render/extract/diff** (temporal replay fidelity)
9. **Load → extract semantic anchors → safe rewrite/incremental append → reload → verify stable anchors or alias map** (anchor stability)
10. **Load ambiguous file → inspect hypothesis set → force candidate or auto-collapse → compare receipts** (hypothesis truthfulness)
11. **Load source + target → import/copy pages/resources between documents → save → reload → verify provenance/remap/render** (cross-document import fidelity)

**Specific failure modes each chain is designed to catch:**

*Chain 1 (visual stability):* Detects: graphics state leaks across save/reload (state not properly serialized), font metric drift (widths change after round-trip), transparency compositing differences due to lost group attributes, color space information loss, clipping path corruption.

*Chain 2 (annotation integrity):* Detects: geometry drift from coordinate system misunderstanding (especially with rotated pages), appearance stream generation errors, annotation property loss (colors, border styles, contents), broken annotation-to-page associations, popup annotation disconnection.

*Chain 3 (mutation integrity):* Detects: page tree corruption after insert/delete, broken parent references, incorrect page count, orphaned resources (resources referenced by removed pages still present), missing resources (resources needed by surviving pages removed), xref corruption after object renumbering.

*Chain 4 (generation correctness):* Detects: structural invalidity in generated output (missing required dictionary keys, wrong object types), font embedding errors (incomplete subsetting, wrong metrics), color space specification errors, content stream operator errors.

*Chain 5 (extraction accuracy):* Detects: text decoding errors (wrong Unicode values), position inaccuracy (characters placed at wrong coordinates), missing characters (encoding failures), wrong reading order, font name misreporting.

*Chain 6 (signature preservation):* Detects: byte-range corruption (existing bytes modified), structural changes to signed content, incremental-append errors that invalidate the document structure.

*Chain 7 (rewrite fidelity):* Detects: lossy canonicalization (information destroyed during save that affects rendering), stream re-encoding errors, object serialization bugs, font re-embedding errors.

*Chain 8 (temporal replay fidelity):* Detects: incorrect reconstruction of historical incremental states, replay paths that accidentally inspect the latest snapshot instead of the requested frame, and revision-local signature/context leakage.

*Chain 9 (anchor stability):* Detects: semantic-anchor drift for unchanged content, unstable geometry hashing, missing alias maps when safe rewrites legitimately change internal object identities, and query-interface regressions that break downstream automation.

*Chain 10 (hypothesis truthfulness):* Detects: silent candidate collapse, missing hypothesis lineage in receipts/ledgers, and repair policies that return materially different semantics without user-visible evidence.

*Chain 11 (cross-document import fidelity):* Detects: source-to-target `ObjRef` collisions, incomplete import-closure remapping, lost named destinations or form-field identity, silent active-content/signature escalation across documents, and provenance gaps that make imported pages impossible to audit later.

### Compatibility ledger schema

The compatibility ledger is a structured, machine-readable record produced for every document processed. It is the backbone of the proof infrastructure and the primary mechanism for tracking what the engine can and cannot handle.

**Schema:**

```
CompatibilityLedger {
  schema_version: string,       // ledger schema version for forward compatibility
  document_id: string,          // hash of input file
  oracle_manifest_id: string,   // identifies the oracle version set used for this run
  expectation_manifest_id: Option<string>, // identifies the fixture expectation manifest, if present
  reproducibility_manifest_id: string, // identifies the pinned run/build/environment manifest
  file_name: string,
  file_size: u64,
  pdf_version: string,          // e.g., "1.7", "2.0"
  producer: Option<string>,     // /Producer metadata value
  creator: Option<string>,      // /Creator metadata value
  parse_mode: "strict" | "tolerant" | "preserve",
  timestamp: ISO8601,

  features: [FeatureEntry],     // one per detected feature category
  repairs: [RepairEntry],       // one per repair action taken
  diagnostics: [DiagnosticEntry], // warnings, notes, errors
  ambiguities: [AmbiguityEntry],  // competing recovery candidates and why they differed
  plan_selection_refs: [ArtifactRef], // write/import/open/backend selections relevant to this run
  oracle_disagreement_refs: [ArtifactRef], // typed disagreement records, if any
  pages: [PageLedger],           // per-page feature/diagnostic breakdown
  summary: LedgerSummary,
}

FeatureEntry {
  category: string,             // e.g., "font.type1", "transparency.softmask",
                                // "encryption.aes256", "xfa.dynamic"
  tier: 1 | 2 | 3,
  status: "supported" | "partial" | "degraded" | "unsupported",
  detail: string,               // human-readable description
  objects: [ObjectRef],         // which objects triggered this entry
  impact: "visual" | "structural" | "metadata" | "interactive",
}

RepairEntry {
  category: string,             // e.g., "xref.wrong_offset", "stream.wrong_length"
  original_value: string,       // what the file said
  repaired_value: string,       // what the engine determined
  strategy: string,             // which repair strategy succeeded
  object: Option<ObjectRef>,
  confidence: "high" | "medium" | "low",
}

DiagnosticEntry {
  severity: "error" | "warning" | "info",
  category: string,
  message: string,
  object: Option<ObjectRef>,
  byte_offset: Option<u64>,
}

LedgerSummary {
  total_pages: u32,
  tier1_features: u32,         // count of features fully supported
  tier2_features: u32,         // count of features partially handled
  tier3_features: u32,         // count of features detected but unsupported
  repairs_applied: u32,
  errors: u32,
  warnings: u32,
  overall_confidence: "high" | "medium" | "low",
}

ObjectRef {
  object_number: u32,
  generation_number: u16,
}

PageLedger {
  page_index: u32,
  features: [FeatureEntry],
  diagnostics: [DiagnosticEntry],
  degraded_regions: [RegionRef],
}

RegionRef {
  bbox: [f32; 4],
  reason: string,
}

ArtifactRef {
  kind: string,
  digest: string,
}
```

### Compatibility ledger JSON schema

The compatibility ledger's canonical serialization format is JSON, with a concrete schema that
downstream tools (dashboards, CI gates, regression detectors) can consume programmatically:

```json
{
  "schema_version": "1.0",
  "engine_version": "0.1.0",
  "timestamp": "2026-03-15T12:00:00Z",
  "reproducibility_manifest_id": "repro-2026-03-15-linux-x86_64-canonical",
  "input": {
    "filename": "example.pdf",
    "sha256": "abc123...",
    "declared_version": "1.7",
    "effective_version": "2.0",
    "size_bytes": 1234567,
    "page_count": 42,
    "producer": "Adobe Acrobat 2024",
    "creator": "Microsoft Word"
  },
  "features": [
    {
      "code": "transparency.isolated_knockout_group",
      "tier": 1,
      "status": "supported",
      "pages": [3, 7, 12],
      "details": "Isolated knockout transparency groups on 3 pages"
    }
  ],
  "repairs": [
    {
      "code": "parse.xref.wrong_offset",
      "severity": "warning",
      "object": "42 0",
      "original_value": "12345",
      "corrected_value": "12389",
      "strategy": "backward_scan",
      "confidence": 0.95
    }
  ],
  "degradations": [
    {
      "code": "compat.xfa.dynamic_no_fallback",
      "tier": 3,
      "severity": "error",
      "pages": [1],
      "description": "Dynamic XFA form with no AcroForm fallback; pages render as blank"
    }
  ],
  "plan_selection_refs": [
    {
      "kind": "write_plan",
      "digest": "plan123..."
    }
  ],
  "oracle_disagreement_refs": [
    {
      "kind": "render_arbitration",
      "digest": "oracle456..."
    }
  ],
  "summary": {
    "total_features": 156,
    "tier1_count": 142,
    "tier2_count": 8,
    "tier3_count": 6,
    "repair_count": 3,
    "degradation_count": 2,
    "overall_status": "degraded"
  }
}
```

The schema is versioned. Breaking changes increment the major version. The proof harness validates
ledger output against the schema. Downstream tools (dashboards, CI gates, regression detectors)
consume the ledger via the schema.

### Required code families for the expansion lanes

The ledger taxonomy MUST reserve stable code families for the expansion lanes so proof, dashboards,
and APR triage can track them without free-form strings:

- `print.*` — halftone, transfer, bg_ucr, overprint_sim, softproof, output_intent, separations, tac, preflight, trap
- `signature.*` — pades, dss, vri, chain_build, cert_path, ocsp, crl, tsa, creation, offline_ltv
- `tagged.*` / `pdfua.*` — standard_role, role_map, attributes, actualtext, alt, expansion_text, lang, pronunciation, artifact, destination, audit, heading_hierarchy, table_headers
- `forms.*` — fdf, xfdf, flatten, js_actions, submit_target, signature_field, barcode, xfa_static_flatten
- `actions.*` — goto, goto_remote, goto_embedded, goto_3d_view, thread, launch, uri, sound, movie, hide, named, ocg_state, rendition, transition, javascript, submit, import, reset, richmedia
- `catalog.*` — threads, beads, transitions, thumbnails, portfolio, collection_schema, alternate_presentation, pieceinfo, web_capture
- `multimedia.*` — screen, sound, movie, media_clip, rendition_tree, rendition_action, player_params

Tier assignment for these families is not optional. Even before Tier 1 implementation exists, the
engine must detect, classify, and ledger them.


### Ledger extensions for roots, hypotheses, certificates, and reproducibility

The compatibility ledger MUST grow to record substrate-aware evidence, not only human-readable
feature summaries. Concretely, the schema family should include:
- snapshot root digests for the analyzed document state
- temporal revision depth and, when relevant, the specific revision frame inspected
- hypothesis-set summaries for ambiguous files
- invariant-certificate references for writes, diffs, redactions, and replay exports
- semantic-surface summaries (layout graph present, semantic anchors present, anchor policy)
- reproducibility manifest IDs for the enclosing proof run
- plan-selection references for open/save/import/backend decisions that materially affected outcome
- typed oracle-disagreement references when consensus arbitration was required

This does **not** mean the ledger becomes a dumping ground for giant internal graphs. Large artifacts
remain externalized and content-addressed; the ledger stores digests, summaries, and references.

### External schema doctrine

The following outputs are schema-versioned external interfaces:
- `CompatibilityLedger`
- `CapabilityReport`
- `WritePlanReport`
- `DiffReport`
- `BenchmarkWitness`
- `TraceEventStream`
- CLI JSON envelope
- `ExpectationManifest`
- `OracleManifest`
- `ReproducibilityManifest`
- `PlanSelectionRecord`
- `OracleDisagreementRecord`

Backward compatibility is guaranteed within a major version for all of the above.
Breaking changes require a schema major-version bump and fixture updates in CI.

### Reproducibility manifest contract

Canonical proof needs a pinned description of the exact run environment, not
just pinned renderers. Monkeybee therefore emits a `ReproducibilityManifest`
for every proof/CI invocation.

```
pub struct ReproducibilityManifest {
    pub schema_version: String,
    pub run_id: String,
    pub canonical: bool,
    pub engine_version: String,
    pub engine_commit: String,
    pub target_triple: String,
    pub rustc_version: String,
    pub oracle_manifest_id: String,
    pub provider_manifest_id: String,
    pub feature_module_manifest_id: String,
    pub expectation_manifest_id: Option<String>,
    pub policy_digest: [u8; 32],
    pub fixture_set_digest: [u8; 32],
    pub environment_digest: [u8; 32],
}
```

Rules:
- canonical proof/CI runs emit exactly one schema-versioned
  `ReproducibilityManifest` and every ledger, plan-selection record,
  oracle-disagreement record, and failure capsule links back to it
- canonical runs require pinned oracle/provider/module manifests, deterministic
  settings, stable locale/timezone/seed policy, and a resolved `policy_digest`
- ad hoc developer runs may emit non-canonical manifests, but they must set
  `canonical=false` rather than pretending to be release evidence
- reproducibility manifests are content-addressed artifacts and participate in
  the same backward-compatibility rules as other external schemas

### Witness and benchmark evidence contract

Monkeybee's receipts, ledgers, disagreement records, and traces are witness
artifacts. Performance claims additionally require a schema-versioned
`BenchmarkWitness`.

```
pub struct BenchmarkWitness {
    pub witness_id: String,
    pub reproducibility_manifest_id: String,
    pub benchmark_profile_id: String,
    pub support_class: String,
    pub render_determinism_class: RenderDeterminismClass,
    pub fixture_set_digest: [u8; 32],
    pub warm_cache: bool,
    pub metrics: Vec<MetricObservation>,
    pub threshold_verdicts: Vec<ThresholdVerdict>,
}

pub struct MetricObservation {
    pub name: String,
    pub p50: Option<f64>,
    pub p95: Option<f64>,
    pub worst: Option<f64>,
    pub unit: String,
}

pub struct ThresholdVerdict {
    pub metric: String,
    pub target: String,
    pub actual: String,
    pub verdict: String,
}
```

Rules:
- every canonical benchmark class emits at least one `BenchmarkWitness` linked to
  the enclosing `ReproducibilityManifest`
- benchmark witnesses record support class, render determinism class, cache
  temperature, fixture set, and threshold verdicts; ad hoc timing logs are not
  release evidence
- README, release notes, dashboards, and CLI capability/performance summaries
  may cite only witness-backed metrics from canonical runs or explicitly labeled
  non-canonical runs
- benchmark witnesses follow the same persisted artifact durability contract as
  ledgers, capsules, and receipts

**Aggregation:** The proof harness aggregates individual ledgers across the entire corpus into a corpus-level compatibility report: feature coverage matrix (which features are Tier 1/2/3 across the corpus), repair frequency histogram (which repairs fire most often), producer-specific breakdown, and regression tracking (did a feature that was Tier 1 last week become Tier 3?).

### CI integration contract

The proof harness integrates with CI as follows:

**Invocation:** `cargo test --workspace` runs unit/integration tests. `monkeybee proof <corpus>`
runs the full proof harness as a separate CI step (it takes longer and requires reference
renderer binaries).

**Artifacts produced per CI run:**
1. `proof-report.json` — aggregate compatibility report across the corpus
2. `regressions.json` — list of test classes that passed in the previous run but fail now
3. `ledger/` — per-document compatibility ledger JSON files
4. `diffs/` — render comparison images for any page with MS-SSIM below threshold
5. `timing.json` — per-test-class timing data for performance regression detection
6. `certificates/` — invariant certificates and digest validation summaries
7. `history/` — temporal replay artifacts for revision-backed fixtures
8. `queries/` — semantic-anchor and typed-query expectation results
9. `capsules/` — self-contained failure capsules for every material proof failure
10. `reproducibility.json` — canonical or ad hoc run manifest for the entire CI/proof invocation
11. `plan-selections/` — typed plan-selection records for save/import/backend/open decisions
12. `oracle-disagreements/` — typed disagreement records with resolution status and gating class
13. `benchmark-witnesses/` — schema-versioned benchmark evidence with threshold verdicts and render determinism classes



### Strategy tournament doctrine

When Monkeybee has multiple candidate strategies for the same job — for example repair heuristics,
render backends, layout analyzers, or optimization passes — the proof harness should run explicit
strategy tournaments rather than relying on taste, anecdote, or benchmark cherry-picking.

Rules:
- every experimental strategy declares the baseline it competes with
- tournaments compare correctness first, cost second
- promotion requires corpus-level wins under pinned manifests and documented thresholds
- every promoted or rejected strategy emits plan-selection records tied to the
  reproducibility manifest for the tournament run
- unresolved blocking oracle disagreements freeze promotion even if raw scores
  otherwise favor the candidate
- automatic exploration (mutation search, candidate generation, offline tuning) is allowed, but no
  strategy self-promotes without human-readable evidence and ordinary review

This captures the best part of the "self-evolving harness" idea while staying faithful to the
spec's auditable baseline-first doctrine.

### Failure capsule doctrine

Every proof-harness regression, oracle disagreement above threshold, ambiguous
repair drift, or native-decoder crash MUST emit a `FailureCapsule`.

```
pub struct DecoderAttestation {
    pub decoder: DecoderType,
    pub backend: String,
    pub isolation_class: NativeIsolationClass,
    pub version: String,
    pub verdict: String,
    pub crash_fingerprint: Option<String>,
}

pub struct FailureCapsule {
    pub input_sha256: String,
    pub minimized_fixture: Option<String>,
    pub oracle_manifest: OracleManifest,
    pub reproducibility_manifest_id: String,
    pub compatibility_ledger: CompatibilityLedger,
    pub trace_stream_ref: Option<String>,
    pub write_receipt_ref: Option<String>,
    pub plan_selection_ref: Option<String>,
    pub oracle_disagreement_ref: Option<String>,
    pub decoder_attestations: Vec<DecoderAttestation>,
    pub repro_command: String,
    pub failure_kind: String,
}

pub struct RepairStabilityRecord {
    pub fixture_id: String,
    pub expected_candidate: RecoveryCandidateId,
    pub actual_candidate: RecoveryCandidateId,
    pub semantic_digest_changed: bool,
}
```

`monkeybee-proof` MUST emit `capsules/` alongside `ledger/`, `diffs/`, and
`regressions.json`.

**Regression detection:**
- The proof harness compares `proof-report.json` against a committed baseline
  (`tests/proof-baseline.json`). Any test class that drops below its pass threshold is a
  regression.
- Performance regressions: any benchmark class that exceeds 1.5x its baseline timing is flagged
  (not blocking, but reported).
- The baseline is updated by committing a new `proof-baseline.json` when regressions are
  intentionally accepted (with a justification in the commit message).

Canonical benchmark runs record:
- benchmark profile id
- hardware / OS image
- compiler version
- security profile
- render determinism class
- provider manifest
- native module / isolation manifest information when applicable
- warm/cold cache state
- percentile outputs (`p50`, `p95`, worst-case`)
- threshold verdicts and witness ids

**Reference renderer setup:**
- CI uses container images with pinned versions of PDFium, MuPDF, Ghostscript, and pdf.js.
- Container digests are recorded in `oracle-manifest.json` at the workspace root.
- The proof harness refuses to run if the oracle manifest doesn't match the actual renderer
  versions (prevents silent oracle drift).

**Corpus management in CI:**
- `tests/corpus/public/` — committed to the repo (small files, permissive licenses)
- `tests/corpus/generated/` — generated by `monkeybee generate` during CI (deterministic)
- `tests/corpus/restricted/` — fetched from a private artifact store during CI (large or
  license-restricted files); not committed to the repo
- `tests/corpus/minimized/` — minimized crashers and regression cases, committed

### Failure accounting

Every test run produces a structured failure report: what failed, on which document, in which subsystem, with what error category, and what the compatibility tier is. Failures are not hidden. They are categorized, tracked, and reduced over time.

### Release gates

v1 may not be released until:
- The pathological corpus passes at a defined threshold across all subsystem harnesses.
- Round-trip chains pass on representative documents from every corpus category.
- The compatibility ledger accounts for all known feature categories with correct tier assignments.
- Performance benchmarks on representative hard workloads are within defined bounds.
- Public correctness and performance claims are backed by canonical witness artifacts linked to reproducibility manifests.
- No silent failures exist: every degradation is detected and reported.
- The proof harness runs in CI and produces machine-readable evidence.

---

## Part 7 — Performance and safety doctrine

### Memory safety rules

- All public APIs are safe Rust. No `unsafe` in public interfaces.
- Internal `unsafe` is permitted only when: (a) it provides a measurable and significant performance improvement on a proven hot path, (b) it is minimal and isolated behind a safe abstraction, (c) it is explicitly documented with a safety justification, and (d) it is covered by aggressive testing including fuzzing.
- Parser code that handles untrusted input must be especially scrutinized. Buffer overflows, integer overflows, and unbounded allocations in parser code are treated as critical bugs.
- All `unsafe` blocks are tagged and auditable.

### Security profiles

### Support-class doctrine

Compatibility claims are qualified by support class:
- `native-compatible`
- `native-hardened`
- `native-strict`
- `wasm-strict`
- `proof-canonical`

Feature tables, ledgers, and generated capability docs must report support in this qualified form.

Monkeybee distinguishes memory safety from execution safety.
`ExecutionContext` selects a security profile:
- `Compatible` — widest feature coverage; all decoders and native bridges enabled
- `Hardened` — bounded / isolated risky decoders; complexity limits enforced more aggressively
- `Strict` — disable risky or non-native features with explicit degradation reporting

High-risk domains include JBIG2Decode, JPXDecode, native font/image bridges, XFA XML packet handling, and Type 4 calculator functions.
All high-risk decode jobs and all optional native bridges execute through `monkeybee-security`
and `monkeybee-native`, with explicit memory/time budgets and optional worker isolation; no crate outside `monkeybee-codec` may invoke them directly.
In hardened mode these run in isolated workers or are disabled; no external-entity XML resolution is ever permitted.

### Native isolation class doctrine

Native integration is support-class-qualified, not a hidden implementation detail.

```
pub enum NativeIsolationClass {
    PureRust,
    InProcessAudited,
    WorkerIsolated,
    BrokeredSubprocess,
    Denied,
}
```

Rules:
- every optional native path declares a `NativeIsolationClass` per support class
  and surfaces it in traces, benchmark witnesses, and failure capsules
- risky decoders and native render/color/font bridges in `native-hardened` and
  `proof-canonical` may run only as `PureRust`, `WorkerIsolated`,
  `BrokeredSubprocess`, or `Denied`
- `InProcessAudited` is an explicit compatible-mode choice that requires narrow
  adapter boundaries, published safety review, and measurable benefit on a
  proven hot path; it is never silently assumed for risky decoders
- isolation downgrades or broker unavailability are plan-selection and
  diagnostic events, not silent fallback
- native invocation outputs are immutable handoff artifacts with explicit size,
  time, and crash/timeout verdicts; partial mutation of engine-owned state is
  forbidden

### Targeted formal verification

Monkeybee does not require "formal methods theater" everywhere. But for a small number of critical invariants where bugs would be catastrophic (security-relevant parser paths, preserve-mode byte integrity), formal verification provides stronger assurance than testing alone:

**Kani proof harnesses for parser safety:**
- **Bounded allocation proof:** For each parser entry point (`parse_object`, `parse_xref`, `parse_content_stream`), prove via Kani that the parser cannot allocate more than `MAX_ALLOCATION_LIMIT` bytes for any input of bounded size. This is a proof that the parser is not vulnerable to zip-bomb or allocation-bomb attacks.
- **No-panic proof:** For the core lexer and tokenizer, prove via Kani that no input sequence (up to a bounded length) can trigger a panic. This covers index-out-of-bounds, integer overflow, and unwrap-on-None paths.
- **Reference resolution termination:** Prove that the indirect reference resolver terminates for all inputs (no infinite loops from circular references). The proof formalizes the visited-set invariant.

**Preserve-mode byte integrity proof:**
- The preserve-mode write path's central invariant is: bytes within the signed byte ranges of existing signatures are never modified. This can be formalized as a post-condition on the `write_incremental` function: for all (offset, length) pairs in the signature's `/ByteRange`, `output[offset..offset+length] == input[offset..offset+length]`. A Kani harness for bounded document sizes can verify this property exhaustively.

These are not aspirational targets. They are specific, scoped verification goals with concrete proof harness designs. The engine can ship v1 without them complete, but the proof harness infrastructure must support them, and at least the no-panic and bounded-allocation proofs for the lexer should be delivered in v1.

### Runtime layering doctrine

Core library crates are runtime-agnostic. They accept `&ExecutionContext` for
cancellation, budgets, and diagnostics but never import asupersync directly.

The `monkeybee` facade, `monkeybee-bytes`, `monkeybee-proof`, and `monkeybee-cli`
are asupersync-native. In these crates, asupersync is not an adapter — it is the
canonical orchestration substrate:

- Session lifecycle is modeled as asupersync regions with parent-child ownership.
- Operations return `Outcome<T, E>` (four-valued: Ok/Err/Cancelled/Panicked).
- Budgets use asupersync's `Budget` semiring with automatic `combine()` tightening
  for child operations.
- Cancellation checkpoints in core crates delegate to `cx.checkpoint()` through
  the `ExecutionContext` bridge.
- The proof harness uses `LabRuntime` with DPOR, oracle suite, and chaos injection
  for deterministic concurrency testing.
- Progressive rendering uses asupersync watch channels for tile completion.
- Fetch scheduling uses asupersync async I/O with structured region ownership.
- Rayon remains the CPU-bound execution layer. The bridge contract is:
  asupersync owns lifecycle and scheduling, Rayon owns pure compute.

A minimal WASM build validates runtime independence: WASM uses a simple
`ExecutionContext` impl without asupersync.

### Open strategies

The engine supports three open strategies that determine how bytes are acquired and objects are resolved:

- `eager`: parse everything available locally. This is the default for CLI, proof, and library workflows on local files.
- `lazy`: resolve objects on demand from a local byte source. Useful for huge local documents where parsing everything upfront is wasteful.
- `remote`: use range requests and a prefetch planner for first-page / region-first latency. Linearization is used when present but not required. The fetch scheduler in `monkeybee-bytes` manages byte-range requests and prefetch planning.

The open strategy is set per `OpenSession` and propagates to all downstream caches and resolution paths.

### Performance doctrine

- Performance is part of the definition of seriousness, not a post-v1 garnish.
- Efficient parsing: avoid unnecessary allocations, use zero-copy where practical, stream processing where possible.
- Disciplined allocation: arena allocation for document objects where appropriate, avoid per-object heap allocation in hot paths.
- Caching: resource cache, font cache, decoded stream cache, with explicit eviction policy.
- Parallelism: page-level parallelism for rendering and extraction where the document model supports it.
- Benchmark classes: small simple PDFs (latency), large complex PDFs (throughput), pathological PDFs (robustness under stress).
- Release-facing performance claims must be backed by schema-versioned
  `BenchmarkWitness` artifacts from canonical or explicitly labeled
  non-canonical runs.

**Benchmark class specifics:**

*Latency profile `desktop-x86_64-cold`:* defined CPU SKU, OS image, compiler version,
security profile, provider manifest, and cold-cache state. Gates use `p50` and `p95`.

*Throughput profile `desktop-x86_64-warm`:* defined hardware + warm-cache state.
Gates use sustained throughput and regression budget against previous canonical run.

*Stress class (pathological PDFs):* Documents designed to stress specific subsystems: pages with 1M+ content stream operators, deeply nested transparency groups (20+ levels), documents with 10,000+ fonts, files with 100+ incremental updates. Target: no operation takes more than 10x the expected time for the content size; no operation causes unbounded memory growth. This class validates that resource limits and algorithmic complexity are under control.

*Round-trip class:* Measures the overhead of load-save-reload cycles. Target: save time under 2x the initial parse time for full-rewrite mode; save time under 0.1x parse time for incremental-append mode (since only changed objects are written). This class measures writer efficiency and change-tracking overhead.

*Memory profile:* defined allocator, artifact-store policy, and corpus subset.
Gates use peak RSS and peak decoded-bytes counters.

**WASM-friendly core target:**

The engine's core crates should remain WASM-friendly. A minimal WASM build is a non-gating proof
surface until baseline v1 is proven. This constraint influences architecture:

- No system font fallback in WASM: the engine must support an explicit font-provision API where the caller supplies font data. The Base 14 font metrics must be compiled in.
- No filesystem access in WASM: all input/output is via byte buffers. The API must support `parse_from_bytes(&[u8])` and `write_to_bytes() -> Vec<u8>`.
- No threads in single-threaded WASM: page-level parallelism must gracefully degrade to sequential execution. Use `cfg(target_arch = "wasm32")` to disable thread pool initialization.
- SIMD: WASM SIMD (128-bit) is available in modern browsers and can be used for compositing and color conversion hot paths. The engine should have SIMD paths for both native (SSE2/AVX2/NEON) and WASM SIMD, with scalar fallbacks.
- Binary size: the WASM binary should be under 5 MiB compressed for the core rendering pipeline. This constrains the use of large lookup tables (CJK CMap data, ICC profiles) — these should be loadable on demand rather than compiled in.

The WASM surface is a live proof artifact: a browser demo where users can load a PDF and see Monkeybee render it, inspect its structure, and add annotations. This is not a v1 shipping product surface, but the architecture must not preclude it.

### WASM constraint propagation

The following crates and features have WASM-specific constraints:

| Crate | WASM constraint | Mitigation |
|---|---|---|
| monkeybee-bytes | No mmap, no filesystem | `ByteSource::InMemory` only; no mmap feature |
| monkeybee-codec | No openjpeg-sys, no native JBIG2 | Pure-Rust decoders only; `Strict` profile |
| monkeybee-security | No process isolation | Budget enforcement via cooperative checks only |
| monkeybee-text | No system font discovery | Explicit `FontProvider` required; Base 14 metrics compiled in |
| monkeybee-render | No threads | Sequential page rendering via `cfg(target_arch = "wasm32")` |
| monkeybee-render | SIMD via wasm-simd128 | Conditional compilation for WASM SIMD paths |
| monkeybee-3d | WebGPU availability varies by browser | Feature-detect WebGPU, degrade to static placeholder when unavailable |
| monkeybee-gpu | WebGPU availability varies by browser | Keep CPU baseline active when GPU backend is unavailable |
| monkeybee-forensics | No native databases or OS inspection | Keep analysis pure-Rust and data-driven; no host probes |
| monkeybee-proof | Not applicable | Proof harness is native-only |
| monkeybee-cli | Not applicable | CLI is native-only |
| all crates | No `std::time::Instant` | Use `web_time` crate or abstract via trait |

**Conditional compilation strategy:**
- `#[cfg(target_arch = "wasm32")]` guards thread pool initialization, mmap paths, and native
  decoder bindings.
- `#[cfg(feature = "wasm")]` gates WASM-specific code paths (web_time, JS interop).
- The `wasm` feature is a workspace-level feature that propagates to all crates.
- WASM builds use `--no-default-features --features wasm` to exclude native dependencies.

**Caching strategy:**

The engine maintains several layered caches, each with explicit eviction policy:

1. **Parsed object cache:** After an object is parsed from bytes, its structured representation is cached keyed on (snapshot_id, object_number, generation_number). Eviction: LRU with a configurable maximum count (default: 50,000 objects). For small documents, everything fits in cache. For very large documents (millions of objects), the LRU ensures recently accessed objects are hot.

2. **Decoded stream cache:** Decoded stream bytes (after filter-chain decompression) are cached keyed on (object_number, generation_number, filter_chain_hash). Eviction: LRU with a configurable maximum total size (default: 256 MiB). Large image streams are the primary consumers. For pages with shared resources (e.g., a logo image used on every page), the cache avoids repeated decompression.

3. **Font cache:** Parsed font data (glyph outlines, metrics, encoding tables, CMap data) is cached keyed on font fingerprint. Fonts are typically small relative to the document and used across many pages. The font cache follows the global `CachePolicy`; referenced fonts may be pinned temporarily, but not unbounded. The font cache is the single largest contributor to multi-page rendering performance.

4. **Glyph rasterization cache:** Rasterized glyph bitmaps, keyed on (font_id, glyph_id, quantized_size, quantized_subpixel_position). Eviction: LRU with configurable maximum count (default: 100,000 entries). Size is quantized to 0.5pt buckets; subpixel position to 1/4 pixel. This prevents cache explosion from continuous-size fonts while maintaining sufficient visual quality.

5. **Color space cache:** Parsed ICC profiles and precomputed LUTs, keyed on the ICC profile stream's object ID. ICC profile parsing and LUT construction are expensive (especially for CMYK→RGB with large cLUT tables); caching is essential.

6. **Resource resolution cache:** Per-page resolved resource dictionaries (the result of page tree inheritance resolution). Avoids repeated tree traversal for multi-pass operations (render + extract on the same page).

7. **PagePlan cache:** Normalized per-page IR keyed on (snapshot_id, page_index, dependency_fingerprint). Eviction: LRU by estimated op count or memory cost. Invalidated when page content, inherited resources, or referenced form/pattern resources change. Enables repeated render/extract/inspect/diff passes without re-interpreting the content stream.

**Parallelism model:**

PDF documents are naturally parallelizable at the page level: pages are independent rendering units with independent content streams, resources, and output buffers. The engine exploits this:

1. **Page-level rendering parallelism:** When rendering multiple pages (e.g., a 100-page document for print), pages are distributed across a thread pool. Each page renders independently. Shared resources (fonts, ICC profiles) are read-only after initial parsing, enabling safe concurrent access.

2. **Page-level extraction parallelism:** Text extraction, image extraction, and diagnostic generation can be parallelized across pages with the same model as rendering.

3. **Stream decompression parallelism:** For documents with many large streams (e.g., scanned documents with one image per page), stream decompression can be parallelized across streams. This provides significant speedup for image-heavy documents.

4. **Limitations on parallelism:** Object parsing from the file is inherently sequential (objects are at fixed byte offsets, and the parser needs random access). For linearized files, pages can be parsed in page order without seeking. The object cache must be thread-safe (concurrent reads, exclusive writes); a read-write lock per cache entry is acceptable given the read-heavy access pattern.

5. **Work-stealing:** Use Rayon or a similar work-stealing thread pool for page-level parallelism. The number of worker threads defaults to the number of available CPU cores. For benchmarking, expose a way to fix the thread count for deterministic measurements.

### Hot-path constraints

The following paths must be profiled and optimized:
- PDF parsing (tokenization, object construction, xref lookup)
- Content stream interpretation (operator dispatch, graphics state updates)
- Text rendering (font lookup, encoding resolution, glyph positioning)
- Image decoding and color space conversion
- Transparency compositing
- Object serialization

**Mathematical hot-path optimization — alien artifact doctrine:**

Monkeybee does not settle for "correct but naive." Where mathematically stronger techniques materially improve correctness, performance, or both, the engine uses them. This is not decorative cleverness — it is the application of genuinely advanced methods to genuinely hard problems that competing PDF engines solve with brute force or hand-waving. The techniques below are selected because they compound: each one improves a hot path that is exercised millions of times per document, and the cumulative effect is an engine that feels qualitatively different from anything else in the open-source PDF ecosystem.

#### Exact analytic area coverage for path rasterization (experimental)

The naive approach to anti-aliased path rasterization is supersampling: render at Nx resolution and downsample. This is slow and produces quality proportional to the supersampling factor. Monkeybee uses exact analytic area coverage instead.

The core idea is Green's theorem: the area integral over a region bounded by a closed curve can be converted to a line integral along the boundary. For each pixel that a path edge crosses, the engine computes the exact fraction of the pixel covered by the path boundary, not an approximation. For cubic Bézier segments (the fundamental curve type in PDF), this means:

1. For each scanline row, find all T-values where the cubic Bézier crosses the pixel row boundaries. This is a cubic polynomial root-finding problem — solve using the depressed cubic formula (Cardano's formula for the discriminant, then the appropriate real-root branch).
2. For each pixel column intersected by the curve segment within the row, compute the signed area contribution using the exact integral of the parametric cubic over the T-interval. The signed area for a cubic Bézier P(t) = (x(t), y(t)) over [t0, t1] within a pixel is: `∫[t0,t1] (x(t) - x_pixel_left) · y'(t) dt`, which expands to a degree-6 polynomial integral that can be evaluated analytically in closed form (the coefficients are precomputed from the control points).
3. Accumulate signed area contributions using a winding-number accumulator per pixel column. After all path segments are processed, each pixel's coverage is the clamped absolute value of the accumulated signed area (for even-odd fill) or the clamped winding number mapped to [0, 1] (for nonzero fill).

This produces mathematically exact anti-aliasing with zero supersampling overhead. The result is provably correct to floating-point precision.

**Fast paths that compose with exact coverage:**
- Axis-aligned rectangles (>60% of all PDF paths: table cells, form fields, backgrounds): detected by checking that all segments are horizontal or vertical. Coverage computation degenerates to simple arithmetic on pixel-edge intersections. No root-finding, no polynomial integration.
- Horizontal/vertical lines: single-pixel-row or single-pixel-column coverage with trivial area computation.
- Quadratic Bézier segments (from TrueType fonts): the signed area integral is a degree-4 polynomial — cheaper than the cubic case.

**Winding number accumulation with SIMD:** The per-pixel winding accumulation across a scanline row is a prefix sum (each pixel's final value is the sum of all contributions from the left). Prefix sums are SIMD-friendly: use SSE2/AVX2 for 4/8-wide parallel prefix sums. For a 1000-pixel-wide scanline, this reduces the accumulation step to ~125 SIMD operations instead of 1000 scalar additions.

#### GPU-accelerated rendering pipeline (experimental)

When wgpu is available, the engine can offload key rendering operations to the GPU:

- **Compute shader path rasterization:** Port the exact analytic area coverage algorithm to a
  compute shader. Each workgroup processes one tile; threads within the workgroup cooperatively
  scan path edges and accumulate winding numbers in shared memory. The GPU path produces
  bit-identical output to the CPU path, verified by the proof harness.
- **Parallel tile compositing:** Transparency group compositing across tiles runs as independent
  GPU compute dispatches. The tile grid maps directly to workgroups and targets the
  transparency-heavy cases where CPU compositing is most expensive, with expected 10-50x speedups
  on those workloads.
- **Texture atlas glyph caching:** Frequently used glyphs are rasterized once and stored in a GPU
  texture atlas (`4096×4096`, `R8` format). Text rendering becomes textured-quad drawing rather
  than per-glyph rasterization, which is orders of magnitude faster on text-heavy pages.
- **Hardware blending for separable modes:** The 12 separable PDF blend modes map directly to GPU
  blend equations. The 4 non-separable modes (Hue, Saturation, Color, Luminosity) use compute
  shader fallback when the backend is active.

#### Advanced compression techniques (experimental)

- **Zopfli maximum-compression Flate output:** For documents where file size matters more than
  write speed, use Zopfli for Deflate compression. Zopfli produces 3-8% smaller output than
  standard zlib at roughly 100x the compression time and is available as a write-mode option.
- **Content stream optimization:** Before compression, analyze content streams for redundant state
  operators (setting a color that is already current, saving/restoring state with no intervening
  changes) and coalesce them. This reduces stream size by roughly 5-20% on producer-generated
  content.
- **Cross-stream deduplication:** Identify identical decoded stream data referenced by different
  objects, common in merged documents or producer-generated duplication, and record dedup
  opportunities in the optimization plan.

#### Performance micro-optimizations

- **Perfect hashing for operator dispatch:** The 73 PDF operators form a fixed set. Use a
  compile-time minimal perfect hash for zero-collision O(1) operator dispatch rather than
  string-heavy matching.
- **SIMD batch color conversion:** Process 4 (SSE2) or 8 (AVX2) pixels simultaneously through the
  ICC color conversion pipeline. The tetrahedral interpolation inner loop is naturally
  SIMD-friendly: 4 vertex lookups plus 3 lerps per pixel.
- **Vectorized string search:** Text search in extracted content uses SIMD-accelerated substring
  search (SSE4.2 `PCMPESTRI` or AVX2 `VPCMPESTRI`) for the inner loop with Rabin-Karp
  fingerprinting for the outer loop.
- **Branch-free pixel blending:** The separable blend-mode inner loop uses conditional moves or
  SIMD select instructions instead of branches, reducing misprediction on mode-switching spans.
- **Bloom filters for name tree lookup:** Large name trees (10,000+ entries, common in documents
  with many named destinations) use a Bloom filter for fast negative lookup before actual tree
  traversal.

#### Robust geometric predicates for clipping and intersection

PDF clipping paths require computing the intersection of arbitrary path regions. Floating-point arithmetic introduces catastrophic errors at geometric degeneracies: near-parallel lines, near-tangent curves, points near edge boundaries. These errors produce visible artifacts — missing pixels, incorrect clipping, spurious regions.

Monkeybee uses Jonathan Shewchuk's robust geometric predicates for all critical geometric decisions:
- **Orientation test:** For three points (a, b, c), determine whether c is to the left, right, or exactly on the line through a and b. Uses the standard 2×2 determinant, but with adaptive-precision floating-point expansion that produces the exact sign even when the determinant is smaller than the rounding error of naive evaluation. The fast filter (simple floating-point evaluation) handles >99% of cases in O(1); the slow path (exact arithmetic via error-free transformations) fires only for near-degenerate configurations.
- **In-circle test:** For point-in-region queries needed during clipping region construction.
- **Segment intersection:** For path-path intersection (needed when clipping paths intersect the content paths), use the robust predicates to classify intersection topology before computing the intersection point itself.

This eliminates an entire class of rendering artifacts that plague every PDF renderer that uses naive floating-point geometry.

#### Spectral-aware color science pipeline (experimental)

Most PDF renderers treat color conversion as a black box: look up the ICC profile, interpolate in the CLUT, done. Monkeybee goes deeper.

**Tetrahedral interpolation for 3D→3D ICC transforms:** The standard approach (trilinear) interpolates in a cube. Tetrahedral interpolation partitions each cube into 6 tetrahedra and interpolates within the containing tetrahedron. This is more accurate for color (the error is bounded by the second derivative of the color mapping within each tetrahedron, which is tighter than the cube bound) and is actually faster because it requires 4 vertex lookups and 3 multiplications instead of 8 lookups and 7 multiplications. The tetrahedron selection is branchless: compare the fractional coordinates and use a lookup table of the 6 possible orderings.

**Precomputed spectral LUTs for known profiles:** For the most common ICC profiles (sRGB IEC 61966-2-1, Adobe RGB 1998, FOGRA39 CMYK, Japan Color 2001 Coated), precompute high-resolution LUTs at initialization (33³ grid points for 3-component, 17⁴ for 4-component). The LUT construction evaluates the full ICC pipeline (Bradford chromatic adaptation → PCS → matrix/TRC or cLUT → output) once per grid point. Runtime conversion is then pure interpolation — zero profile parsing per pixel.

**Chromatic adaptation via Bradford transform:** When converting between color spaces with different white points (common when mixing sRGB content with CMYK proofing), use the Bradford chromatic adaptation matrix rather than the simpler von Kries or XYZ scaling. Bradford's cone-response-domain adaptation is the industry standard (used by ICC v4) and produces visually superior results for saturated colors.

**Perceptual gamut mapping for out-of-gamut colors:** When rendering CMYK content to an RGB output, colors near the gamut boundary of the target space require mapping. Use a minimum-ΔE₀₀ algorithm (CIE DE2000 color difference formula) to find the nearest in-gamut color. This is more perceptually accurate than simple clipping, which can produce hue shifts. Precompute the gamut boundary descriptor as a convex hull in CIELAB and use a k-d tree for fast nearest-boundary-point queries.

**Default color space override with proper fall-through:** When a page defines `/DefaultRGB`, `/DefaultCMYK`, or `/DefaultGray` in its resources, all device color space references implicitly map to the specified CIE-based spaces. The color pipeline must insert this indirection at the correct point — after color space name resolution but before color value interpretation. This is a common source of subtle color errors in PDF renderers.

#### Algebraic blend mode optimization

The 16 PDF blend modes can be partitioned by algebraic properties:
- **Commutative modes** (Multiply, Screen, Difference, Exclusion): B(Cb, Cs) = B(Cs, Cb). For these, the compositing order within a group does not affect the blending result (though it affects alpha compositing). This enables reordering optimizations in tile-based rendering.
- **Idempotent modes** (Darken, Lighten): B(C, C) = C. Consecutive elements with the same color in these modes can be collapsed.
- **Identity-on-white** (Multiply): B(1, Cs) = Cs. Elements composited over a white backdrop in Multiply mode can skip the blending step entirely.
- **Identity-on-black** (Screen): B(0, Cs) = Cs. Similarly for black backdrops.
- **Non-separable modes** (Hue, Saturation, Color, Luminosity): These require conversion to HSL-like space. Implement using the exact formulas from ISO 32000-2 Annex B, which define SetLum() and SetSat() in terms of min/max/mid component extraction. The non-separable modes are 3-5x more expensive than separable modes per pixel — flag them during content stream interpretation so the compositing pipeline can select the fast (separable) or slow (non-separable) inner loop per span.

**Tile-based lazy compositing:** Instead of materializing a full-page RGBA buffer for each transparency group, divide the page into tiles (default 256×256 pixels). Only allocate and composite tiles that are actually touched by the group's content. For documents where transparency groups affect only a small region of the page (common for watermarks, stamps, and overlays), this reduces memory usage and compositing work by 10-100x. The tile grid also enables parallel compositing across tiles.

#### Signed distance field glyph rendering

For text rendering at typical document DPI (150-300), traditional glyph rasterization (rasterize at target size, cache the bitmap) is adequate. But for the WASM/browser surface and for high-quality zoom, Monkeybee supports an optional signed distance field (SDF) path:

1. For each glyph, precompute a low-resolution SDF grid (typically 48×48 or 64×64) where each grid cell stores the signed distance to the nearest glyph outline edge (positive outside, negative inside).
2. At render time, bilinearly interpolate the SDF at each pixel's subpixel position. The interpolated distance value directly gives a smooth coverage estimate: `coverage = smoothstep(-0.5/scale, 0.5/scale, distance)` where scale is the ratio of pixel size to SDF texel size.
3. The SDF representation is resolution-independent: the same 48×48 grid produces crisp text at any size from 6pt to 600pt, with zero re-rasterization.
4. SDF computation for cubic Bézier outlines uses exact closest-point-on-cubic distance: solve the distance-minimization polynomial (degree 5 for cubic curves) using Sturm chain root isolation followed by Newton refinement. This is computed once during SDF construction, not per pixel.

The SDF path is optional (controlled by a rendering mode flag) because it trades memory (one SDF grid per unique glyph in the font cache) for rendering flexibility. For the WASM browser demo, SDFs are the correct approach because they enable GPU-accelerated text rendering via a simple fragment shader.

#### Adaptive curvature-aware mesh subdivision for shading types 4-7

The mesh-based shading types (Coons patch, tensor-product patch, Gouraud triangle mesh) require subdivision into triangles for rasterization. Fixed-depth subdivision wastes work on flat regions and produces artifacts on highly curved regions.

Monkeybee uses adaptive subdivision driven by actual geometric curvature:

1. **Curvature estimation:** For a Bézier patch P(s,t), estimate the maximum curvature κ over the patch using the second partial derivatives: `κ ≈ max(||∂²P/∂s²||, ||∂²P/∂t²||, ||∂²P/∂s∂t||)`. For cubic Bézier patches, the second derivatives are linear in (s,t), so the maximum is at one of the parameter corners — evaluate at all four corners and take the max.
2. **Screen-space error bound:** Compute the maximum screen-space deviation of a linear approximation from the true patch surface: `error_pixels = κ · (patch_screen_size)² / 8`. This is the standard curvature-based flatness criterion.
3. **Recursive subdivision:** If error_pixels > threshold (default: 0.5 pixels), subdivide the patch at the parameter midpoint using de Casteljau splitting. This produces 4 sub-patches for surface subdivision or 2 sub-curves for boundary subdivision. The recursion terminates when all sub-patches are flat enough.
4. **Color interpolation accuracy:** Separately check whether bilinear color interpolation across the patch is adequate by comparing the interpolated midpoint color to the true parametric midpoint color. If the ΔE₀₀ difference exceeds a threshold (default: 1.0 — just noticeable difference), subdivide even if the geometry is flat. This prevents color banding in regions where the geometry is flat but the color gradient is steep.

This produces the minimum number of triangles needed for a given visual quality, with provable error bounds.

#### Information-theoretic repair confidence scoring

When the tolerant parser must choose between multiple recovery strategies (e.g., multiple candidate xref offsets, ambiguous encoding interpretations, conflicting incremental updates), it scores each candidate using a Bayesian confidence framework:

1. **Prior probability:** Based on historical frequency of each failure mode across the pathological corpus. For example, "wrong startxref offset by exactly N bytes" has a known distribution of N values clustered around small offsets — a candidate at offset +3 is more likely than one at offset +4096.
2. **Likelihood:** The structural plausibility of the recovered result. A repaired xref table where every entry's offset points to a valid `N 0 obj` header has higher likelihood than one where some entries point to midstream positions. A font encoding where the decoded text contains valid Unicode codepoints in a single script has higher likelihood than one producing random symbols.
3. **Posterior confidence:** Combine prior and likelihood to produce a normalized confidence score. The repair entry in the compatibility ledger records this score. When multiple strategies succeed, the engine selects the highest-confidence result and records all alternatives with their scores.

This transforms repair from "try things until something works" into a principled selection process with auditable reasoning. The confidence scores feed back into the proof harness: low-confidence repairs are flagged for manual review, and the distribution of confidence scores across the corpus becomes a meta-diagnostic of the engine's robustness.

#### Multi-scale structural similarity (MS-SSIM) for render comparison

The proof harness's render comparison pipeline must distinguish between genuine rendering errors and inconsequential differences (rounding, anti-aliasing, sub-pixel positioning). Pixel-wise comparison (RMSE, PSNR) fails at this: a 1-pixel shift in a thin line registers as a massive error in pixel space but is visually imperceptible.

Monkeybee's render comparison uses multi-scale structural similarity (MS-SSIM):

1. **Compute SSIM at multiple resolutions:** Downsample both images (Monkeybee output and reference output) by factors of 2, 4, 8, 16. At each scale, compute the SSIM index using a Gaussian window (σ=1.5). SSIM combines luminance comparison `l(x,y)`, contrast comparison `c(x,y)`, and structure comparison `s(x,y)`.
2. **Multi-scale combination:** The final MS-SSIM score weights the components across scales: use the luminance comparison only at the finest scale (human luminance sensitivity is scale-dependent), but use contrast and structure comparisons at all scales. The weighting coefficients are from Wang et al. (2003): {0.0448, 0.2856, 0.3001, 0.2363, 0.1333} for 5 scales.
3. **Per-region error localization:** When MS-SSIM drops below threshold (indicating a genuine rendering difference), the engine produces a spatial error map showing exactly which regions differ. This map is the heatmap visualization in the render diff report.
4. **Perceptual significance filtering:** Small MS-SSIM drops in regions of high visual complexity (dense text, detailed images) are less perceptually significant than equal MS-SSIM drops in smooth regions. Weight the error map by local visual complexity (estimated from the Laplacian of the reference image) to filter out perceptually insignificant differences.

This produces render comparison results that correlate with human perception rather than with pixel-level arithmetic, dramatically reducing false positives in the proof harness while catching genuine rendering bugs.

#### Entropy-optimal stream encoding for the write path

The write path must choose compression parameters that minimize output file size. For FlateDecode (the dominant filter), the zlib compression level is a quality-speed tradeoff. But the more interesting optimization is the predictor selection:

1. **PNG predictor selection per row:** PNG predictors (Sub, Up, Average, Paeth) prepend a filter byte to each row before compression. Different rows benefit from different predictors. The optimal strategy: for each row, try all four predictors, compress each with a fast entropy estimate (sum of absolute values of the filtered bytes — this approximates the Shannon entropy without actually running Deflate), and select the predictor with the lowest estimate. This per-row adaptive selection typically reduces file size by 5-15% compared to fixed predictor selection, at negligible CPU cost.
2. **Object stream packing order:** When packing objects into object streams, the order affects compression ratio (similar objects adjacent in the stream compress better). Use a greedy clustering: sort objects by type (dictionaries together, arrays together), then by size, before packing. For font descriptor dictionaries (which share many common keys), this can improve compression by 10-20%.
3. **Cross-reference stream compression:** Use the PNG Up predictor on cross-reference stream entries (each entry's delta from the previous entry is typically small). This reduces the cross-reference stream size by 40-60% compared to raw encoding.

#### Probabilistic layout analysis for text extraction

The text extraction pipeline's reading order inference (Part 3, extraction section) can be significantly improved with probabilistic methods:

1. **Line detection via DBSCAN clustering on Y-coordinates:** Instead of a fixed tolerance for grouping characters into lines, use density-based clustering (DBSCAN with the Y-coordinate as the feature and ε estimated from the median font size). This adapts automatically to documents with mixed font sizes and irregular line spacing.
2. **Column detection via kernel density estimation:** Estimate the probability density of character X-positions across the page. Columns appear as modes (peaks) in this density, separated by valleys (low-density gaps). The number and positions of columns are detected automatically without assuming a grid layout.
3. **Table detection via spatial autocorrelation:** Tables produce regular spatial patterns in character positions. Detect tables by computing the Moran's I spatial autocorrelation statistic on the character position grid. High autocorrelation in both X and Y directions indicates tabular structure. This detects tables even when they lack visible grid lines.
4. **Reading order via topological sort on spatial constraints:** Once lines, columns, and tables are detected, construct a directed graph where edges encode "block A should be read before block B" based on: column assignment (left columns before right in LTR scripts), vertical position within a column (top before bottom), and table structure (row-by-row, left-to-right within each row). Topologically sort this graph to produce the final reading order. When the graph has ambiguities (e.g., floating figures), use the geometric centroid as a tiebreaker.

### Resource limits and adversarial input protection

The engine must enforce configurable resource limits to prevent adversarial inputs from consuming unbounded resources:

- **Maximum nesting depth** for arrays, dictionaries, and page tree traversal (default: 256).
- **Maximum object count** (default: 10,000,000). Documents claiming more objects than this limit are rejected with a diagnostic.
- **Maximum stream decompressed size** (default: 1 GiB). Prevents zip-bomb attacks. The decompression pipeline monitors output size and aborts if the limit is exceeded.
- **Maximum content stream operator count** per page (default: 5,000,000). Pages with more operators than this are truncated with a diagnostic.
- **Maximum page count** (default: 100,000).
- **Timeout** for any single-page operation (configurable, no default — the caller sets this).

All limits are configured and accounted through `ExecutionContext`; actual budget consumption is reportable in diagnostics and traces. The defaults are chosen to handle all legitimate PDFs while rejecting pathological adversarial inputs.

---

## Part 8 — Release gates for v1

v1 is the point where Monkeybee PDF publicly claims engine reality. The following must be true:

### Scope registry doctrine

Every feature is assigned exactly one scope class:
- `v1_gating`
- `v1_supported_non_gating`
- `v1_advisory`
- `post_v1`
- `experimental`

The canonical scope registry lives at `docs/scope_registry.yaml`.
It is machine-readable and CI-validated against:
- release gates
- proof doctrine test classes
- bead appendix
- generated capability docs
- README capability tables
- workspace feature flags

Build-time code generation produces:
- `monkeybee-core::generated::scope_registry`
- `scope-manifest.json`
- `capability-codes.rs`

The proof harness, CLI, generated README capability tables, and workspace feature-flag checks
MUST consume these generated artifacts rather than hand-maintained enum copies.
CI fails on any drift between `docs/scope_registry.yaml`, generated Rust code, and emitted capability docs.

Crate-boundary sections and feature narratives MUST use scope-qualified language.
No subsystem contract may say "must support" for a feature whose registry class is
`post_v1`, `experimental`, or target-qualified non-baseline.
- CLI `capabilities --json`
- workspace feature flags

Each registry entry includes:
- `feature_id`
- `scope_class`
- `support_classes`
- `owning_crate`
- `proof_class`
- `schema_surfaces`
- `bead_ids`
- `notes`

No feature may be `v1_gating` in one section and `post_v1` in another.

`tagged_structure_preservation` MUST have an explicit scope class.
Recommended initial classification: `v1_advisory`.

### Functional gates

- [ ] Parser handles all corpus categories with correct Tier 1/2/3 classification.
- [ ] Renderer produces trustworthy visual output on representative hard documents.
- [ ] 3D annotation detection, PRC/U3D parsing, and baseline 3D rendering work on representative fixtures.
- [ ] Text extraction with positions works on representative documents.
- [ ] Annotation creation, save, and round-trip work on representative documents.
- [ ] Security forensics surfaces detect hidden content, bad redactions, and suspicious post-signing edits on representative fixtures.
- [ ] Document generation produces valid, renderable output.
- [ ] Page-level editing (add, remove, reorder) works with structural validity.
- [ ] Cross-document page import, document merge/split, and provenance-aware remap work without silent collisions.
- [ ] Metadata inspection and modification work.
- [ ] CLI exposes baseline workflows: render, extract, inspect, annotate, edit-pages, validate, diagnose.
- [ ] All three parse modes (Strict/Tolerant/Preserve) are functional.
- [ ] Both write modes (full rewrite, incremental append) are functional.
- [ ] Incremental-append save preserves existing digital signatures on representative signed documents.

- [ ] Content-addressed snapshots, deltas, and structural sharing are operational and externally inspectable.
- [ ] Policy composition and plan selection are inspectable and reject invalid combinations before execution.
- [ ] Historical revision inspection works on representative incremental-update documents.
- [ ] Write receipts carry preservation claims and invariant-certificate references when requested.

### Proof gates

- [ ] Pathological corpus is curated, indexed, and CI-exercised.
- [ ] Render comparison harness runs against at least PDFium and MuPDF.
- [ ] Multi-oracle rendering arbitration is operational (typed disagreements recorded, resolved, or explicitly triaged).
- [ ] Round-trip harness covers all eleven chain types on representative documents.
- [ ] Annotation round-trip harness passes on representative documents.
- [ ] Compatibility ledger is complete and machine-readable per the schema in Part 6.
- [ ] Baseline parser/render/write paths pass the v1 proof gates without experimental backends.
- [ ] The baseline writer passes all write/round-trip gates with plain indirect objects and classic cross-reference tables.
- [ ] Core Arlington validation for catalog/page tree/font/resource/writeback invariants is v1-gating.
- [ ] PDF/A-4 and PDF/X-6 profile validation is advisory in v1 unless backed by public corpus coverage
  and pinned oracle evidence.
- [ ] Experimental backends are optional and benchmarked head-to-head against the baseline.
- [ ] Performance benchmarks exist for all benchmark classes.
- [ ] Canonical benchmark witnesses exist for each benchmark class and record support class, render determinism class, and threshold verdicts.

- [ ] Invariant certificates are emitted, schema-validated, and independently recomputable in proof mode.
- [ ] Ambiguous-repair fixtures produce hypothesis-set evidence rather than silent collapse.
- [ ] Temporal replay and semantic-anchor stability harnesses run on representative fixtures.
- [ ] Canonical proof runs emit reproducibility manifests, plan-selection records, and typed oracle-disagreement artifacts linked from ledgers and capsules.

### Test obligation matrix

Each gated test class has a defined pass threshold and responsible crate:

| Test class | Primary crate | Pass threshold | Metric |
|---|---|---|---|
| xref-repair | parser | 100% of corpus fixtures | All repairs produce valid xref |
| font-fallback | text | ≥95% Unicode coverage on corpus | Extraction F1 vs ground truth |
| transparency-compositing | render | MS-SSIM ≥0.97 vs consensus | Per-page, per-fixture |
| producer-quirks | parser + render | ≥90% of quirk fixtures render correctly | MS-SSIM ≥0.95 |
| incremental-update | parser + write | 100% of corpus fixtures | Parse-save-reparse |
| encryption-read | parser | 100% of standard handlers (V1-V5) | Decrypt success |
| 3d-render | 3d + render | ≥95% of PRC/U3D fixtures render correctly | Screenshot similarity + scene-graph validation |
| annotation-roundtrip | annotate + write | 100% of annotation types | Geometry ≤0.5pt drift |
| page-mutation | edit + document | 100% of mutation ops | Structural validity |
| generation | compose + write | 100% of generation tests | Strict-mode self-parse + ref render |
| adversarial | parser + security | 0 panics, 0 OOM on fuzz corpus | Pass/fail |
| color-space | render | ΔE₀₀ ≤2.0 vs reference on corpus | Per-pixel average |
| content-stream-stress | content | Complete without timeout on corpus | All ops processed |
| signature-preserve | write | 100% byte-range integrity | Byte comparison |

| substrate-delta | substrate + document | 100% of edit fixtures | Changed-subgraph reuse + digest stability |
| historical-replay | bytes + substrate + document | 100% of multi-revision fixtures | Frame-local render/extract/diff consistency |
| hypothesis-recovery | parser + proof | ≥99% candidate-selection stability on ambiguous corpus or explicit unresolved classification | Candidate digests + evidence |
| cross-document-import | document + edit + write | 100% of copy/merge/split fixtures | Provenance map completeness + render/structure validity |
| policy-composition | core + security + write | 100% invalid combinations rejected, 100% canonical combinations stable | Conflict classification + policy digest stability |
| query-acceleration | substrate + extract | ≥95% of large-query fixtures use fresh indexes or explicit scan fallback | Index freshness + fallback accounting |
| reproducibility-manifest | proof | 100% of canonical proof runs | Schema-valid manifest + artifact linkage completeness |
| oracle-disagreement | proof | 100% of above-threshold oracle splits emit typed records | Resolution completeness + blocking-state correctness |
| semantic-anchor-stability | extract | ≥95% stable anchors on semantically unchanged regions | Anchor/alias precision |
| hidden-content-forensics | forensics + extract | ≥95% planted-fixture detection | Precision/recall on known hidden content |
| redaction-audit | forensics + edit | ≥95% intentionally bad redactions detected | Audit precision/recall |
| post-signing-forensics | forensics + signature | ≥95% correct classification on signed corpus | Permitted-vs-suspicious accuracy |
| redaction-safety | edit | Non-gating in v1 unless B-EDIT-003 is separately promoted |

**Regression policy:** A test class that was passing in the previous CI run and fails in the
current run is a blocking regression. The PR cannot merge until the regression is resolved or
explicitly triaged as `known_regression` with a tracking issue.

**Coverage requirement:** Each test class must have ≥10 fixtures in the public corpus tier.
Classes with <10 public fixtures must have a corpus acquisition task tracked in the project.

- [ ] Fuzz testing covers parser, content stream interpreter, and writer (metamorphic + writer fuzzing).
- [ ] Arlington-model validation is integrated for at least document catalog, page tree, and font dictionaries.
- [ ] Public, generated, and minimized corpora exist for every gated feature class; restricted fixtures may supplement but not replace public evidence.
- [ ] Oracle manifests are pinned in canonical CI runs.
- [ ] Every gated fixture has an expectation manifest or explicit `triage_pending` status.
- [ ] CI reports regressions by class and severity, not only by pass/fail.
- [ ] Public scorecards distinguish new failures, expectation drift, known oracle disagreement, and approved degradation.
- [ ] Compatibility ledger JSON is schema-versioned and backward-compatible within major versions.

**Fuzz testing strategy:**

Fuzz testing is the primary mechanism for discovering parser crashes, panics, infinite loops, and unbounded allocations on adversarial input.

*Fuzz targets:* Each parser entry point is a separate fuzz target:
1. `fuzz_parse_document`: full document parsing from arbitrary bytes.
2. `fuzz_parse_object`: single object parsing from arbitrary bytes.
3. `fuzz_parse_xref`: cross-reference parsing from arbitrary bytes.
4. `fuzz_parse_content_stream`: content stream interpretation from arbitrary bytes (with a minimal valid document wrapper).
5. `fuzz_decode_stream`: filter chain decompression from arbitrary bytes (testing each filter individually and in combination).
6. `fuzz_parse_font`: font data parsing (Type 1, TrueType, CFF) from arbitrary bytes.
7. `fuzz_parse_cmap`: CMap parsing from arbitrary bytes.
8. `fuzz_parse_icc`: ICC profile parsing from arbitrary bytes.

*Fuzz corpus seeding:* Each fuzz target is seeded with valid examples from the pathological corpus (real document fragments, real font files, real ICC profiles). The fuzzer mutates these seeds to explore error-handling paths.

*Fuzz invariants (properties that must hold for all inputs):*
- No panics (all fuzz targets catch panics and treat them as failures).
- No undefined behavior (verified by running with address sanitizer and memory sanitizer).
- Memory usage bounded by the configured resource limits.
- Execution time bounded (each fuzz iteration must complete within a timeout, default 10 seconds).
- For `fuzz_parse_document`: if parsing succeeds, the result must be serializable without panics (round-trip sanity).

*Metamorphic testing:* Generate or mutate valid PDFs, apply random edit/annotate/save/reload sequences, and assert render/text/structure invariants. This catches emergent bugs that single-pass fuzzing misses.

*Writer fuzzing:* Fuzz serialization preconditions and parse-own-output invariants, not just the parser. The writer must survive arbitrary valid document graph states without producing invalid output.

*Tooling:* cargo-fuzz with libFuzzer backend for continuous fuzzing. AFL++ as a secondary fuzzer for diversity. Integration with OSS-Fuzz for continuous community-driven fuzzing after open-source release.

### Quality gates

- [ ] Architectural gate: identity model (`DocumentId`, cache-key rules, snapshot/resource identity)
  is finalized before parallel subsystem implementation begins.

- [ ] Architectural gate: persistent substrate root/digest semantics are finalized before document/edit/diff work begins.
- [ ] Architectural gate: incremental query engine dependency semantics are finalized before cache fan-out and performance work begins.
- [ ] Architectural gate: policy-composition rules and plan-selection evidence are finalized before open/save/import API stabilization.
- [ ] Architectural gate: substrate-store lifecycle (root pinning, spill, persistence eligibility, sweep rules) is finalized before large-fixture cache fan-out.
- [ ] Architectural gate: preservation algebra and certificate schema are finalized before save-planning API stabilization.
- [ ] Architectural gate: semantic-equivalence normal forms and alias-map tolerances are finalized before `SemanticEquivalence` becomes proof-gating.
- [ ] Architectural gate: cross-document import/remap/collision semantics are finalized before merge/split/copy-page API stabilization.
- [ ] Architectural gate: dependency graph semantics (raw graph vs condensed DAG) are finalized
  before edit/invalidation/writeback work begins.
- [ ] Architectural gate: session policy and per-operation `ExecutionContext` precedence rules are
  finalized before API stabilization.
- [ ] Architectural gate: annotation/widget appearance generation depends on `monkeybee-paint`, not `monkeybee-render`.
- [ ] Zero silent failures in the proof harness.
- [ ] All errors use the shared error taxonomy.
- [ ] All `unsafe` blocks are documented and tested.
- [ ] Public API is documented with examples.
- [ ] README and website capability tables are generated from proof artifacts + scope registry
      (no manual capability claims).
- [ ] Resource limits are enforced for all adversarial-input-facing code paths.
- [ ] Artifact publication paths for saves, ledgers, capsules, and benchmark witnesses are crash-safe and manifest-last.

### Baseline v1 vs experimental feature classification

The following table consolidates the baseline/experimental classification from across the spec:

| Feature | Classification | Rationale |
|---|---|---|
| Classic xref tables | Baseline | Simpler to audit, required for all PDFs |
| Cross-reference streams | Baseline (read), Post-baseline (write) | Must read; write deferred unless forced by compact mode |
| Object stream packing | Post-baseline | Requires xref streams, adds complexity |

| Content-addressed persistent snapshots | Baseline | Grounds structural sharing, undo, diff, and exact invalidation |
| Incremental query engine | Baseline | Required for large-document edit performance and exact reuse semantics |
| Preservation algebra + invariant certificates | Baseline | Required for explainable save planning and signature evidence |
| Recovery hypothesis tracking | Baseline | Required for truthful tolerant parsing on ambiguous inputs |
| Temporal revision replay | v1_supported_non_gating | High-value forensic surface built on the incremental substrate |
| Document forensics | v1_supported_non_gating | High-value security surface on top of baseline parse/signature/extract data |
| Hidden content detection | v1_supported_non_gating | Baseline forensic detector for deceptive page content |
| Redaction sufficiency audit | v1_supported_non_gating | Baseline forensic audit for unsafe visual-only redactions |
| Post-signing modification forensics | v1_supported_non_gating | High-value signed-document analysis surface |
| Spatial-semantic graph + stable anchors | v1_advisory | Valuable for extraction/automation; not a v1 gate |
| Agent-safe JSON/WASM edit API | Post-v1 | Depends on anchor stability and policy surfaces |
| Collaborative CRDT merge | Post-v1 | Built on serializable deltas; not part of the v1 kernel |
| Reactive document bindings | Experimental | Sandbox-heavy and non-essential to baseline engine reality |
| External cryptographic / zk attestations | Experimental | Baseline Merkle receipts first; stronger proofs later |
| All standard filters (Flate, LZW, ASCII85, etc.) | Baseline | Required for real-world PDFs |
| JBIG2 decode | Baseline on `native-compatible`/`native-hardened`; explicit degradation on `wasm-strict` unless a proven pure-Rust path exists | Target-qualified support |
| JPEG 2000 decode | Baseline on `native-compatible`/`native-hardened`; explicit degradation on `wasm-strict` unless a proven pure-Rust path exists | Target-qualified support |
| 3D PDF rendering (PRC/U3D) | Baseline (Slice A) | Flagship differentiator and first-class reader-kernel surface |
| 3D named views and rendering modes | Baseline | Core 3D interactivity surface |
| 3D cross-sections | v1_supported_non_gating | Valuable 3D interaction feature without blocking the core loop |
| GPU 2D rendering | Experimental | Must beat the CPU baseline under the proof harness |
| Encryption V1-V5 (read) | Baseline | Required for real-world PDFs |
| AES-GCM decryption | Baseline | PDF 2.0 normative supplement |
| Document integrity dictionaries | Baseline (read) | PDF 2.0 normative supplement |
| Encryption (write) | Post-baseline / out of v1 gating | Not needed for v1 proof; deferred entirely from v1 release gates |
| Hash algorithm agility (SHA-3 / SHAKE) | Post-baseline | Depends on CryptoProvider capability expansion |
| Associated files (AF) | Baseline (read), v1_advisory (write) | PDF 2.0 attachment relationships must be visible and preserved where possible |
| Structure namespaces | v1_advisory | Preserve and resolve namespace-qualified structure roles |
| Requirement handlers | Baseline (read) | Must report unsatisfied declared viewer requirements |
| Mesh shadings (types 4-7) | Post-baseline | Rare, complex, not v1-gating |
| Overprint/OPM=1 | Post-baseline | CMYK-specific, not v1-gating |
| Page-level output intents | Baseline | PDF 2.0 color-management correctness |
| Black point compensation | Baseline | Required for correct ICC evaluation in some workflows |
| Geospatial features | Tier 2 | Parse and preserve, not a baseline render gate |
| Exact analytic coverage raster | Experimental | Must beat tiny-skia baseline |
| Subpixel text rendering | v1_supported_non_gating | Quality improvement for viewer-like profiles, not proof-canonical by default |
| Spectral color pipeline | Experimental | Must beat lcms2 baseline |
| SDF glyph rendering | Experimental | Optional, WASM-focused |
| Adaptive mesh subdivision | Experimental | Depends on mesh shading (post-baseline) |
| Bayesian repair scoring | Experimental | Baseline uses strategy-order heuristic |
| MS-SSIM render comparison | Baseline | Required for proof harness |
| Entropy-optimal encoding | Post-baseline | Optimization, not correctness |
| Zopfli compression | Experimental | File-size optimization, not correctness-critical |
| Content stream optimization | Post-baseline | Optimization and compaction, not baseline correctness |
| Path boolean operations | Post-baseline | Advanced clipping and optimization surface |
| Perfect hash operator dispatch | Baseline | Zero-risk performance gain on a fixed operator set |
| SIMD batch color conversion | Baseline | Hot-path acceleration aligned with the baseline color pipeline |
| Probabilistic layout analysis | Post-baseline | Baseline uses geometric heuristics |
| Arlington validation (core) | Baseline | Catalog, page tree, fonts gated |
| Arlington validation (full) | Post-baseline | Full spec coverage deferred |
| PDF/A-4 validation | Advisory in v1 | Unless backed by public corpus |
| PDF/X-6 validation | Advisory in v1 | Unless backed by public corpus |
| WASM build | Non-gating proof surface | Architecture validation, not release gate |
| Kani proofs (lexer) | Baseline | No-panic + bounded-allocation for lexer |
| Kani proofs (full) | Post-baseline | Infrastructure ready, proofs expand post-v1 |
| Linearized output | Post-v1 | Read-only for v1 |
| XFA rendering | Not in v1 (Tier 2/3) | Detect and AcroForm fallback only |
| PostScript XObjects | Not in v1 (Tier 2/3) | Detect and simple subset only |
| JavaScript execution | Not in v1 | Detect and preserve only |

Experimental algorithm details live in a dedicated annex.

### Experimental annex rule

All research-heavy algorithm descriptions are informational unless a scope-registry entry marks them `v1_gating`.
Mainline subsystem contracts must be satisfiable by the auditable baseline implementation.

No baseline subsystem contract may require an experimental algorithm for correctness.
Each experimental item must declare:
- baseline implementation it competes with
- proof metric it must beat
- cost metric it must beat
- fallback behavior when disabled

Experimental items:
- exact analytic area coverage rasterizer
- robust geometric predicates
- spectral-aware color pipeline
- algebraic blend optimization
- SDF glyph path
- adaptive mesh subdivision
- Bayesian repair scoring
- MS-SSIM enhancements beyond baseline comparison
- entropy-optimal write encoding
- probabilistic layout analysis

- differential document morphing / layout optimization
- reactive document binding DSL and sandbox
- external attestations / zk-backed redaction and provenance proofs
- collaboration layer and CRDT-backed delta merge
- strategy tournaments and auto-tuning infrastructure

---

## Part 9 — Bead conversion appendix

When this spec stabilizes through APR refinement, it should be decomposed into beads. The following is the anticipated bead decomposition structure. Each bead will be self-contained, dependency-aware, and carry its own test/E2E/logging obligations.

### Foundation beads
- B-CORE-001: PDF object type definitions and shared primitives
- B-CORE-002: Shared coordinate geometry and transformation pipeline
- B-CORE-003: Error taxonomy and diagnostic types
- B-CORE-004: ExecutionContext (budgets, cancellation, providers, tracing)
- B-CORE-005: Trait definitions (ByteSource, FontProvider, ColorProfileProvider, CryptoProvider)
- B-CORE-006: DiagnosticSink trait and implementations (VecSink, CallbackSink, FilteringSink, CountingSink)
- B-CORE-007: PDF version tracking (input/feature/output versions)
- B-CORE-008: Policy composition, conflict taxonomy, and plan-selection records

### Byte layer beads
- B-BYTES-001: ByteSource trait and implementations (mmap, in-memory, range-backed)
- B-BYTES-002: Revision chain tracking
- B-BYTES-003: Raw span ownership for preserve-mode
- B-BYTES-004: Fetch scheduler and prefetch planning for range-backed sources


### Substrate beads
- B-SUBSTRATE-001: Content-addressed node store and digest scheme
- B-SUBSTRATE-002: Snapshot roots, structural sharing, and subtree-delta computation
- B-SUBSTRATE-003: Incremental query engine and dependency tracking
- B-SUBSTRATE-004: Temporal revision graph and historical snapshot materialization
- B-SUBSTRATE-005: Invariant certificates and Merkle-backed provenance receipts
- B-SUBSTRATE-006: Hypothesis-set storage and candidate-collapse records
- B-SUBSTRATE-007: Store lifecycle, root pinning, spill policy, and maintenance reporting
- B-SUBSTRATE-008: Materialized acceleration indexes and freshness/invalidation tracking

### Document layer beads
- B-DOC-001: Document model and ObjectStore
- B-DOC-002: Cross-reference management and resolution
- B-DOC-003: Page tree and inherited attribute resolution
- B-DOC-004: Resource dictionary resolution chain
- B-DOC-005: Object ownership classification (Owned/ForeignPreserved/OpaqueUnsupported)
- B-DOC-006: Incremental update tracking and merge
- B-DOC-007: EditTransaction and change tracking
- B-DOC-008: Reference integrity index (forward and reverse lookups)
- B-DOC-009: Dependency graph computation, storage, and query
- B-DOC-010: Cache management integration (bounded budgets, eviction, pinning)
- B-DOC-011: Snapshot query interface and semantic-graph federation
- B-DOC-012: Cross-document import, provenance remap, and collision policy
- B-DOC-013: Semantic normal forms and alias-map generation

### Content layer beads
- B-CONTENT-001: Content stream parsing and operator dispatch
- B-CONTENT-002: Graphics state machine
- B-CONTENT-003: Streaming event model
- B-CONTENT-004: PagePlan IR (immutable display list, cached workflows)
- B-CONTENT-005: Marked content span tracking
- B-CONTENT-006: Consumer sink adapters (RenderSink, ExtractSink, InspectSink, EditSink)
- B-CONTENT-007: Content stream error recovery protocol

### Codec beads
- B-CODEC-001: Stream filter implementations (FlateDecode, LZWDecode, ASCII85Decode, etc.)
- B-CODEC-002: Predictor logic (PNG predictors, TIFF predictor)
- B-CODEC-003: Image decode/encode adapters
- B-CODEC-004: Native/isolated decoder shims (JBIG2, JPEG 2000)
- B-CODEC-005: Bounded decode pipelines and decode telemetry

### Security beads
- B-SECURITY-001: Security profiles (Compatible, Hardened, Strict)
- B-SECURITY-002: Budget broker and enforcement
- B-SECURITY-003: Worker isolation / kill-on-overrun
- B-SECURITY-004: Risky-decoder allow/deny policy and hostile-input policy

### Text beads
- B-TEXT-001: Font program parsing and caching
- B-TEXT-002: CMap / ToUnicode handling
- B-TEXT-003: Unicode fallback chain
- B-TEXT-004: Shaping, bidi, and font fallback
- B-TEXT-005: Subsetting and ToUnicode generation for emitted PDFs
- B-TEXT-006: Search, hit-testing, and selection primitives
- B-TEXT-007: Final subset materialization for composed output

### Syntax beads
- B-SYNTAX-001: Immutable parsed COS object representation
- B-SYNTAX-002: Token/span provenance and raw formatting retention
- B-SYNTAX-003: Xref provenance and object-stream membership tracking
- B-SYNTAX-004: Repair record storage and query
- B-SYNTAX-005: Preservation boundary contract enforcement

### Parser beads
- B-PARSE-001: Lexer and tokenizer
- B-PARSE-002: Object parser (all types)
- B-PARSE-003: Delegation to monkeybee-codec for stream decompression and filter chains
- B-PARSE-004: Content stream parser
- B-PARSE-005: Encryption/decryption support
- B-PARSE-006: Tolerant mode and repair strategies
- B-PARSE-007: Parser diagnostics and error recovery
- B-PARSE-008: Strict mode and conformance validation
- B-PARSE-009: Preserve mode and raw byte span retention
- B-PARSE-010: Producer quirk detection and shim layer

### Render beads
B-PAINT-001: Shared appearance/painter primitives
B-CONTENT-002 remains the sole owner of the graphics state machine.
- B-RENDER-002: Path rendering (stroke, fill, clip)
- B-RENDER-003: Text rendering pipeline (font → encoding → glyph → position)
- B-RENDER-004: Image rendering and color space conversion
- B-RENDER-005: Transparency compositing
- B-RENDER-006: Pattern rendering (tiling and shading)
- B-RENDER-007: Page assembly and output backend
- B-RENDER-008: Color space resolution chain (all types including ICC)
- B-RENDER-009: Font type handlers (Type 1, TrueType, CFF, CIDFont, Type 3)
- B-RENDER-010: Shading types 4-7 (mesh-based gradient rendering) [experimental]
- B-RENDER-011: Overprint and overprint mode implementation [experimental until corpus-backed]
- B-RENDER-012: SIMD-optimized compositing inner loops

### 3D beads
- B-3D-001: PRC format parser (ISO 14739-1:2014) — compressed B-rep, tessellated mesh extraction
- B-3D-002: U3D format parser (ECMA-363) — mesh/point/line sets, CLOD progressive mesh decoding
- B-3D-003: Unified scene graph construction from PRC and U3D
- B-3D-004: wgpu render pipeline setup (device/queue/swapchain, shared with monkeybee-gpu)
- B-3D-005: PBR lighting model (Cook-Torrance BRDF, metallic-roughness)
- B-3D-006: Shadow mapping (cascaded shadow maps)
- B-3D-007: Screen-space ambient occlusion (SSAO)
- B-3D-008: Mesh decimation and LOD selection (Garland-Heckbert quadric error metrics)
- B-3D-009: Named view system and camera interpolation
- B-3D-010: Rendering mode switching (solid/wireframe/transparent/illustration/hidden-line)
- B-3D-011: Cross-section plane computation (real-time CSG intersection)
- B-3D-012: Product structure tree navigation and part visibility
- B-3D-013: 3D/RichMedia annotation dictionary parsing and activation behavior
- B-3D-014: 2D/3D compositing (render 3D to texture, composite into page)
- B-3D-015: Order-independent transparency (weighted blended OIT)

### GPU beads
- B-GPU-001: wgpu device/queue management and shared resource pool
- B-GPU-002: Compute shader path rasterization (exact area coverage on GPU)
- B-GPU-003: Parallel tile compositing on GPU
- B-GPU-004: GPU texture atlas for glyph caching
- B-GPU-005: Hardware blend mode implementation (separable modes via GPU blend, non-separable via compute)

### Compose beads
- B-COMPOSE-001: Document/page/content builders for new documents
- B-COMPOSE-002: Resource naming and assembly
- B-COMPOSE-003: Annotation appearance stream generation helpers
- B-COMPOSE-004: Form/widget appearance composition
- B-COMPOSE-005: Font embedding planning and subsetting requests
- B-COMPOSE-006: Content stream emission from high-level drawing/text operations
- B-COMPOSE-007: Generated content stream assembly for pages, appearances, and flattening
- B-COMPOSE-008: Resource closure handoff for serialization

### Write beads
- B-WRITE-001: Object serialization
- B-WRITE-002: Cross-reference generation
- B-WRITE-003: Stream compression
- B-WRITE-004: Full document rewrite (deterministic mode)
- B-WRITE-005: Incremental save (append mode)
- B-WRITE-007: Self-consistency validation (parse own output in strict mode)
- B-WRITE-008: Signature-safe preserve write path
- B-WRITE-010: Output encryption (AES-256) [non-gating / post-baseline]
- B-WRITE-011: WritePlan computation and classification
- B-WRITE-012: Signature impact analysis for save planning

### Edit beads
- B-EDIT-001: EditTransaction framework
- B-EDIT-002: Resource GC and deduplication
- B-EDIT-003: Redaction application (high-assurance rewrite) [post-v1 unless separately proven]
- B-EDIT-004: Optimization operations (compaction, recompression)

### Validate beads
- B-VALIDATE-001: Arlington-model conformance validation
- B-VALIDATE-002: Profile-specific validation (PDF/A-4, PDF/X-6)
- B-VALIDATE-003: Write preflight checks
- B-VALIDATE-004: Signature byte-range verification

### Forms beads
- B-FORMS-001: AcroForm field tree parsing and inheritance resolution
- B-FORMS-002: Field value model (text, button, choice, signature)
- B-FORMS-003: Appearance regeneration for widget annotations
- B-FORMS-004: Calculation order (detection, preservation, evaluation model)
- B-FORMS-005: Widget/annotation bridge
- B-FORMS-006: Signature-field helpers (byte-range, CMS envelope, incremental-append)
- B-FORMS-007: Form data import/export

### Annotation beads
- B-ANNOT-001: Annotation model and type support (non-form annotations)
- B-ANNOT-002: Geometry-aware placement (with rotation and CropBox handling)
- B-ANNOT-003: Appearance stream generation (per-type, per Part 5 contract)
- B-ANNOT-004: Annotation flattening
- B-ANNOT-005: Annotation round-trip validation
- B-ANNOT-006: Bridge to monkeybee-forms for Widget annotations

### Extraction beads
- B-EXTRACT-001: Text extraction with positions
- B-EXTRACT-002: Metadata extraction
- B-EXTRACT-003: Structure and resource inspection
- B-EXTRACT-004: Image and asset extraction
- B-EXTRACT-005: Diagnostic report generation
- B-EXTRACT-006: Spatial-semantic graph and stable anchor generation
- B-EXTRACT-007: Anchor aliasing and agent-facing proposal validation hooks

### Forensics beads
- B-FORENSICS-001: Hidden content detection (white-on-white, off-page, behind-image, invisible text)
- B-FORENSICS-002: Redaction sufficiency audit
- B-FORENSICS-003: Post-signing modification forensics and classification
- B-FORENSICS-004: Known CVE pattern detection
- B-FORENSICS-005: Producer fingerprinting from structural patterns
- B-FORENSICS-006: Font fingerprinting via glyph outline matching

### Proof beads
- B-PROOF-001: Pathological corpus acquisition and indexing
- B-PROOF-002: Render comparison harness
- B-PROOF-003: Round-trip validation harness (all eleven chains)
- B-PROOF-004: Compatibility ledger system (per schema in Part 6)
- B-PROOF-005: Performance benchmark harness
- B-PROOF-006: Fuzz testing harness
- B-PROOF-007: CI integration and evidence artifact pipeline
- B-PROOF-008: Multi-oracle rendering arbitration
- B-PROOF-009: Arlington-model conformance validation integration
- B-PROOF-010: Corpus-level compatibility aggregation and regression tracking
- B-PROOF-011: Capability-matrix generation for README/site/CLI
- B-PROOF-012: Strategy tournament framework for competing backends
- B-PROOF-013: Historical replay harness
- B-PROOF-014: Semantic-anchor stability harness
- B-PROOF-015: Reproducibility manifest generation and artifact linkage
- B-PROOF-016: Oracle disagreement records and plan-selection evidence

### CLI beads
- B-CLI-001: Render command
- B-CLI-002: Extract command
- B-CLI-003: Inspect command
- B-CLI-004: Annotate command
- B-CLI-005: Edit command
- B-CLI-006: Generate command
- B-CLI-007: Validate command
- B-CLI-008: Diagnose command
- B-CLI-009: Proof command
- B-CLI-010: Conformance command
- B-CLI-011: `capabilities` command (emit generated support matrix and proof provenance)
- B-CLI-012: `history` command (enumerate and inspect revision frames)
- B-CLI-013: `query` command (typed structural/semantic queries)
- B-CLI-014: `explain` command (decision trace and preservation claims)

### Checkpoint beads
- B-CHECK-001: Post-foundation Come-to-Jesus checkpoint (core + parser done, verify against North Star)
- B-CHECK-002: Post-render checkpoint (renderer working, verify closed-loop progress)
- B-CHECK-003: Post-writeback checkpoint (round trips working, verify bidirectionality claim)
- B-CHECK-004: Pre-release checkpoint (all gates, verify proof surfaces are externally legible)
- B-CHECK-005: Post-proof checkpoint (proof harness operational, corpus coverage thresholds met)

### Experimental / post-v1 beads
- B-ALIEN-001: Exact analytic area coverage rasterizer (Green's theorem, signed area accumulation, SIMD prefix sums)
- B-ALIEN-002: Robust geometric predicates (Shewchuk-style adaptive precision for orientation, intersection, clipping)
- B-ALIEN-003: Spectral-aware color science pipeline (tetrahedral interpolation, Bradford adaptation, perceptual gamut mapping)
- B-ALIEN-004: Algebraic blend mode optimization (commutativity/idempotency classification, tile-based lazy compositing)
- B-ALIEN-005: SDF glyph rendering path (exact cubic distance, resolution-independent text, WASM-compatible)
- B-ALIEN-006: Adaptive curvature-aware mesh subdivision (curvature estimation, screen-space error bounds, color-accuracy subdivision)
- B-ALIEN-007: Information-theoretic repair confidence scoring (Bayesian prior/likelihood/posterior framework)
- B-ALIEN-008: MS-SSIM render comparison pipeline (multi-scale, perceptual significance filtering, spatial error localization)
- B-ALIEN-009: Entropy-optimal stream encoding (per-row predictor selection, object stream packing order, xref compression)
- B-ALIEN-010: Probabilistic layout analysis for extraction (DBSCAN line detection, KDE column detection, Moran's I table detection)
- B-ALIEN-011: Arlington-model conformance validation (codegen from TSV, strict/tolerant integration, profile-specific checking)
- B-ALIEN-012: Kani proof harnesses (bounded allocation, no-panic lexer, reference termination, preserve-mode byte integrity)

- B-ALIEN-013: Differential document morphing / layout optimization
- B-ALIEN-014: Agent-facing semantic-anchor RPC surface
- B-ALIEN-015: Merkle + optional zk attestation layer for writes and redactions
- B-ALIEN-016: Temporal active-content replay sandbox
- B-ALIEN-017: Collaborative delta / CRDT merge layer
- B-ALIEN-018: Reactive document binding DSL and sandbox
- B-ALIEN-019: Strategy tournament and offline auto-tuning harness
- B-ALIEN-020: Subpixel text rendering (LCD geometry, ClearType filtering, gamma-correct blending)
- B-ALIEN-021: GPU 2D rendering backend (compute shader rasterizer, hardware compositing)
- B-ALIEN-022: Zopfli maximum-compression Flate output
- B-ALIEN-023: Content stream optimization (redundant operator elimination, coalescing)
- B-ALIEN-024: Path boolean operations (Weiler-Atherton with robust predicates)
- B-ALIEN-025: Arc-length parameterization for exact dash patterns (Gauss-Legendre quadrature)
- B-ALIEN-026: SIMD batch ICC color conversion (4/8-wide pixel processing)
- B-ALIEN-027: Perfect hash operator dispatch (compile-time generated, zero-collision)
- B-ALIEN-028: Vectorized string search (SSE4.2/AVX2 accelerated)
- B-ALIEN-029: Post-quantum signature support (ML-DSA, SLH-DSA hybrid PAdES)
- B-ALIEN-030: Vello-style hybrid GPU rendering exploration

## Part 10 — Algorithm inventory summary

The current spec inventory names 104 algorithms and techniques.

- Original current-spec inventory: 57
- 3D rendering: +15
  - PRC parsing, U3D/CLOD parsing, BVH construction, Cook-Torrance BRDF, cascaded shadow maps,
    SSAO, OIT transparency sorting, Garland-Heckbert mesh decimation, cross-section CSG, view
    interpolation, illustration-mode edge detection, toon shading, 2D/3D compositing, product
    structure traversal, progressive mesh decoding
- GPU 2D: +5
  - compute shader rasterization, GPU tile compositing, texture atlas management, hardware blend
    equations, shared device/queue management
- PDF 2.0 crypto supplements: +3
  - AES-GCM authenticated encryption, document integrity dictionary verification, SHA-3/SHAKE hash
    evaluation
- Subpixel text: +4
  - LCD subpixel geometry, ClearType filtering kernel, gamma-correct linear blending,
    fractional-pixel glyph rasterization
- Advanced paths: +4
  - algebraic offset curves, Minkowski-sum round geometry, Weiler-Atherton path booleans,
    Gauss-Legendre arc-length quadrature
- Forensics: +7
  - hidden content detection, redaction sufficiency scan, post-signing modification classification,
    CVE pattern matching, producer fingerprinting, font outline fingerprinting, steganographic
    channel analysis
- Compression: +3
  - Zopfli optimal Deflate, content-stream operator coalescing, cross-stream deduplication
- Performance micro-optimizations: +5
  - perfect hash dispatch, SIMD batch ICC, vectorized string search, branch-free blending, Bloom
    filter name trees
- Post-quantum signatures: +1
  - ML-DSA/SLH-DSA hybrid PAdES

**Total:** 57 + 47 = 104 named algorithms and techniques.

### Inventory expansion addendum

The 104-item total above remains the current locked baseline inventory. This specification now also
names additional expansion lanes so APR/proof work can treat them as real contracts rather than
vibes. Depending on counting policy, they can be tracked in three additive buckets:

- **Priority algorithm/capability uplift: +39**
  - Print production: +9
    - halftone rendering (including Types 1, 5, 6, 10, 16, spot functions, and threshold screens)
    - transfer-function evaluation (`/TR`, `/TR2`)
    - black generation and undercolor removal (`/BG`, `/BG2`, `/UCR`, `/UCR2`)
    - RGB-display overprint simulation
    - soft proofing against output intents or caller-supplied ICC targets
    - ink-coverage / TAC analysis
    - color-separation preview
    - print preflight
    - trap-network detection and rendering
  - Digital signature lifecycle: +8
    - PAdES conformance levels (B-B, B-T, B-LT, B-LTA)
    - DSS modeling
    - VRI modeling
    - certificate-chain building
    - OCSP response handling
    - CRL processing
    - TSA / RFC 3161 timestamp integration
    - signature creation
  - Tagged PDF / accessibility: +10
    - standard structure-element coverage
    - attribute objects and class maps
    - ActualText / Alt-aware semantic extraction
    - expansion text (`/E`)
    - language propagation (`/Lang`)
    - pronunciation metadata
    - artifact marking
    - structure destinations
    - PDF/UA-style validation
    - reading-order visualization
  - Advanced rendering quality: +4
    - Lanczos/Mitchell resampling
    - N-dimensional sampled-function interpolation
    - shading-edge anti-aliasing
    - matte un-premultiplication
  - Advanced forms and interchange: +7
    - XFA static flattening
    - FDF/XFDF round-trip
    - form flattening
    - JavaScript-form-hook detection
    - submit-form target analysis
    - signature-field creation
    - barcode-field handling
  - Full action catalog and link-map extraction: +1
    - typed inventory of the full action family plus navigational link-map extraction
- **Broader catalog/inventory uplift: +12**
  - Document structure surfaces: +7
    - article threads
    - article beads
    - page transitions
    - thumbnails
    - collections / portfolios
    - alternate presentations
    - page-piece dictionaries and web-capture structures
  - Multimedia and rich-content inventory: +5
    - screen annotations
    - sound annotations / sound objects
    - movie annotations
    - media clips
    - rendition trees and player parameters
- **Deep correctness / hardening uplift: +26**
  - Redaction, signatures, and active-content forensics: +3
    - redaction canary scanner across the full emitted file
    - JavaScript action timing graph
    - certification-vs-approval signature classification and MDP-chain validation
  - Font resilience and text correctness: +4
    - CFF subroutine dependency analysis, dead-code elimination, renumbering, and bias recalculation
    - Type 1 alternate-key recovery for damaged encrypted font programs
    - font-descriptor `/Flags` cross-validation against embedded font data
    - CIDFont vertical metrics (`/W2`, `/DW2`)
  - Structure and metadata integrity: +5
    - RoleMap-chain termination and circular detection
    - marked-content nesting repair and audit
    - page/resource-level metadata stream enumeration and preservation
    - web-capture provenance (`/SourceInfo`)
    - structure destinations (`/SD`)
  - Parser/render hardening: +6
    - inline-image resource leakage tolerance
    - name/number-tree `/Limits` validation and repair
    - blend-mode preference-list handling
    - Type 4 function complexity analysis
    - stream-extent cross-validation
    - `/Identity` crypt-filter no-op handling
  - Prepress and color fidelity: +6
    - `/Trapped` tri-state semantics
    - ICC profile version detection and mixed-profile hazard reporting
    - alternate image representations
    - custom spot-function library/catalog
    - PDF 2.0 DeviceN attributes and mixing hints
    - output-intent condition-identifier lookup
  - OCG and annotation rendering detail: +2
    - optional-content configuration sequences (`/Configs`)
    - cloudy annotation border effects (`/BE`)

This yields four useful forward-looking counts:
- `104 + 39 = 143` when tracking only the priority uplift families.
- `104 + 51 = 155` when the document-structure and multimedia inventory lanes are counted too.
- `104 + 39 + 26 = 169` when the hardening uplift is counted alongside the priority uplift.
- `104 + 39 + 12 + 26 = 181` when all currently named uplift families are counted together.

All of these counts are legitimate as long as the counting policy is stated. The architectural
requirement is the same in every case: these lanes are now part of the named ambition and must be
represented in scope, ledger, proof, and implementation planning.

For sequencing purposes, APR rounds should prioritize the 39-item uplift in this order:
signature lifecycle, tagged-accessibility audit, enterprise prepress, form interchange, full
action inventory, then the broader document-structure/multimedia catalog lanes and rendering
quality uplifts. That ordering is about proof leverage, not about shrinking any later lane.

Within the 26-item hardening uplift, the first APR/proof emphasis should be:
redaction canary scanning, certification/approval signature classification, JavaScript timing
graphs, CFF/Type1 font repair and subsetting correctness, then the print-fidelity hazard surfaces
(`Trapped`, ICC versioning, DeviceN attributes, output-condition identifiers, and alternate image
selection). Those items create the clearest information gain per proof artifact.
