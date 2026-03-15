Six rounds have built a deeply specified engine. The architecture, types, lifecycle, contracts, security gating, and proof infrastructure are now concrete. This round addresses remaining operational gaps: error propagation consistency, library API error contract, CLI output formatting, WASM constraints propagation, CI integration specifics, and a few cross-cutting consistency issues.

## 1) Define the library API error contract

The spec defines a comprehensive error taxonomy and diagnostic streaming model, but doesn't specify how errors surface to the library caller. Every public API needs a defined return type that distinguishes fatal errors (operation failed) from degraded results (operation succeeded with warnings). Without this, callers won't know whether to trust the result.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ API surface
 This API ensures:
+
+### Library API error contract
+
+Every public API that processes PDF data returns a `Result<T, MonkeybeeError>` where:
+
+- `Err(MonkeybeeError)` indicates a fatal failure — the operation did not produce a usable result.
+  Examples: file cannot be opened, decryption fails with wrong password, no valid xref found
+  even after repair.
+- `Ok(result)` indicates the operation completed. The result may include degradations.
+
+Successful results carry a `Diagnostics` collection alongside the primary value:
+
+```
+pub struct WithDiagnostics<T> {
+    pub value: T,
+    pub diagnostics: Vec<Diagnostic>,
+    pub has_errors: bool,      // true if any Error-severity diagnostics were emitted
+    pub has_warnings: bool,    // true if any Warning-severity diagnostics were emitted
+}
+```
+
+API methods return `Result<WithDiagnostics<T>, MonkeybeeError>`. The caller can:
+1. Check `result.has_errors` to detect degraded results
+2. Inspect `result.diagnostics` for specific degradation details
+3. Ignore diagnostics entirely if they only care about the primary value
+
+The `DiagnosticSink` on `ExecutionContext` receives diagnostics in real time during processing.
+The `WithDiagnostics` collection is the post-hoc summary. Both exist because different callers
+have different needs: a viewer wants real-time progress; a batch tool wants a summary.
+
+**Error coarsening rule:** Subsystem-specific errors (ParseError, RenderError, WriteError) are
+wrapped in `MonkeybeeError` at API boundaries. Within a crate, functions may use crate-specific
+error types. At the public API surface, everything is `MonkeybeeError`. This prevents leaking
+internal error types across crate boundaries.
```

## 2) Specify the CLI output format contract

The CLI section lists commands but doesn't specify the output format discipline. For machine consumption (piping, CI integration, scripting), the CLI needs a consistent output contract.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ monkeybee-cli
 - `monkeybee signatures <file>` — inspect signature dictionaries
+
+### CLI output discipline
+
+All CLI commands follow a consistent output contract:
+
+**Stdout:** Primary output (rendered files, extracted text, JSON reports). Machine-parseable.
+**Stderr:** Diagnostics, progress, and error messages. Human-readable.
+
+**Exit codes:**
+- 0: success, no errors or degradations
+- 1: operation completed but with errors or degradations (check stderr or `--json` output)
+- 2: operation failed (fatal error)
+- 3: invalid arguments or usage error
+
+**JSON mode:** Every command supports `--json` which formats all output (including diagnostics
+and errors) as JSON to stdout. In JSON mode, stderr receives only fatal errors. The JSON output
+wraps the primary result in an envelope:
+
+```json
+{
+  "status": "success" | "degraded" | "failed",
+  "result": { /* command-specific */ },
+  "diagnostics": [ /* array of Diagnostic objects */ ],
+  "timing": { "wall_ms": 1234, "parse_ms": 500, "render_ms": 700 }
+}
+```
+
+**Progress reporting:** Long-running operations (multi-page render, corpus proof) report
+progress to stderr as `[page N/M]` or `[file N/M]` lines. In JSON mode, progress is suppressed.
+
+**Quiet mode:** `--quiet` suppresses all diagnostics and progress on stderr. Only fatal errors
+are reported. Exit code still reflects the operation status.
```

## 3) Propagate WASM constraints through the architecture

The spec mentions WASM compatibility (Part 7) but doesn't trace the constraints through the crate boundaries. Several crates have WASM-incompatible dependencies or patterns that need explicit treatment.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ WASM-friendly core target
 the architecture must not preclude it.
