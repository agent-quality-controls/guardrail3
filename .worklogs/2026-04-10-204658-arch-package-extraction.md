# Summary

Extracted the clean non-file-tree `arch` slice into `packages/rs`. The new package set now has real source checks, config checks, and ingestion, with package-only crawl pipeline tests proving the extracted rules run end to end.

# Decisions made

- Narrowed the `arch` scope to the clean non-file-tree rules only.
  - Source: `RS-ARCH-02`, `RS-ARCH-04`, `RS-ARCH-08`, `RS-ARCH-09`
  - Config: `RS-ARCH-05`, `RS-ARCH-06`
  - Rejected for this pass: `RS-ARCH-01`, `03`, `07` because they are file-tree-led or mixed with file-tree shape.
- Kept `RS-ARCH-08` in the source lane even though it also reads Cargo feature facts.
  - Why: the rule is driven by facade export semantics and only needs small crate feature facts alongside the facade surface.
  - Rejected: splitting one rule ID across two packages.
- Rebuilt collection against `g3rs-workspace-crawl` instead of trying to keep the app view.
  - Why: the package boundary has to be self-contained now.
  - Rejected: depending on legacy app family view types.

# Key files for context

- `.plans/2026-04-10-202849-arch-hexarch-package-extraction.md`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

# Next steps

- Run an adversarial review pass on the new `arch` packages and harden any edge cases it finds.
- Start `hexarch` from the same narrowed boundary:
  - source first: `RS-HEXARCH-22`, `23`
  - then config rules `08`, `10`, `11`, `13`-`27`
- Update the extraction plan to reflect the narrowed `arch` slice and the staged `hexarch` order.
