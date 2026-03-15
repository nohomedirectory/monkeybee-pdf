# Round-Trip Harness

## Purpose

The round-trip harness validates Monkeybee's bidirectional claim: that documents survive load → modify → save → reload → validate cycles without corruption, silent drift, or structural degradation.

## Round-trip chains

### Chain 1: Visual stability
`Load → Render → Save → Reload → Render → Compare`

Both renders must produce visually identical output (within a defined perceptual diff threshold). This proves that the save/reload cycle does not degrade visual fidelity.

### Chain 2: Annotation integrity
`Load → Annotate → Save → Reload → Verify annotations`

Added annotations must survive the round trip: correct type, correct geometry, correct content, correct appearance. Original annotations must be preserved.

### Chain 3: Mutation integrity
`Load → Edit (page ops, metadata, resource changes) → Save → Reload → Validate structure`

The modified document must be structurally valid: correct xref, valid object graph, no dangling references, no corrupted streams.

### Chain 4: Generation correctness
`Generate new document → Render (Monkeybee) → Render (reference) → Compare`

Generated documents must render equivalently under Monkeybee and reference implementations.

### Chain 5: Extraction accuracy
`Load → Extract text → Compare against ground truth`

Extracted text with positions must match known ground truth within defined tolerances.

## Diff thresholds

- Visual diff: SSIM ≥ 0.99 for structurally identical documents, ≥ 0.95 for acceptable visual fidelity after mutation.
- Structural diff: zero dangling references, zero invalid xref entries, zero corrupted streams.
- Geometry diff: annotation positions within 0.5pt after round trip.
- Text diff: character-level accuracy ≥ 99% against ground truth where ground truth exists.

## Failure reporting

Every round-trip failure produces:
- The input file
- The saved intermediate file
- Both render outputs (if visual chain)
- A structured diff report
- The error category and subsystem attribution
