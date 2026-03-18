# Split source_scan.rs into two files

**Date:** 2026-03-16 10:17
**Task:** Split ts/validate/source_scan.rs (889 lines) by extracting T23-T29 comment checks into ts_comment_checks.rs

## Goal
Both files under 500 lines, all tests passing.

## Approach

1. Create `ts_comment_checks.rs` with:
   - `is_tsx_path` as `pub(super)` (shared helper)
   - `check_eslint_disable`, `check_eslint_disable_from_comments`, `check_eslint_disable_grep`
   - `check_ts_ignore`, `check_ts_ignore_from_comments`, `check_ts_ignore_grep`
   - Their 7 tests
2. Update `source_scan.rs`:
   - Remove moved functions (lines 87-375, tests 661-765)
   - Import from `ts_comment_checks` for orchestrator calls
   - Use `super::ts_comment_checks::is_tsx_path` where needed
3. Add `mod ts_comment_checks;` to mod.rs
4. Verify with `wc -l` and `cargo test`

## Files to Modify
- `src/ts/validate/ts_comment_checks.rs` — new file with T23-T29 checks
- `src/ts/validate/source_scan.rs` — remove moved code, update imports
- `src/ts/validate/mod.rs` — add module declaration
