# Arch File-Tree Lane

## Summary

Extracted the remaining `arch` rules into a real package file-tree lane: `RS-ARCH-01`, `RS-ARCH-03`, and `RS-ARCH-07`. The work also closed a real ingestion bug where root-level crate complexity was counting excluded or foreign nested crates and could falsely trigger `RS-ARCH-07`.

## Decisions made

- Added a dedicated `g3rs-arch-file-tree-checks` package instead of forcing these rules into source.
  - Why: all three rules are driven by file layout and structural counts, not config contents or source-body semantics.
- Fixed nested-crate leakage in ingestion, not in `RS-ARCH-07`.
  - Why: the wrong complexity facts would have corrupted any future file-tree consumer too.
- Kept `RS-ARCH-03` semantics aligned with the live app family, including the second directory-scan pass for `#[path]`-wired directories.
  - Rejected: simplifying to only `mod foo;` declarations, because that would regress known coverage.
- Corrected the broken `arch` package READMEs while touching the lane.
  - Why: both still described the `code` family and would mislead the next session.

## Key files for context

- `.plans/2026-04-11-222255-arch-file-tree-lane.md`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_01_crate_has_facade.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_03_mod_rs_required.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07_force_crate_split.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`

## Next steps

- Re-audit the remaining partially migrated families against live app code, not the stale family plans.
- `garde`, `code`, `cargo`, `clippy`, `fmt`, `toolchain`, `deny`, `deps`, and `release` still need the same rule-by-rule comparison.
