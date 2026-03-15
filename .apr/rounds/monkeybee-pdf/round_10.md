I reviewed [SPEC.md](sandbox:/mnt/data/SPEC.md) . I did not receive substantive README text beyond the placeholder, so the review below is grounded in the plan itself.

Overall: this is an unusually strong plan. The thesis is clear, the closed loop is the right north star, the proof doctrine is serious, and the document already contains many of the right primitives: immutable snapshots, explicit parse/write modes, a compatibility ledger, incremental-save planning, progressive range-backed open, and a real proof harness. The biggest improvements are not “add more features.” They are: tighten the execution spine, remove a few architectural contradictions, make the save/ownership model even more explicit, and promote a couple of high-value user surfaces from implied to first-class.

## 1. Add a real delivery spine, not just baseline/experimental labels

### Why this makes it better

The plan already says baseline v1 should prefer simple, auditable defaults and that advanced paths must beat the baseline under the proof harness. But the spec still reads like constitution + architecture + roadmap + research agenda all at once. That creates execution risk: too many parallel fronts, too much room for “baseline” work to be displaced by interesting math, and too little clarity on what exactly must be true at release. The plan needs one compact shipping spine that sequences the closed loop into release slices. That makes the project more likely to ship, easier to staff, easier to communicate, and much easier to defend publicly. 

### Diff

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Part 0 — Constitutional basis
+### Delivery spine
+
+Monkeybee defines four release slices:
+
+1. **Slice A — Reader kernel**
+   - open/parse/repair
+   - inspect/extract
+   - baseline raster render
+   - deterministic full rewrite
+   - strict parse-own-output validation
+
+2. **Slice B — Bidirectional preserve loop**
+   - immutable snapshots + EditTransaction
+   - incremental append
+   - annotation add/save/reopen
+   - AcroForm read/fill/appearance regeneration
+   - signature-safe save planning
+
+3. **Slice C — Remote/progressive**
+   - range-backed ByteSource
+   - progressive page/region render
+   - prefetch planning + refinement
+
+4. **Slice D — External proof**
+   - pathological corpus
+   - compatibility ledger
+   - multi-oracle render comparison
+   - CI scorecards and regression gates
+
+**v1 release requirement:** Slice A + Slice B + Slice D.
+**v1 optional beta lane:** Slice C may ship behind a feature flag if it threatens v1 critical path.
+
+Every task in the roadmap must declare its owning slice.
```

---

## 2. Introduce `OperationProfile` presets to tame mode explosion

### Why this makes it better

Right now the plan has parse modes, write modes, security profiles, open strategies, determinism settings, provider registries, and execution budgets. That is powerful, but it is also a combinatorial footgun. Different parts of the engine can drift into subtly incompatible assumptions. A user-facing or API-facing preset layer would collapse the complexity into a handful of opinionated bundles. That improves usability, reduces invalid combinations, and gives the codebase a cleaner testing matrix. It also makes WASM, server, CLI, and forensic use cases much more concrete. 

### Diff

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Operational mode doctrine
+### Operation profiles
+
+Modes are low-level contracts. Most callers should start from an `OperationProfile` preset:
+
+- `ViewerFast`
+  - parse=tolerant, write=deterministic, security=compatible, open=eager|lazy
+- `ForensicPreserve`
+  - parse=preserve, write=incremental_append, security=hardened, open=eager
+- `EditorSafe`
+  - parse=tolerant, write=plan_selected, security=hardened, open=eager
+- `BatchProof`
+  - parse=tolerant, write=deterministic, security=strict_or_hardened, open=eager, determinism=on
+- `BrowserWasm`
+  - parse=tolerant, write=deterministic, security=strict, open=in_memory_remote
+
+`ExecutionContext::from_profile(profile)` materializes budgets, cache policy, provider policy,
+determinism, and default write/open behavior from the preset.
```

---

## 3. Create a small `monkeybee-paint` / appearance kernel and remove `annotate -> render`

### Why this makes it better

This is the cleanest architectural fix in the document.

The current plan says `monkeybee-annotate` depends on `monkeybee-render` primitives for appearance stream content realization, while the quality gates also say annotation/widget appearance generation must no longer depend on render. The bead appendix even explicitly notes a similar ownership duplication for the graphics state machine. That is a real seam problem. Appearance generation and rendering should share geometric/text/path/color primitives, but render should not be a dependency of annotate. Otherwise you get cycles, testing pain, and confused ownership. 

