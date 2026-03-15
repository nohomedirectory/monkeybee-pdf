I only received `SPEC.md`, not a separate README, so this review is grounded in the plan itself. The plan is already unusually strong on closed-loop thinking, explicit modes, immutable snapshots, and proof-backed compatibility rather than “roadmap theater.” 

The biggest improvements I’d make are not about adding more breadth. They are about making the existing breadth safer to implement, harder to contradict, and easier to prove. Right now the main risks are: foundational invariants living outside the delivery spine, a few scope contradictions between sections, preserve-mode claims stopping at object classes instead of concrete byte patches, and reproducibility depending too much on ambient host state.

## 1) Add a blocking “Foundation freeze” slice before Slice A

The spec already says several architectural decisions must be finalized before parallel subsystem work begins: identity model, dependency graph semantics, `ExecutionContext` precedence, and the paint/render boundary. But those are currently quality gates, not a first-class slice. That makes them easy to underweight even though they are the real schedule-critical dependencies. I would promote them into an explicit pre-A slice and make all later beads depend on it. This reduces architectural churn, parallel rework, and “local optimizations” that later break preserve mode or cache correctness.

```diff
@@ ### Delivery spine
-Monkeybee defines four release slices:
+Monkeybee defines five release slices:
+
+0. **Slice F — Foundation freeze**
+   - identity model (`DocumentId`, `SnapshotId`, `ResourceFingerprint`)
+   - `ExecutionContext` precedence and provider-policy rules
+   - ownership lattice + mutation/writeback invariants
+   - render/content/paint boundary freeze
+   - scope registry bootstrap (see Part 8)
+   - cache-key doctrine finalized before subsystem fan-out

 1. **Slice A — Reader kernel**
    - open/parse/repair
    - inspect/extract
    - baseline raster render
    - deterministic full rewrite
    - strict parse-own-output validation
@@
-**v1 release requirement:** Slice A + Slice B + Slice D.
+**v1 release requirement:** Slice F + Slice A + Slice B + Slice D.
+
+No feature bead may begin implementation until its Slice F dependencies are ratified.
```

## 2) Add a machine-readable scope registry and use it to eliminate spec contradictions

The spec is disciplined, but there are still a few contradictions that will become delivery hazards. Example: output encryption is marked post-baseline/non-gating in the classification/bead appendix, yet the proof doctrine’s encryption class still talks about proving that Monkeybee-generated encrypted output can be decrypted by references. Redaction is marked post-v1 unless separately proven, yet redaction safety still appears in the v1 test obligation matrix. Cross-reference-stream write is described in serialization rules even though baseline classification defers it. I would fix this with one authoritative `ScopeRegistry` checked by CI.

```diff
@@ ## Part 8 — Release gates for v1
+### Scope registry doctrine
+
+Every feature is assigned exactly one scope class:
+- `v1_gating`
+- `v1_supported_non_gating`
+- `v1_advisory`
+- `post_v1`
+- `experimental`
+
+The scope registry is machine-readable and CI-validated against:
+- release gates
+- proof doctrine test classes
+- bead appendix
+- generated capability docs
+
+No feature may be `v1_gating` in one section and `post_v1` in another.

@@ ### Pathological corpus / specific test case classes
-*Class: encryption*
+*Class: encryption-read*
 Test cases: V1/R2 ..., permission restrictions.
-Proves: decryption works for all standard security handler versions. Encryption of output files produces files that reference renderers can decrypt.
+Proves: decryption works for all standard security handler versions.
+
+*Class: encryption-write* [post-v1 unless explicitly promoted]
+Test cases: output files encrypted by Monkeybee can be opened by reference renderers.
+Proves: output-encryption interoperability.

@@ ### Test obligation matrix
-| encryption | parser | 100% of standard handlers (V1-V5) | Decrypt success |
+| encryption-read | parser | 100% of standard handlers (V1-V5) | Decrypt success |
@@
-| redaction-safety | edit | 0 recoverable redacted content | Extraction scan |
+| redaction-safety | edit | Non-gating in v1 unless B-EDIT-003 is separately promoted |
```

## 3) Insert a concrete `BytePatchPlan` between `WritePlan` and emitted bytes

