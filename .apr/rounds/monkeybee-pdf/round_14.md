I read the README, implementation master, and the SPEC together. The core thesis is already very strong: one closed-loop engine, an explicit syntax-preservation boundary, an ownership-aware mutation/writeback model, a proof-first release posture, and a compatibility doctrine that refuses silent failure. The biggest improvements I’d make are not simplifications; they’re additive changes that turn a few cross-cutting promises into first-class subsystems, and they remove some current spec/implementation drift before code starts to harden.   

The most important architectural issue I found is that the docs do not yet fully agree on workspace topology. The README presents `monkeybee` as the stable public facade and exposes `DiffReport`; the SPEC says `monkeybee-diff` is an implementation crate; but the implementation workspace member list omits both `crates/monkeybee` and `crates/monkeybee-diff` even while describing `monkeybee` public modules. Even if that omission is only editorial, it is exactly the kind of drift that will later turn into broken ownership boundaries, confused dependency graphs, and “where does this logic actually live?” churn.   

## 1) Make the workspace topology match the public promise

This is my first change because it is both a correction and an architectural improvement. Revision diffs are already a public workflow and part of “operational explainability,” so diffing should exist as a real implementation owner, not just as a report name hanging off the facade. Likewise, the stable public facade should be explicitly present in the workspace definition wherever the implementation reference enumerates members. If you fix only one thing before implementation begins, fix this.    

```diff
diff --git a/README.md b/README.md
@@
 | `monkeybee` | Stable public facade: semver-governed `Engine`, `OpenProbe`, `Session`, `Snapshot`, `EditTransaction`, `WritePlan`, `CapabilityReport`, `DiffReport`, and high-level open/render/extract/edit/save APIs |
+| `monkeybee-diff` | Structural, text, render, and save-impact comparison engine reused by the facade, proof harness, and CLI |
@@
 ├── crates/
 │   ├── monkeybee/
+│   ├── monkeybee-diff/
 │   ├── monkeybee-core/
diff --git a/implementation_master.md b/implementation_master.md
@@
 ├── crates/
+│   ├── monkeybee/               # stable public facade crate
 │   ├── monkeybee-core/
+│   ├── monkeybee-diff/          # structural/text/render/save-impact diff engine
@@
 members = [
+    "crates/monkeybee",
     "crates/monkeybee-core",
@@
+    "crates/monkeybee-diff",
     "crates/monkeybee-cli",
 ]
@@
+monkeybee-diff         (depends on: core, document, content, extract, render, write)
+monkeybee              (depends on: diff, core, bytes, document, render, extract, edit, write, validate)
```

## 2) Pull signature-safe workflows into a dedicated `monkeybee-signature` crate

Signature-safe modification is one of the plan’s marquee user workflows, but today the responsibility is spread across forms, validate, write planning, capability reporting, and the crypto provider. That is workable on paper, but in implementation it creates a dangerous kind of distributed ownership: one subsystem knows byte ranges, another knows DocMDP/FieldMDP, another knows verification, another decides whether a save is allowed. A dedicated implementation crate would make the flagship promise much more auditable: signature parsing, byte-range maps, permission policies, verification wiring, and save-impact classification would live together. That would also make the “explain edit safety before bytes are emitted” promise materially stronger.     

```diff
diff --git a/SPEC.md b/SPEC.md
@@
-`monkeybee-diff` is an implementation crate that owns structural/text/render/save-impact comparison; `monkeybee` re-exports the stable diff API.
+`monkeybee-diff` is an implementation crate that owns structural/text/render/save-impact comparison; `monkeybee` re-exports the stable diff API.
+`monkeybee-signature` is an implementation crate that owns signature dictionaries, byte-range maps,
+DocMDP/FieldMDP policy, timestamp/trust metadata, verification plumbing, and write-impact classification.
@@
-5. **Facade/report layer** — `monkeybee` (stable public API), `monkeybee-diff`, and `monkeybee-cli`.
+5. **Facade/report layer** — `monkeybee` (stable public API), `monkeybee-diff`, `monkeybee-signature`, and `monkeybee-cli`.
@@
 pub struct CapabilityReport {
-    pub signed: bool,
+    pub signed: bool,
+    pub signature_summary: SignatureSummary,
@@
 pub struct SaveConstraintReport {
     pub doc_mdp: Option<DocMdpPolicy>,
     pub field_mdp: Vec<FieldMdpPolicy>,
     pub encrypt_permissions: Option<PermissionBits>,
     pub allowed_incremental_ops: Vec<SaveOperationKind>,
     pub blocked_ops: Vec<BlockedSaveOperation>,
+    pub signature_impact: SignatureImpactReport,
 }
diff --git a/README.md b/README.md
@@
+| `monkeybee-signature` | Signature parsing, byte-range preservation, DocMDP/FieldMDP policy, verification wiring, and save-impact analysis |
diff --git a/implementation_master.md b/implementation_master.md
@@
+│   ├── monkeybee-signature/     # signature dictionaries, byte-range maps, policy + verification
```

