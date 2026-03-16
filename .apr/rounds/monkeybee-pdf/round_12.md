I read the README, Implementation Master, and SPEC together. The core thesis is strong and I would keep it: one engine, one document model, immutable snapshots, a shared content interpreter, and proof-backed release gates. The best revisions are mostly about tightening boundaries and removing a few places where the plan currently says two different things at once.   

I would not change the four-strata model, the snapshot-based lifecycle, or the proof-first doctrine. I would change the following.

## 1) Make `monkeybee-paint` a real crate, or delete it everywhere

This is the clearest architectural contradiction in the current plan. The SPEC treats `monkeybee-paint` as a real boundary, a quality gate, and a bead. But the README crate map and the implementation topology do not include it, and they currently place appearance generation inside `monkeybee-compose`. That will cause boundary drift almost immediately: render, compose, annotate, and forms will all end up wanting the same paint-side helpers, but without a shared crate they will either duplicate code or smuggle rendering concepts into authoring. The better architecture is to make `monkeybee-paint` explicit and very small: page-independent paint primitives, no rasterization, no content interpretation.   

```diff
--- a/README.md
+++ b/README.md
@@ Architecture at a glance
+| `monkeybee-paint` | Shared page-independent paint and appearance primitives reused by render, compose, forms, and annotate |

@@ Repo structure
+│   ├── monkeybee-paint/

--- a/implementation_master.md
+++ b/implementation_master.md
@@ Workspace topology
+│   ├── monkeybee-paint/         # shared paint/appearance primitives (non-raster, page-independent)

@@ Crate dependency graph
+monkeybee-paint       (depends on: core, text)
-monkeybee-render      (depends on: core, content, document, text, codec)
-monkeybee-compose     (depends on: core, document, text, content)
-monkeybee-annotate    (depends on: core, document, content, compose, forms)
+monkeybee-render      (depends on: core, content, document, text, codec, paint)
+monkeybee-compose     (depends on: core, document, text, content, paint)
+monkeybee-forms       (depends on: core, document, text, compose, paint)
+monkeybee-annotate    (depends on: core, document, content, compose, forms, paint)
```

## 2) Keep `monkeybee-write` pure; move authoring beads fully into `compose`/`text`

The documents already say `monkeybee-compose` owns authoring/builders and `monkeybee-write` is a pure serializer. But the bead appendix leaks authoring responsibilities back into write with “content stream generation” and “font embedding and subsetting for generated content.” That is the wrong boundary. Once write owns content generation, it stops being a serializer and starts becoming a second authoring stack. The cleaner split is: `compose` builds content and resource plans, `text` materializes subsets/ToUnicode, `write` serializes the already-formed semantic graph.   

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Write beads
-- B-WRITE-006: Content stream generation
-- B-WRITE-009: Font embedding and subsetting for generated content

@@ Compose beads
 - B-COMPOSE-006: Content stream emission from high-level drawing/text operations
+- B-COMPOSE-007: Generated content stream assembly for pages, appearances, and flattening
+- B-COMPOSE-008: Resource closure handoff for serialization

@@ Text beads
 - B-TEXT-005: Subsetting and ToUnicode generation for emitted PDFs
+- B-TEXT-007: Final subset materialization for composed output

@@ monkeybee-write contract
- `monkeybee-write` serializes a semantically complete document.
+ `monkeybee-write` serializes a semantically complete document and never owns high-level content authoring.
```

## 3) Make the scope registry the single source of truth, not just a doctrine

The scope-registry idea is one of the best parts of the SPEC, but it is still too rhetorical. It should become a real machine-readable artifact that the README, CLI, proof harness, and bead appendix all validate against. That eliminates one of the biggest risks in a project this ambitious: promising something as v1 in one place and quietly downgrading it somewhere else. This is especially important because the spec already has several borderline scope tensions. 

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Scope registry doctrine
-The scope registry is machine-readable and CI-validated against:
+The canonical scope registry lives at `docs/scope_registry.yaml`.
+It is machine-readable and CI-validated against:
 - release gates
 - proof doctrine test classes
 - bead appendix
 - generated capability docs
+- README capability tables
+- CLI `capabilities --json`
+- workspace feature flags

+Each registry entry includes:
+- `feature_id`
+- `scope_class`
+- `support_classes`
+- `owning_crate`
+- `proof_class`
+- `schema_surfaces`
+- `bead_ids`
+- `notes`

--- a/README.md
+++ b/README.md
@@ Repo structure
+├── docs/
+│   ├── scope_registry.yaml
```

## 4) Promote `OpenProbe`, `CapabilityReport`, and `DiffReport` to the public surface everywhere

The SPEC makes `OpenProbe` and an early `CapabilityReport` central to the product story: edit safety, expected degradation, risky decoders, signatures, remote/progressive workflows, and revision diffs all depend on that early preflight view. But that importance is not reflected strongly enough in the README and implementation-facing topology. I would make probe/report surfaces first-class in the stable facade and CLI. That makes the engine more compelling operationally because users can ask “what can I safely do with this file?” before doing expensive work.  