This is the most important architecture change for preserve mode. Right now the spec has a strong `WritePlan` classification model and explicit incremental byte-range accounting, plus a formal preserve-mode proof target. But `WritePlan` is still object-class oriented. For signature-safe workflows, I would add a concrete byte-level patch artifact that says exactly which existing spans are preserved, which appended spans are created, and why. That turns preserve-mode safety from “the serializer should behave” into “the patch plan is inspectable and mechanically verifiable before write.” It also makes `plan-save` dramatically more useful.

````diff
@@ ### Save planning invariant
 Before any write, Monkeybee computes a `WritePlan` ...
@@
 `WritePlan` additionally records:
 - `edit_intent`
 - `ownership_transitions`
 - `blocked_preserve_regions`
 - `full_rewrite_reasons`
+
+After `WritePlan`, the writer must compile a concrete `BytePatchPlan`:
+
+```
+BytePatchPlan {
+  immutable_prefix_end: u64,
+  preserved_spans: Vec<ByteSpan>,
+  appended_segments: Vec<AppendedSegment>,
+  signed_range_audit: Vec<SignedRangeCheck>,
+  planned_startxref: u64,
+  patch_sha256: [u8; 32],
+}
+```
+
+`BytePatchPlan` is the last inspectable artifact before byte emission.
+Preserve-mode and signature-safe guarantees are made against `BytePatchPlan`, not only against
+object-level classifications.
+
@@ ### CLI output discipline
 - `monkeybee plan-save <file> [--incremental|--rewrite]` — preview ownership, rewritten regions,
   signature impact, and fallback reasons before saving; emits a schema-versioned `WritePlanReport`
+- `monkeybee plan-save <file> --patches` — emit `BytePatchPlan` with preserved ranges,
+  append ranges, and signed-range audit
````

## 4) Make tolerant recovery explicitly ambiguity-aware

The spec is right to prioritize ugly real-world PDFs, and it already records repairs. But it still assumes the tolerant parser can usually pick one repaired interpretation. That is fine when the result is clearly dominant. It is not fine when two recoveries are both plausible and materially different. The current experimental Bayesian repair scoring helps rank candidates, but that should not be the only protection. I would add a baseline ambiguity contract: when multiple materially different recoveries survive validation and no deterministic tiebreaker exists, Monkeybee must surface that ambiguity explicitly rather than silently committing to one meaning. That is especially important for forensic, signature, and extraction workloads.

```diff
@@ **Parse modes:**
 - **Tolerant mode:** Recovers from malformed real-world PDFs ...
+  **Ambiguity rule:** if multiple recovery strategies produce materially different semantic
+  outcomes (page count, object graph, text decode, signature coverage, or write impact) and no
+  deterministic tiebreaker exists, tolerant mode emits `parse.repair.ambiguous`.
+  By default, `engine.open()` returns the highest-confidence candidate plus the ambiguity record;
+  `ForensicPreserve` may instead reject ambiguous recovery unless
+  `allow_ambiguous_recovery=true`.

@@ pub struct CapabilityReport {
     pub expected_degradations: Vec<FeatureCode>,
+    pub recovery_confidence: RecoveryConfidence,
+    pub ambiguity_count: u32,
 }

@@ ### Compatibility ledger schema
+  ambiguities: [AmbiguityEntry],  // competing recovery candidates and why they differed
```

## 5) Replace ambient-host fallback with explicit resource-pack policy

The provider/oracle story is good, but the current default still allows non-CI runs to fall through to ambient system fonts and ICC profiles. That is practical, but it is also a reproducibility trap and a licensing trap, especially for CJK fallback. I would make resource provenance first-class: a pinned resource pack should be the default truth source, and ambient fallback should be a caller-selected policy, not a silent convenience. This improves determinism, makes cross-host bugs reproducible, and makes public claims cleaner.

