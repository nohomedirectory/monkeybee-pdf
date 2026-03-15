I read the README, implementation master, and the spec together. The plan is already unusually strong: the closed-loop thesis is crisp, the preservation boundary is well chosen, the proof doctrine is serious, and the baseline-vs-experimental split is exactly the right instinct. The best revisions are mostly about tightening a few architectural seams before they harden into expensive constraints, and about turning the existing proof machinery into a more compelling user-facing surface.   

Below are the changes I would make, in priority order.

## 1. Add a single stable public facade crate

Right now the workspace decomposition is excellent for implementation, but it also makes nearly every subsystem look “public.” That is dangerous for semver. Engines like this learn from ugly documents; the parser/document/content/render boundaries will almost certainly get revised once the corpus starts teaching hard lessons. If downstream users bind directly to `monkeybee-parser`, `monkeybee-document`, `monkeybee-content`, and friends, you will freeze the architecture too early.

I would add one semver-governed facade crate and make everything else internal-by-default. That preserves the current layered workspace, but it stops the workspace layout from becoming the public API contract. It also gives you a clean place to expose only the concepts that matter externally: `Engine`, `Session`, `Snapshot`, `EditTransaction`, `WritePlan`, `CapabilityReport`, and later a WASM/FFI-friendly surface.  

```diff
--- a/README.md
+++ b/README.md
@@ Architecture at a glance
+| `monkeybee` | Stable public facade: semver-governed `Engine`, `Session`, `Snapshot`, `EditTransaction`, `WritePlan`, `CapabilityReport`, and high-level open/render/extract/edit/save APIs |
 | `monkeybee-core` | Shared primitives: object IDs, geometry, errors, diagnostics, execution budgets, diagnostic streaming (DiagnosticSink), PDF version tracking, StreamHandle contract, provider traits (CryptoProvider, OracleProvider) |

@@ Repo structure
 ├── crates/
+│   ├── monkeybee/
 │   ├── monkeybee-core/
 │   ├── monkeybee-bytes/
 ...

--- a/SPEC.md
+++ b/SPEC.md
@@ Part 3 — System architecture / Workspace layout
+`monkeybee` is the only semver-stable public library crate.
+All other workspace crates are implementation crates unless explicitly re-exported by `monkeybee`.
+The workspace layout is not itself the public API contract.
```

## 2. Fix identity and caching now: add `DocumentId` and two-level caches

This is the first thing I would change before any real implementation begins.

One implementation detail is actively risky: the cache manager keys parsed fonts only by `ObjRef`, while other caches are snapshot-scoped. `ObjRef` is only unique inside one document; two concurrently open documents can absolutely collide on `12 0 R`. That is a correctness bug waiting to happen. More broadly, the plan needs a sharper identity model: document identity, snapshot identity, object identity, and semantic-resource identity should be distinct. 

The fix is to introduce `DocumentId` and split caching into two levels:

* document/snapshot-scoped caches for mutable semantic things
* global immutable caches keyed by semantic fingerprints for parsed fonts, ICC profiles, CMaps, decoded immutable blobs, etc.

That gives you both correctness and cross-snapshot reuse. It also makes progressive render and edit-heavy workflows cheaper, because unchanged resources can survive snapshot churn.

```diff
--- a/implementation_master.md
+++ b/implementation_master.md
@@ Core data structures
+#[derive(Clone, Copy, PartialEq, Eq, Hash)]
+pub struct DocumentId(pub u128);
+
+#[derive(Clone, Copy, PartialEq, Eq, Hash)]
+pub struct ObjectKey {
+    pub document_id: DocumentId,
+    pub obj_ref: ObjRef,
+}
+
+#[derive(Clone, PartialEq, Eq, Hash)]
+pub struct ResourceFingerprint(pub [u8; 32]);

@@ Cache management
 pub struct CacheManager {
     pub config: CacheConfig,
     pub decoded_streams: DashMap<(SnapshotId, ObjRef, u64), Arc<[u8]>>,
-    pub fonts: DashMap<ObjRef, Arc<ParsedFont>>,
+    pub doc_fonts: DashMap<(SnapshotId, ObjRef), Arc<ParsedFontInstance>>,
+    pub shared_font_programs: DashMap<ResourceFingerprint, Arc<ParsedFontProgram>>,
+    pub shared_icc_profiles: DashMap<ResourceFingerprint, Arc<ParsedIccProfile>>,
+    pub shared_cmaps: DashMap<ResourceFingerprint, Arc<ParsedCMap>>,
     pub page_plans: DashMap<(SnapshotId, usize), Arc<PagePlan>>,
     pub raster_tiles: DashMap<(SnapshotId, usize, TileId, u32), Arc<TileData>>,
 }

--- a/SPEC.md
+++ b/SPEC.md
@@ Part 4 — Shared invariants / Object identity
+Object identity is layered:
+- `ObjRef` is document-local identity only.
+- `DocumentId + ObjRef` is cross-session object identity.
+- `SnapshotId` identifies semantic state.
+- `ResourceFingerprint` identifies immutable reusable artifacts across snapshots/documents.
+
+No cache may key cross-document data by `ObjRef` alone.
```