```diff
--- a/README.md
+++ b/README.md
@@ Architecture at a glance
-| `monkeybee` | Stable public facade: semver-governed `Engine`, `Session`, `Snapshot`, `EditTransaction`, `WritePlan`, `CapabilityReport`, and high-level open/render/extract/edit/save APIs |
+| `monkeybee` | Stable public facade: semver-governed `Engine`, `OpenProbe`, `Session`, `Snapshot`, `EditTransaction`, `WritePlan`, `CapabilityReport`, `DiffReport`, and high-level open/render/extract/edit/save APIs |

--- a/implementation_master.md
+++ b/implementation_master.md
@@ Workspace topology / public facade notes
+`monkeybee` public modules:
+- `probe.rs`      # bounded pre-open inspection and complexity classification
+- `report.rs`     # CapabilityReport, WritePlanReport, DiffReport
+- `session.rs`    # Engine / Session / Snapshot facade

--- a/SPEC.md
+++ b/SPEC.md
@@ Open probe contract
-`OpenProbe` is bounded and cheap.
+`OpenProbe` is bounded, cheap, and the default preflight for viewer/editor/CLI flows.
+`engine.open()` SHOULD accept a prior probe to avoid duplicate work.
```

## 5) Move research-heavy algorithm prescriptions out of the mainline spec

The project wants a simple, auditable baseline and explicitly says experimental paths should not be required for correctness. But large parts of the SPEC still read like a research manifesto inside the mainline implementation contract: exact analytic coverage, robust predicates, Bayesian repair scoring, probabilistic layout analysis, entropy-optimal packing, spectral color, SDF text. Those are interesting, but they materially raise spec-to-code impedance. The mainline spec should describe behavior, invariants, and proof thresholds; a separate annex should describe advanced candidate algorithms. That makes the plan more implementable without reducing ambition. 

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Part 0 / Alien artifact doctrine
-In concrete terms, "alien artifact" for this domain means:
-1. Mathematical sophistication threaded through every hot path.
-2. Correctness that is provable, not merely tested.
-3. Performance that comes from algorithmic insight, not just Rust's speed.
-4. An architecture that feels internally inevitable once understood.
+In concrete terms, "alien artifact" means unusual coherence, breadth, and evidence.
+The baseline contract specifies required behavior and proof thresholds.
+Advanced algorithm candidates live in `docs/architecture/EXPERIMENTAL_ANNEX.md`.

@@ Experimental annex rule
+All research-heavy algorithm descriptions are informational unless a scope-registry entry marks them `v1_gating`.
+Mainline subsystem contracts must be satisfiable by the auditable baseline implementation.
```

## 6) Resolve the output-encryption contradiction

Right now the plan says two different things: the encryption section says output encryption should default to AES-256, while the scope table says encryption-write is post-baseline and out of v1 gating. That is exactly the kind of contradiction the scope registry is supposed to prevent. My revision is to make write encryption explicitly feature-flagged and non-default until promoted by proof. That preserves the long-term goal without overclaiming v1.  

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Encryption and security handler recovery
-For output encryption, default to AES-256 (V5, R6) as the strongest standard option.
+Output encryption is a post-baseline, non-gating feature.
+It is disabled in the baseline v1 build.
+When the optional `write-encryption` feature is enabled and promoted, the default SHOULD be AES-256 (V5, R6).

--- a/implementation_master.md
+++ b/implementation_master.md
@@ Feature flag strategy
+| `write-encryption` | monkeybee-write | Enable output encryption (default: off; non-gating) |

@@ Baseline v1 builds with:
-Baseline v1 builds with: `tiny-skia`, `lcms2`, `openjpeg` (Compatible profile).
+Baseline v1 builds with: `tiny-skia`, `lcms2`, `openjpeg` (Compatible profile), and without `write-encryption`.
```

## 7) Add an explicit crash-safe save contract

For a bidirectional PDF engine, “serialize bytes” is not enough. The user-facing save path needs crash safety. The spec is strong on self-consistency and signature-safe append, but it does not yet make atomic file replacement a first-class contract. It should. A real editor/save path should stage output, fsync it, validate it, then atomically replace the destination. That materially improves robustness and makes the closed loop safer in practice. 

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Serialization contract
+### Save commit contract
+
+For file-backed saves, Monkeybee uses staged commit semantics:
+1. Serialize to a temp file in the target directory.
+2. `fsync` the temp file.
+3. Re-open and validate according to the requested save policy.
+4. `fsync` the parent directory when supported.
+5. Atomically rename the temp file over the destination.
+6. On failure, preserve the original destination unchanged.
+
+Library APIs expose:
+- `save_to_bytes()`
+- `save_atomic(path, SaveCommitOptions)`
+- `save_atomic_with_backup(path, SaveCommitOptions)`

