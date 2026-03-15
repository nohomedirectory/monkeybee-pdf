I reviewed the [README](sandbox:/mnt/data/README.md) and [SPEC](sandbox:/mnt/data/SPEC.md). The core thesis is unusually strong: the closed loop, explicit Tier 1/2/3 compatibility doctrine, evidence-first release gates, preserve mode, and the shared content interpreter are all exactly the right center of gravity. The main weakness is not lack of ambition. It is that the plan still lets too many hard things become “baseline by implication,” and a few cross-cutting subsystems are not isolated enough yet.  

These are the revisions I would make.

## 1) Split v1 into an explicit baseline lane and an experimental lane, and make the default writer simple

This is the highest-leverage change.

Your README already says advanced backends are welcome but not v1 blockers. The spec, though, still makes a lot of ambitious behavior read like baseline scope: compact object-stream output by default, xref streams by default, broad profile-emission claims, heavy advanced algorithms, WASM demo pressure, and very wide rendering/conformance ambitions. That creates schedule risk and, more importantly, architectural risk: people start designing for the hardest tail cases before the baseline closed loop is incontrovertibly real.  

I would make “baseline v1” painfully explicit:

* Baseline v1 proves the closed loop on ugly PDFs with the simplest auditable implementations.
* Experimental backends are optional and benchmarked head-to-head.
* Compact output modes, rare shading/pattern paths, advanced perceptual diffing, formal proofs beyond core parser safety, and profile-constrained emission stay non-gating until the baseline wins under proof.

The most concrete consequence: **the deterministic writer should default to plain indirect objects plus classic xref tables**, not object streams + xref streams. That path is easier to diff, fuzz, recover, debug, and explain. Compact modes can come later.

```diff
diff --git a/README.md b/README.md
@@
-Monkeybee v1 is not a demo or a roadmap. It must ship with automated, repeatable evidence of the closed loop on ugly documents using a correct baseline engine. Advanced algorithmic backends are welcome, but they are not v1 blockers until they outperform the baseline under proof.
+Monkeybee v1 is not a demo or a roadmap. It must ship with automated, repeatable evidence of the closed loop on ugly documents using a correct baseline engine.
+
+v1 has two explicit lanes:
+- **Baseline v1 (release-gating):** the smallest coherent engine that proves the closed loop on ugly documents with simple, auditable implementations.
+- **Experimental backends (non-gating):** advanced algorithms, compact encoders, and specialized optimizations that remain optional until they beat the baseline under proof.

diff --git a/SPEC.md b/SPEC.md
@@
-3. An experimental path becomes default only after it beats the baseline on correctness or cost under the proof harness.
+3. Baseline v1 must prefer simple, auditable defaults over compact or exotic defaults.
+4. An experimental path becomes default only after it beats the baseline on correctness or cost under the proof harness.

@@ Serialization contract
-**Object stream packing:** For PDF 1.5+ output, small non-stream objects ... should pack objects into object streams by default for full-rewrite mode.
+**Object stream packing:** Compact rewrite mode may pack objects into object streams once it is independently proven under the proof harness. The baseline deterministic writer should emit plain indirect objects by default because that path is simpler to audit, diff, fuzz, and recover from.

-**Cross-reference stream output:** For PDF 1.5+ output, prefer cross-reference streams over cross-reference tables.
+**Cross-reference stream output:** Support both cross-reference tables and cross-reference streams. The baseline deterministic writer prefers classic cross-reference tables; compact mode may prefer cross-reference streams after proof stability is established.

@@ Part 8 — Release gates for v1
+- [ ] The baseline writer passes all write/round-trip gates with plain indirect objects and classic cross-reference tables.
+- [ ] Profile validation is v1-gating; profile-constrained emission is non-gating until baseline rewrite and incremental-append paths are proven stable.
```

## 2) Add a dedicated codec/security boundary instead of letting risky decode work bleed into parser/render

Right now the spec talks correctly about security profiles, risky decoders, and isolation, but the architecture still reads as if parser/render own too much of that surface directly. That keeps the trusted computing base larger than it needs to be and makes fuzzing, hardening, and WASM portability harder than necessary.