+
+### WASM constraint propagation
+
+The following crates and features have WASM-specific constraints:
+
+| Crate | WASM constraint | Mitigation |
+|---|---|---|
+| monkeybee-bytes | No mmap, no filesystem | `ByteSource::InMemory` only; no mmap feature |
+| monkeybee-codec | No openjpeg-sys, no native JBIG2 | Pure-Rust decoders only; `Strict` profile |
+| monkeybee-security | No process isolation | Budget enforcement via cooperative checks only |
+| monkeybee-text | No system font discovery | Explicit `FontProvider` required; Base 14 metrics compiled in |
+| monkeybee-render | No threads | Sequential page rendering via `cfg(target_arch = "wasm32")` |
+| monkeybee-render | SIMD via wasm-simd128 | Conditional compilation for WASM SIMD paths |
+| monkeybee-proof | Not applicable | Proof harness is native-only |
+| monkeybee-cli | Not applicable | CLI is native-only |
+| all crates | No `std::time::Instant` | Use `web_time` crate or abstract via trait |
+
+**Conditional compilation strategy:**
+- `#[cfg(target_arch = "wasm32")]` guards thread pool initialization, mmap paths, and native
+  decoder bindings.
+- `#[cfg(feature = "wasm")]` gates WASM-specific code paths (web_time, JS interop).
+- The `wasm` feature is a workspace-level feature that propagates to all crates.
+- WASM builds use `--no-default-features --features wasm` to exclude native dependencies.
```

## 4) Define the proof harness CI integration contract

The spec says the proof harness runs in CI (Part 6, Part 8) but doesn't specify how. CI integration details matter: how results are reported, how regressions are detected, and what artifacts are produced.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Proof doctrine
+### CI integration contract
+
+The proof harness integrates with CI as follows:
+
+**Invocation:** `cargo test --workspace` runs unit/integration tests. `monkeybee proof <corpus>`
+runs the full proof harness as a separate CI step (it takes longer and requires reference
+renderer binaries).
+
+**Artifacts produced per CI run:**
+1. `proof-report.json` — aggregate compatibility report across the corpus
+2. `regressions.json` — list of test classes that passed in the previous run but fail now
+3. `ledger/` — per-document compatibility ledger JSON files
+4. `diffs/` — render comparison images for any page with MS-SSIM below threshold
+5. `timing.json` — per-test-class timing data for performance regression detection
+
+**Regression detection:**
+- The proof harness compares `proof-report.json` against a committed baseline
+  (`tests/proof-baseline.json`). Any test class that drops below its pass threshold is a
+  regression.
+- Performance regressions: any benchmark class that exceeds 1.5x its baseline timing is flagged
+  (not blocking, but reported).
+- The baseline is updated by committing a new `proof-baseline.json` when regressions are
+  intentionally accepted (with a justification in the commit message).
+
+**Reference renderer setup:**
+- CI uses container images with pinned versions of PDFium, MuPDF, Ghostscript, and pdf.js.
+- Container digests are recorded in `oracle-manifest.json` at the workspace root.
+- The proof harness refuses to run if the oracle manifest doesn't match the actual renderer
+  versions (prevents silent oracle drift).
+
+**Corpus management in CI:**
+- `tests/corpus/public/` — committed to the repo (small files, permissive licenses)
+- `tests/corpus/generated/` — generated by `monkeybee generate` during CI (deterministic)
+- `tests/corpus/restricted/` — fetched from a private artifact store during CI (large or
+  license-restricted files); not committed to the repo
+- `tests/corpus/minimized/` — minimized crashers and regression cases, committed
```

## 5) Specify the `EditTransaction` validation and conflict detection rules

The spec describes EditTransaction as staging edits and producing a new snapshot, but doesn't specify what validation runs at commit time or how conflicts are detected (e.g., two transactions trying to edit the same object).

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Mutation safety
 4. Either commit atomically as a delta or roll back
