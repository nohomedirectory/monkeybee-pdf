# Logging and Evidence Artifacts

## Purpose

Monkeybee's proof is not just pass/fail test results. It is a system of machine-readable evidence artifacts that make the engine's behavior inspectable, auditable, and legible to outsiders.

## Evidence artifact types

### Compatibility ledger
A JSON document produced for every processed PDF. Records every feature encountered, what tier applied, what succeeded, what degraded, and what failed. Aggregated across the corpus into a master compatibility report.

### Render diff reports
Visual comparison output showing Monkeybee renders side-by-side with reference renders, with perceptual diff heatmaps highlighting regions of disagreement.

### Round-trip traces
Structured logs of every round-trip chain execution: input document hash, operations performed, output document hash, validation results, any diffs detected.

### Parser diagnostic logs
Per-file logs recording: tokens parsed, objects constructed, repairs performed, warnings issued, errors encountered, recovery actions taken.

### Performance traces
Timing data for hot paths: parse time, render time per page, write time, memory high-water mark. Stored per corpus file for regression tracking.

### Failure manifests
Machine-readable summaries of all failures in a CI run: file, subsystem, error category, severity, whether this is a regression from previous run.

## Log schema

All structured logs use JSON with a consistent envelope:
```json
{
  "timestamp": "ISO-8601",
  "subsystem": "parser|render|write|annotate|extract|proof",
  "file_id": "corpus-id or path",
  "event": "event-type",
  "severity": "info|warn|error|fatal",
  "data": { ... }
}
```

## Storage and retention

Evidence artifacts are stored alongside CI run metadata. At minimum, the most recent run's full artifact set is always available. Regression-triggering runs are permanently archived.