I would introduce two explicit subsystems:

* `monkeybee-codec`: stream filters, image decode/encode adapters, predictor logic, bounded byte transformations.
* `monkeybee-security`: security profiles, budget broker, worker isolation, hostile-input policy.

The parser should remain a structural machine. It can request codec services, but it should not directly link risky native decoders on hot paths. This makes security policy real, not rhetorical.

```diff
diff --git a/README.md b/README.md
@@ Architecture at a glance
 | `monkeybee-core` | Shared primitives: object IDs, geometry, errors, diagnostics, execution budgets |
 | `monkeybee-bytes` | Byte sources, mmap/in-memory/range-backed access, revision chain, raw span ownership |
 | `monkeybee-parser` | PDF syntax parsing, repair, tolerant/strict modes, raw token/span retention |
+| `monkeybee-codec` | Filter chains, image decode/encode, predictor logic, bounded decode pipelines |
+| `monkeybee-security` | Security profiles, worker isolation, budget broker, hostile-input policy |
 | `monkeybee-document` | Semantic document graph, page tree, inherited state, resource resolution, ownership classes |

diff --git a/SPEC.md b/SPEC.md
@@ Crate boundaries
+#### `monkeybee-codec`
+
+Bounded byte transformations shared across parse, render, extract, and write:
+- Stream filters and predictors
+- Image decode/encode adapters
+- Native/isolated decoder shims
+- Decode telemetry for proof and diagnostics
+
+#### `monkeybee-security`
+
+Execution-safety policy and enforcement:
+- Security profiles
+- Budget broker
+- Worker isolation / kill-on-overrun
+- Risky-decoder allow/deny policy
+
 #### `monkeybee-parser`
@@
-- Stream decompression and filter chains (FlateDecode, LZWDecode, ASCII85Decode, ...)
-- Encryption/decryption support (standard security handlers)
+- Structural parse and repair only
+- Delegation to `monkeybee-codec` for filter-chain decode/encode work
+- Delegation to `monkeybee-security` for risky-decoder policy and budget enforcement

@@ Security profiles
-High-risk domains include JBIG2Decode, JPXDecode, native font bridges, XFA XML packet handling, and Type 4 calculator functions.
+High-risk domains include JBIG2Decode, JPXDecode, native font/image bridges, XFA XML packet handling, and Type 4 calculator functions.
+All high-risk decode jobs execute through `monkeybee-security` with explicit memory/time budgets and optional worker isolation; no crate outside `monkeybee-codec` may invoke them directly.
```

## 3) Pull all text/font/shaping logic into one shared text subsystem

The current plan gets the content interpreter right, but text is still too fragmented conceptually. Font parsing, Unicode mapping, CMaps, shaping, fallback, extraction, appearance regeneration, and write-path subsetting are spread across render/extract/write/annotate contracts. That is a classic source of drift.

I would add a `monkeybee-text` crate and make it the **single source of truth for text**:

* font program parsing and caching
* CMap/ToUnicode resolution
* Unicode mapping fallback chain
* shaping/bidi/fallback for generation and FreeText
* subsetting for write path
* text search / hit-test primitives

That makes text behave the same in render, extract, generate, annotate, and inspect.

