# monkeybee-pdf

Memory-safe Rust PDF engine. Generator, renderer, and the first open-source interactive 3D PDF viewer. A clean-room alien artifact.

---

## What This Is

monkeybee-pdf is not a port. It is not an incremental improvement on existing PDF libraries. It is a from-scratch, memory-safe Rust implementation of a PDF generator and renderer targeting ISO 32000-2:2020 (PDF 2.0), designed to be a **drop-in replacement that is dramatically better** than the C/C++ incumbents it replaces.

The PDF ecosystem has a gap that shouldn't still exist: the dominant implementations are C/C++ codebases carrying decades of CVEs (Poppler, MuPDF, PDFium) or managed-language libraries that trade safety for painful performance. monkeybee-pdf closes that gap — not by porting those implementations into Rust, but by reimagining the problem space from first principles using mathematically rigorous algorithms, formal verification approaches, and extreme optimization techniques that human engineering teams would not typically apply to PDF processing.

This is an **alien artifact**: software so complex, so optimized, and so correct that it would ordinarily require large teams of skilled engineers years — possibly decades — to produce. It is being built entirely by AI coding agents in days, and it is designed to communicate just how weird and absurdly capable that process is becoming.

## North Star

> We are creating the world's best, most performant memory-safe Rust implementation of a PDF generator/renderer — an alien artifact that demonstrates what AI-agent-driven engineering can produce when paired with rigorous planning and extreme ambition.

This is the project's invariant. Every design decision must serve it. Work that doesn't lead toward it is unfocused and gets cut.

## What Makes This an Alien Artifact

The term comes from the observation that AI agents can now produce software artifacts that *look like they came from somewhere else* — too complex, too optimized, too thorough to have been built by a small team in a short time. FrankenTUI demonstrated this by applying over 30 specialized algorithms (Bayesian statistics, advanced mathematical methods) to problems that TUI frameworks typically solve with heuristics or ignore entirely. monkeybee-pdf applies the same philosophy to PDF:

**Mathematically rigorous, not just cleverly engineered.** Every hot path in PDF processing — color space conversion, transparency compositing, path rasterization, font hinting, compression selection, shading interpolation — is an optimization problem with a mathematical structure. We don't settle for "good enough" heuristics where exact or provably-optimal solutions exist. We delegate the search for applicable cutting-edge and esoteric mathematical research to AI agents during the planning phase, then implement the results.

Concrete examples of where this applies:

- **Color conversion** — ICC profile transforms are called millions of times per page. High-precision LUT generation, SIMD-accelerated tetrahedral interpolation, cache-optimized transform chains. Not the naive matrix multiply that most implementations use.
- **Transparency compositing** — The PDF transparency model (ISO 32000-2 Clause 11) is the single hardest rendering problem in the spec. Isolated groups, knockout groups, luminosity masks, 16 blend modes, shape-vs-opacity tracking, color space conversion at blend boundaries. Most renderers get edge cases wrong. We target mathematical correctness with tile-based compositing and intelligent buffer pooling for performance.
- **Path rasterization** — Exact-area antialiasing with adaptive Bézier flattening, not the approximations that accumulate visible error at high zoom. SDF-based clipping for hierarchical clip regions.
- **Font rendering** — Sub-pixel positioning, hinting interpretation, gamma-correct antialiasing, glyph cache architecture designed for cache-line efficiency. The most visible quality indicator to end users.
- **Compression pipeline** — Per-stream optimal filter selection by content analysis. Parallel compression. Advanced Flate strategies (optimal lazy matching, custom dictionaries, PNG predictor selection). File size is a measurable, benchmarkable metric and we intend to win on it.
- **Shading interpolation** — Mesh shadings (Types 4–7) involve solving interpolation over arbitrary Coons and tensor-product patch meshes. Adaptive subdivision with GPU-friendly tessellation, not brute-force sampling.
- **Graphics state management** — The state stack is the hot path for every rendering operation. Lock-free, cache-friendly state management. Predictive state diffing: know what changed between saves/restores without comparing every field.

