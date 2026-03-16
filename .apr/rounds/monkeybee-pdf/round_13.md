I reviewed the implementation reference  and the spec . I did **not** receive a separate README file in the uploaded materials, so this review is grounded in those two documents.

My overall take: the plan is already strong on subsystem boundaries, preservation doctrine, proof culture, and hostile-input realism. The biggest weaknesses are not “missing cleverness”; they are a handful of **spec/implementation mismatches** and a few **under-modeled critical control planes** that will otherwise create friction later: the public API surface, result semantics, provider ownership, signature/permission-aware save planning, active content policy, cache namespace correctness, and ambiguity replay.  

Here are the revisions I would make.

## 1) Make the public surface explicit: add a real `monkeybee` facade crate and a real `monkeybee-diff` crate

**Why this makes it better**

The spec says `monkeybee` is the only semver-stable public crate and even gives `monkeybee-diff` a first-class subsystem contract. But the implementation workspace topology currently enumerates only implementation crates plus the CLI, and “diff” shows up as a report/module concern instead of an owned crate boundary. That is a classic future-maintenance trap: semver ownership gets fuzzy, public docs drift, and “diff” becomes scattered across proof, render, extract, and facade code instead of having a coherent home. Making both crates explicit now will sharpen the architecture and lower future refactor cost.  

It also makes the project more compelling for users. “Render/extract/edit/save/diff” is a real product surface. If diff remains only an implicit report format, it will feel bolted on rather than first-class.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
-`monkeybee` is the only semver-stable public library crate.
-All other workspace crates are implementation crates unless explicitly re-exported by `monkeybee`.
+`monkeybee` is a dedicated facade crate at `crates/monkeybee/` and the only
+semver-stable public library crate.
+`monkeybee-diff` is an implementation crate that owns structural/text/render/
+save-impact comparison; `monkeybee` re-exports the stable diff API.
+All other workspace crates are implementation crates unless explicitly
+re-exported by `monkeybee`.

@@
-Monkeybee is a Cargo workspace with four explicit strata:
+Monkeybee is a Cargo workspace with five explicit strata:
 1. **Byte/revision layer** — immutable source bytes plus appended revisions.
 2. **Syntax/COS layer (`monkeybee-syntax`)** — immutable parsed objects, token/span provenance,
    xref provenance, object-stream membership, raw formatting retention, and repair records.
    This is the preservation boundary.
 3. **Semantic document layer (`monkeybee-document`)** — resolved page/resource/object graph built
    from syntax snapshots; it owns semantic meaning, not raw-byte fidelity.
 4. **Content layer** — parsed content-stream IR and interpreter shared by render/extract/inspect/edit.
+5. **Facade/report layer** — `monkeybee` (stable public API), `monkeybee-diff`,
+   and `monkeybee-cli`.

@@
 #### `monkeybee-diff`
 
 Semantic and multi-surface comparison between documents or snapshots.