```diff
@@ ### Provider trait contracts
-The oracle provides deterministic resource resolution for CI/proof runs. In non-CI mode, the
-oracle falls through to ambient system resources (fonts, ICC profiles). In CI/proof mode, the
-oracle uses pinned resource packs for reproducibility.
+The engine uses explicit resource-pack policy in all modes:
+- `PinnedOnly`
+- `PinnedThenAmbient`
+- `AmbientAllowed`
+
+Pinned resource packs include fallback fonts, Base 14 metrics, CJK fallbacks, standard CMaps,
+and ICC defaults with provenance + license metadata.

-Proof/CI mode must use pinned providers rather than ambient system discovery.
+Proof/CI mode must use `PinnedOnly`.
+`ViewerFast` may use `PinnedThenAmbient`.
+Every fallback resolution records provenance (`pack`, `ambient`, or `caller-supplied`) in the
+diagnostic stream and compatibility ledger.

@@ ### Operation profiles
 - `ViewerFast`
-  - parse=tolerant, write=deterministic, security=compatible, open=eager|lazy
+  - parse=tolerant, write=deterministic, security=compatible, open=eager|lazy,
+    provider_policy=pinned_then_ambient
 - `BatchProof`
-  - parse=tolerant, write=deterministic, security=strict_or_hardened, open=eager, determinism=on
+  - parse=tolerant, write=deterministic, security=strict_or_hardened, open=eager,
+    determinism=on, provider_policy=pinned_only
 - `BrowserWasm`
-  - parse=tolerant, write=deterministic, security=strict, open=in_memory_remote
+  - parse=tolerant, write=deterministic, security=strict, open=in_memory_remote,
+    provider_policy=pinned_only
```

## 6) Add an optional persistent derived-artifact store

The cache policy is solid for in-memory + spill behavior, but repeated openings of large, remote, or proof-corpus PDFs will still do too much repeated work. I would add an opt-in persistent artifact store keyed by document hash, engine build, provider manifest, and security profile. This should store only safe, validated derived artifacts such as repaired xref indices, parsed font/CMap results, page dependency graphs, and prefetch plans. This is a high-leverage performance feature because it accelerates both human workflows and proof runs without complicating the semantic model. It should be off by default for encrypted docs or privacy-sensitive contexts.

```diff
@@ ### Cache management doctrine
 `CachePolicy` defines:
 - in-memory byte budget
 - spill-store byte budget
+- optional persistent derived-artifact store policy
 - per-cache admission rules
@@
 **Scratch spill store:** bounded local store for oversized decoded streams, raster tiles,
   isolated-decoder outputs, and other large intermediate artifacts.
+
+**Persistent derived-artifact store:** optional disk-backed cache keyed by:
+`(input_sha256, engine_version, provider_manifest, security_profile, artifact_kind)`.
+Eligible artifacts:
+- repaired xref index
+- parsed object-stream index
+- parsed font / CMap / ICC metadata
+- page dependency graph
+- progressive prefetch plans
+
+Ineligible artifacts:
+- raw decrypted streams
+- caller-sensitive extracted text
+- artifacts derived from ambiguous recovery unless explicitly allowed
+
+The persistent store is disabled by default for encrypted inputs and for restricted corpus tiers.
```

## 7) Return structured operation reports, not just diagnostics

The plan has a strong diagnostics model and a strong compatibility ledger, and progressive rendering already talks about placeholders and missing-resource regions. But the public API examples still return just `rendered_page` or `text`. I would make every major operation return both the primary value and a structured operation report: degraded regions, substitutions used, missing resources, placeholder ranges, and budget/cancellation events. That makes the engine much more compelling for real viewers, editors, and debuggers, because they can explain what went wrong at the page or region level without waiting for session-close ledger aggregation.

```diff
@@ **API surface:**
-rendered_page = snapshot.render_page(page_index, render_opts, &exec_ctx)?;
-text = snapshot.extract_text(page_index, extract_opts, &exec_ctx)?;
+rendered = snapshot.render_page(page_index, render_opts, &exec_ctx)?;
+text = snapshot.extract_text(page_index, extract_opts, &exec_ctx)?;
@@
+pub struct RenderResult {
+    pub pixels: RasterSurface,
+    pub report: RenderReport,
+}
+
+pub struct RenderReport {
+    pub degraded_regions: Vec<RegionRef>,
+    pub placeholder_regions: Vec<PlaceholderRef>,
+    pub missing_resources: Vec<ResourceKey>,
+    pub substituted_fonts: Vec<FontSubstitution>,
+    pub budget_events: Vec<BudgetEvent>,
+}
+
+pub struct ExtractResult {
+    pub surface: PhysicalText | LogicalText | TaggedText,
+    pub report: ExtractReport,
+}
+
+pub struct ExtractReport {
+    pub unmappable_spans: Vec<TextGap>,
+    pub substituted_fonts: Vec<FontSubstitution>,
+    pub degraded_regions: Vec<RegionRef>,
+}
```

## 8) Qualify compatibility claims by target and security profile