## 3. Replace the “dependency graph is a DAG” model with a cycle-aware graph model

The spec says the dependency graph is a directed acyclic graph and that cycles are detected and reported as errors. That is too strong for PDFs. Parent links, structure trees, form/XObject relationships, name trees, parent trees, and plenty of real-world oddities create cycles or cycle-like backreferences that are not useful to classify as outright architectural violations. The implementation master repeats the DAG assumption.  

I would replace that with a three-view model:

* `ReferenceGraph`: the raw directed graph; cycles are allowed
* `OwnershipGraph`: a stricter semantic view for things that really should be tree/forest-shaped
* `CondensedDependencyGraph`: an SCC-condensed DAG used for invalidation, GC, and page-impact queries

That preserves the usefulness of DAG-style planning where it matters, without lying about the underlying format.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Part 3 — System architecture / Dependency graph contract
-The dependency graph is a directed acyclic graph (cycles are detected and reported as errors)
-mapping objects to the objects they transitively depend on.
+Monkeybee maintains three related graph views:
+1. `ReferenceGraph`: the raw directed object-reference graph; cycles are legal.
+2. `OwnershipGraph`: semantic ownership/inheritance edges with stronger invariants.
+3. `CondensedDependencyGraph`: a strongly-connected-components condensation DAG used for
+   invalidation, page-impact, and resource-GC planning.

@@
-**Queries:**
+**Queries:**
 1. `dependents_of(obj_id) -> Set<ObjRef>`
 2. `dependencies_of(obj_id) -> Set<ObjRef>`
 3. `page_dependencies(page_index) -> Set<ObjRef>`
 4. `edit_impact(changed_ids: Set<ObjRef>) -> EditImpact`
+5. `scc_of(obj_id) -> SccId`
+6. `ownership_violations() -> Vec<OwnershipViolation>`