+
+### EditTransaction validation rules
+
+At `commit()` time, the transaction performs the following validations:
+
+1. **Reference integrity:** Every indirect reference created or modified by the transaction must
+   point to an existing object (either pre-existing in the snapshot or newly created in the
+   transaction). Dangling references are rejected.
+
+2. **Page tree validity:** If the transaction modified the page tree (add/remove/reorder pages),
+   the resulting tree must have correct `/Count` values, valid `/Parent` back-references, and at
+   least one leaf page. An empty page tree is rejected.
+
+3. **Resource completeness:** For newly created or modified content streams, the resources
+   referenced by the content stream operators must exist in the page's resource dictionary (or
+   the resource dictionary must be updated in the same transaction). Missing resources produce a
+   warning (not a rejection — missing resources are handled at render time), unless the
+   transaction is for a generated document (where missing resources are an error).
+
+4. **Ownership constraints:** Edits to `OpaqueUnsupported` objects are rejected unless the edit
+   explicitly takes ownership (transitions to `Owned`). Edits to `ForeignPreserved` objects
+   transition them to `Owned` automatically with a diagnostic.
+
+5. **Structural cycle detection:** The dependency graph is checked for cycles introduced by the
+   transaction (e.g., a form XObject that references itself). Cycles are rejected.
+
+**Conflict detection:** Transactions are optimistically concurrent. Two transactions based on the
+same source snapshot can both commit — the second commit detects conflicts by checking whether
+any object it modified was also modified by the first (via snapshot_id comparison). Conflicts
+are reported to the caller, who must resolve them (typically by rebasing the second transaction
+on the new snapshot). The engine does not automatically merge conflicting transactions.
```

## 6) Add image color space conversion specifics

The spec describes color spaces extensively but doesn't specify the actual conversion paths that the renderer must implement between common color space pairs. This is where real-world rendering bugs concentrate.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Color space resolution chain
 7. **Pattern:** Either a tiling pattern or a shading pattern.
+
+### Color space conversion paths
+
+The renderer must implement the following conversion paths for the output color space (typically
+sRGB for screen, CMYK for print):
+
+| Source | Target | Path |
+|---|---|---|
+| DeviceRGB | sRGB | Identity (assume sRGB) or via DefaultRGB if defined |
+| DeviceCMYK | sRGB | Via ICC profile (FOGRA39 default) or naive inversion |
+| DeviceGray | sRGB | G → (G, G, G) |
+| CalRGB | sRGB | CalRGB → CIEXYZ (via gamma + matrix) → sRGB |
+| CalGray | sRGB | CalGray → CIEXYZ (via gamma + white point) → sRGB |
+| Lab | sRGB | Lab → CIEXYZ → sRGB |
+| ICCBased | sRGB | Via ICC profile A2B/B2A tables or matrix/TRC |
+| Indexed | sRGB | Lookup base color → convert base to sRGB |
+| Separation | sRGB | Evaluate tint transform → convert alternate space to sRGB |
+| DeviceN | sRGB | Evaluate tint transform → convert alternate space to sRGB |
+
+**Naive CMYK→RGB inversion** (used when no ICC profile is available):
+```
+R = 1 - min(1, C × (1 - K) + K)
+G = 1 - min(1, M × (1 - K) + K)
+B = 1 - min(1, Y × (1 - K) + K)
+```
+This is visually acceptable for screen display but not color-accurate. The proof harness must
+track which documents use naive CMYK inversion versus ICC-profiled conversion.
+
+**DefaultRGB/DefaultCMYK/DefaultGray override:** When a page defines these in its resource
+dictionary, all device color space references on that page are implicitly redirected to the
+corresponding CIE-based space. The override is per-page (not document-global) and applies only
+to device color spaces set by color operators, not to image color spaces specified in image
+XObject dictionaries.
```

## 7) Specify the structure tree preservation contract for tagged PDFs

The spec says "preserve existing tags during round-trip" but doesn't specify the mechanism. Structure tree preservation is non-trivial because the tree references content via marked content IDs (MCIDs), and content stream edits can invalidate these references.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Tagged PDF awareness
 4. **Not corrupt tags during annotation.**
+
+### Structure tree preservation contract
+
+The structure tree (`/StructTreeRoot`) contains structure elements that reference page content
+via Marked Content IDs (MCIDs) and Marked Content References (MCR). Preserving this linkage
+during round-trips requires:
+
+1. **MCID stability:** When a content stream is re-serialized without modification, the MCIDs
+   within it must be preserved exactly (same numeric values, same positions relative to the
+   content they mark). The content stream rewriter in `monkeybee-edit` must preserve BMC/BDC/EMC
+   operators and their MCID properties for any content that is not being edited.
+
+2. **Structure element preservation:** Structure elements in the tree carry `/K` (kids) arrays
+   that reference MCIDs, other structure elements, or object references. During incremental save,
+   unmodified structure elements are preserved byte-for-byte. During full rewrite, structure
+   elements are re-serialized but their semantic content (tag types, MCID references, attributes)
+   is preserved.
+
+3. **Content stream editing impact:** When content is removed (e.g., page deletion, redaction),
+   the corresponding structure elements become orphaned. The edit transaction must:
+   - Remove structure elements whose content was deleted
+   - Update parent element `/K` arrays to remove deleted children
+   - Update `/ParentTree` number tree entries for affected pages
+   - Preserve the `/IDTree` name tree for structure element IDs
+
+4. **Annotation structure:** When adding annotations to tagged PDFs, the annotation gets an
+   `/StructParent` entry that links it into the parent tree. The engine must allocate a new
+   parent tree index and add the annotation's structure element.
+
+5. **Detection without full interpretation:** The engine detects the presence and complexity of
+   the structure tree (number of elements, nesting depth, MCID coverage) and reports it in the
+   compatibility ledger. Full structure tree validation (verifying that every MCID in every
+   content stream has a corresponding structure element) is a proof-harness check, not a
+   runtime check.
```

## 8) Add linearization detection and bypass contract

The spec mentions linearization in passing but doesn't specify how the parser detects it or what "bypass" means operationally. Linearized PDFs have a specific structure that affects first-page latency.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Linearization damage
 Never attempt to repair linearization hints — only bypass them.
+
+### Linearization detection and use
+
+**Detection:** A linearized PDF has:
+1. A linearization dictionary as the first indirect object after the header. It contains
+   `/Linearized` (version number, typically 1.0), `/L` (file length), `/O` (first-page object
+   number), `/E` (end of first-page cross-reference), `/N` (page count), `/T` (offset of main
+   xref), and `/H` (hint stream offsets/lengths).
+2. A first-page cross-reference section immediately after the linearization dictionary.
+3. All objects needed for the first page appear before the first-page xref.
+4. Hint tables (primary and optional overflow) that map pages to byte ranges.
+
+**Use in eager/lazy mode:** When linearization is intact, the parser can use the linearization
+dictionary to locate the first-page objects directly without reading the main xref at the end
+of the file. This enables faster first-page display for large files.
+
+**Use in remote mode:** The fetch scheduler uses linearization hints to issue range requests for
+specific pages. The primary hint table maps page numbers to byte ranges. The parser requests the
+first-page range, renders it, then requests subsequent pages on demand.
+
+**Bypass:** When linearization is detected but damaged (hint tables corrupted, first-page xref
+invalid, or file length doesn't match `/L`), the parser:
+1. Records a `parse.linearization.damaged` diagnostic
+2. Falls back to reading the standard xref at end-of-file (via `startxref`)
+3. Proceeds as if the file were not linearized
+4. The fetch scheduler falls back to heuristic prefetching (request the last 64KB first to get
+   the xref, then request pages based on xref offsets)
+
+Linearized output is explicitly deferred to post-v1 (writing linearized files requires careful
+object ordering and hint table generation). For v1, the engine reads linearized files but always
+writes non-linearized output.
```