The current baseline table says JBIG2 and JPEG 2000 decode are baseline, but the WASM constraints explicitly say those paths may be unavailable there and that WASM should use strict/pure-Rust behavior. The current form is technically true in some build profiles and misleading in others. I would add a support matrix that qualifies claims by target (`native`, `wasm`) and security profile (`Compatible`, `Hardened`, `Strict`). That makes public claims much more trustworthy and prevents “works on desktop, degrades in browser” from looking like a regression.

```diff
@@ ### Security profiles
+### Support-class doctrine
+
+Compatibility claims are qualified by support class:
+- `native-compatible`
+- `native-hardened`
+- `native-strict`
+- `wasm-strict`
+- `proof-canonical`
+
+Feature tables, ledgers, and generated capability docs must report support in this qualified form.

@@ ### Baseline v1 vs experimental feature classification
-| JBIG2 decode | Baseline (via isolated JBIG2-capable decoder path) | Common in scanned docs |
-| JPEG 2000 decode | Baseline (via openjpeg-sys) | Common in print-quality PDFs |
+| JBIG2 decode | Baseline on `native-compatible`/`native-hardened`; explicit degradation on `wasm-strict` unless a proven pure-Rust path exists | Target-qualified support |
+| JPEG 2000 decode | Baseline on `native-compatible`/`native-hardened`; explicit degradation on `wasm-strict` unless a proven pure-Rust path exists | Target-qualified support |
```

## 9) Generate the README/capability matrix from proof artifacts

The quality gate already says the README must reflect proven capabilities. I would go one step further and make that automatic. The compatibility ledger, oracle manifest, scope registry, and public corpus already give you the raw material. Use them to generate a capability matrix for the README/site/CLI, and fail CI when docs drift from proof. That turns the project’s strongest cultural value—evidence over rhetoric—into a mechanical property. It also makes the project more compelling externally because users can see exactly what is supported, where, and under which profile.

```diff
@@ ### Quality gates
-- [ ] README accurately reflects proven capabilities (no roadmap theater).
+- [ ] README and website capability tables are generated from proof artifacts + scope registry
+      (no manual capability claims).

@@ ### Proof beads
 - B-PROOF-010: Corpus-level compatibility aggregation and regression tracking
+- B-PROOF-011: Capability-matrix generation for README/site/CLI

@@ ### CLI beads
 - B-CLI-010: Conformance command
+- B-CLI-011: `capabilities` command (emit generated support matrix and proof provenance)
```

## 10) Make performance gates reproducible: benchmark profiles, not vague hardware prose

The performance doctrine is directionally excellent, but phrases like “modern desktop CPU” and absolute targets without cache-state/profile context will create endless argument. I would formalize benchmark profiles: hardware SKU, compiler flags, security profile, decoder policy, provider manifest, warm/cold cache state, corpus subset, and percentile thresholds (`p50`, `p95`). That makes the benchmarks reproducible and turns them into usable gates instead of aspirational numbers.

```diff
@@ ### Performance doctrine
-*Latency class (small simple PDFs):* ... Target: first-page render under 50ms at 150 DPI on a modern desktop CPU.
+*Latency profile `desktop-x86_64-cold`:* defined CPU SKU, OS image, compiler version,
+security profile, provider manifest, and cold-cache state. Gates use `p50` and `p95`.

-*Throughput class (large complex PDFs):* ... Target: sustained throughput of 10+ pages/second ...
+*Throughput profile `desktop-x86_64-warm`:* defined hardware + warm-cache state.
+Gates use sustained throughput and regression budget against previous canonical run.

-*Memory class:* Peak memory usage ...
+*Memory profile:* defined allocator, artifact-store policy, and corpus subset.
+Gates use peak RSS and peak decoded-bytes counters.

@@ ### CI integration contract
+Canonical benchmark runs record:
+- benchmark profile id
+- hardware / OS image
+- compiler version
+- security profile
+- provider manifest
+- warm/cold cache state
+- percentile outputs (`p50`, `p95`, worst-case`)
```

My two highest-priority changes are #1 and #3. #1 stops the project from fragmenting before the invariants are truly fixed. #3 makes the preserve/signature-safe story materially stronger by moving it from object-level intent to byte-level evidence. The rest mostly tighten truthfulness, reproducibility, and ergonomics around an already very good core design.