+
+`monkeybee-diff` is a required implementation crate, not a report-only concept.
+Its outputs are re-exported by the public `monkeybee` facade.
```

## 2) Unify the public result model: stop mixing `Outcome<T, E>` with `Result<WithDiagnostics<T>, E>`

**Why this makes it better**

Right now the spec says two incompatible things: the Rust API preserves `Outcome<T, E>`, but the public API later returns `Result<WithDiagnostics<T>, MonkeybeeError>`. The implementation reference leans into `Outcome`. If you do not resolve that now, every subsystem will invent slightly different wrappers for cancellation, partial success, diagnostics, and operation-specific reports. That will spread complexity everywhere.  

The clean fix is: make one envelope the public contract. My recommendation is `Outcome<OperationSuccess<T>, MonkeybeeError>`, where `OperationSuccess<T>` contains the value, diagnostics, and a typed operation report. That preserves cancellation semantics, keeps diagnostics first-class, and avoids the “sometimes cancellation is an error, sometimes it is a value” mess.

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
-Operations that can be cancelled return `Outcome<T, E>` rather than `Result<T, E>`.
+Public operations return `OperationOutcome<T>` rather than raw `Result<T, E>`:
+
+```
+pub type OperationOutcome<T> = Outcome<OperationSuccess<T>, MonkeybeeError>;
+```

@@
-At library boundaries (FFI, C API, WASM), Outcome is collapsed to Result with
-structured error discrimination. Within the Rust API, Outcome is preserved.
+At FFI boundaries (C API, WASM bindings), `OperationOutcome<T>` may be collapsed
+to a `Result`-shaped representation with explicit cancellation/panic tags.
+Within the Rust API, `OperationOutcome<T>` is preserved.

@@
-### Library API error contract
-
-Every public API that processes PDF data returns a `Result<T, MonkeybeeError>` where:
+### Library API result contract
+
+Every public API that processes PDF data returns `OperationOutcome<T>`.

@@
-Successful results carry a `Diagnostics` collection alongside the primary value:
+Successful operations carry diagnostics and an operation-specific report alongside
+the primary value:

````

-pub struct WithDiagnostics<T> {
+pub struct OperationSuccess<T> {
pub value: T,
pub diagnostics: Vec<Diagnostic>,
pub has_errors: bool,      // true if any Error-severity diagnostics were emitted
pub has_warnings: bool,    // true if any Warning-severity diagnostics were emitted

* pub report: Option<OperationReport>,
* pub budget_summary: BudgetSummary,
* pub cache_summary: CacheSummary,
  }
*

+pub enum OperationReport {

* Probe(CapabilityReport),
* Render(RenderReport),
* Extract(ExtractReport),
* Write(WriteReport),
* Diff(DiffReport),
  +}

```

@@
-API methods return `Result<WithDiagnostics<T>, MonkeybeeError>`.
+API methods return `OperationOutcome<T>`.
```

## 3) Reconcile provider ownership: the engine should own providers; `ExecutionContext` should carry policy and overrides

**Why this makes it better**

The spec says `ExecutionContext` carries the provider registry, but the implementation puts providers on `MonkeybeeEngine`. The implementation shape is the better default: providers are long-lived, expensive, and process-scoped; they belong on the engine. What *should* be operation-scoped is the resolution policy and any targeted overrides.  

This change improves clarity and performance. It also makes proof-mode, deterministic mode, and caller-supplied overrides easier: the engine owns defaults, while the execution context can narrow or replace them for a single operation without pretending to be the registry owner.

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 Every top-level API accepts an operation-scoped `ExecutionContext` carrying:
 - Resource budgets (objects, decompressed bytes, operators, recursion depth)
 - Cooperative cancellation / deadline
 - Determinism settings for CI and proof
-- Provider registry
+- Provider policy and optional per-operation provider overrides
 - Trace / metrics sink

@@
-Provider interfaces include `FontProvider`, `ColorProfileProvider`, `CryptoProvider`, and `OracleProvider`.
+Provider interfaces include `FontProvider`, `ColorProfileProvider`, `CryptoProvider`,
+and `OracleProvider`.
+Default provider instances live on `MonkeybeeEngine`.
+`ExecutionContext` does not own the provider registry; it carries only the
+policy and any per-call override layer used to resolve providers.

+```
+pub struct ProviderOverrides {
+    pub font_provider: Option<Arc<dyn FontProvider>>,
+    pub color_profile_provider: Option<Arc<dyn ColorProfileProvider>>,
+    pub crypto_provider: Option<Arc<dyn CryptoProvider>>,
+    pub oracle_provider: Option<Arc<dyn OracleProvider>>,
+}
+```

@@
-`ExecutionContext::from_profile(profile)` materializes budgets, cache policy, provider policy,
-determinism, and default write/open behavior from the preset.
+`ExecutionContext::from_profile(profile)` materializes budgets, cache policy,
+provider policy, optional provider overrides, determinism, and default
+write/open behavior from the preset.
````

## 4) Make save planning permission-aware, not just byte-range-aware

**Why this makes it better**

The save plan is already one of the strongest parts of the spec, but it is still too centered on byte ranges. For signed PDFs, that is necessary but not sufficient. Real signed-document workflows care about `/Perms`, `DocMDP`, `FieldMDP`, certification signatures, field locks, and encryption permissions. A plan can preserve signed bytes and still violate the document’s allowed modification policy. That is the difference between “did not rewrite signed bytes” and “was actually safe to do.”  

This change makes Monkeybee much more credible for preserve-mode workflows. It also improves user trust because the tool can explain **why** an edit is blocked or why only some incremental operations are allowed.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 pub struct CapabilityReport {
     pub signed: bool,
     pub encrypted: bool,
     pub tagged: bool,
@@
     pub edit_safety: EditSafetyClass,
+    pub save_constraints: SaveConstraintReport,
     pub preserve_constraints: Vec<PreserveConstraint>,
     pub expected_degradations: Vec<FeatureCode>,
     pub recovery_confidence: RecoveryConfidence,
     pub ambiguity_count: u32,
 }

+pub struct SaveConstraintReport {
+    pub doc_mdp: Option<DocMdpPolicy>,
+    pub field_mdp: Vec<FieldMdpPolicy>,
+    pub encrypt_permissions: Option<PermissionBits>,
+    pub allowed_incremental_ops: Vec<SaveOperationKind>,
+    pub blocked_ops: Vec<BlockedSaveOperation>,
+}

@@
 `WritePlan` additionally records:
 - `edit_intent`
 - `ownership_transitions`
 - `blocked_preserve_regions`
 - `full_rewrite_reasons`
 - `structure_impact`
 - `accessibility_impact`
+- `permission_impact`
+- `byte_patch_plan`

@@
-`BytePatchPlan` is the last inspectable artifact before byte emission.
+`BytePatchPlan` is the last inspectable artifact before byte emission and MUST
+be computable in dry-run mode.
 Preserve-mode and signature-safe guarantees are made against `BytePatchPlan`, not only against
 object-level classifications.
```