```diff
diff --git a/README.md b/README.md
@@ Architecture at a glance
+| `monkeybee-text` | Font programs, CMaps, Unicode mapping, shaping/bidi, subsetting, search/hit-test primitives |
-| `monkeybee-render` | Page rendering: content streams, text, fonts, images, transparency, vector graphics, masks, blending |
+| `monkeybee-render` | Page rendering: content streams, positioned glyphs, images, transparency, vector graphics, masks, blending |

diff --git a/SPEC.md b/SPEC.md
@@ Crate boundaries
+#### `monkeybee-text`
+
+Shared text subsystem used by render, extract, write, annotate, and inspect:
+- Font program parsing and caching
+- CMap / ToUnicode handling
+- Unicode fallback chain
+- Shaping, bidi, and font fallback
+- Subsetting and ToUnicode generation for emitted PDFs
+- Search, hit-testing, and selection primitives

 #### `monkeybee-render`
@@
-- Text rendering: font selection, encoding resolution, glyph positioning, kerning, text state
-- Font handling: Type 1, TrueType, OpenType/CFF, CIDFont, Type 3, font subsetting, ToUnicode mapping
+- Text rendering via `monkeybee-text`: font selection, shaping, bidi, fallback, glyph positioning, and Unicode-aware diagnostics

 #### `monkeybee-write`
@@
-- Font embedding and subsetting for generated content
+- Font embedding and subsetting delegated through `monkeybee-text` so render/extract/write use one font truth

 #### `monkeybee-extract`
@@
-- Text extraction with character positions, font information, and reading order heuristics
+- Text extraction surfaces built on `monkeybee-text`, including exact glyph geometry, logical reading order, and search primitives
```

## 4) Introduce an explicit Engine → Session → Snapshot → Transaction model

The spec already has `ExecutionContext`, change tracking, preserve mode, caches, and `EditTransaction`. What it lacks is a fully explicit **lifecycle model** for immutable read concurrency and versioned derived artifacts.

I would formalize:

* `MonkeybeeEngine`: providers, caches, worker pools, policy defaults
* `OpenSession`: byte source + revision chain + telemetry context
* `PdfSnapshot`: immutable parsed/resolved state, shareable across threads
* `EditTransaction`: snapshot in, new snapshot + delta out

This matters because your plan depends on:

* page-parallel render/extract
* stable PagePlan caches
* preserve/incremental save correctness
* reproducible proof artifacts
* future viewer/editor use

That all gets easier when nothing mutates in place.

```diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 3 — System architecture
+### Engine / session / snapshot model
+
+- `MonkeybeeEngine` owns global policy: providers, caches, worker pools, oracle manifests, and security defaults.
+- `OpenSession` binds a byte source and revision chain to that engine.
+- `PdfSnapshot` is immutable, shareable across threads, and identified by `snapshot_id`.
+- `EditTransaction` consumes a snapshot and produces a new snapshot plus a serializable delta.
+
+All caches, proofs, and invalidation logic key off `snapshot_id`, never mutable in-place document state.

@@ Mutation safety
-Mutations occur inside an `EditTransaction`.
+Mutations occur inside an `EditTransaction` against an immutable `PdfSnapshot`; commit produces a new snapshot, never a partially mutated live document.

@@ Caching strategy
-1. **Parsed object cache:** After an object is parsed from bytes, its structured representation is cached keyed on (object_number, generation_number).
+1. **Parsed object cache:** After an object is parsed from bytes, its structured representation is cached keyed on (snapshot_id, object_number, generation_number).
```

## 5) Add a first-class dependency graph and exact invalidation model

You already have a reference integrity index and some cache invalidation notes, but the plan still under-specifies one of the hardest practical problems: **how derived artifacts stay correct after edits**.

A PDF engine like this needs an explicit dependency graph that tracks:

* page → inherited resources
* page → content streams
* content streams → fonts/images/forms/patterns/extgstates/colorspaces
* annotations/widgets → appearance streams/resources
* derived artifacts → source objects and inherited state

Without that, you either get stale caches or gross over-invalidation. Both are painful.

```diff
diff --git a/SPEC.md b/SPEC.md
@@ `monkeybee-document`
 - Reference integrity index (forward and reverse lookups)
+ - Dependency graph (page -> resources -> forms/patterns/xobjects/fonts/images/annotations)
+ - Derived-artifact invalidation (PagePlan, resolved resources, decoded streams, widget appearances)

@@ Part 4 — Shared invariants
+### Dependency and invalidation invariant
+
+Every cached or derived artifact must be traceable to the objects and inherited state that produced it.
+Invalidation is exact and versioned by snapshot:
+- edit an object -> invalidate only dependents reachable in the dependency graph
+- commit a new snapshot -> preserve cache entries for untouched subgraphs
+- traces report why each cache entry was reused or invalidated

@@ Caching strategy
-7. **PagePlan cache:** Normalized per-page IR keyed on (revision_id, page_index, content_hash).
+7. **PagePlan cache:** Normalized per-page IR keyed on (snapshot_id, page_index, dependency_fingerprint).
```