## 3) Turn the scope registry from a doctrine into generated code

The scope-registry idea is excellent. Right now it is already machine-readable and CI-validated against release gates, proof classes, capability docs, README tables, and feature flags. I would take the next step and generate Rust code plus a scope manifest from it, then require the CLI, proof harness, docs generator, and feature-flag validation to consume the generated artifact rather than hand-maintained mirrors. That would prevent exactly the kind of drift already visible in the workspace topology, and it would make support-class claims provably consistent across code, docs, and CI.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 The canonical scope registry lives at `docs/scope_registry.yaml`.
 It is machine-readable and CI-validated against:
@@
 - workspace feature flags
+
+Build-time code generation produces:
+- `monkeybee-core::generated::scope_registry`
+- `scope-manifest.json`
+- `capability-codes.rs`
+
+The proof harness, CLI, generated README capability tables, and workspace feature-flag checks
+MUST consume these generated artifacts rather than hand-maintained enum copies.
+CI fails on any drift between `docs/scope_registry.yaml`, generated Rust code, and emitted capability docs.
diff --git a/implementation_master.md b/implementation_master.md
@@
 │   │   ├── version.rs        # PdfVersion tracking (input, feature, output), version-gated feature registry
+│   │   ├── scope.rs          # generated support/scope registry bindings
```

## 4) Freeze tolerant-repair decisions with expectation manifests, not just diagnostics

The tolerant parser already has the right conceptual model: ambiguous recovery yields `RecoveryCandidate`s and a `RepairDecision`, with `semantic_digest` and `write_impact` carried forward. What is missing is a regression contract for those choices. Without that, you can change repair heuristics, still get a parse, still render “something,” and silently shift page count, text decode, signature coverage, or future write semantics. I would add repair expectations to fixture manifests and make chosen-candidate drift a triaged proof event. That is the right way to make tolerant mode powerful without making it slippery.    

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 Every fixture also carries an expectation manifest: expected tier assignments, allowed degradations, render-score thresholds, extraction goldens or invariants, signature expectations, and triage status (`approved`, `pending`, `known_bad`, etc.).
+Expectation manifests may also freeze tolerant-repair behavior:
+- expected chosen `RecoveryCandidateId`
+- expected `semantic_digest`
+- allowed alternative candidate ids
+- whether `write_impact` equivalence is required
+
+Changing the chosen repair candidate for an existing fixture is a proof regression unless explicitly triaged.
diff --git a/implementation_master.md b/implementation_master.md
@@
 - Corpus manifest tests: every fixture has an `ExpectationManifest`.
+- Repair expectation tests: fixtures with ambiguous recovery assert chosen candidate id, semantic digest,
+  and write-impact class unless explicitly waived.
```

## 5) Introduce a first-class `LayoutGraph` for extraction

Extraction is one of the plan’s central proof surfaces, and the current surfaces are already strong: `PhysicalText`, `LogicalText`, and `TaggedText`, with reading-order confidence. But the advanced probabilistic reading-order work is explicitly deferred post-baseline. The best way to keep baseline and advanced extraction coherent is to define a shared internal IR now: a `LayoutGraph` or `DocumentSurfaceGraph` containing spans, lines, blocks, reading-order edges, tag links, table hypotheses, and confidence. Baseline geometric heuristics can populate it today; later probabilistic models can improve the same graph instead of inventing a second extraction worldview. That would also make diffing, search, hit-testing, tagging, and future structure tooling align better.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 #### `monkeybee-extract`
@@
 - Multi-surface text extraction, metadata, structure inspection, asset extraction, diagnostics
+- Internal `LayoutGraph` IR: spans, lines, blocks, reading-order edges, tag links, table/column hypotheses, and confidence scores
@@
-pub struct ExtractResult {
-    pub surface: PhysicalText | LogicalText | TaggedText,
-    pub report: ExtractReport,
-}
+pub struct ExtractResult {
+    pub layout_graph: Arc<LayoutGraph>,
+    pub surface: ExtractSurface,
+    pub report: ExtractReport,
+}
+
+pub enum ExtractSurface {
+    Physical(PhysicalText),
+    Logical(LogicalText),
+    Tagged(TaggedText),
+}
diff --git a/implementation_master.md b/implementation_master.md
@@
 │   │   ├── physical.rs       # PhysicalText: exact glyph geometry
 │   │   ├── logical.rs        # LogicalText: reading-order with confidence
 │   │   ├── tagged.rs         # TaggedText: structure-tree-driven extraction
