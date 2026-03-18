# Update Guardrail Rules - Phase 1

**Date:** 2026-03-16 18:34
**Task:** Kill R39, add R-TEST-09, fix test exclusions, update help/guide

## Goal
Six changes to guardrail3 rule set: remove R39 warning, add R-TEST-09 check for inline tests in src/, ensure source scan checks properly exclude test files, exclude test files from R-GARDE-05/R-ARCH-02/T-ARCH-02, and update help text.

## Approach

### 1. Kill R39 (structure_checks.rs)
- Remove the `else if effective_lines > 400` branch from `check_file_length`
- Remove R39 test `file_length_warns_between_400_and_500`
- Update comment from "R38-R39" to "R38"

### 2. Add R-TEST-09 (test_checks.rs)
- Add `check_no_tests_in_src()` function that walks src/ .rs files
- Parse with syn, check for `#[test]`/`#[tokio::test]` attributes or `#[cfg(test)]` modules
- Skip files already in a `tests/` path
- Wire into `test_checks::check()` orchestrator

### 3. Fix test exclusions in source_scan.rs
Current state of source_scan::check():
- R30-R33 (allow): R30 downgrades to Info for test files (not skip), R32-R33 run on all files
- R34-R35 (garde skip): NOT excluded for test files
- R42 (unsafe): NOT excluded for test files
- R44 (unwrap/expect): Already excluded via `if !is_test_file` block
- R58 (std::fs): Already excluded

Changes needed:
- R34-R35: Pass `is_test_file` and skip if true
- R42: Pass `is_test_file` and skip if true
- R30-R33: Already handled (R30 downgrades severity, R32-R33 should still run per current logic)

Wait - task says R30-R33 should be excluded. Currently R30 downgrades to Info. Let me re-read...

Task says: "Make sure these checks are EXCLUDED for test files: R30-R33 (allow checks)"

So: skip R30-R33 for test files entirely.

### 4. Exclude test files from R-GARDE-05, R-ARCH-02, T-ARCH-02
- R-GARDE-05: In `garde_checks.rs`, filter out test files from `rs_files` before derive inventory scan
- R-ARCH-02: In `hex_arch_checks.rs`, skip dependency flow checks for test files
- T-ARCH-02: In `ts/validate/` - check how import boundaries work and add test exclusion

### 5. Update help_gen.rs and guide.rs
- Remove R39 line from RS_VALIDATE_HELP
- Add R-TEST-09 to TESTS section
- Same in guide.rs

### 6. Run validation and tests

## Files to Modify
- `apps/guardrail3/src/app/rs/validate/structure_checks.rs` - Kill R39
- `apps/guardrail3/src/app/rs/validate/test_checks.rs` - Add R-TEST-09
- `apps/guardrail3/src/app/rs/validate/source_scan.rs` - Fix test exclusions for R34-R35, R42
- `apps/guardrail3/src/app/rs/validate/allow_checks.rs` - Skip R30-R33 for test files
- `apps/guardrail3/src/app/rs/validate/garde_checks.rs` - Exclude test files from R-GARDE-05
- `apps/guardrail3/src/app/rs/validate/hex_arch_checks.rs` - Exclude test files from R-ARCH-02
- `apps/guardrail3/src/help_gen.rs` - Update check list
- `apps/guardrail3/src/domain/modules/guide.rs` - Update check list
- `apps/guardrail3/src/app/rs/validate/mod.rs` - Wire R-TEST-09
