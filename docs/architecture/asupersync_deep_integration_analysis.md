# Asupersync Deep Integration Analysis

## Executive Summary

Our current spec treats asupersync as an executor swap — "the default orchestration runtime for CLI and proof, not a semantic dependency of the core engine model." This directly violates the mega-skill's first non-negotiable: **"Do not treat Asupersync as an executor swap."**

Jeffrey (asupersync's author) has confirmed asupersync should be deeply used. After reading the full mega-skill, all 28 reference documents, the v4 API skeleton, and auditing our current spec, the conclusion is clear: we are leaving 80% of asupersync's value on the table. The remaining 80% is exactly the kind of structural leverage that aligns with monkeybee's "alien artifact" doctrine.

The fix is not to make core crates depend on asupersync. It is to:
1. Redesign `ExecutionContext` as a bridge layer that exposes asupersync's Budget/Outcome/Cx semantics to runtime-agnostic core crates
2. Make the orchestration layer (facade, session lifecycle, fetch scheduler, progressive rendering, proof harness) deeply asupersync-native
3. Use LabRuntime as the foundation of the proof harness
4. Adopt Outcome<T, E> as the canonical result type for operations that can be cancelled

---

## Part 1: What We're Currently Missing

### 1.1 Outcome<T, E> — Four-Valued Results

**Current state:** All our APIs return `Result<T, MonkeybeeError>`. Cancellation is detected at checkpoints but collapses into an error variant.

**What asupersync provides:** `Outcome<T, E>` with four states: `Ok(T)`, `Err(E)`, `Cancelled(CancelReason)`, `Panicked(PanicPayload)`. The severity lattice (`Ok < Err < Cancelled < Panicked`) composes across joins, races, retries, and supervision.

**Why this matters for monkeybee:**
- `Cancelled` is not an error — it's a viewport change, user abort, budget exhaustion, or shutdown. Our `RenderReport` already distinguishes "cancelled tiles" from "failed tiles." Outcome makes this first-class.
- `Panicked` is not a domain error — it's a native decoder segfault, a malformed-PDF-induced stack overflow in parsing, or a bug. Our `monkeybee-native` quarantine zone needs this distinction.
- Supervision, retry, and diagnostic behavior all differ based on whether the outcome was Err vs Cancelled vs Panicked. Flattening to `Result` discards this.

**Concrete impact:**
```
// Current:
pub fn render_page(...) -> Result<RenderResult, RenderError>

// Proposed:
pub fn render_page(...) -> Outcome<RenderResult, RenderError>
// Where Cancelled carries: which tiles completed, cancel reason
// Where Panicked carries: which native decoder crashed, panic payload
```

### 1.2 Budget as Product Semiring

**Current state:** Our `ExecutionContext` carries resource budgets (objects, decompressed bytes, operators, recursion depth) as hand-rolled fields with manual checking.

**What asupersync provides:** `Budget` with (deadline, poll_quota, cost_quota, priority) and automatic `combine()` algebra where child budgets inherit the tighter constraint.

**Why this matters:**
- Our budget system has the right fields but lacks the algebraic structure. When a render operation spawns decode sub-operations, the child should automatically inherit a tighter budget. Currently this is manual and error-prone.
- asupersync's `Budget.combine()` (meet semiring) is exactly the invariant we need: risky decoders get tighter deadlines, speculative prefetch gets lower priority, cleanup gets bounded cost.
- The `poll_quota` concept maps to our "operator count budget" — a pathological content stream with 1M+ operators should be interruptible by poll quota, not just by a timer.

**Concrete mapping:**
```
asupersync Budget.deadline     → our ExecutionContext.deadline
asupersync Budget.cost_quota   → our operator_count + decompressed_bytes budgets
asupersync Budget.poll_quota   → our checkpoint frequency
asupersync Budget.priority     → render priority (viewport-visible > prefetch > background)
```

### 1.3 &Cx for Cancellation Checkpoints

**Current state:** Our render pipeline checks `exec_ctx.cancellation_token.is_cancelled()` at per-operator, per-tile, per-page, and per-resource checkpoints. The CancellationToken is `Arc<AtomicBool>`.

**What asupersync provides:** `cx.checkpoint()` which is budget-aware, trace-aware, and scheduler-cooperative. It checks cancellation AND budget exhaustion AND yields to the scheduler in one call.

**Why this matters:**
- Our current checkpoint checks only cancellation. Budget exhaustion requires separate checks at each checkpoint. With Cx, one `checkpoint()` call handles both.
- `cx.checkpoint()` also emits trace events, enabling the LabRuntime to deterministically replay and test cancellation behavior.
- The scheduler lane model (cancel > timed > ready) means cancellation cleanup gets priority over new work, which is exactly right for viewport-change cancel storms.

### 1.4 Structured Regions for Lifecycle

**Current state:** Our lifecycle model (Engine → Session → Snapshot → Transaction) has no formal ownership hierarchy enforced by the runtime.

**What asupersync provides:** Region-based structured concurrency where every task is owned by exactly one region, region close implies quiescence (no live children + all finalizers complete), and cancellation propagates from parent to children.

**Why this matters — the mapping is natural:**

| Monkeybee concept | asupersync region |
|---|---|
| `MonkeybeeEngine` lifetime | Root region (or AppSpec for supervised engine) |
| `OpenSession` lifetime | Session region (child of engine) |
| `render_page()` call | Render region (child of session, with deadline budget) |
| `extract_text()` call | Extract region (child of session) |
| `EditTransaction` scope | Transaction region (child of session, with tighter budget) |
| `WritePlan.execute()` | Write region (child of session) |
| Risky native decoder | Quarantine child region (FailFast policy) |
| Progressive tile batch | Tile region (CollectAll policy — partial results OK) |

**Critical property:** When a Session is closed, the region guarantees all child operations (renders, extracts, decodes) are fully drained. No leaked background tasks. No orphaned cache entries being written. This is exactly our "quiescence" requirement.

### 1.5 LabRuntime for Proof Harness

**Current state:** Our proof harness (`monkeybee-proof`) is described as corpus-backed testing with round-trip validation and render comparison. The runtime orchestration flow mentions "LabRuntime entry" but doesn't leverage its capabilities.

**What asupersync provides:** Deterministic testing runtime with:
- Fixed seed = deterministic scheduling = reproducible bugs
- Virtual time wheel (sleeps complete instantly, time is controlled)
- DPOR schedule exploration (systematic interleaving coverage)
- Oracle suite: quiescence, obligation leaks, loser drain, cancellation protocol
- Chaos injection: cancellation storms, budget exhaustion, wakeup storms
- Crashpacks: deterministic repro anchors for concurrency bugs
- Futurelock detection: stuck tasks that hold obligations

**Why this matters for monkeybee's proof doctrine:**

The proof doctrine says: "No feature ships without evidence. No release gate passes on rhetoric." LabRuntime turns this from a testing aspiration into a mechanical guarantee:

1. **Concurrent render correctness:** Run 8-page parallel render under LabRuntime with DPOR exploration. Prove that all scheduling interleavings produce the same pixel output. This catches cache races, shared font corruption, and DashMap contention bugs that random testing misses.

2. **Cancellation correctness:** Use chaos injection to fire cancellation at every checkpoint in the render pipeline. Prove that partial results are always usable, cache state is always consistent, and no resources leak.

3. **Progressive rendering correctness:** Use LabRuntime to simulate viewport changes during progressive tile loading. Prove that cancelled tiles are properly cleaned up and new viewport tiles are prioritized.

4. **Session lifecycle correctness:** Use obligation leak oracle to prove that closing a session leaves no orphaned tasks, no pending cache writes, no dangling fetch requests.

5. **Crash-safe save correctness:** Use LabRuntime to inject cancellation during the staged commit sequence. Prove that the original file is never corrupted.

### 1.6 Capability Narrowing

**Current state:** Our security profiles (Compatible, Hardened, Strict) constrain behavior but through configuration, not type-level enforcement.

**What asupersync provides:** Compile-time capability narrowing where handlers receive only the effects they need (`[SPAWN, TIME, RANDOM, IO, REMOTE]`), with `cx_narrow()` and `cx_readonly()`.

**Monkeybee mapping:**
- **Pure document operations** (parse, render, extract, write): `cx_readonly()` or no Cx at all — these are pure compute on immutable snapshots
- **Orchestration** (session management, fetch scheduling): narrowed Cx with SPAWN + TIME
- **Native decoder quarantine**: narrowed Cx with IO only (subprocess broker)
- **Proof harness**: full Cx for testing all paths

### 1.7 Two-Phase Channels for Crash-Safe Save

**Current state:** Our crash-safe save contract specifies: serialize to temp → fsync → validate → fsync parent → atomic rename. This is a manual protocol.

**What asupersync provides:** Two-phase send (reserve/commit) that prevents data loss on cancellation. The reserve/commit pattern is exactly the "prepare/execute" pattern our crash-safe save needs.

**Monkeybee mapping:**
```
reserve(temp_file)     → serialize output
commit(temp_file)      → fsync + validate + atomic rename
abort(on_cancel)       → delete temp, preserve original
```

### 1.8 Combinators for Document Processing Pipelines

**Current state:** Our render/extract/decode pipelines are described procedurally.

**What asupersync provides:** Native combinators that are cancel-aware, budget-respecting, and loser-drain-correct:
- `map_reduce` — page-level parallel render/extract
- `pipeline` — staged decode chain (decompress → decrypt → decode → color convert)
- `bracket` — acquire/use/release for cache entries, temp files, quarantine processes
- `timeout` — per-operation deadlines
- `bulkhead` — isolate native decoder failures from engine stability

---

## Part 2: The Architecture — Runtime-Agnostic Core, asupersync-Native Orchestration

### 2.1 The Key Insight

The resolution to "core crates must be runtime-agnostic" AND "asupersync should be deeply used" is:

**Core crates define traits and contracts. The orchestration layer implements them using asupersync primitives.**

This is not "asupersync at the CLI edge." It is "asupersync IS the orchestration substrate."

```
┌─────────────────────────────────────────────────────┐
│  monkeybee (public facade)                          │
│  ├── asupersync::RuntimeBuilder / LabRuntime        │
│  ├── Session lifecycle as asupersync Regions        │
│  ├── Outcome<T, E> as public return type            │
│  ├── Budget algebra for child operations            │
│  └── &Cx → ExecutionContext bridge                  │
├─────────────────────────────────────────────────────┤
│  monkeybee-bytes (fetch scheduler)                  │
│  ├── asupersync I/O for range requests              │
│  ├── asupersync channels for prefetch coordination  │
│  └── &Cx for cancellation + budget                  │
├─────────────────────────────────────────────────────┤
│  monkeybee-proof (proof harness)                    │
│  ├── LabRuntime for deterministic testing            │
│  ├── DPOR for concurrency coverage                  │
│  ├── Oracle suite for lifecycle correctness          │
│  ├── Chaos injection for robustness                 │
│  └── Crashpacks for reproducible failure artifacts  │
├─────────────────────────────────────────────────────┤
│  Core crates (runtime-agnostic)                     │
│  ├── monkeybee-parser: &ExecutionContext only        │
│  ├── monkeybee-render: &ExecutionContext only        │
│  ├── monkeybee-write:  &ExecutionContext only        │
│  ├── monkeybee-content: &ExecutionContext only       │
│  └── etc.                                           │
│                                                     │
│  ExecutionContext contains:                          │
│  ├── CancellationCheckpoint (trait, impl by Cx)     │
│  ├── BudgetState (derived from asupersync Budget)   │
│  ├── DiagnosticSink (impl emits to Cx trace)        │
│  └── ProviderRegistry                               │
└─────────────────────────────────────────────────────┘
```

### 2.2 ExecutionContext as the Bridge

```rust
/// monkeybee-core (runtime-agnostic)
pub trait CancellationCheckpoint {
    /// Returns Err if cancelled or budget exhausted.
    fn check(&self) -> Result<(), CancellationReason>;
}

pub struct ExecutionContext {
    pub checkpoint: Box<dyn CancellationCheckpoint>,
    pub budget_state: BudgetState,
    pub diagnostics: Box<dyn DiagnosticSink>,
    pub providers: ProviderRegistry,
    pub determinism: DeterminismConfig,
}

/// monkeybee (facade, asupersync-native)
struct CxCheckpoint<'a>(&'a asupersync::Cx);

impl CancellationCheckpoint for CxCheckpoint<'_> {
    fn check(&self) -> Result<(), CancellationReason> {
        // Delegates to cx.checkpoint() which is:
        // - cancellation-aware
        // - budget-aware
        // - scheduler-cooperative
        // - trace-emitting
        self.0.checkpoint().map_err(|cr| map_cancel_reason(cr))
    }
}

/// Create ExecutionContext from asupersync Cx
pub fn exec_ctx_from_cx(cx: &asupersync::Cx) -> ExecutionContext {
    ExecutionContext {
        checkpoint: Box::new(CxCheckpoint(cx)),
        budget_state: BudgetState::from_asupersync(cx.budget()),
        diagnostics: Box::new(CxDiagnosticSink(cx)),
        providers: ...,
        determinism: ...,
    }
}
```

This means:
- Core crates call `exec_ctx.checkpoint.check()` at every checkpoint — no asupersync dependency
- Under the hood, `check()` delegates to `cx.checkpoint()` — full asupersync integration
- WASM builds provide a different `CancellationCheckpoint` impl (simple AtomicBool)
- Test builds can provide a deterministic impl
- The bridge is zero-cost for the common case (single function pointer indirection)

### 2.3 Session Lifecycle as Regions

```rust
// monkeybee facade (asupersync-native)
impl MonkeybeeEngine {
    pub async fn open(
        &self,
        cx: &Cx,
        byte_source: Box<dyn ByteSource>,
        opts: OpenOptions,
    ) -> Outcome<OpenSession, OpenError> {
        // Create a session region with budget derived from cx
        cx.region(SessionPolicy, |scope| async {
            let exec_ctx = exec_ctx_from_cx(&scope.cx());

            // Parse in the session region
            let snapshot = parse_document(&byte_source, &exec_ctx)?;

            // Register finalizer for cleanup
            scope.defer(|cx| async { cleanup_session_resources(cx).await });

            Outcome::Ok(OpenSession {
                engine: self.clone(),
                scope_handle: scope,
                snapshot: Arc::new(snapshot),
            })
        }).await
    }
}

impl OpenSession {
    pub async fn render_page(
        &self,
        cx: &Cx,
        page_index: usize,
        opts: RenderOptions,
    ) -> Outcome<RenderResult, RenderError> {
        // Create a child render region with deadline budget
        let render_budget = cx.budget().combine(&Budget::from_deadline(opts.deadline));

        cx.region(CollectAll, |scope| async {
            let exec_ctx = exec_ctx_from_cx(&scope.cx());

            // Rayon does the actual compute, cx owns the lifecycle
            let pixels = scope.spawn(|cx| async {
                rayon::scope(|s| {
                    // Page rendering happens in Rayon
                    render_page_impl(page_index, &exec_ctx, s)
                })
            }).await;

            Outcome::from_result(pixels)
        }).await
    }
}
```

### 2.4 Rayon Bridge Contract

The bridge between asupersync and Rayon is explicit and structured:

```rust
/// Pattern: asupersync owns lifecycle, Rayon owns compute
async fn render_pages_parallel(
    cx: &Cx,
    snapshot: &PdfSnapshot,
    pages: &[usize],
    exec_ctx: &ExecutionContext,
) -> Outcome<Vec<RenderResult>, RenderError> {
    // asupersync scope for lifecycle ownership
    cx.region(CollectAll, |scope| async {
        // Spawn one asupersync task per page
        let handles: Vec<_> = pages.iter().map(|&page| {
            scope.spawn_named(&format!("render-page-{}", page), |cx| async move {
                let exec_ctx = exec_ctx_from_cx(&cx);

                // Hand off to Rayon for CPU-bound compute
                let (tx, rx) = asupersync::channel::oneshot();
                rayon::spawn(move || {
                    let result = render_page_cpu(page, snapshot, &exec_ctx);
                    let _ = tx.send(result);
                });

                // Wait in asupersync context (cancellable)
                rx.recv(&cx).await
            })
        }).collect();

        // Join all page renders
        let results = scope.join_all(handles).await;
        aggregate_render_results(results)
    }).await
}
```

**Key invariants:**
1. asupersync regions own task lifetime — no orphaned Rayon jobs
2. Rayon does only pure compute — no async I/O, no state mutation
3. oneshot channels bridge the boundary — cancel-safe
4. Budget/cancellation propagates through ExecutionContext into Rayon work
5. If the asupersync region cancels, the Rayon work sees it at the next checkpoint

### 2.5 Progressive Rendering with Watch Channels

```rust
/// Progressive tile rendering for viewer workloads
async fn render_progressive(
    cx: &Cx,
    snapshot: &PdfSnapshot,
    viewport: Viewport,
) -> Outcome<(), RenderError> {
    // Watch channel for tile completion notifications
    let (tile_tx, tile_rx) = asupersync::channel::watch(TileState::empty());

    cx.region(CollectAll, |scope| async {
        // Tile worker region
        scope.spawn_named("tile-scheduler", |cx| async move {
            for tile_id in viewport.visible_tiles() {
                cx.checkpoint()?;

                let tile_budget = cx.budget().combine(
                    &Budget::with_priority(tile_priority(tile_id, viewport))
                );

                // Render tile on Rayon, report via watch
                let result = render_tile_on_rayon(&cx, snapshot, tile_id).await;
                tile_tx.send(TileState::completed(tile_id, result));
            }
            Ok(())
        });

        // Viewport can observe progress via tile_rx
        // On viewport change: cancel this region, start new one
        Outcome::ok(())
    }).await
}
```

---

## Part 3: Proof Harness — LabRuntime Deep Integration

This is where the "alien artifact" quality becomes real. No other PDF engine has deterministic concurrency testing with DPOR, oracle suites, and crashpacks.

### 3.1 Deterministic Concurrent Render Testing

```rust
#[test]
fn test_parallel_render_deterministic() {
    let lab = LabRuntime::new(
        LabConfig::new(42)
            .panic_on_leak(true)
            .futurelock_max_idle_steps(10_000)
            .panic_on_futurelock(true)
            .capture_trace(true),
    );

    lab.run(|cx| async {
        let engine = MonkeybeeEngine::new(test_config());
        let session = engine.open(&cx, test_pdf(), OpenOptions::default()).await.unwrap();
        let snapshot = session.current_snapshot();

        // Render all pages in parallel
        let results = render_pages_parallel(&cx, &snapshot, &[0, 1, 2, 3], &exec_ctx).await;

        assert!(results.is_ok());
        // Pixel-exact comparison against reference
        assert_render_matches_reference(&results.ok().unwrap());
    });

    // Oracle assertions
    assert!(lab.quiescence_oracle().is_ok(), "no orphan tasks after session close");
    assert!(lab.obligation_leak_oracle().is_ok(), "no leaked obligations");
}
```

### 3.2 Cancellation Chaos Testing

```rust
#[test]
fn test_render_cancellation_safety() {
    for seed in 0..100 {
        let lab = LabRuntime::new(
            LabConfig::new(seed)
                .with_heavy_chaos()  // inject cancellations at random checkpoints
                .panic_on_leak(true)
                .capture_trace(true),
        );

        lab.run(|cx| async {
            let engine = MonkeybeeEngine::new(test_config());
            let session = engine.open(&cx, pathological_pdf(), OpenOptions::default()).await;

            match session {
                Outcome::Ok(session) => {
                    let result = session.render_page(&cx, 0, RenderOptions::default()).await;
                    match result {
                        Outcome::Ok(r) => assert!(r.pixels.is_valid()),
                        Outcome::Cancelled(_) => { /* expected under chaos */ }
                        Outcome::Err(e) => assert!(e.is_recoverable()),
                        Outcome::Panicked(p) => panic!("render panicked: {:?}", p),
                    }
                }
                Outcome::Cancelled(_) => { /* expected under chaos */ }
                Outcome::Err(_) => { /* expected for pathological PDFs */ }
                Outcome::Panicked(p) => panic!("open panicked: {:?}", p),
            }
        });

        assert!(lab.quiescence_oracle().is_ok());
        assert!(lab.obligation_leak_oracle().is_ok());
    }
}
```

### 3.3 Crash-Safe Save Testing

```rust
#[test]
fn test_crash_safe_save_atomicity() {
    let lab = LabRuntime::new(
        LabConfig::new(42)
            .with_chaos(ChaosConfig {
                cancel_probability: 0.3,  // 30% chance of cancel at each checkpoint
                ..Default::default()
            })
            .panic_on_leak(true),
    );

    lab.run(|cx| async {
        let original_bytes = std::fs::read("test.pdf").unwrap();
        let engine = MonkeybeeEngine::new(test_config());

        // Open, edit, try to save with chaos
        let session = engine.open(&cx, &original_bytes, OpenOptions::default()).await.unwrap();
        let mut tx = EditTransaction::new(session.current_snapshot());
        tx.add_annotation(0, test_annotation()).unwrap();
        let new_snapshot = tx.commit().unwrap();

        let save_result = save_atomic(&cx, &new_snapshot, "output.pdf").await;

        match save_result {
            Outcome::Ok(_) => {
                // If save succeeded, output must be valid
                let check = engine.open(&cx, "output.pdf", OpenOptions::strict()).await;
                assert!(check.is_ok());
            }
            Outcome::Cancelled(_) => {
                // If save was cancelled, original must be untouched
                let current_bytes = std::fs::read("test.pdf").unwrap();
                assert_eq!(original_bytes, current_bytes);
            }
            _ => {}
        }
    });
}
```

---

## Part 4: WASM Compatibility

**Core crates remain runtime-agnostic.** The `CancellationCheckpoint` trait in `ExecutionContext` has a simple `AtomicBool`-based impl for WASM. No asupersync dependency in WASM builds.

**But:** asupersync itself has WASM support (`asupersync-wasm`, `asupersync-browser-core`) with four canonical browser profiles. If we want a browser-based PDF viewer demo (which the spec says "the architecture must not preclude"), we could use asupersync's browser runtime directly.

**Pragmatic approach:** WASM builds use the simple `ExecutionContext` impl (no asupersync). If the browser demo needs async orchestration (e.g., fetch scheduling for remote PDFs), we evaluate asupersync-wasm at that time.

---

## Part 5: Concrete Spec Changes Needed

### 5.1 Runtime Layering Doctrine (SPEC.md, lines 3272-3285)

**Current:**
> Core library crates are runtime-agnostic.
> `asupersync` is the default orchestration runtime for CLI and proof, not a semantic dependency of the core engine model.

**Proposed:**
```
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
```

### 5.2 Execution Context Doctrine (SPEC.md, lines 141-151)

**Current:**
> Every top-level API accepts an operation-scoped `ExecutionContext` carrying: ...

**Proposed addition:**
```
### ExecutionContext as runtime bridge

`ExecutionContext` is the contract between runtime-agnostic core crates and the
asupersync-native orchestration layer.

In asupersync-native callers (facade, CLI, proof), `ExecutionContext` is derived
from `&Cx`:
- `CancellationCheckpoint` delegates to `cx.checkpoint()` (budget-aware, trace-aware,
  scheduler-cooperative)
- `BudgetState` is derived from `cx.budget()` with monkeybee-specific field mapping
- `DiagnosticSink` emits to `cx.trace()` for LabRuntime observability

In runtime-agnostic callers (WASM, third-party integrations), `ExecutionContext`
uses simple implementations (AtomicBool cancellation, manual budget tracking).

The bridge is intentionally zero-cost: a single function pointer indirection for
checkpoint calls, which are already on the order of microseconds between operators.
```

### 5.3 Outcome Type for Public APIs

**Proposed addition to API surface:**
```
### Outcome discipline

Operations that can be cancelled return `Outcome<T, E>` rather than `Result<T, E>`.
The four-valued Outcome distinguishes:
- `Ok(T)` — operation succeeded with full result
- `Err(E)` — domain error (malformed PDF, unsupported feature, validation failure)
- `Cancelled(CancelReason)` — operation was cancelled (viewport change, user abort,
  budget exhaustion, shutdown). Partial results may be available.
- `Panicked(PanicPayload)` — unrecoverable failure (native decoder crash, bug).
  Must be surfaced to supervision/diagnostics, never silently swallowed.

CancelReason carries structured kind: User, Timeout, FailFast, ParentCancelled,
Shutdown, BudgetExhausted. These map to different retry, diagnostic, and supervision
policies.

At library boundaries (FFI, C API, WASM), Outcome is collapsed to Result with
structured error discrimination. Within the Rust API, Outcome is preserved.
```

### 5.4 Session Lifecycle as Regions

**Proposed addition:**
```
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
```

### 5.5 Proof Harness with LabRuntime

**Proposed addition to Part 8 (Release gates):**
```
### LabRuntime proof integration

The proof harness uses asupersync `LabRuntime` as its execution substrate:

- **Deterministic concurrent testing:** All multi-page parallel render/extract tests
  run under LabRuntime with fixed seeds. Same seed = same scheduling = reproducible
  results.

- **DPOR exploration:** Critical concurrency tests use DPOR schedule exploration to
  systematically cover scheduling interleavings, catching races that random testing
  misses.

- **Oracle suite:** Every proof run asserts:
  - Quiescence oracle: no orphan tasks after session close
  - Obligation leak oracle: no leaked permits/channels/resources
  - Loser drain oracle: cancelled operations fully drained
  - Cancellation protocol oracle: request → drain → finalize sequence observed

- **Chaos injection:** Robustness tests use chaos presets:
  - `with_light_chaos()` for CI regression
  - `with_heavy_chaos()` for release-gate shakeout
  - Focused cancellation campaigns for crash-safe save, progressive render, and
    native decoder quarantine paths

- **Crashpacks:** Concurrency failures automatically produce crashpacks with seed,
  trace, oracle failures, and replay command. These are CI artifacts linked to the
  compatibility ledger.

- **Futurelock detection:** Tests panic on futurelock (tasks stuck holding obligations
  without making progress). This catches shutdown wedges and leaked cleanup
  responsibility.
```

### 5.6 Rayon Bridge Contract

**Proposed addition to implementation_master.md Runtime section:**
```
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
```

---

## Part 6: Risk Assessment

### 6.1 asupersync Maturity

**Risk:** asupersync is v0.2.6, nightly-only, single developer.

**Mitigation:** Core crates remain runtime-agnostic. The asupersync dependency is confined to:
- `monkeybee` (facade)
- `monkeybee-bytes` (fetch scheduler)
- `monkeybee-proof` (proof harness)
- `monkeybee-cli`

If asupersync fails, only the orchestration layer needs replacement. Core rendering, parsing, writing, and editing are unaffected. The `ExecutionContext` bridge means we could swap to tokio/smol/custom with ~2000 lines of adapter code.

### 6.2 Nightly Rust Requirement

**Risk:** asupersync requires nightly Rust.

**Mitigation:** monkeybee already uses `edition = "2024"` which implies recent toolchain. The nightly features asupersync uses (likely `never_type`, GATs, etc.) are on stabilization paths. If necessary, we can pin a nightly version for CI.

### 6.3 Complexity Budget

**Risk:** Deep asupersync integration adds conceptual overhead.

**Mitigation:** The integration is confined to the orchestration layer. A contributor working on `monkeybee-render` sees `&ExecutionContext` and calls `exec_ctx.checkpoint.check()` — they never touch asupersync directly. The complexity is hidden behind the bridge.

---

## Part 7: Priority Order

If we adopt incrementally:

1. **ExecutionContext bridge + Outcome type** — Define the `CancellationCheckpoint` trait, adopt `Outcome<T, E>` for public APIs. This is the foundation everything else builds on.

2. **Session lifecycle as regions** — Model `engine.open()` and `snapshot.render_page()` as asupersync regions. This gives us structured lifecycle ownership immediately.

3. **LabRuntime proof harness** — Use LabRuntime for all concurrent tests. Add oracle assertions. This is the highest-leverage proof improvement.

4. **Rayon bridge contract** — Formalize the Rayon↔asupersync bridge with oneshot channels and cancellation propagation.

5. **Budget algebra** — Map our ExecutionContext budgets to asupersync Budget semiring with automatic child tightening.

6. **Progressive rendering with watch channels** — Use asupersync channels for tile completion in progressive rendering.

7. **Chaos testing + crashpacks** — Add chaos injection to proof harness for robustness testing.

---

## Appendix: Mega-Skill Surface Relevance Matrix

| asupersync Surface | Monkeybee Relevance | Notes |
|---|---|---|
| `Cx` + `Scope` | **Critical** | Core orchestration primitive |
| `Outcome<T, E>` | **Critical** | Public API return type |
| `Budget` semiring | **Critical** | Budget management |
| `LabRuntime` | **Critical** | Proof harness foundation |
| Regions/structured concurrency | **Critical** | Session/operation lifecycle |
| `channel::oneshot` | **High** | Rayon bridge |
| `channel::watch` | **High** | Progressive tile notifications |
| DPOR/oracle suite | **High** | Proof harness depth |
| Chaos injection | **High** | Robustness testing |
| Crashpacks | **High** | Failure reproduction |
| `bracket` combinator | **Medium** | Resource acquire/release |
| `map_reduce` combinator | **Medium** | Page-level parallel rendering |
| `pipeline` combinator | **Medium** | Decode chain orchestration |
| `bulkhead` combinator | **Medium** | Native decoder isolation |
| `timeout` combinator | **Medium** | Per-operation deadlines |
| Capability narrowing | **Medium** | Security profile enforcement |
| `actor` / `GenServer` | **Low** | Maybe for cache manager |
| `AppSpec` / supervision | **Low** | Maybe for long-running engine |
| `web` / `grpc` / `http` | **Not needed** | Monkeybee is a library |
| `database` | **Not needed** | |
| `remote` / `distributed` | **Not needed** | |
| `messaging` | **Not needed** | |
| QUIC / HTTP3 | **Not needed** | |
| RaptorQ | **Not needed** | |

---

## Appendix B: Supplementary Findings from Deep Research

Research agents read all 28 asupersync mega-skill reference documents. Key additional findings:

### B.1 Three-Lane Scheduler Architecture

asupersync's scheduler has three priority lanes: **cancel > timed > ready**. This maps perfectly to monkeybee's viewport-change cancel storms: when a user scrolls, cancellation cleanup for the old viewport gets scheduler priority over new tile rendering work. This is exactly right for progressive rendering responsiveness.

### B.2 Pipeline Combinator for Codec Decode Chains

The `pipeline` combinator provides staged transforms with explicit backpressure. Our codec decode chain (decompress → decrypt → decode → color convert) is a natural fit. The combinator is cancel-aware and budget-respecting, meaning a cancelled decode properly cleans up at each stage.

### B.3 Hedge Combinator for Tail-Latency Control

For documents with a mix of fast and slow pages (e.g., one page has a JPEG2000 image), the `hedge` combinator can start a backup rendering strategy after a timeout. Example: if JPEG2000 decode hasn't completed in 500ms, start a degraded-quality decode path. Loser drain is explicit.

### B.4 Progress Certificates for Session Close Monitoring

asupersync tracks cancellation drain progress with phase labels: `warmup → rapid_drain → slow_tail → stalled → quiescent`. This maps to session close monitoring: when closing a session with many active operations, progress certificates tell us whether drain is progressing normally or stuck. Uses Freedman/Azuma confidence bounds for statistical guarantees.

### B.5 TLA+ Export for Critical Invariants

The trace infrastructure can export execution traces as TLA+ behaviors for bounded model checking. This could be used to verify critical monkeybee invariants:
- WritePlan signature-safe preservation
- Crash-safe save atomicity
- Cache consistency under concurrent access
- EditTransaction snapshot isolation

### B.6 Spectral Health Monitor

asupersync's spectral health monitor provides early-warning severity levels: `none → watch → warning → critical`. This could monitor cache contention (DashMap hotspots), decode pipeline backpressure, and native decoder quarantine health.

### B.7 Error Taxonomy Mapping

asupersync's `ErrorKind` includes: `Cancelled`, `Timeout`, `BudgetExhausted`, `ObligationLeak`, `RegionCloseTimeout`, `FuturelockViolation`, `ChannelClosed`, `LockPoisoned`, `IoError`. Each has a `Recoverability` field (`Recoverable`, `NonRecoverable`, `Unknown`) and suggested `RecoveryAction` (`Retry`, `RetryWithBackoff`, `Abort`, `Escalate`). Our diagnostic error model could adopt this taxonomy.

### B.8 ContendedMutex for Cache Hot-Path Auditing

`ContendedMutex` wraps `parking_lot::Mutex` with optional lock-metrics instrumentation (wait/hold time tracking). For our DashMap-based caches, using `ContendedMutex` on shard locks would provide empirical evidence about cache contention, informing shard count and eviction policy decisions.

### B.9 Virtual Time for Fast Proof Runs

LabRuntime's virtual time wheel completes sleeps instantly. For proof harness runs that involve timeouts, deadlines, and progressive rendering waits, this means the entire proof suite runs at scheduler speed with no wall-clock delays. This dramatically accelerates CI proof runs.

### B.10 Two-Phase Send Across ALL Channel Types

Not just oneshot — ALL channel types (mpsc, oneshot, broadcast) use the reserve/commit pattern. This means any inter-component communication in monkeybee (e.g., fetch scheduler → byte source, render scheduler → tile worker) gets cancel-safety for free.

### B.11 GenServer for CacheManager

The agents specifically flagged that our `CacheManager` (currently `DashMap`-based shared state) is a candidate for `GenServer` promotion. A `GenServer`-backed cache would have:
- Single-owner state (no concurrent mutation bugs)
- Reply obligations (caller knows cache lookup completed)
- Bounded mailbox (backpressure under cache storm)
- Explicit stop semantics (clean eviction on shutdown)
- Deterministic testing under LabRuntime

This is a "maybe later" optimization but worth noting in the architecture.
