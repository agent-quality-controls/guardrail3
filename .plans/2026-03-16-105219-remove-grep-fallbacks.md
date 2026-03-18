# Remove All Grep Fallbacks — AST-Only Source Scanning

**Date:** 2026-03-16 10:52
**Task:** Delete every grep fallback from AST-migrated checks. If syn/tree-sitter can't parse it, skip it.

## Goal
Zero grep fallback functions in source scan checks. AST is the ONLY path for Rust and TypeScript source scanning. Grep-only checks (config files, line counting) stay.

## What Gets DELETED (15 fallback functions)

### Rust (9 functions)
1. `allow_checks.rs` — `check_crate_level_allow_grep` (R30-R31 fallback)
2. `allow_checks.rs` — `check_item_level_allow_grep` (R32-R33 fallback)
3. `allow_checks.rs` — `check_garde_skip_grep` (R34-R35 fallback)
4. `allow_checks.rs` — `check_cfg_attr_allow_grep` (R37 fallback)
5. `structure_checks.rs` — use-count grep branch (R40-R41 fallback)
6. `structure_checks.rs` — `check_unsafe_grep` (R42 fallback)
7. `code_quality_checks.rs` — `check_todo_macros_grep` (R43 fallback)
8. `code_quality_checks.rs` — `check_unwrap_expect_grep` (R44 fallback)
9. `code_quality_checks.rs` — `check_direct_fs_usage_grep` (R58 fallback)

### TypeScript (6 functions)
10. `ts/source_scan.rs` — `check_process_env_grep` (T30 fallback)
11. `ts/source_scan.rs` — `check_any_types_grep` (T31 fallback)
12. `ts/ts_comment_checks.rs` — `check_eslint_disable_grep` (T23-T26 fallback)
13. `ts/ts_comment_checks.rs` — `check_ts_ignore_grep` (T27-T29 fallback)
14. `ts/test_checks.rs` — `check_skip_without_reason_grep` (T-TEST-04 fallback)
15. `ts/test_checks.rs` — `check_only_in_source_grep` (T-TEST-05 fallback)

## What Gets DELETED (supplements + helpers that only support deleted code)
16. `allow_checks.rs` — `check_item_level_allow_grep_supplement` (buggy macro body scanner)
17. `allow_checks.rs` — `compute_multiline_string_lines` (only used by supplement)
18. `code_quality_checks.rs` — `check_inline_std_fs_calls` + `check_inline_std_fs_line` → REPLACE with syn expression visitor for inline std::fs calls
19. `source_scan.rs` — `strip_string_literals` → only needed if filter_non_comment_lines is still used
20. Grep fallback tests in source_scan unit tests (test_process_env_grep_fallback_t30, test_any_type_grep_fallback_t31)

## What STAYS (grep-only, no AST alternative)
- R36 `check_exception_comments` — scans TOML config files
- R38-R39, T32-T33 `check_file_length` — line counting
- T34-T35 `check_comment_pattern` — comment pattern scanning
- `filter_non_comment_lines` — still needed by file length, exception comments
- All eslint_check.rs, eslint_audit.rs, npmrc_check.rs — config file scanning
- R-TEST-04 grep fallback for `content_has_test_grep` — KEEP (simple, low risk)

## What Gets REPLACED
- `check_inline_std_fs_calls` → new syn expression visitor `find_inline_std_fs_calls` that walks ExprPath and ExprCall nodes looking for std::fs:: paths

## Approach
For each check function that has `if let Some(file) = parse_file(content) { ... } else { grep_fallback() }`:
1. Change to `if let Some(file) = parse_file(content) { ... }` — no else branch
2. If parse fails, emit zero results (file is unparseable, skip it)
3. Delete the grep fallback function
4. Delete any tests that tested the grep fallback path
5. Delete helper functions that become unused

## Risks
- Files with syntax errors get zero checks instead of partial grep coverage
- Macro bodies (`macro_rules!`) remain invisible (was already true for AST path)
- This is ACCEPTABLE: unparseable files can't compile, so violations are moot