**Extreme optimization as a first-class concern.** Profile-driven performance optimization is not a polish step — it is part of the architecture from day one. Every crate is designed with SIMD vectorization, cache locality, zero-copy parsing, and allocation minimization as structural constraints, not afterthoughts.

**3D interactive rendering — a capability no open-source implementation has.** No open-source PDF renderer supports interactive 3D content. Not Poppler. Not MuPDF. Not PDFium. Not pdf.js. Only Adobe Acrobat (proprietary) renders 3D PDFs interactively. monkeybee-pdf will be the first, parsing PRC (ISO 14739-1:2014) and U3D (ECMA-363) data streams and rendering them via wgpu (Vulkan/Metal/DX12 native, WebGPU in browser). This is a V1 deliverable, not a future aspiration.

## Project Origin

This project was inspired by [a post from Gary Basin](https://x.com/garybasin/status/2020643166047928494) identifying high-leverage software projects suitable for AI-agent-driven development:

> *"A clean-room PDF renderer/generator. PDF is the de facto document interchange format, and the existing implementations (Poppler, MuPDF, the various Java/Python libraries) are all either C/C++ with decades of CVEs or painfully slow. A correct, memory-safe, fast PDF engine would be used everywhere from browsers to enterprise document pipelines."*

The critical insight, as articulated in the broader discussion: **it would be a wasted opportunity to simply port existing software as-is but memory-safe when we can replace them with alien artifacts that are drop-in replacements but much better.**

Jeffrey ([@doodlestein](https://github.com/doodlestein)) identified the PDF engine as a strong candidate and advised Joey ([@nohomedirectory](https://github.com/nohomedirectory)) to take it on. The entire project — planning, specification, implementation — is being built with AI coding agents. FrankenTUI, Jeffrey's prior alien artifact, demonstrated the viability: a TUI engine churned out by an agent swarm in days that appears more efficient and feature-rich than established open-source alternatives with thousands of human dev hours invested. monkeybee-pdf aims to repeat that at a larger scale and with a harder problem.

## Architecture

monkeybee-pdf is a Cargo workspace organized as a flat crate structure under `crates/`, following the pattern established by [FrankenTUI](https://github.com/Dicklesworthstone/frankentui):

```
monkeybee-pdf/
├── Cargo.toml              # Workspace root (virtual manifest)
├── Cargo.lock
├── .beads/                 # Bead definitions (task DAG for agent work)
├── crates/
│   ├── monkeybee-core/     # Object model, types, shared primitives
│   ├── monkeybee-parse/    # PDF reader: tokenizer, xref, object resolution
│   ├── monkeybee-gen/      # PDF writer: serialization, compression, conformance
│   ├── monkeybee-render/   # Page rendering: graphics state, compositing, rasterization
│   └── monkeybee-3d/       # 3D content: PRC/U3D parsing + rendering via wgpu
├── docs/
│   ├── SPEC.md             # Full technical specification (all feature domains)
│   └── IMPLEMENTATION.md   # Architecture, build order, infrastructure decisions
├── tests/
├── fuzz/
├── scripts/
├── AGENTS.md
├── README.md
└── Makefile
```

### Crate Responsibilities

**monkeybee-core** — The foundation. PDF object types (Boolean, Integer, Real, String, Name, Array, Dictionary, Stream, Null, indirect references). Coordinate geometry. Color space definitions. Operator enums. Error types. Shared traits. This crate has no I/O and no dependencies on parsing or rendering strategy. `#![forbid(unsafe_code)]`.

**monkeybee-parse** — Reads PDF files. Tokenizer, cross-reference table/stream parsing, object stream decompression, incremental update chain following, stream filter decoding (Flate, LZW, DCT, JPX, JBIG2, CCITTFax, ASCII85, ASCIIHex, RunLength, Crypt). Three explicit modes:
- *Strict* — Spec-pedantic. Reject malformed structures deterministically.
- *Tolerant* — Real-world survival. Recover from malformed PDFs. Never panic, never unbounded allocate. Emit structured diagnostics.
- *Preserve* — Byte-preserving. Signature-safe. Don't rewrite bytes you don't own.

Normalizes legacy PDF versions (1.4–1.7) into the 2.0-native core object model.

**monkeybee-gen** — Writes PDF files. Object serialization, stream compression with optimal filter selection, cross-reference stream emission, incremental update appending, conformance profile validation gates (PDF/A-4, PDF/UA-2, PDF/X-6 as initial targets). Deterministic write mode for reproducible output. Font embedding and subsetting (CFF and TrueType, including complex scripts). ToUnicode CMap generation. Tagged PDF generation for accessibility.

**monkeybee-render** — Turns parsed PDF pages into pixels. Full content stream interpretation, complete graphics state stack, path construction and exact-area rasterization, text shaping (via harfbuzz or equivalent) and glyph rendering, image decoding and high-quality resampling, all color space conversions (Device, CIE-based, ICC, Indexed, Pattern, Separation, DeviceN), full transparency compositing model (all 16 blend modes, isolated/knockout groups, soft masks), pattern tiling, all 7 shading types, annotation rendering. CPU software renderer as primary backend (SIMD-optimized). WASM compilation target for browser demo.

**monkeybee-3d** — Parses PRC (ISO 14739-1:2014) and U3D (ECMA-363) 3D data streams embedded in PDF 3D and RichMedia annotations. Builds a unified scene graph from either format. Renders via wgpu: Vulkan/Metal/DX12 on desktop, WebGPU/WebGL2 in browser. Supports named views, lighting schemes, rendering modes (solid/wireframe/transparent/illustration), cross-sections, product structure navigation. The alien artifact differentiator.

## Key Design Decisions

**PDF 2.0 native.** Generator emits `%PDF-2.0` by default. Internal object model is 2.0-native. Parser accepts 1.4+ and normalizes legacy upward.

**No XFA, no Flash, no PostScript XObjects.** Deprecated, security-hostile legacy features. We do not implement them.

**Rendering correctness by oracle consensus.** Rendering output is validated against PDFium (primary), MuPDF (secondary), pdf.js (tertiary cross-check), and Ghostscript (strict-mode canary). Consensus rules determine correctness, not any single engine. Divergence cases become permanent regression tests with spec citations. See SPEC.md for the full arbitration policy.

**Arlington-driven code generation.** The PDF Association's Arlington PDF Model (machine-readable TSVs defining every PDF dictionary type) is used as input data for automated generation of schema validators, typed accessors, negative test scaffolds, and documentation stubs. This is a mechanical accelerant for correctness coverage.

**3D in V1.** Interactive 3D rendering is a V1 deliverable. It is the single capability that separates monkeybee-pdf from every other open-source PDF implementation in existence.

**Mathematical rigor over heuristics.** Where a problem has mathematical structure, we find and implement the provably correct or optimal solution. We delegate the search for applicable research to AI agents. See the "What Makes This an Alien Artifact" section above for specific examples.

## Showcase Binaries

Two frontend binaries implement the library crates:

**CLI tool** (`monkeybee-cli`) — Render PDFs to images (PNG, JPEG). Extract text. Validate PDF structure (Arlington-driven). Fill forms. Inspect metadata. Merge, split, sign, encrypt, linearize. Render 3D content to images.

**WASM demo** (`monkeybee-wasm`) — Browser-based PDF viewer. Page navigation, zoom/pan, text selection, form interaction, performance metrics. Interactive 3D content rendered via WebGPU — the first open-source PDF viewer to do this in a browser.

## Truth Layer

The specification is governed by a hierarchy of normative sources:

- **Core baseline:** ISO 32000-2:2020 (PDF 2.0), pinned to Errata Collection 2 (through July 2024)
- **Normative supplements:** ISO/TS 32001 (hash algorithms), 32002 (digital signatures), 32003 (AES-GCM encryption), 32004 (integrity protection), 32005 (structure namespaces)
- **3D format specs:** ECMA-363 4th Edition (U3D format, free from ECMA International) and ISO 14739-1:2014 (PRC format — primary implementation reference drawn from open-source implementations and published format documentation)
- **Accessibility guidance:** Well-Tagged PDF (WTPDF) — implementation reference for PDF/UA-2 conformance
- **Living delta:** PDF Association pdf-issues corpus, periodically snapshotted into tests
- **Schema truth:** Arlington PDF Model TSVs (pinned to a specific git commit), used as input data for code generation and structural validation
- **Policy:** We do not commit ISO spec PDFs or copyrighted text into this repository. During development, agents access normative spec text through a queryable spec oracle (RLM-based) that enables cited, verifiable lookups against the full specification corpus without context window contamination. Spec ambiguities identified during planning are resolved against the source text during implementation.

## Reference Libraries

Used during planning and bead creation — not as truth sources:

- **[LibPDF](https://github.com/LibPDF-js/core) (TypeScript)** — Primary planning reference. Full feature-space map: parsing, generation, incremental saves, digital signatures (PAdES B-B through B-LTA), encryption, form filling/flattening, lenient recovery parsing. Built by Documenso for production use.
- **[krilla](https://github.com/LaurenzV/krilla) (Rust)** — Rust architectural and testing methodology reference. Generation-only, but its multi-viewer regression testing discipline (6 PDF viewers in CI), `#![forbid(unsafe_code)]` stance, and excellent font subsetting directly inform our approach.

## Development Methodology

This project uses a three-phase process:

1. **Planning** — Define the project. Make architectural decisions. Draft README, SPEC, and IMPLEMENTATION documents. Iterate all three using [Automated Plan Reviser Pro](https://github.com/Dicklesworthstone/automated_plan_reviser_pro) to converge through multi-round AI review (GPT Pro Extended Reasoning). The three documents are revised together — the README and IMPLEMENTATION are updated alongside the SPEC every few rounds to surface coupling problems early and prevent revision shock. This mirrors APR's core insight: each round is a perturbation in an optimization process, and mirroring changes across all documents as you go reduces the shock of applying N revisions at once.

2. **Bead Conversion** — Decompose the converged specification into atomic work units (beads) organized as a dependency DAG in `.beads/`. Each bead is a self-contained task an AI agent can execute independently. The bead graph encodes ordering, dependencies, and parallelism.

3. **Implementation** — AI agents execute beads, guided by `AGENTS.md` and the bead DAG. Agents have access to structured tooling: a spec oracle (RLM-based, providing cited lookups against ISO 32000-2 and all governing specs without context contamination), the Arlington PDF Model (direct programmatic queries against machine-readable object schemas), and methodology skills (alien artifact optimization patterns, FrankenTUI structural conventions). CI validates correctness continuously via oracle consensus rendering comparisons, Arlington structural validation, and veraPDF conformance checks.

The repository structure and methodology follow the pattern established by [FrankenTUI](https://github.com/Dicklesworthstone/frankentui), which demonstrated agent-swarm implementation of a complex Rust project across 817 commits.

## Current Status

**Phase: Planning**

The specification is being drafted and will undergo iterative refinement through APR Pro before bead conversion begins.

## Building

```
git clone https://github.com/nohomedirectory/monkeybee-pdf.git
cd monkeybee-pdf
cargo build --release
```

*Detailed build instructions, dependency requirements, and WASM compilation steps will be added as the workspace is initialized.*

## License

*License selection is pending.*

## Links

- **Repository:** [github.com/nohomedirectory/monkeybee-pdf](https://github.com/nohomedirectory/monkeybee-pdf)
- **Specification:** [docs/SPEC.md](docs/SPEC.md)
- **Implementation Plan:** [docs/IMPLEMENTATION.md](docs/IMPLEMENTATION.md)
- **Joey:** [@nohomedirectory](https://github.com/nohomedirectory) on GitHub, [@nohomedirectory](https://x.com/nohomedirectory) on X
- **Jeffrey:** [@doodlestein](https://github.com/doodlestein) on GitHub, [@doodlestein](https://x.com/doodlestein) on X
- **Gary's original post:** [x.com/garybasin/status/2020643166047928494](https://x.com/garybasin/status/2020643166047928494)
- **FrankenTUI (prior art):** [github.com/Dicklesworthstone/frankentui](https://github.com/Dicklesworthstone/frankentui)
- **APR Pro (planning tool):** [github.com/Dicklesworthstone/automated_plan_reviser_pro](https://github.com/Dicklesworthstone/automated_plan_reviser_pro)
