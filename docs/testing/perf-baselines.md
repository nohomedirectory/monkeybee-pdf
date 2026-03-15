# Performance Baselines

## Purpose

Performance is part of Monkeybee's definition of seriousness, not a post-v1 garnish. This document defines the benchmark classes, measurement methodology, and regression policy.

## Benchmark classes

### Latency class (small simple PDFs)
Target: fast enough that CLI commands feel instantaneous on typical documents.
Representative files: 1-5 page text-only documents, simple vector graphics, basic images.
Metrics: parse time, render time per page, total wall-clock time.

### Throughput class (large complex PDFs)
Target: competitive with existing open-source engines on bulk processing.
Representative files: 100+ page documents, image-heavy files, complex typography.
Metrics: pages per second, peak memory, total processing time.

### Robustness class (pathological PDFs)
Target: no pathological performance cliffs on malformed or adversarial inputs.
Representative files: deeply nested objects, extremely large streams, circular references (post-repair), adversarial xref tables.
Metrics: parse time (must be bounded), memory usage (must be bounded), no hangs.

## Measurement methodology

- All benchmarks run on a defined baseline machine configuration (documented in CI).
- Each benchmark runs multiple iterations with warm cache.
- Results stored as structured data for trend analysis.
- Criterion.rs for microbenchmarks, wall-clock for E2E.

## Regression policy

- Any PR that regresses latency-class benchmarks by >10% requires justification.
- Any PR that regresses throughput-class benchmarks by >20% requires justification.
- Any PR that introduces an unbounded performance cliff on robustness-class inputs is blocked until fixed.
