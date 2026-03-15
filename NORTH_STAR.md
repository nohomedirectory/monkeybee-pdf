# Monkeybee-pdf — North Star

## Mission

Monkeybee-pdf is an open-source, memory-safe, high-performance Rust PDF engine for ugly real-world PDFs.

Its purpose is not to prove that a narrow subset of the PDF specification can be parsed elegantly. Its purpose is to become a genuinely formidable document engine: one that can ingest hostile and malformed files, accurately render them, surface useful structure from them, mutate them in disciplined ways, generate and emit valid documents, and survive repeated round trips without collapsing into corruption, hand-waving, or roadmap theater.

This project exists to make a much stronger claim than "we have a renderer" or "we have a parser" or "we have editing plans." The claim is that Monkeybee-pdf is being built to behave like a real, serious, bidirectional PDF engine that owns document reality rather than merely sampling it.

That is the north star.

## What this document is

This document is a constitutional thesis for the project.

It is not an implementation roadmap, not a milestone schedule, not a bead graph, and not an execution appendix. It defines the project's identity, ambition, constraints, architectural doctrine, and proof standard. Its job is to set the attractor that later planning, conversion, refinement, and agent swarming must converge toward, without shrinking the ambition or blurring the thesis.

## The central thesis

Monkeybee-pdf is a closed-loop PDF engine.

The essential loop is:

**open -> understand -> render -> inspect/extract -> annotate/edit -> save/generate -> reopen -> validate**

That loop is the core idea. Not "more features." Not "a broad roadmap." Not "PDF-adjacent capabilities." A single closed-loop claim.

The loop is how the thesis manifests. The deeper proposition is that Monkeybee-pdf owns enough of document reality to operate across that loop without hand-waving, purity evasion, or one-way lossy collapse.

Rendering, extraction, inspection, annotation, mutation, generation, serialization, reload, and validation are not separate ambitions bolted together to inflate scope. They are the mutually reinforcing surfaces of one deeper proposition: Monkeybee-pdf owns enough of document reality to operate in both directions without pretending that bidirectionality is a future rescue release.

## Why this is the right ambition

The correct response to the agent era is not to shrink the thesis. It is to sharpen it.

When implementation throughput is exploding, the limiting factor is no longer primarily "how much code can be typed." It is whether the project is aimed at something worthy, coherent, and difficult to dismiss. Smallness is not seriousness. Narrowness is not rigor. A project can be extremely large while remaining extremely crisp if it is organized around one dominant thesis and one evidence standard.

For Monkeybee-pdf, a renderer-only or parser-only public thesis is too weak. The right north star is not a timid MVP. It is a monolithic engine thesis with a hard reality standard.

## Scope lock

Monkeybee-pdf is a serious, open-source, high-performance, memory-safe Rust PDF engine for ugly real-world PDFs that can read, understand, render, inspect, extract, annotate, edit, generate, serialize, save, reopen, and validate documents under an explicit compatibility-and-safety doctrine, with reality established by automated pathological-corpus, reference-guided, and round-trip proof rather than by screenshots or roadmap rhetoric.

Everything in v1 must strengthen that statement.

Anything that does not strengthen that statement is, by default, suspect.

## The actual target

The target is not the clean subset of the PDF specification that makes engineering feel elegant.

The target is the long tail of real PDFs in the wild: malformed cross-reference structures, broken object graphs, historical baggage, weird producer quirks, bad font metadata, broken encodings, incremental-update oddities, scanned documents, multilingual documents, form-heavy files, vector-heavy files, huge files, hostile inputs, and the classes of legacy nonsense that still matter because Acrobat can cope with them and users expect reality, not purity.

Monkeybee-pdf should not dismiss hard categories with slogan-level declarations about deprecation or hostility. It should investigate them with a bias toward safe, clever, contained handling wherever possible. Where they cannot yet be supported natively and sanely, the system must detect them explicitly and degrade in principled, instrumented, non-silent ways. The goal is not purity. The goal is maximum real-world usefulness consistent with Rust's safety model.