## 5) Add an explicit `ActiveContentPolicy` for JavaScript, actions, embedded files, and external references

**Why this makes it better**

The current security model is very good for risky decoders, but “active content” is under-modeled. The spec already detects JavaScript/XFA/RichMedia presence in various places, yet the architecture has no single policy surface for `OpenAction`, `/AA`, `Launch`, `URI`, `SubmitForm`, remote go-to actions, embedded files, or rich media preservation/stripping. That is a safety gap, and it is also a product gap: inspection tools need to explain these things cleanly. 

This is exactly the kind of control plane that keeps an engine serious. Separate decoder safety from active-content safety. They are related, but not the same problem.

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 ### Feature module registry
@@
 Canonical CI/proof runs record the active feature-module manifest alongside the oracle manifest.
+
+### Active content policy
+
+Decoder security and active-content handling are separate control planes.
+JavaScript, action dictionaries, embedded files, rich media, and external
+references are governed by `ActiveContentPolicy`.
+
+```
+pub enum ActiveContentPolicy {
+    PreserveButDenyExecute,
+    StripOnWrite,
+    ErrorOnPresence,
+    AllowTrustedHandlers,
+}
+
+pub struct ActiveContentReport {
+    pub has_open_action: bool,
+    pub has_additional_actions: bool,
+    pub has_javascript: bool,
+    pub has_launch: bool,
+    pub has_uri: bool,
+    pub has_submit_form: bool,
+    pub has_remote_goto: bool,
+    pub has_embedded_files: bool,
+    pub has_rich_media: bool,
+}
+```
+
+`CapabilityReport` MUST include `active_content: ActiveContentReport`.
+The default v1 behavior is `PreserveButDenyExecute`.
````

## 6) Strengthen cache correctness and proof reproducibility with a real cache namespace and stronger determinism controls