--- a/implementation_master.md
+++ b/implementation_master.md
@@ Dependency graph
-/// Dependency graph: DAG mapping objects to their transitive dependencies.
-pub struct DependencyGraph {
+/// Raw directed reference graph; cycles are legal.
+pub struct ReferenceGraph {
     pub forward: DashMap<ObjRef, Vec<(ObjRef, EdgeType)>>,
     pub reverse: DashMap<ObjRef, Vec<(ObjRef, EdgeType)>>,
     pub snapshot_id: SnapshotId,
 }
+
+/// SCC-condensed DAG used by invalidation and GC.
+pub struct CondensedDependencyGraph {
+    pub scc_nodes: DashMap<SccId, Vec<ObjRef>>,
+    pub dag_forward: DashMap<SccId, Vec<SccId>>,
+    pub dag_reverse: DashMap<SccId, Vec<SccId>>,
+    pub snapshot_id: SnapshotId,
+}
```

## 4. Separate session defaults from per-operation `ExecutionContext`

The spec says every top-level API accepts an `ExecutionContext`, which is good. But the implementation sketch also stores `exec_ctx` on `OpenSession`, and `security_profile` exists both on the engine and in `ExecutionContext`. That creates ambiguous precedence and stale-state problems: which policy wins, and why should a long-lived session own cancellation/deadline state that is supposed to be per operation?  

I would make the layering explicit:

* `EnginePolicy`: immutable engine-wide defaults and providers
* `SessionConfig`: byte source, password/open strategy, document-local defaults
* `ExecutionContext`: strictly per-operation budgets, cancellation, deadline, tracing, and optional overrides

This prevents subtle bugs and makes concurrent calls on one session much cleaner.

```diff
--- a/implementation_master.md
+++ b/implementation_master.md
@@ Engine and session model
 pub struct MonkeybeeEngine {
     pub config: EngineConfig,
     pub caches: CacheManager,
     pub font_provider: Box<dyn FontProvider>,
     pub color_profile_provider: Box<dyn ColorProfileProvider>,
     pub crypto_provider: Option<Box<dyn CryptoProvider>>,
     pub oracle_provider: Option<Box<dyn OracleProvider>>,
-    pub security_profile: SecurityProfile,
+    pub engine_policy: EnginePolicy,
 }

 pub struct OpenSession {
     pub engine: Arc<MonkeybeeEngine>,
     pub byte_source: Box<dyn ByteSource>,
     pub revision_chain: RevisionChain,
     pub current_snapshot: Arc<PdfSnapshot>,
-    pub open_strategy: OpenStrategy,
-    pub exec_ctx: ExecutionContext,
+    pub session_config: SessionConfig,
 }

+pub struct EnginePolicy {
+    pub default_security_profile: SecurityProfile,
+    pub provider_policy: ProviderPolicy,
+}
+
+pub struct SessionConfig {
+    pub open_strategy: OpenStrategy,
+    pub password: Option<SecretString>,
+    pub session_overrides: SessionOverrides,
+}

--- a/SPEC.md
+++ b/SPEC.md
@@ Part 0 — Execution context doctrine
-Every top-level API accepts an `ExecutionContext` carrying:
+Every top-level API accepts an operation-scoped `ExecutionContext` carrying:
 - Resource budgets
 - Cooperative cancellation / deadline
 - Determinism settings for CI and proof
 - Provider registry
 - Trace / metrics sink
+
+`ExecutionContext` is never stored on `OpenSession`.
+Sessions are long-lived document handles; execution contexts are per-call control planes.
```

## 5. Remove the `annotate -> render` dependency; appearance generation should live in authoring/composition

The implementation master explicitly says `monkeybee-annotate` depends on `monkeybee-render` for appearance stream generation. I would change that. Annotation/widget appearances are authored PDF content streams, not rendered pixels. Making annotate depend on render inverts the layering and couples edit-time authoring to the full render stack. 

The clean model is:

* `compose` owns appearance synthesis (`AppearanceBuilder`, text emit, graphics emit)
* `annotate` and `forms` ask `compose` to build appearance streams
* `render` only consumes those streams later

That lowers coupling, shortens rebuild time, and makes headless edit/save workflows lighter and easier to reason about.

```diff
--- a/implementation_master.md
+++ b/implementation_master.md
@@ Crate dependency graph
-monkeybee-annotate      (depends on: core, document, content, compose, forms, render)
+monkeybee-annotate      (depends on: core, document, content, compose, forms)

@@
-Note: monkeybee-annotate depends on monkeybee-render for appearance stream generation —
-specifically for rendering text and graphics within annotation appearance form XObjects.
-The compose crate handles the builder API; render provides the actual glyph/path realization.
+Note: annotation/widget appearance streams are authored PDF content, not raster output.
+Appearance synthesis lives in `monkeybee-compose` via `AppearanceBuilder`, `TextEmit`,
+and graphics-state-aware content emission. `monkeybee-render` consumes the resulting streams
+but is not a dependency of annotate/forms.

--- a/README.md
+++ b/README.md
@@ Architecture at a glance
-| `monkeybee-compose` | High-level authoring and composition: document/page builders, resource naming, appearance generation, font embedding planning |
+| `monkeybee-compose` | High-level authoring and composition: document/page builders, resource naming, annotation/widget appearance synthesis, font embedding planning |
```

## 6. Make structural sharing a hard requirement of the snapshot model

The snapshot/transaction model is one of the best parts of the plan. But the spec and implementation still read a bit like structural sharing is an implementation detail that can be filled in later. I would make it explicit now. If each committed edit materializes a full new `PdfDocument`, `SyntaxSnapshot`, and dependency graph, large real-world documents will get expensive quickly, especially in edit-heavy or proof-heavy flows.  

I would require persistent data structures and first-class deltas:

* snapshots are mostly shared structure plus a delta
* commit returns both a new snapshot and a machine-readable `SnapshotDelta`
* undo/redo, save planning, proof reconciliation, and document diff all build on the same delta

That makes the “living document model” feel real rather than merely immutable-by-convention.

```diff
--- a/implementation_master.md
+++ b/implementation_master.md
@@ Engine and session model
 pub struct PdfSnapshot {
     pub snapshot_id: SnapshotId,
-    pub document: PdfDocument,
-    pub syntax_snapshot: SyntaxSnapshot,
-    pub dep_graph: DependencyGraph,
+    pub document: Arc<PersistentPdfDocument>,
+    pub syntax_snapshot: Arc<SyntaxSnapshot>,
+    pub dep_graph: Arc<CondensedDependencyGraph>,
+    pub parent_snapshot: Option<SnapshotId>,
+    pub delta_from_parent: Option<SnapshotDelta>,
 }

@@ Change tracking
+pub struct SnapshotDelta {
+    pub changed_objects: Vec<ObjectChange>,
+    pub changed_pages: Vec<usize>,
+    pub invalidated_cache_keys: Vec<CacheKey>,
+    pub regenerated_artifacts: Vec<ArtifactId>,
+}

--- a/SPEC.md
+++ b/SPEC.md
@@ Part 3 — Engine / session / snapshot model
-`PdfSnapshot` is immutable, shareable across threads, and identified by `snapshot_id`.
+`PdfSnapshot` is immutable, shareable across threads, and structurally shared by default.
+Snapshot creation must be copy-on-write / persistent-data-structure based; full-document cloning
+is a fallback of last resort, not the baseline design.
```

## 7. Quarantine all native bridges into one crate and one policy boundary

The plan already makes the right high-level call: prefer pure Rust, but allow native bridges where quality requires them. The implementation master and feature strategy point at JPX, ICC, and possibly FreeType-backed paths; the spec also already has security-profile-based isolation for risky decoders. I would harden that further by quarantining all FFI into a single workspace crate and one runtime boundary.  

Why this helps:

* one place to audit `unsafe`
* one place to enforce subprocess isolation / broker policy
* cleaner build matrix (`pure-rust`, `native-jpx`, `native-color`, `native-fonts`)
* easier WASM and Strict-mode builds
* much less accidental FFI bleed into core crates

This is also where I would fix the spec’s small but important inconsistency around “JBIG2 decode via openjpeg-sys”; that should be corrected before it leaks into tickets or code.

```diff
--- a/README.md
+++ b/README.md
@@ Architecture at a glance
+| `monkeybee-native` | Optional native bridge quarantine: JPX/JBIG2/ICC/FreeType adapters, FFI audit surface, subprocess-friendly broker hooks |

--- a/implementation_master.md
+++ b/implementation_master.md
@@ Workspace topology
+│   ├── monkeybee-native/         # all optional FFI/native bridges and broker adapters

@@ External dependency strategy
-- **`openjpeg-sys` or `jpeg2k`** — JPXDecode (JPEG 2000)
-- **`freetype-rs` or `ttf-parser` + `ab_glyph`** — font parsing and glyph rasterization
+- **`openjpeg-sys` or `jpeg2k`** — JPXDecode, isolated behind `monkeybee-native`
+- **`lcms2`** — ICC evaluation, isolated behind `monkeybee-native`
+- **`freetype-rs`** — optional hinted rasterization, isolated behind `monkeybee-native`

--- a/SPEC.md
+++ b/SPEC.md
@@ Part 7 — Security profiles
-All high-risk decode jobs execute through `monkeybee-security` with explicit memory/time budgets and optional worker isolation
+All high-risk decode jobs and all optional native bridges execute through `monkeybee-security`
+and `monkeybee-native`, with explicit memory/time budgets and optional worker isolation.

@@ Part 8 — Baseline v1 vs experimental feature classification
-| JBIG2 decode | Baseline (via openjpeg-sys) | Common in scanned docs |
+| JBIG2 decode | Baseline (via isolated JBIG2-capable decoder path) | Common in scanned docs |
 | JPEG 2000 decode | Baseline (via openjpeg-sys) | Common in print-quality PDFs |
```

## 8. Add a spillable scratch store and a persistent remote range cache

The current cache doctrine is mostly RAM-budgeted. That is fine for normal files, but workflow 8 explicitly targets huge and range-backed PDFs, and the render/cache/fetch plan already includes progressive rendering, tile invalidation, and prefetch planning. On ugly large files, you need a bounded scratch layer, not just better RAM LRU behavior.  

I would add two new pieces:

* `ScratchStore` / `BlobStore` for spillable decoded streams, tiles, isolated-decoder outputs, and large intermediates
* `RangeCache` for remote byte ranges keyed by URL + validator (`ETag`, `Last-Modified`, content hash)

That change improves both robustness and performance: memory stays bounded, repeated remote opens get faster, and progressive mode stops feeling ephemeral.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Part 3 — Cache management doctrine
 - **Decoded stream cache:** 256 MB.
 - **Font cache:** 128 MB.
 - **PagePlan cache:** 64 MB.
 - **Raster tile cache:** Configurable, default 512 MB.
+- **Scratch spill store:** bounded local store for oversized decoded streams, raster tiles,
+  isolated-decoder outputs, and other large intermediate artifacts.
+
+When memory pressure exceeds in-memory cache budgets, eligible artifacts may spill to the
+scratch store instead of being dropped outright.

@@ Part 3 — monkeybee-bytes
 - Fetch scheduler and prefetch planning for remote/lazy byte sources
+- Persistent range cache keyed by source URL + validator (ETag/Last-Modified/content hash)

--- a/implementation_master.md
+++ b/implementation_master.md
@@ Core data structures
+pub enum BlobHandle {
+    InMemory(Arc<[u8]>),
+    Spill(ScratchBlobId),
+}

@@ Fetch scheduler
 pub struct FetchStatistics {
     pub requests_issued: u64,
     pub bytes_fetched: u64,
     pub avg_latency_ms: f64,
+    pub cache_hits: u64,
+    pub validator_reuses: u64,
 }

+pub struct RangeCacheKey {
+    pub source_id: SourceId,
+    pub offset: u64,
+    pub length: u64,
+    pub validator: SourceValidator,
+}
```

## 9. Turn the proof machinery into a first-class product feature: `CapabilityReport`, `diff`, and HTML dossiers

The spec already gives you a lot here: `diagnose`, `plan-save`, `trace`, `validate`, `proof`, structural/text/render comparison, and signature inspection. That is a gold mine. I would go one step further and expose a lighter-weight capability surface on open, plus a real document-diff feature. 

Two additions would make Monkeybee much more compelling to users:

* `CapabilityReport`: cheap, immediate answer to “what kind of PDF is this and how safe is it to edit?”
* `monkeybee diff before.pdf after.pdf`: structural diff, text diff, render diff, signature impact diff, and compatibility-tier drift

And I would add `diagnose --html` to emit a self-contained “PDF dossier” with page thumbnails, ledger summary, risky zones, object/update tree, font inventory, signatures, and edit-safety guidance. That turns the compatibility ledger from CI-only evidence into a genuinely useful forensic/debugging tool.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Part 1 — User workflows and visible proof
+### Workflow 10: Explain compatibility and diff revisions
+
+A user opens a PDF and immediately receives a `CapabilityReport` summarizing: signatures,
+encryption, tagged-structure presence, XFA/JS/risky-decoder presence, edit-safety class,
+preserve-mode constraints, and expected degradation zones.
+
+A user compares two PDFs or two snapshots and receives a unified diff: structural changes,
+text changes, render deltas, signature impact, and compatibility-tier changes.

@@ Part 3 — Engine / session / snapshot model
+pub struct CapabilityReport {
+    pub signed: bool,
+    pub encrypted: bool,
+    pub tagged: bool,
+    pub has_xfa: bool,
+    pub has_javascript: bool,
+    pub risky_decoder_set: Vec<DecoderType>,
+    pub edit_safety: EditSafetyClass,
+    pub preserve_constraints: Vec<PreserveConstraint>,
+    pub expected_degradations: Vec<FeatureCode>,
+}

@@ Part 3 — monkeybee-cli
 - `monkeybee diagnose <file>` — full compatibility report
+ - `monkeybee diagnose <file> [--html]` — JSON or self-contained HTML dossier
+ - `monkeybee diff <before> <after> [--render|--text|--structure|--save-impact]`

--- a/README.md
+++ b/README.md
@@ What v1 must prove
 - **Compatibility accounting**: every unsupported or degraded zone is explicitly detected, categorized, and surfaced — never silently swallowed.
+- **Operational explainability**: the engine can explain edit safety, signature impact, and revision-to-revision deltas in a way users can act on.
```

## 10. Tighten the release plan around these architectural risks

This is less a new subsystem than a reprioritization rule: I would explicitly mark the following as “must settle before code fan-out” items:

1. identity/caching
2. cycle-aware dependency modeling
3. session-vs-operation policy layering
4. appearance-generation boundary

Those are the changes most likely to save you a painful midstream rewrite. The rest can land iteratively.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Part 8 — Release gates for v1 / Quality gates
+- [ ] Architectural gate: identity model (`DocumentId`, cache-key rules, snapshot/resource identity)
+  is finalized before parallel subsystem implementation begins.
+- [ ] Architectural gate: dependency graph semantics (raw graph vs condensed DAG) are finalized
+  before edit/invalidation/writeback work begins.
+- [ ] Architectural gate: session policy and per-operation `ExecutionContext` precedence rules are
+  finalized before API stabilization.
+- [ ] Architectural gate: annotation/widget appearance generation no longer depends on render.
```

My shortest version of the recommendation is this:

The plan’s *thesis* is already right. The best revisions are about making the core abstractions harder to regret later: fix identity, stop pretending the object graph is acyclic, separate long-lived session state from per-operation control, remove the annotate→render layering violation, and let the proof system power user-visible explain/diff tooling instead of living only in CI.   

If you only take three changes before implementation starts, I’d take 2, 3, and 5 first.