## What Monkeybee-pdf must already be

### 1. A real renderer

Monkeybee-pdf must produce trustworthy visual output on ugly real-world PDFs.

That means it must genuinely resolve document structure, page inheritance, resources, content streams, transforms, clipping, text state, fonts, encodings, raster images, vector graphics, masks, transparency, blending, and the rest of the visual behavior required for the project to be judged by rendered output on hard documents rather than by parser trivia. A successful parse that yields visually untrustworthy output is failure, not partial success.

The benchmark is reality: if the file is supposed to render and users care about how it renders, Monkeybee-pdf is in the business of rendering it correctly.

### 2. A real document understanding core

Monkeybee-pdf must not treat the document as a disposable stream of rendering commands.

Its core must model the document honestly as a document: objects, references, cross-reference structures, trailers, inherited state, pages, resources, fonts, images, annotations, updates, metadata, content streams, and document-level structures must exist in reusable forms that later subsystems can consume directly.

### 3. A real write path

Monkeybee-pdf must not be read-only.

It must be able to emit valid PDFs, both by generating new documents and by serializing principled modifications to existing ones. It does not need a giant desktop-publishing abstraction layer. It does need the ability to construct pages, resources, and objects coherently; write structurally valid output; preserve or reconstruct enough correctness that emitted files are not toy artifacts; and save edits without turning the file into corruption bait. Full rewrite and incremental-save postures should both remain architecturally available where they materially improve correctness or usability.

### 4. A real mutation substrate

Monkeybee-pdf must make disciplined mutation believable.

The system should be able to load an existing document, inspect it, change it, and emit the changed result without the whole thing becoming a fragile science experiment. That includes common structural edits, page and resource manipulation, content-stream and reference updates where practical, metadata changes, and document-level structural work.

The point is not to instantly become full Acrobat. The point is to prove that the internal model is a real substrate for editing rather than a dead-end parse tree.

### 5. Annotation as a flagship proof point

Annotation deserves special elevation.

It sits between rendering and arbitrary editing. It requires geometry understanding, page-state understanding, resource discipline, writeback correctness, and round-trip integrity, while staying more constrained than unrestricted full-document mutation. That makes it one of the sharpest visible proofs that the closed-loop thesis is real.

### 6. A serious inspection and extraction layer

A real engine does not only display documents; it surfaces useful structure from them.

Monkeybee-pdf must therefore expose meaningful extraction and inspection surfaces: text extraction with positions where feasible, metadata and structure inspection, page/resource/object introspection, sensible asset extraction, and diagnostics about unsupported or degraded regions. The target is usefulness, not magical perfect semantic recovery from every hostile PDF ever made. But it must clearly be a foundation other tools can build on.

### 7. Performance as identity, not cleanup

Monkeybee-pdf must already be plainly fast.

Performance is not a post-v1 garnish. Efficient parsing, disciplined allocation behavior, strong hot-path engineering, judicious caching, and performance work on representative real workloads belong inside the definition of seriousness from the start.

### 8. Memory safety as constitutional identity

Monkeybee-pdf handles malformed and potentially hostile input.

Memory safety is therefore not just an implementation convenience. It is part of the product's public identity and one of the reasons the project deserves to exist. Unsafe code cannot become the default escape hatch for compatibility pressure. If any unsafe usage is truly unavoidable, it must be minimal, explicit, justified, and surrounded by aggressive scrutiny and testing.

## Compatibility doctrine

Monkeybee-pdf should adopt an explicit, unapologetic compatibility doctrine.

Not vague ambition. Not a promise that "we care about hard files." A doctrine.

### Tier 1: full native support

If a feature can be supported safely and sanely within the architecture, it should be implemented directly.

### Tier 2: safe contained handling

If native support is not yet practical, but partial, sandboxed, constrained, or otherwise contained handling is possible without violating the safety model or polluting the architecture, that route should be explored.

### Tier 3: explicit detected degradation