### Diff

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Crate boundaries
+#### `monkeybee-paint`
+
+Shared page-independent painting and appearance primitives:
+- path stroking/filling helpers
+- text run realization for appearance generation
+- color and ExtGState emission helpers
+- form XObject appearance composition primitives
+- paint-side geometry utilities reused by render/compose/annotate
+
+`monkeybee-paint` does not rasterize pages and does not own content-stream interpretation.
+It is the shared kernel for emitted appearance content.
+
 #### `monkeybee-annotate`
@@
-- Depends on `monkeybee-render` primitives for appearance stream content realization
-  (glyph positioning, path construction, color setting within form XObjects)
+- Depends on `monkeybee-paint` for appearance stream content realization
+  (glyph positioning, path construction, color setting within form XObjects)
@@
 #### `monkeybee-render`
@@
-- Consumption of `monkeybee-content` events or `PagePlan` IR through backend adapters
+- Consumption of `monkeybee-content` events or `PagePlan` IR through backend adapters
+- Reuse of `monkeybee-paint` primitives where page rendering and appearance composition overlap
```

And in the bead appendix:

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Render beads
-- B-RENDER-001: Graphics state machine
-Note: B-RENDER-001 duplicates B-CONTENT-002 (graphics state machine). The state machine is
-owned by monkeybee-content, not monkeybee-render. Remove B-RENDER-001 and ensure B-CONTENT-002
-covers the full graphics state contract from Part 5.
+B-PAINT-001: Shared appearance/painter primitives
+B-CONTENT-002 remains the sole owner of the graphics state machine.
```

---

## 4. Replace the two cache doctrines with one canonical `CachePolicy`

### Why this makes it better

The plan currently contains a strong engine-level cache doctrine with bounded budgets, but later it also includes a second caching section that describes overlapping caches differently, including a font cache that is effectively unbounded. That is exactly the kind of contradiction that becomes expensive later: performance work will be built on inconsistent assumptions, and WASM/native behavior will drift. Unifying cache semantics also opens the door to better cross-snapshot reuse and more predictable memory behavior. 

### Diff

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Cache management doctrine
-Engine-level caches are bounded by configurable memory budgets. The default budgets are:
+All caches are governed by a single `CachePolicy`.
+
+`CachePolicy` defines:
+- in-memory byte budget
+- spill-store byte budget
+- per-cache admission rules
+- pinning rules
+- eviction rules
+- deterministic mode behavior
+- wasm/native default profiles
+
+Canonical caches:
+- `ParsedObjectCache`      key=(document_id, revision_id, objref)
+- `DecodedStreamCache`     key=(resource_fingerprint, filter_chain_hash)
+- `ParsedFontCache`        key=(font_fingerprint)
+- `PagePlanCache`          key=(snapshot_id, page_index, dependency_fingerprint, profile_hash)
+- `RasterTileCache`        key=(snapshot_id, page_index, tile_id, dpi, completeness, profile_hash)
+- `ColorTransformCache`    key=(icc_fingerprint, intent, target_space)
+- `ResolvedResourceCache`  key=(snapshot_id, page_index, inheritance_fingerprint)
 
-- **Font cache:** 128 MB. Keyed by font dictionary object_id. Parsed font programs, glyph outlines,
-  and CMap tables. LRU eviction. Fonts referenced by the current page's resources are pinned.
+- No cache is unbounded in any runtime, including native and WASM.
+- WASM uses smaller default budgets, not different cache semantics.
+- Cross-snapshot reuse is allowed only for immutable artifacts identified by fingerprints.
```

And explicitly delete the conflicting later section:

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ WASM caching strategy
-- Fonts are never evicted until the document is closed.
+- Font cache follows the global `CachePolicy`; referenced fonts may be pinned temporarily, but not unbounded.
```

---

## 5. Add `OpenProbe`: a cheap preflight triage before full open

### Why this makes it better

The plan promises a `CapabilityReport`, remote/progressive open, security profiles, and save-impact explanation. What it lacks is a clear *early* probe stage that can cheaply answer “what kind of document is this, and what path should we take?” before expensive parsing or fetching. This would materially improve first-page latency, server safety, and user trust. It is also a natural place to surface risky decoders, signatures, encryption, XFA/JS, update-chain depth, and likely degradation before the engine commits to a heavy operation. 

### Diff

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Engine / session / snapshot model
+### Open probe contract
+
+Before full open, the engine may perform an `OpenProbe`:
+
+```
+probe = engine.probe(byte_source, probe_opts, &exec_ctx)?;
+```
+
+`OpenProbe` is bounded and cheap. It may inspect:
+- header and declared version
+- tail region (`startxref`, `%%EOF`, update depth estimate)
+- linearization dictionary and first-page hint presence
+- encryption dictionary presence
+- signature field presence and `/ByteRange` inventory
+- `/Catalog` feature hints (AcroForm, XFA, StructTreeRoot, OCGs, JavaScript)
+- likely risky decoder set
+- approximate page count / object count when cheaply knowable
+
+`OpenProbe` returns a preliminary `CapabilityReport`, an estimated complexity class, and a
+recommended `OperationProfile`.
+
+`engine.open(...)` may accept a prior probe result to avoid duplicate work.
````

