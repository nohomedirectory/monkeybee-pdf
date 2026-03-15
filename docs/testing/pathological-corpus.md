# Pathological Corpus

## Purpose

The pathological corpus is Monkeybee's primary evidence surface. It is a curated, indexed, continuously exercised collection of ugly, hard, and adversarial PDFs that the engine must handle correctly or degrade explicitly.

## Corpus categories

### Structural pathologies
- Malformed cross-reference tables (wrong offsets, missing entries, mixed table/stream)
- Broken or circular object graphs
- Missing or invalid trailers
- Incremental-update chains with conflicting state
- Damaged or absent linearization headers
- Hybrid cross-reference files

### Font and encoding
- Missing or incomplete ToUnicode CMaps
- Type 1 fonts with broken metrics
- CIDFont subsetting errors
- Encoding conflicts between font dictionaries and content streams
- Embedded fonts with invalid tables
- CJK fonts with non-standard encodings

### Rendering edge cases
- Transparency groups with isolated/knockout combinations
- Soft masks (luminosity and alpha)
- Overprint and overprint mode interactions
- Blend mode stacking
- Type 3 font glyphs with embedded graphics state
- Tiling patterns with unusual matrices
- Shading patterns across color spaces

### Producer diversity
- Acrobat (various versions)
- Microsoft Word / Office
- LaTeX (pdfTeX, LuaTeX, XeTeX)
- Chrome/Chromium print-to-PDF
- LibreOffice
- InDesign
- Quartz/macOS Preview
- Ghostscript-generated
- Various scanning software

### Legacy and hostile
- XFA forms and hybrid XFA/AcroForm documents
- Encrypted files (standard handlers, various key lengths)
- Adversarial inputs (hand-crafted, fuzz-generated)
- Very large files (100+ pages, large embedded resources)
- Scanned documents (various quality levels)

### Internationalization
- CJK documents (Chinese, Japanese, Korean)
- RTL documents (Arabic, Hebrew)
- Multilingual mixed-direction documents
- Documents with unusual Unicode mappings

## Corpus indexing schema

Each corpus file is registered with:
- `id`: unique corpus identifier
- `filename`: original filename
- `categories`: list of applicable categories from above
- `producer`: known producing software (if identifiable)
- `features_exercised`: specific PDF features this file exercises
- `known_issues`: known rendering or parsing challenges
- `reference_renders`: pre-computed renders from PDFium, MuPDF, pdf.js, Ghostscript
- `ground_truth`: any known ground-truth data (text content, structure, metadata)

## Acquisition strategy

- Public test suites: PDF Association test corpus, Isartor test suite, veraPDF test corpus
- Real-world collection: government forms, academic papers, scanned archives, financial reports
- Fuzz-generated: AFL/libFuzzer-generated malformed inputs
- Hand-crafted: specifically built to exercise known edge cases
- Donated: community-contributed problematic files

## CI integration

The corpus runs in CI on every meaningful change. Each run produces:
- Parse success/failure per file
- Render output per file (stored as artifacts)
- Comparison diffs against reference renders
- Compatibility ledger updates
- Regression alerts (any file that previously passed and now fails)