If support is not yet feasible, the engine should detect the situation explicitly, surface it to diagnostics and compatibility accounting, and fail or degrade in principled, instrumented ways. Silent evasion is unacceptable.

## Representative hard classes that must stay in view

The project must maintain explicit awareness of representative ugly and historically gnarly categories, including but not limited to:

- XFA and hybrid forms
- RichMedia / Flash-era embedded-interactivity baggage and similar historical traps
- PostScript XObjects and other legacy rendering hazards
- broken or strange font and encoding situations
- malformed cross-reference tables and streams
- incremental-update oddities
- encryption and permission edge cases where handling is appropriate
- transparency, masks, and blend edge cases
- scanned, multilingual, and producer-quirk-heavy documents

These categories are not to be used as excuses for purity speeches. They are to be treated as concrete truth surfaces against which the engine's seriousness is measured.

## Architectural doctrine

Monkeybee-pdf should be designed around several non-negotiable architectural principles.

### Native architecture ownership

Existing implementations may be used as semantic and behavioral references, not as architectural authorities. The project should import obligations from reality while retaining freedom to design a native architecture optimized for Monkeybee-pdf's own thesis: memory safety, high performance, honest bidirectionality, compatibility under explicit doctrine, and proof-oriented engineering.

### Reusable document core

The document model must be genuinely reusable across rendering, inspection, extraction, mutation, annotation, and writeback. No dead-end internal structures that only serve one subsystem and force later duplication.

### Explicit object graph and updates

Object identity, cross-reference structures, inherited state, and incremental updates must be modeled explicitly rather than hidden inside parser side effects.

### Canonicalization without destructive loss

The engine should normalize enough to make implementation sane, but not so much that it destroys provenance, geometry, structure, or writeback-relevant information needed later.

### Separation of lanes

Parsing, validation, canonicalization, rendering, extraction, mutation, serialization, and proof machinery should be separable enough to remain testable, hardenable, and evolvable.

### Mutation-friendly representations

Content and resource models should preserve enough structure that disciplined mutation and re-serialization are practical, rather than forcing the system into one-way lossy collapse.

### Shared geometry and page-state understanding

Rendering, annotation placement, extraction with positions, and common edits should all draw from shared geometry and page-state understanding, not duplicated and drifting subsystem-specific logic.

### Isolated compatibility shims

Producer-specific weirdness, malformed-input repairs, and legacy-handling quirks should be isolated and labeled rather than smeared throughout the codebase.

### Testability and observability

The system should produce useful evidence: output diffs, failure categories, corpus metadata, reproduction paths, round-trip traces, and compatibility status.

## Proof doctrine

Monkeybee-pdf must be difficult to dismiss.

That means the proof machinery is part of the deliverable, not an afterthought.

The project should be backed by a fully automated pathological corpus and end-to-end harness that continuously exercises the engine against ugly real files. That corpus should span scanned PDFs, form-heavy documents, encrypted cases where handling is appropriate, embedded fonts and broken metadata, transparency and blend edge cases, CJK and right-to-left documents, very large files, malformed and adversarial inputs, complex vector art, and files from many different producers and print pipelines. The harness must include not only render checks, but round trips such as load -> render -> modify -> save -> reload -> render again, load -> extract -> mutate -> save -> compare structural expectations, generate -> render under Monkeybee-pdf and reference implementations, and annotation/editing round trips on representative documents.

Where correctness is ambiguous, reality should be consulted through serious existing implementations used as behavioral references. That does not mean copying their architecture. It means refusing wishful thinking.

## Public-library doctrine

By the time Monkeybee-pdf first publicly claims engine reality, it should already feel adoptable.

It does not need perfect ergonomics in every corner, but it should already expose disciplined library surfaces for opening documents, inspecting structure, rendering pages, extracting useful information, making common mutations and annotations, generating new documents, saving results, and surfacing diagnostics clearly.

## What is not the goal

Monkeybee-pdf is not trying to become "everything vaguely adjacent to PDFs."

