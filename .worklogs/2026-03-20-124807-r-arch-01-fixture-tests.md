# R-ARCH-01 fixture-based tests + hex-in-hex + crates/ root loose files

**Date:** 2026-03-20 12:48

## Summary
Added golden fixture with hex-in-hex structure, fixture-mutation test harness for R-ARCH-01, and fixed three bugs the tests caught.

## Changes

### hex_arch_structure.rs
- Extracted `check_crates_dir` from `check_single_app` for reuse in hex-in-hex recursion
- `validate_container_folder` now recognizes hex-in-hex: subdirs with `crates/` recurse instead of requiring `Cargo.toml`
- Added `check_loose_files` call on `crates/` root — catches `lib.rs`, `main.rs`, `fs.rs`
- Fixed `validate_container_folder` empty-dir detection: uses `metadata()` to distinguish "doesn't exist" from "exists but empty"
- Fixed duplicated `crates/` label prefix in `check_inbound_outbound` error messages

### Golden fixture
- `tests/fixtures/r_arch_01/golden/` — complete hex arch structure with hex-in-hex (CLI adapter has its own inner hex)
- Includes root workspace (packages only), app workspace, 4 outer crates + 3 inner crates, .gitkeep placeholders

### Test harness (test_r_arch_01.rs)
- Copy golden to tempdir, mutate, run check, assert exact error count + title
- 13 tests covering rules 1-6 and 12 (all currently implemented rules)
- Every test uses `assert_single_error` for strict count + title matching

## Bugs found by tests
1. **Empty dir vs nonexistent dir** — `list_dir` returns empty for both, causing empty containers to go undetected
2. **Duplicated label prefix** — `check_inbound_outbound` prepended `crates/` to a label that already had it
3. **Missing crates/ root file check** — `crates/` root never checked for loose files

## Key files
- `crates/app/rs/validate/hex_arch_structure.rs` — the check
- `tests/unit/test_r_arch_01.rs` — fixture-mutation tests
- `tests/fixtures/r_arch_01/golden/` — golden fixture

## Next steps
- Write failing tests for rules 7-11 (workspace membership, app workspace, root workspace)
- Implement rules 7-11 to make them pass
- Plan for refactoring guardrail3 to hex arch crate split