## 9) Consolidate the experimental vs baseline classification

The spec marks several features as "experimental" or "non-gating" throughout, but there's no single consolidated list. This makes it hard to audit what's in the baseline v1 and what isn't.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Release gates for v1
+### Baseline v1 vs experimental feature classification
+
+The following table consolidates the baseline/experimental classification from across the spec:
+
+| Feature | Classification | Rationale |
+|---|---|---|
+| Classic xref tables | Baseline | Simpler to audit, required for all PDFs |
+| Cross-reference streams | Baseline (read), Post-baseline (write) | Must read; write deferred |
+| Object stream packing | Post-baseline | Requires xref streams, adds complexity |
+| All standard filters (Flate, LZW, ASCII85, etc.) | Baseline | Required for real-world PDFs |
+| JBIG2 decode | Baseline (via openjpeg-sys) | Common in scanned docs |
+| JPEG 2000 decode | Baseline (via openjpeg-sys) | Common in print-quality PDFs |
+| Encryption V1-V5 (read) | Baseline | Required for real-world PDFs |
+| Encryption (write) | Post-baseline | Not needed for v1 proof |
+| Mesh shadings (types 4-7) | Post-baseline | Rare, complex, not v1-gating |
+| Overprint/OPM=1 | Post-baseline | CMYK-specific, not v1-gating |
+| Exact analytic coverage raster | Experimental | Must beat tiny-skia baseline |
+| Spectral color pipeline | Experimental | Must beat lcms2 baseline |
+| SDF glyph rendering | Experimental | Optional, WASM-focused |
+| Adaptive mesh subdivision | Experimental | Depends on mesh shading (post-baseline) |
+| Bayesian repair scoring | Experimental | Baseline uses strategy-order heuristic |
+| MS-SSIM render comparison | Baseline | Required for proof harness |
+| Entropy-optimal encoding | Post-baseline | Optimization, not correctness |
+| Probabilistic layout analysis | Post-baseline | Baseline uses geometric heuristics |
+| Arlington validation (core) | Baseline | Catalog, page tree, fonts gated |
+| Arlington validation (full) | Post-baseline | Full spec coverage deferred |
+| PDF/A-4 validation | Advisory in v1 | Unless backed by public corpus |
+| PDF/X-6 validation | Advisory in v1 | Unless backed by public corpus |
+| WASM build | Non-gating proof surface | Architecture validation, not release gate |
+| Kani proofs (lexer) | Baseline | No-panic + bounded-allocation for lexer |
+| Kani proofs (full) | Post-baseline | Infrastructure ready, proofs expand post-v1 |
+| Linearized output | Post-v1 | Read-only for v1 |
+| XFA rendering | Not in v1 (Tier 2/3) | Detect and AcroForm fallback only |
+| PostScript XObjects | Not in v1 (Tier 2/3) | Detect and simple subset only |
+| JavaScript execution | Not in v1 | Detect and preserve only |
```