**Why this makes it better**

The spec’s cache doctrine is conceptually good, but the implementation sketch is still too coarse in a few places, especially the page-plan cache. A `PagePlan` or rendered tile can vary not just by snapshot and page number, but also by view state, security profile, provider manifest, optional-content configuration, substitution policy, and determinism mode. The implementation cache keys are not rich enough yet, which creates a real risk of cross-profile contamination or hard-to-reproduce proof failures.  

This is one of those changes that quietly improves both correctness and performance: you avoid invalid reuse while still keeping cache reuse explicit and explainable.

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 `CachePolicy` defines:
 - in-memory byte budget
 - spill-store byte budget
 - optional persistent derived-artifact store policy
 - per-cache admission rules
 - pinning rules
 - eviction rules
 - deterministic mode behavior
 - wasm/native default profiles
+
+Every cache key belongs to a `CacheNamespace`:
+
+```
+CacheNamespace = (
+  snapshot_id,
+  security_profile,
+  provider_manifest_id,
+  determinism_class,
+  view_state_hash
+)
+```
+
+`view_state_hash` covers any setting that can change visible or extracted output
+without changing document bytes (for example optional-content configuration,
+substitute-font policy, and active-content policy).

@@
-- `PagePlanCache`          key=(snapshot_id, page_index, dependency_fingerprint, profile_hash)
-- `RasterTileCache`        key=(snapshot_id, page_index, tile_id, dpi, completeness, profile_hash)
+- `PagePlanCache`          key=(cache_namespace, page_index, dependency_fingerprint, pageplan_mode_hash)
+- `RasterTileCache`        key=(cache_namespace, page_index, tile_id, dpi, completeness, render_profile_hash)

@@
 pub struct DeterminismSettings {
     pub deterministic_output: bool,    // canonical serialization order, stable hashers
     pub pinned_fallback_fonts: bool,   // use pinned font pack instead of system fonts
     pub fixed_thread_count: Option<usize>,  // for reproducible benchmarks
+    pub stable_task_order: bool,
+    pub canonical_float_reductions: bool,
+    pub deterministic_diagnostic_order: bool,
 }
````

## 7) Turn ambiguity handling into a first-class replayable artifact

**Why this makes it better**

The spec already says tolerant mode can emit `parse.repair.ambiguous`, and the compatibility ledger schema already allows ambiguity entries. That is good. But it is still missing the concrete replay model: what exactly is a candidate, how is it identified, how do you reopen with a specific candidate, how do you compare their write impact? Without that, ambiguity remains diagnostic text rather than a reproducible artifact. 

This is a high-value change for reliability and forensics. It turns “we saw ambiguity” into “we can replay candidate B, inspect why it differs, and compare its save impact.”

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@
-  **Ambiguity rule:** if multiple recovery strategies produce materially different semantic
-  outcomes (page count, object graph, text decode, signature coverage, or write impact) and no
-  deterministic tiebreaker exists, tolerant mode emits `parse.repair.ambiguous`.
-  By default, `engine.open()` returns the highest-confidence candidate plus the ambiguity record;
-  `ForensicPreserve` may instead reject ambiguous recovery unless
-  `allow_ambiguous_recovery=true`.
+  **Ambiguity rule:** if multiple recovery strategies produce materially different semantic
+  outcomes (page count, object graph, text decode, signature coverage, or write impact) and no
+  deterministic tiebreaker exists, tolerant mode emits `parse.repair.ambiguous`.
+  By default, `engine.open()` returns the highest-confidence candidate plus a `RepairDecision`
+  that records every materially different `RecoveryCandidate`.
+  `ForensicPreserve` may instead reject ambiguous recovery unless
+  `allow_ambiguous_recovery=true`.