## 6) Make progressive, lazy, and remote PDFs first-class instead of “future range-backed”

The bytes layer already hints at range-backed access, but the rest of the architecture still reads mostly like a local-file engine. That leaves a lot of performance and usefulness on the table.

I would explicitly support three open strategies:

* `eager` for local/CLI/proof work
* `lazy` for huge local documents
* `remote` for range-backed/networked documents

Then wire that into:

* first-page latency
* region render
* thumbnail render
* prefetch planning
* linearization-aware fast paths when present

This makes the engine much more compelling for browser/server/document-service use, and it aligns naturally with your WASM ambitions.

```diff
diff --git a/SPEC.md b/SPEC.md
@@ Part 1 — User workflows and visible proof
+### Workflow 8: Open huge or remote PDFs progressively
+
+A user opens a very large or range-backed PDF, renders the first page or a region quickly, and lets the engine fetch additional bytes lazily as needed. Linearization is used when present, but not required.
+
+Proof surfaces: first-page latency benchmarks, prefetch-plan traces, partial-open regression tests, byte-range accounting.

@@ `monkeybee-bytes`
-- Byte sources, mmap/in-memory/range-backed access, revision chain, raw span ownership
+- Byte sources, mmap/in-memory/range-backed access, fetch scheduler, prefetch planning, revision chain, raw span ownership

@@ Part 7 — Performance doctrine
+### Open strategies
+- `eager`: parse everything available locally
+- `lazy`: resolve objects on demand from a local byte source
+- `remote`: use range requests and a prefetch planner for first-page / region-first latency

@@ `monkeybee-render`
-- Output targets: raster (PNG/JPEG), vector (SVG), and extensible backend interface
+- Output targets: raster (PNG/JPEG), vector (SVG), region render, thumbnail render, and extensible backend interface
```

## 7) Give AcroForm its own subsystem instead of treating it as annotation spillover

This is a strong plan for annotations, but forms are big enough, common enough, and semantically different enough that they should not live as a sidecar inside `monkeybee-annotate`.

Widgets are annotations visually, but AcroForm adds:

* field tree semantics
* inherited field properties
* appearance regeneration rules
* value synchronization between field and widget
* signature field handling
* calculation order and export/import semantics

That deserves its own crate and workflow. This would make the project materially more useful on real-world PDFs, especially form-heavy and signed documents.  

```diff
diff --git a/README.md b/README.md
@@ Architecture at a glance
+| `monkeybee-forms` | AcroForm field tree, value model, appearance regeneration, widget/signature bridge |
-| `monkeybee-annotate` | Annotation creation, modification, flattening, geometry-aware placement |
+| `monkeybee-annotate` | Non-form annotations: creation, modification, flattening, geometry-aware placement |

diff --git a/SPEC.md b/SPEC.md
@@ Part 1 — User workflows and visible proof
+### Workflow 9: Fill, regenerate, and preserve forms
+
+A user loads an AcroForm-heavy PDF, reads or updates field values, regenerates widget appearances, saves, and reopens without breaking the field tree or signed byte ranges outside the edited scope.
+
+Proof surfaces: field round trips, appearance regeneration tests, signature-preserving form fills.

@@ Crate boundaries
+#### `monkeybee-forms`
+
+AcroForm field tree, value model, appearance regeneration, calculation order, widget bridge, and signature-field helpers.

 #### `monkeybee-annotate`
@@
-- Form field interaction (AcroForm widgets)
+- Bridge generic annotation handling to `monkeybee-forms` widgets
```

## 8) Make the proof harness expectation-driven, not only oracle-consensus-driven

The multi-oracle idea is good. It is not sufficient by itself.

