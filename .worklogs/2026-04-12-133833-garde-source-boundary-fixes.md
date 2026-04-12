# Garde Source Boundary Fixes

## Summary

Fixed the remaining garde package-model bugs from the multi-agent test attack. Source ingestion now stays inside the pointed root package source surface, excludes nested Cargo roots under `src/**`, excludes bare `src/test.rs` and `src/tests.rs`, and the config lane now has direct invalid-clippy coverage at both runtime and ingestion boundaries.

## Decisions made

- Fixed source boundary leaks in ingestion selection, not in source checks.
  - Why: the source package should trust its explicit governed file list rather than second-guess ownership.
- Rejected the old app shim findings as package work.
  - Why: the old app is inventory only in the current model; package correctness is the target.
- Treated nested `Cargo.toml` roots under `src/**` as out of scope for one garde source input.
  - Why: a nested package root is a different package invocation, even if its path starts under the outer `src/` tree.
- Added invalid-clippy tests instead of changing config logic further.
  - Why: the logic was already correct after the previous commit; the attack found missing coverage, not a new config bug.

## Key files for context

- `.plans/2026-04-12-133345-garde-source-boundary-fixes.md`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/select.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/source/selection.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Next steps

- Optional: add one `.clippy.toml` invalid-case test to mirror the root-file fallback branch.
- For future garde work, keep using focused attack agents on boundary selection whenever ingestion scope changes.