+`monkeybee-write` remains a serializer; staged commit is owned by the public facade / CLI adapter.
```

## 8) Align the cache contract with the implementation, especially for render hot paths

The spec’s cache story is better than the implementation document’s current `CacheManager`. The SPEC talks about cross-snapshot reuse by immutable fingerprints, progressive tile completeness, render-profile-sensitive tile keys, and even a glyph rasterization cache. The implementation snippet still has simpler keys and omits some of those caches entirely. I would align the implementation plan upward, because render performance and progressive correctness depend on it.  

```diff
--- a/implementation_master.md
+++ b/implementation_master.md
@@ CacheManager
- pub decoded_streams: DashMap<(SnapshotId, ObjRef, u64), Arc<[u8]>>,
+ pub decoded_streams_local: DashMap<(SnapshotId, ObjRef, u64), Arc<[u8]>>,
+ pub decoded_streams_shared: DashMap<(ResourceFingerprint, u64), Arc<[u8]>>,
  pub shared_font_programs: DashMap<ResourceFingerprint, Arc<ParsedFontProgram>>,
  pub shared_icc_profiles: DashMap<ResourceFingerprint, Arc<ParsedIccProfile>>,
  pub shared_cmaps: DashMap<ResourceFingerprint, Arc<ParsedCMap>>,
+ pub glyph_bitmaps: DashMap<(ResourceFingerprint, GlyphId, QuantizedSize, QuantizedSubpixel), Arc<GlyphBitmap>>,
  pub page_plans: DashMap<(SnapshotId, usize), Arc<PagePlan>>,
- pub raster_tiles: DashMap<(SnapshotId, usize, TileId, u32), Arc<TileData>>,
+ pub raster_tiles: DashMap<(SnapshotId, usize, TileId, u32, TileCompleteness, RenderProfileHash), Arc<TileData>>,
+ pub color_transforms: DashMap<(ResourceFingerprint, RenderingIntent, TargetSpace), Arc<ColorTransform>>

@@ Cache statistics
+pub struct RenderProfileHash(pub u64);
+pub enum TileCompleteness { Partial, Complete }
```

## 9) Surface tagged-structure risk explicitly in `CapabilityReport` and `WritePlan`

The spec already has serious structure-tree/MCID preservation rules, but they are buried. That is risky because edits on tagged PDFs can silently break structure even when rendering still looks correct. I would not make full structure repair v1-gating, but I would make structure impact explicit at open-time and save-time so the engine can say “this edit is visually safe but may degrade tagging.” That makes the project more honest and more operationally useful. 

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ CapabilityReport
 pub struct CapabilityReport {
   pub signed: bool,
   pub encrypted: bool,
   pub tagged: bool,
+  pub structure_complexity: Option<StructureComplexity>,
+  pub structure_edit_risk: Option<StructureEditRisk>,
   pub has_xfa: bool,
   pub has_javascript: bool,
   pub risky_decoder_set: Vec<DecoderType>,
   pub edit_safety: EditSafetyClass,
@@ WritePlan additional records
 - `edit_intent`
 - `ownership_transitions`
 - `blocked_preserve_regions`
 - `full_rewrite_reasons`
+- `structure_impact`
+- `accessibility_impact`

@@ Scope registry doctrine
+`tagged_structure_preservation` MUST have an explicit scope class.
+Recommended initial classification: `v1_advisory`.
```

## 10) Make corpus expectation manifests and triage status part of the implementation plan, not just proof prose

The proof doctrine is strong, but the implementation document should reflect the existence of per-fixture expectation manifests because those are what make the proof harness operational instead of rhetorical. They are also the missing connective tissue between the compatibility ledger, scope registry, CI regression rules, and known-bad/approved triage.  

```diff
--- a/README.md
+++ b/README.md
@@ tests/
 │   ├── corpus/
+│   │   ├── public/**/expectation.yaml
+│   │   ├── restricted/**/expectation.yaml
+│   │   ├── generated/**/expectation.yaml
+│   │   └── minimized/**/expectation.yaml

--- a/implementation_master.md
+++ b/implementation_master.md
@@ monkeybee-proof
+- Corpus manifest tests: every fixture has an `ExpectationManifest`.
+- Regression tests: unknown degradations or scope-class violations fail unless triaged.
+- Triage fields: `approved`, `pending`, `known_bad`, `waived_until`, `owner`, `notes`.

@@ Subordinate implementation docs
+- `docs/implementation/proof-manifests.md` — expectation manifest schema, triage workflow, CI semantics
```

## My priority order

If you only adopt four of these, I would do them in this order:

1. Add `monkeybee-paint` as a real crate.
2. Keep `monkeybee-write` pure and move authoring beads out of it.
3. Make the scope registry canonical and generated.
4. Add the crash-safe save contract.

Those four changes would eliminate the biggest architecture ambiguities while making the plan more implementable, more honest about scope, and safer in real use.   
