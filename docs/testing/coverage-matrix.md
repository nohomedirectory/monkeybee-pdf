# Coverage Matrix

## Purpose

The coverage matrix tracks which PDF feature categories are tested, at what depth, and by which harness. It prevents the common failure mode where certain feature zones are tested heavily while others have zero coverage.

## Matrix structure

| Feature category | Unit tests | Corpus tests | Round-trip | E2E | Fuzz | Reference diff | Status |
|---|---|---|---|---|---|---|---|
| Basic parsing (objects, xref, trailer) | | | | | | | Not started |
| Stream decompression (all filters) | | | | | | | Not started |
| Content stream interpretation | | | | | | | Not started |
| Text rendering (basic fonts) | | | | | | | Not started |
| Text rendering (CJK, RTL) | | | | | | | Not started |
| Image rendering (JPEG, PNG) | | | | | | | Not started |
| Image rendering (JPEG2000, JBIG2) | | | | | | | Not started |
| Color spaces (Device*) | | | | | | | Not started |
| Color spaces (ICC, CalRGB, Lab) | | | | | | | Not started |
| Transparency compositing | | | | | | | Not started |
| Blend modes | | | | | | | Not started |
| Patterns (tiling, shading) | | | | | | | Not started |
| Annotations (basic types) | | | | | | | Not started |
| Annotations (round-trip) | | | | | | | Not started |
| Write path (full rewrite) | | | | | | | Not started |
| Write path (incremental save) | | | | | | | Not started |
| Text extraction with positions | | | | | | | Not started |
| Metadata extraction | | | | | | | Not started |
| Document generation | | | | | | | Not started |
| Page editing (add/remove/reorder) | | | | | | | Not started |
| Encryption/decryption | | | | | | | Not started |
| Malformed input handling | | | | | | | Not started |
| Incremental update chains | | | | | | | Not started |
| XFA detection (Tier 3) | | | | | | | Not started |

## Update cadence

This matrix is updated with every bead completion and every CI run. Gaps in coverage are treated as risks, not as "we'll get to it later."
