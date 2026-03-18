# Create centralized fs module and fix clippy errors

**Date:** 2026-03-15 16:07
**Task:** Create src/fs.rs as centralized filesystem module, migrate all fs access, fix remaining clippy errors

## Goal
Zero clippy errors. All filesystem access goes through `crate::fs`.

## Approach

1. Create `src/fs.rs` with `read_file`, `read_file_err`, `list_dir`, `write_file`, `create_dir_all`, `metadata` — all with `#[allow(clippy::disallowed_methods)]`
2. Add `mod fs;` to `src/main.rs`
3. Replace all `std::fs::read_to_string` → `crate::fs::read_file` or `read_file_err` (10 occurrences)
4. Replace all `std::fs::read_dir` → `crate::fs::list_dir` (2 occurrences)
5. Replace `std::fs::metadata` → `crate::fs::metadata` (1 occurrence)
6. Fix `doc_markdown` error in eslint_check.rs (ESLint → `ESLint`)
7. Fix `type_complexity` errors in eslint_check.rs and tsconfig_check.rs (add type aliases)

## Files to Modify
- `src/fs.rs` — NEW: centralized fs module
- `src/main.rs` — add `mod fs;`
- `src/hooks/hook_checks.rs` — metadata call
- `src/hooks/validate.rs` — read_dir call
- `src/rs/validate/cargo_lints.rs` — 2x read_to_string
- `src/rs/validate/mod.rs` — read_to_string
- `src/ts/validate/eslint_audit.rs` — read_to_string
- `src/ts/validate/eslint_check.rs` — doc_markdown + type_complexity
- `src/ts/validate/jscpd_check.rs` — 2x read_to_string
- `src/ts/validate/npmrc_check.rs` — read_to_string
- `src/ts/validate/package_check.rs` — read_to_string
- `src/ts/validate/source_scan.rs` — read_to_string + read_dir
- `src/ts/validate/tsconfig_check.rs` — read_to_string + 3x type_complexity
