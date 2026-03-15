# E2E Playbook

## Purpose

End-to-end tests exercise complete user workflows through the CLI and library API, proving that the engine works as a coherent system — not just as isolated subsystems.

## E2E scenarios

### E2E-001: Render a hostile PDF
Input: A malformed PDF from the pathological corpus with known rendering challenges.
Steps: `monkeybee render <file> --format png --dpi 150 --pages all`
Validate: Output exists for every page, no panics, render matches reference within threshold, diagnostics report any degraded features.

### E2E-002: Extract text from a scanned document
Input: A scanned PDF with embedded text layer.
Steps: `monkeybee extract <file> --text --positions`
Validate: Extracted text matches ground truth, positions are within tolerance.

### E2E-003: Annotate and round-trip
Input: A complex multi-page PDF.
Steps: `monkeybee annotate <file> --add highlight --page 1 --rect "100,200,300,250" -o annotated.pdf`, then `monkeybee validate annotated.pdf --roundtrip`.
Validate: Annotation present in output, geometry correct, original content preserved.

### E2E-004: Generate a new document
Steps: Use the library API to create a multi-page document with text, images, and vector graphics. Save. Render under Monkeybee. Render under PDFium. Compare.
Validate: Visual output matches, structural validity confirmed.

### E2E-005: Full inspect/diagnose
Input: A PDF with known structural oddities.
Steps: `monkeybee diagnose <file>`
Validate: Diagnostic output correctly identifies all known issues, compatibility tier assignments are correct.

### E2E-006: Edit and preserve
Input: A multi-page PDF with annotations and form fields.
Steps: Remove a page, reorder remaining pages, update metadata, save, reload.
Validate: Structural validity, remaining pages render correctly, annotations on surviving pages are preserved.

## CI integration

All E2E scenarios run in CI. Each produces structured pass/fail output and artifact diffs on failure.
