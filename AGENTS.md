# AGENTS.md — Monkeybee PDF Agent Coordination

## For AI agents working on this project

You are working on Monkeybee PDF, a memory-safe, high-performance Rust PDF engine for ugly real-world PDFs.

### Before you write any code

1. Read `NORTH_STAR.md` — this is the project's constitutional thesis. Do not violate it.
2. Read `SPEC.md` — this is the operational master plan. Your work must fit within it.
3. Read the relevant subsystem doc in `docs/implementation/` — this is your design contract.
4. Check your bead — if you were assigned a bead, it contains your task, dependencies, test obligations, and acceptance conditions. Follow them.

### Core rules

- **Memory safety is non-negotiable.** Minimize `unsafe`. Justify every block. Test aggressively.
- **Tests are not optional.** Every bead carries test obligations. Ship tests with code.
- **The closed loop must work.** If your change breaks load → modify → save → reload → validate on any corpus file that previously passed, your change is blocked.
- **Explicit degradation over silent failure.** If you encounter a PDF feature you cannot handle, detect it, report it via the diagnostic system, and degrade explicitly. Never silently produce wrong output.
- **Use the shared infrastructure.** Geometry goes through `monkeybee-core::geometry`. Graphics state goes through the shared state machine. Errors use the shared taxonomy. Do not duplicate.

### Crate boundaries

- `monkeybee-core`: You may depend on this from any crate. Do not add rendering or parsing logic here.
- `monkeybee-parser`: Only parsing, repair, and decryption. Output is the core document model.
- `monkeybee-render`: Only rendering. Consumes the core model. Does not write files.
- `monkeybee-write`: Only serialization and generation. Consumes the core model.
- `monkeybee-annotate`: Annotation operations only. May use render for appearance streams and write for save.
- `monkeybee-extract`: Extraction and inspection only. May reuse render's text pipeline.
- `monkeybee-proof`: Validation harness. Depends on everything. Does not ship as a user-facing library.
- `monkeybee-cli`: Thin CLI layer over the libraries. No business logic here.

### How to signal completion

When your bead is done:
1. All specified tests pass.
2. `cargo clippy` is clean.
3. `cargo test` passes for your crate and integration tests.
4. You've added a compatibility ledger entry for any new feature support.
5. Document any open questions or unexpected findings.