---

## 6. Promote diffing into a first-class subsystem instead of a CLI promise

### Why this makes it better

Workflow 10 is compelling: compare two PDFs or snapshots and explain structural, textual, render, signature, and compatibility changes. But in the current plan, that capability mostly exists as a workflow statement and a CLI command. That is too thin for something this valuable. Making diff a first-class semantic artifact turns a cool feature into a durable product surface. It also feeds proof, save planning, regression reports, and human debugging. This is one of the biggest “more compelling/useful” upgrades you can make. 

### Diff

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Crate boundaries
+#### `monkeybee-diff`
+
+Semantic and multi-surface comparison between documents or snapshots.
+
+Key responsibilities:
+- structural object/page/resource deltas
+- text deltas (physical/logical/tagged surfaces)
+- render deltas (reusing proof metrics and region maps)
+- signature impact deltas
+- capability/compatibility deltas
+- save-plan deltas (`why would this become full rewrite?`)
+
+Canonical output:
+```
+DiffReport {
+  schema_version,
+  left_document_id,
+  right_document_id,
+  structural_delta,
+  text_delta,
+  render_delta,
+  signature_delta,
+  capability_delta,
+  write_impact_delta,
+  diagnostics,
+}
+```
@@ CLI
-- `monkeybee diff <before> <after> [--render|--text|--structure|--save-impact]`
+- `monkeybee diff <before> <after> [--render|--text|--structure|--save-impact]`
+  emits a schema-versioned `DiffReport`
````

---

## 7. Make ownership escalation explicit with `EditIntent`

### Why this makes it better

You already have `Owned`, `ForeignPreserved`, `OpaqueUnsupported`, a `WritePlan`, and transaction validation. That is excellent. The missing piece is *intent*. Right now the plan still allows “edits to `ForeignPreserved` objects transition them to `Owned` automatically with a diagnostic.” That is too implicit for a project that cares so much about preserve mode, signatures, and explainability. The engine should distinguish between “I want forensic preservation” and “I explicitly accept semantic rewrite/canonicalization.” That makes behavior safer, more predictable, and easier to explain to users. 

### Diff

````diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Mutation safety
+### Edit intent contract
+
+Every `EditTransaction` declares an `EditIntent`:
+
+```
+enum EditIntent {
+  ForensicPreserve,
+  SafeIncremental,
+  SemanticRewrite,
+  CanonicalizeOwned,
+  Optimize,
+}
+```
+
+`EditIntent` constrains ownership escalation and write planning.
+
@@ EditTransaction validation rules
-- Edits to `ForeignPreserved` objects transition them to `Owned` automatically with a diagnostic.
+- In `ForensicPreserve` and `SafeIncremental`, edits to `ForeignPreserved` objects do **not**
+  auto-escalate. The caller must explicitly call `take_ownership(objref, reason)`.
+- In `SemanticRewrite` and `CanonicalizeOwned`, ownership escalation is permitted and recorded.
+- Every ownership escalation produces an `OwnershipTransitionRecord` that is surfaced in the
+  `WritePlan` and compatibility ledger.
````

And extend the write plan:

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Save planning invariant
+`WritePlan` additionally records:
+- `edit_intent`
+- `ownership_transitions`
+- `blocked_preserve_regions`
+- `full_rewrite_reasons`
```

---

## 8. Turn optional/risky capabilities into explicit feature modules

### Why this makes it better

The current provider model is good for fonts, color profiles, crypto, and oracle resources. But the spec also contains several platform-sensitive or safety-sensitive capabilities: JPX, JBIG2, native bridges, PKI verification, XFA inspection, perhaps future OCR or JS preservation helpers. These are exactly the things that benefit from a harder extension boundary. That keeps the core cleaner, improves determinism, and makes security review easier. It also makes WASM/native split less ad hoc. 

### Diff

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Provider trait contracts
+### Feature module registry
+
+Beyond providers, the engine supports optional `FeatureModule`s for capabilities that are:
+- platform-specific
+- safety-sensitive
+- non-baseline
+- externally versioned
+
+Examples:
+- `jpx_native`
+- `jbig2_isolated`
+- `pki_verify`
+- `xfa_inspect`
+- `postscript_subset_translate`
+
+Each module declares:
+- capability codes
+- supported targets (native/wasm)
+- determinism class
+- safety class
+- version/hash for manifesting
+
+Canonical CI/proof runs record the active feature-module manifest alongside the oracle manifest.
```