Consensus render testing is necessary, but if you stop there you will get noisy CI, unstable triage, and arguments every time renderers disagree or extraction heuristics move slightly. The proof system needs a second layer: **fixture-level expectation manifests**.

Each corpus fixture should carry:

* expected feature tiers
* allowed degradations
* expected render score range
* extraction goldens or invariants
* signature preservation expectations
* known oracle disagreements
* triage status (`approved`, `pending`, `known_bad`, etc.)

That turns the proof harness from a raw comparer into a stable public scorecard.

```diff
diff --git a/SPEC.md b/SPEC.md
@@ Pathological corpus
-The corpus is split into `public/`, `restricted/`, `generated/`, and `minimized/` tiers.
+The corpus is split into `public/`, `restricted/`, `generated/`, and `minimized/` tiers.
+Every fixture also carries an expectation manifest: expected tier assignments, allowed degradations, render-score thresholds, extraction goldens or invariants, signature expectations, and triage status.

@@ Compatibility ledger schema
 CompatibilityLedger {
   schema_version: string,
   document_id: string,
+  oracle_manifest_id: string,
+  expectation_manifest_id: Option<string>,
   file_name: string,
   file_size: u64,
   pdf_version: string,
@@ Part 8 — Release gates for v1
+- [ ] Every gated fixture has an expectation manifest or explicit `triage_pending` status.
+- [ ] CI reports regressions by class and severity, not only by pass/fail.
+- [ ] Public scorecards distinguish new failures, expectation drift, known oracle disagreement, and approved degradation.
```

## 9) Turn extraction into a multi-surface API with confidence, instead of one “best guess” surface

Right now extraction is conceptually one pipeline that emits text with positions plus heuristics. I would make it more explicit and more honest.

Expose several extraction surfaces:

* `PhysicalText`: exactly what was painted, with glyphs/quads and geometry
* `LogicalText`: reading-order output derived by heuristics
* `TaggedText`: structure-tree-driven when tags exist
* `SearchIndex`, `SelectionQuads`, and `HitTest`: downstream viewer/editor primitives
* confidence and provenance on each derived block

This improves reliability because consumers can choose the surface they actually need. It also makes the engine much more useful immediately.

```diff
diff --git a/SPEC.md b/SPEC.md
@@ `monkeybee-extract`
-- Text extraction with character positions, font information, and reading order heuristics
+- Multi-surface text extraction:
+  - `PhysicalText` (glyphs, quads, exact geometry)
+  - `LogicalText` (reading-order text with per-block confidence)
+  - `TaggedText` (structure-tree-driven when available)
+  - `SearchIndex`, `SelectionQuads`, and `HitTest` primitives for viewers and editors

@@ Text extraction pipeline
-4. After all content streams for a page are processed, sort characters into reading order.
+4. `PhysicalText` is emitted before any reading-order heuristics are applied.
+5. `LogicalText` is produced as a separate derived surface; each line/block carries a confidence score and the heuristic or tagged path that produced it.

@@ `monkeybee-cli`
 - `monkeybee extract <file> [--text|--meta|--images|--fonts|--structure]`
+ - `monkeybee extract <file> --mode physical|logical|tagged`
+ - `monkeybee search <file> <query>` — document text search with page/quad results
+ - `monkeybee render <file> --region <x,y,w,h>` — region render for viewer-like workflows
```

---

My short version of the whole review:

The plan is already better than most “big engine” specs because it has a real thesis and real proof doctrine. I would not add more ambition. I would make the current ambition **more survivable**:

* simplify the baseline,
* isolate risky subsystems,
* centralize text,
* make state immutable and versioned,
* make invalidation exact,
* treat remote/progressive access as first-class,
* elevate forms,
* harden the proof harness,
* and make extraction more explicit and useful.  

If I were sequencing this rewrite, I would do **#1, #2, #4, and #8 first**. Those four changes would improve the project more than anything else because they tighten scope, reduce security risk, make concurrency sane, and give the proof system the stability needed to justify the whole claim.