+```
+pub struct RecoveryCandidate {
+    pub candidate_id: RecoveryCandidateId,
+    pub confidence: f64,
+    pub semantic_digest: [u8; 32],
+    pub page_count: u32,
+    pub write_impact: WriteImpactPreview,
+    pub diagnostics: Vec<Diagnostic>,
+}
+
+pub struct RepairDecision {
+    pub chosen: RecoveryCandidateId,
+    pub alternatives: Vec<RecoveryCandidateSummary>,
+    pub reason: String,
+}
+```

@@
-`OpenProbe` returns a preliminary `CapabilityReport`, an estimated complexity class, and a
-recommended `OperationProfile`.
+`OpenProbe` returns a preliminary `CapabilityReport`, an estimated complexity class,
+a recommended `OperationProfile`, and any `RecoveryCandidateSummary` records that
+can be determined cheaply.
+
+The facade exposes:
+
+```
+engine.open_with_candidate(byte_source, open_options, candidate_id, &exec_ctx)
+```
````

## 8) Resolve scope-registry contradictions directly in subsystem contracts

**Why this makes it better**

This is the most important editorial/architectural cleanup. The scope registry says mesh shadings and overprint/OPM=1 are not baseline v1 features, but other sections describe them as unconditional renderer responsibilities. That contradiction will generate endless thrash: implementation teams will not know whether to build them now, test gates will be ambiguous, and capability claims will drift. 

The fix is simple: make every subsystem contract use scope-qualified language. If a feature is post-baseline, experimental, or target-qualified, the contract should say so explicitly. That keeps the plan honest without reducing ambition.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@
 Key responsibilities:
 - Consumption of `monkeybee-content` events or `PagePlan` IR through backend adapters
@@
-- Patterns: tiling patterns, shading patterns (all function and axial/radial/mesh types)
-- Graphics state: CTM, clipping, line properties, rendering intent, overprint
+- Patterns: tiling patterns plus function/axial/radial shadings in the baseline;
+  mesh shadings are target-qualified and may degrade explicitly until promoted by
+  the scope registry.
+- Graphics state: CTM, clipping, line properties, rendering intent, and overprint
+  state tracking; full OPM=1 semantics follow the support-class/scope-registry table.

@@
-**Overprint and overprint mode:** Overprint (`/OP`, `/op`) controls whether painting in one colorant erases other colorants in the same area. Overprint mode (`/OPM`) modifies the behavior for DeviceCMYK: OPM=1 means a zero component value does not overwrite the corresponding backdrop component (the "nonzero overprint" rule). This matters for CMYK-heavy print-oriented PDFs and is a common source of visual differences between renderers. The engine must: (a) track the overprint state in the graphics state, (b) implement the OPM=1 nonzero rule for DeviceCMYK, (c) extend overprint semantics to Separation and DeviceN color spaces (overprint applies per-component based on the colorant names).
+**Overprint and overprint mode:** Overprint (`/OP`, `/op`) controls whether painting in one
+colorant erases other colorants in the same area. Baseline v1 MUST track overprint state
+and emit explicit diagnostics when OPM=1 or Separation/DeviceN overprint semantics are not
+available on the active support class. Full OPM=1 nonzero-overprint behavior becomes Tier 1
+only after scope-registry promotion and proof-harness coverage.

@@
 The canonical scope registry lives at `docs/scope_registry.yaml`.
@@
 - workspace feature flags
+
+Crate-boundary sections and feature narratives MUST use scope-qualified language.
+No subsystem contract may say "must support" for a feature whose registry class is
+`post_v1`, `experimental`, or target-qualified non-baseline.
```

---

My priority order would be:

1. Change 2 (unified result model)
2. Change 4 (permission-aware save planning)
3. Change 6 (cache namespace + determinism)
4. Change 1 (explicit facade/diff topology)
5. Change 5 (active content policy)
6. Change 7 (replayable ambiguity model)
7. Change 3 (provider ownership cleanup)
8. Change 8 (scope-registry contradiction cleanup)

Those eight changes would make the project more coherent, safer, easier to implement correctly, and much more credible when it starts making strong claims in public.
