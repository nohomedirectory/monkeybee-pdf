# Reference-Guided Validation

## Purpose

Monkeybee does not treat any single external renderer as ground truth. Instead, it uses consensus-style differential testing against multiple reference implementations to establish correctness.

## Reference implementations

| Renderer | Role | Strengths |
|---|---|---|
| PDFium | Primary reference | Broad compatibility, Chrome's PDF engine, well-maintained |
| MuPDF | Secondary reference | High-quality rendering, good font handling, strict parser |
| pdf.js | Tertiary reference | Browser-native perspective, different architecture |
| Ghostscript | Strict canary | Very strict interpretation, catches spec violations others tolerate |

## Comparison methodology

For each corpus document and each page:
1. Render under Monkeybee at target DPI (default 150).
2. Render under each reference at the same DPI.
3. Compute perceptual diff (SSIM) between Monkeybee and each reference.
4. Where references disagree with each other, record the disagreement for investigation.
5. Where Monkeybee disagrees with consensus, flag as a rendering issue.

## Disagreement categories

- **Monkeybee vs. all references**: Monkeybee bug. High priority.
- **Monkeybee agrees with majority**: Likely correct. Monitor minority reference.
- **All references disagree**: Spec ambiguity zone. Document and choose a defensible interpretation.
- **Monkeybee matches one reference, differs from others**: Investigate which interpretation is spec-correct.

## Automation

Reference renders are pre-computed and stored alongside corpus files. CI compares Monkeybee output against stored references. New corpus files trigger reference render generation before Monkeybee testing begins.
