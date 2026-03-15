# APR Round 1 — GPT-5.4-pro Feedback

This is already a very strong plan. The parts I would keep intact are the closed-loop thesis, the Tier 1/2/3 compatibility doctrine, the proof-first release posture, and the shared geometry/content-pipeline instinct. The biggest improvements I'd make are about boundary clarity, v1 honesty, and operational hardening.

The two core risks in the current draft are:

1. `monkeybee-core` is in danger of becoming a god-crate that tries to simultaneously be raw-byte storage, repaired syntax, semantic document graph, and runtime page state.
2. v1 currently mixes "must ship" capability with "research-grade differentiator" capability in a way that can trap the project in permanent pre-release refinement.

Below are the 10 revisions I'd actually patch in.

### 1. Split the core into explicit byte, syntax, document, and content layers

The README's "single engine over a shared document core" is the right idea, but the current `monkeybee-core` responsibility set is too broad. Preserve mode, tolerant repair, strict conformance, rendering, and writeback all want different representations of "the document." If one layer tries to do all of them, it usually becomes either lossy or impossible to reason about.

I would make the architecture explicitly layered: a byte/revision layer, a syntax-preserving parse layer, a semantic document layer, and a shared content-execution layer. That makes preserve mode real instead of aspirational, gives repairs provenance, and keeps rendering/extraction/editing from depending directly on raw syntax.

### 2. Add transactional editing and ownership classes; move optimization into explicit edit operations

The current change-tracking model is too weak for the edits this engine wants to support. Page reorder, annotation flattening, metadata edits, resource dedup, and redaction are not "replace one object value" operations; they are graph edits with closure, validation, and rollback semantics.

I'd introduce `EditTransaction` plus explicit ownership classes: `Owned`, `ForeignPreserved`, and `OpaqueUnsupported`. That gives preserve mode a credible semantic boundary: the engine knows what it may canonicalize, what it must splice forward byte-preservingly, and what it must refuse to transform. I would also make optimization a first-class edit operation, not something the writer silently does.

### 3. Add a cached `PagePlan` IR on top of the shared interpreter

The shared event model is a great foundation, but event-only is not enough. Repeated workflows like render + extract + inspect + proof diff + annotate are going to re-interpret the same content streams over and over. Some operations also want region-aware introspection, not just linear event replay.

I'd keep the event pipeline as the source of truth, but add a derived immutable page IR — `PagePlan` — that caches normalized draw ops, text runs/quads, dependencies, and degradation markers. That makes repeated passes much faster and makes page-local editing and diagnostics much cleaner.

### 4. Add `ExecutionContext` with budgets, cancellation, provider registry, and tracing

Right now resource limits, determinism, and system dependencies are described, but not unified. I'd thread one `ExecutionContext` through every top-level API. That becomes the place for budgets, cancellation, determinism, providers, and structured tracing.

This solves several problems at once: bounded work, service-friendliness, reproducible proof runs, and the current system-font nondeterminism issue. Proof and CI should never depend on whatever fonts or external tools happen to be installed on the machine.

### 5. Separate v1 delivery from R&D backends

This is the biggest schedule/credibility change. The current spec has brilliant advanced ideas, but several of them are themselves substantial research/engineering programs. If those remain psychologically part of "true v1," the project risks becoming permanently pre-release.

I'd keep every one of those ideas, but I would force them behind pluggable backends with correct baseline implementations. That preserves the ambition while making it possible to actually ship the closed loop first.

### 6. Add a real security doctrine, not just memory-safety rules

Rust memory safety is necessary, but it is not enough for a hostile-file engine. Complexity bombs, decoder bugs in native bridges, and risky formats are still real problems. PDFs have historically been a nasty attack surface precisely because "it parses" and "it is safe to run everywhere" are different claims.

I'd add explicit security profiles and isolate or disable risky subsystems in hardened mode. That makes the engine much more deployable in servers, browsers, and enterprise environments.

### 7. Fix the signature and profile-scope contradictions

There are two places where the current plan over-promises relative to its own text. The first is signatures: Workflow 7 implies post-modification validation, while the AcroForm section explicitly says full crypto verification is not in v1. The second is profile support: `pdf-ua2` is advertised even though accessibility remediation is an anti-goal for v1.

Those should be fixed now, before code and messaging drift around them.

### 8. Treat redaction as high-assurance rewriting, with safe fallback modes

Redaction is the place where optimism is dangerous. The current intent is right, but "remove all content underneath the rectangle" is much harder than it sounds once images, reused XObjects, transparency groups, and partial overlap enter the picture.

I would move redaction application out of pure annotation logic and make it a high-assurance edit operation. When exact semantic removal can be proved, do it. Otherwise use a secure raster surrogate and record the tradeoff. In redaction, safety beats prettiness.

### 9. Make generated text actually multilingual-correct

The current generation API is fine for simple Latin output, but it is not enough for real multilingual documents. `show_text(string)` cannot just mean "emit bytes" if the engine claims useful generation, CJK/RTL support, and modern PDF output.

The fix is not a full layout DSL. It is a shaping layer: bidi, shaping, line breaking, and font fallback for generated text. Keep the low-level API too, but make the high-level API correct by default.

### 10. Make proof reproducible, legally sustainable, and machine-consumable

The proof doctrine is excellent. What it needs is operational reality. Real corpora are often partly non-redistributable. External renderer outputs shift by version. Machine-readable outputs need versioned schemas if anyone is going to build tooling on top of them.

I'd explicitly formalize corpus classes, pin oracle manifests, version the compatibility ledger, add page/region scope, and add metamorphic and writer fuzzing. That turns "proof culture" into something other people can actually trust and reproduce.

---

The implementation order I'd use after making these changes is:

1. layered architecture,
2. transactional edit/validate boundaries,
3. execution context and security profiles,
4. v1 vs experimental split,
5. signature/redaction truthfulness,
6. only then the more exotic rendering and proof upgrades.

That keeps the project's identity intact while making it much more likely to become the thing the README promises.