+│   │   ├── layout_graph.rs   # shared extraction IR for spans/blocks/order/tables/tags
```

## 6) Add a reusable `AccessPlan` for remote/progressive workflows

The current remote/progressive story is good at runtime behavior: placeholders, refinement, byte-range metadata, prefetch planning, and linearization-aware fetching. But the plan is still too render-call-centric. I would add a first-class `AccessPlan` artifact that sits between probe/open and rendering: per-page object/resource dependencies, critical byte ranges, linearization hints, and fallback heuristics, all cacheable and reusable across repeated region renders. That would improve first-paint latency, reduce duplicated dependency discovery, and make the remote path feel like a true architectural lane rather than a renderer extension.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 `OpenProbe` returns a preliminary `CapabilityReport`, an estimated complexity class,
-a recommended `OperationProfile`, and any `RecoveryCandidateSummary` records that
+a recommended `OperationProfile`, an optional `PreliminaryAccessPlan`, and any `RecoveryCandidateSummary` records that
 can be determined cheaply.
+
+`AccessPlan` is a reusable artifact for lazy/remote sessions. It records:
+- page -> object/resource dependency closure
+- critical byte ranges for first paint
+- linearization-derived page hints when available
+- fallback xref-derived byte ranges when linearization is absent or damaged
+- viewport-priority fetch groups for region rendering
diff --git a/implementation_master.md b/implementation_master.md
@@
 │   │   ├── fetch.rs          # fetch scheduler and prefetch planning for remote/lazy sources
+│   │   ├── access_plan.rs    # reusable page/resource/byte-range access plans
```

## 7) Make redaction assurance an explicit report and default fail-closed

The current redaction design is much better than most PDF plans: it distinguishes `SemanticExact`, `SecureRasterizeRegion`, and `SecureRasterizePage`; it makes post-apply verification mandatory; and it already includes redaction-safety proof cases. I would push that one step further and make assurance a first-class API object. `apply_redactions()` should return a `RedactionAssuranceReport`, and save should fail by default unless the achieved assurance level satisfies caller policy. That turns a strong internal doctrine into a user-visible trust contract, which is exactly where high-assurance PDF tooling should differentiate itself.   

```diff
diff --git a/SPEC.md b/SPEC.md
@@
 When a redaction is applied, the engine first constructs a `RedactionPlan` and selects one of:
@@
 Post-apply verification is mandatory: extraction must not recover redacted text, and surviving canary bytes/resources are scanned where feasible.
+
+`apply_redactions()` returns a `RedactionAssuranceReport`:
+- selected apply mode
+- text-extraction verification result
+- resource/canary leakage verification result
+- unresolved risks
+- proof artifact references
+
+Default policy is fail-closed: if the achieved assurance level is below caller policy, the save is rejected.
diff --git a/implementation_master.md b/implementation_master.md
@@
 │   │   ├── redaction.rs      # high-assurance redaction application
+│   │   ├── assurance.rs      # redaction assurance reports and policy evaluation
```

## 8) Automate corpus reduction into the existing `minimized/` tier

The proof doctrine already has the right ingredients: crashpacks, manifests, minimized fixtures, triage fields, and CI evidence. What it does not yet make explicit is an automated reducer pipeline that turns new crashes or oracle divergences into minimized repros preserving the failure signature that matters. I would add that as a first-class part of `monkeybee-proof`. This is one of those additions that does not look glamorous in the spec, but in practice it massively improves robustness, contributor productivity, and long-term maintainability of a pathological corpus.   

```diff
diff --git a/implementation_master.md b/implementation_master.md
@@
 │   │   ├── fuzz.rs           # fuzz testing coordination
 │   │   └── evidence.rs       # artifact generation
+│   │   ├── reducer.rs        # automatic failure minimization preserving crash/divergence signature
@@
 - Crashpacks: Concurrency failures automatically produce crashpacks with seed,
   trace fingerprint, oracle failures, and replay command.
+- Reducer: crashpacks and proof failures may be minimized automatically into
+  `tests/corpus/minimized/` fixtures with stub expectation manifests.
diff --git a/SPEC.md b/SPEC.md
@@
 Crashers and regressions must be minimized into the `minimized/` tier whenever feasible.
+`monkeybee-proof` includes an automated reducer that preserves one or more target signatures:
+- panic fingerprint
+- render-diff signature
+- extraction mismatch signature
+- repair-decision semantic digest
+- signature-impact classification
```

## What I would prioritize

If you want the best revision order, I would do these in two waves.

Before any serious implementation fan-out, I would land 1 through 4. Those are foundational: they eliminate ownership drift, make signature preservation auditable, turn scope policy into enforceable code, and make tolerant repair behavior regression-testable. Those four changes strengthen the architecture itself. 

Then I would land 5 through 8. Those make the project more useful and more compelling in practice: better extraction architecture, better remote-first behavior, explicit redaction assurance, and a proof pipeline that continuously creates smaller repros instead of just reporting larger failures.    

Netting it out, my biggest takeaway is this: the plan already has the right philosophical center, but its next leap should be to promote its most important cross-cutting promises—diff, signatures, scope gating, repair stability, extraction structure, remote access, redaction assurance, and failure reduction—from “excellent narrative contracts” into explicit implementation owners and schema-bound artifacts. That is the shortest path from a brilliant spec to a robust engine.   