---

## 9. Schema-version more than the compatibility ledger

### Why this makes it better

The spec already versions the compatibility ledger, which is exactly right. But the CLI and external-proof story is broader than that: `CapabilityReport`, `WritePlan`, `DiffReport`, trace streams, fixture expectations, and even JSON command envelopes are all external interfaces in practice. Leaving them implicit means downstream tooling will grow against unstable output. That would undercut the “externally legible proof” ambition. Version those surfaces explicitly now. 

### Diff

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Compatibility ledger JSON schema
 The schema is versioned.
+
+### External schema doctrine
+
+The following outputs are schema-versioned external interfaces:
+- `CompatibilityLedger`
+- `CapabilityReport`
+- `WritePlanReport`
+- `DiffReport`
+- `TraceEventStream`
+- CLI JSON envelope
+- `ExpectationManifest`
+- `OracleManifest`
+
+Backward compatibility is guaranteed within a major version for all of the above.
+Breaking changes require a schema major-version bump and fixture updates in CI.
```

And in CLI:

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ CLI output discipline
 {
+  "schema_version": "1.0",
   "status": "success" | "degraded" | "failed",
   "result": { /* command-specific */ },
   "diagnostics": [ /* array of Diagnostic objects */ ],
   "timing": { "wall_ms": 1234, "parse_ms": 500, "render_ms": 700 }
 }
```

---

## 10. Move experimental “alien” algorithms fully out of the baseline narrative

### Why this makes it better

This is not about reducing ambition. It is about making the ambition legible.

The spec already says advanced algorithms ship behind pluggable traits/flags and become default only after they beat the baseline. Good. But they are still threaded through baseline sections often enough that they can blur what is required versus what is aspirational. Moving them into a single annex with explicit baseline replacements would make the whole document calmer, clearer, and more credible. Readers would see that the engine has a serious baseline and a research lane, rather than one giant superposition of both. 

### Diff

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Part 7 / experimental sections
- Experimental algorithm details are interleaved with baseline subsystem contracts.
+ Experimental algorithm details live in a dedicated annex.
+
+### Experimental annex rule
+
+No baseline subsystem contract may require an experimental algorithm for correctness.
+Each experimental item must declare:
+- baseline implementation it competes with
+- proof metric it must beat
+- cost metric it must beat
+- fallback behavior when disabled
+
+Experimental items:
+- exact analytic area coverage rasterizer
+- robust geometric predicates
+- spectral-aware color pipeline
+- algebraic blend optimization
+- SDF glyph path
+- adaptive mesh subdivision
+- Bayesian repair scoring
+- MS-SSIM enhancements beyond baseline comparison
+- entropy-optimal write encoding
+- probabilistic layout analysis
```

---

## Surgical cleanups I would also make immediately

These are smaller than the ten revisions above, but I would fix them in the same pass:

1. **Resolve the cache contradiction.** The spec has both a bounded font cache and a later “no size limit in practice” font cache. Pick one policy. It should be bounded everywhere.

2. **Resolve the annotation/render contradiction.** The plan currently both depends on render for annotation appearance generation and says that dependency must disappear. Make the new `monkeybee-paint` split authoritative.

3. **Make PagePlan cache keys consistent.** In one place PagePlan is keyed by `(snapshot_id, page_index)`; elsewhere a dependency fingerprint appears. The latter is the better design. Use it consistently.

4. **Settle xref-stream write status.** Some wording implies broad support, but the baseline table says xref-stream write is post-baseline. Pick one. I would keep: read now, write later unless forced by compact mode.

5. **Settle output encryption status.** The plan discusses encryption in generated output, but the baseline table treats write-side encryption as post-baseline. I would keep output encryption out of v1 gating.

6. **Make `CapabilityReport` an early open artifact, not just a workflow promise.**

7. **Promote tagged-structure preservation to a gated sub-feature of preserve mode** for any edit that claims not to damage existing semantic structure.

8. **Give `monkeybee plan-save` a stable machine schema.** That command is too important to remain informal.

---

## My recommended priority order

1. Delivery spine
2. `monkeybee-paint` split
3. Unified `CachePolicy`
4. `EditIntent` + explicit ownership escalation
5. `OpenProbe`
6. First-class diff subsystem
7. External schema doctrine
8. Feature module registry
9. Annex experimental math
10. Small consistency cleanup pass

That would make the project feel less like an enormous brilliant document and more like a ruthless, shippable architecture.

The core thesis should stay exactly as-is. The plan does **not** need less ambition. It needs tighter seams, fewer contradictions, and a more explicit path from manifesto to release.
