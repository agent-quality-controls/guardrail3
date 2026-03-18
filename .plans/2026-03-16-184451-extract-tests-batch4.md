# Extract inline tests batch 4 — 5 files

**Date:** 2026-03-16 18:44
**Task:** Move #[cfg(test)] blocks from 5 src/ files to tests/unit/

## Goal
Remove all `#[cfg(test)]` blocks from the 5 specified files, creating external test files in `apps/guardrail3/tests/unit/`.

## Approach

### Analysis
1. **ts_comment_checks.rs** — Has real test module (7 tests). Tests use `check_eslint_disable` (pub) and `check_ts_ignore` (pub). Move to `tests/unit/ts_comment_checks_test.rs`.
2. **report.rs** — Has real test module (3 tests). Tests use `Report::new`, `Section`, `CheckResult`, `Severity` — all pub. Move to `tests/unit/report_test.rs`.
3. **help_gen.rs** — Has real test module (3 tests). Tests use `inject_help` (pub), `Cli::command()`. Move to `tests/unit/help_gen_test.rs`.
4. **lib.rs** — Only has `#[cfg(test)] use proptest as _;` — conditional import suppression. Leave as-is.
5. **main.rs** — Only has `#[cfg(test)] use proptest as _;` and `#[cfg(test)] use tempfile as _;` — conditional import suppressions. Leave as-is.

### Visibility changes needed
- `ts_comment_checks.rs`: `is_tsx_path` is `pub(super)` but not used in tests directly. `check_eslint_disable` and `check_ts_ignore` are `pub`. No changes needed.
- `report.rs`: `count_by_severity` is private but not used directly in tests (tests go through `error_count()` etc.). No changes needed.
- `help_gen.rs`: `inject_help` is already `pub`. No changes needed.

### Steps
1. Create `tests/unit/` directory and `tests/unit/mod.rs`
2. Create 3 test files
3. Remove `#[cfg(test)]` blocks from the 3 source files
4. Run `cargo test`

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/ts_comment_checks.rs` — remove test block
- `apps/guardrail3/src/domain/report.rs` — remove test block
- `apps/guardrail3/src/help_gen.rs` — remove test block
- `apps/guardrail3/tests/unit/mod.rs` — create (module declarations)
- `apps/guardrail3/tests/unit/ts_comment_checks_test.rs` — create
- `apps/guardrail3/tests/unit/report_test.rs` — create
- `apps/guardrail3/tests/unit/help_gen_test.rs` — create
