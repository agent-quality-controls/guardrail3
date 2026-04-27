# Adversarial test attack fixes across 3 packages

**Date:** 2026-04-06 15:54
**Scope:** g3rs-workspace-crawl, g3rs-cargo-config-ingestion, g3rs-cargo-config-checks

## Summary
Ran adversarial test attacks on all three g3rs packages. Fixed 3 check bugs, 1 error type design issue, 1 discarded IO error, and added 14 missing tests across the three packages.

## g3rs-workspace-crawl (7 new tests)
- Fixed `queries.rs` fragile test (missing git_init)
- Added: golden baseline, directory-only gitignore pattern, non-git workspace, .claude/worktrees ban, sort order verification, symlinks skipped, unreadable file detection

## g3rs-cargo-config-ingestion (4 new tests + 2 bug fixes)
- Added Display/Error impls to G3RsCargoConfigIngestionError
- Changed Unreadable variant from tuple to struct, preserving IO error reason
- Added: empty Cargo.toml, nested Cargo.toml exclusion, ignored-but-recovered Cargo.toml, workspace+package combined

## g3rs-cargo-config-checks (3 new tests + 3 bug fixes)
- g3rs-cargo/disallowed-macros-deny: accept "forbid" as valid (was rejecting it as "weakened")
- g3rs-cargo/lint-levels: guard priority check for missing lints (was producing misleading "wrong priority" for nonexistent lints)
- g3rs-cargo/lint-levels: added level validation for EXPECTED_CLIPPY_REQUIRED_ALLOW (was only checking presence, not level)
- Added: forbid acceptance test, standalone package root test, missing-lint-no-priority-error test

## Key Files
- `packages/g3rs-workspace-crawl/crates/runtime/src/crawl_tests/` — 7 new tests
- `packages/g3rs-cargo-config-ingestion/crates/types/src/error.rs` — Display/Error impls, Unreadable restructured
- `packages/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_06_*/rule.rs` — forbid fix
- `packages/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_*/rule.rs` — priority guard + allow validation