It is not, at this stage, trying to be a polished full desktop PDF suite, a giant layout-authoring DSL, perfect semantic recovery from every hostile file, or total commercial-suite parity across every obscure niche workflow. The project's job is to establish serious bidirectional engine reality and a broader platform foundation, not to finish the entire category in one stroke.

## Style of ambition

The ambition here must remain maximal inside the thesis and ruthless outside it.

Monkeybee-pdf should not become timid because some classes are hard. It should also not become incoherent because agents can now generate enormous quantities of code. The proper response to massive implementation throughput is not decorative sprawl. It is deeper coherence.

"Alien artifact" in this domain does not mean arbitrary exotic cleverness. It means that the system feels disturbingly complete and internally inevitable: a memory-safe Rust engine that treats ugly real-world PDFs as a bidirectional semantic substrate rather than a one-way rendering stream; that attacks the compatibility tail instead of hiding from it; that owns its architecture rather than inheriting donor internals; and that proves itself with automated, repeatable evidence instead of charisma.

Monkeybee-pdf is not required to be mathematically exotic for its own sake. It is required to be as sophisticated as necessary. If unusually strong geometric, algorithmic, analytical, probabilistic, or formal techniques materially improve compatibility, correctness, extraction quality, safety, or performance, the project should be willing to use them. The goal is not decorative cleverness. The goal is deeper ownership of document reality.

## Alien artifact doctrine

Monkeybee-pdf should aspire to feel like an alien artifact in the only sense that matters for this domain.

That means a disturbing combination of breadth, depth, coherence, and proof. It means the renderer, document model, extraction layer, mutation substrate, generation path, compatibility handling, and proof machinery all feel like manifestations of one underlying idea rather than a pile of features. It means the architecture feels internally inevitable once understood.

An alien-artifact PDF engine is not merely broad. Many systems are broad in sloppy ways. It is broad and deep: willing to use unusually strong methods wherever they materially improve its grip on document reality. The result should be a system that outsiders can inspect skeptically and still come away with the feeling that it is real, serious, overdetermined, and difficult to dismiss.

## Visible proof and wow-factor doctrine

Monkeybee-pdf should not rely on rhetoric, screenshots, or vague roadmap promises to establish seriousness. By the time it first publicly claims engine reality, it should already have visible proof surfaces that make the thesis legible to outsiders.

Those surfaces should include, at minimum:

- rendering hard and pathological PDFs that simpler engines mishandle or evade
- annotation round trips on ugly real-world files without corruption or silent geometry drift
- disciplined edits and save/reopen cycles that preserve document viability
- extraction and inspection outputs that expose useful structure, positions, resources, and diagnostics
- generated documents that render correctly under Monkeybee-pdf and serious external references
- compatibility accounting that makes unsupported or degraded zones explicit rather than mysterious

The wow factor should come from undeniable evidence of owned document reality, not from marketing language.

## Final constitutional statement

Monkeybee-pdf is not a parser demo, not a clean-subset toy, not a renderer with editing dreams, and not a vague bag of PDF-adjacent aspirations.

Monkeybee-pdf exists to become the world's most formidable open-source, memory-safe, high-performance Rust engine for ugly real-world PDFs: a bidirectional, closed-loop, evidence-first document engine that can read, understand, render, inspect, extract, annotate, edit, generate, serialize, save, reopen, and validate documents while confronting the hostile compatibility tail that still matters in practice because users judge reality, not purity.

Its architecture must be natively owned rather than donor-inherited. Its compatibility doctrine must prefer full support where sane, clever contained handling where necessary, and explicit instrumented degradation where support is not yet feasible. Its internal document model must be reusable enough to power rendering, extraction, annotation, editing, generation, and proof without lossy collapse. Its performance must be obvious on representative hard workloads. Its memory safety must remain part of its public identity. Its proof must be automated, pathological-corpus-backed, round-trip-grounded, and externally legible.

If Monkeybee-pdf feels like an alien artifact, that should be because it combines breadth, depth, coherence, and evidence so completely that the only honest description is that it owns document reality end to end.

That is the north star.
