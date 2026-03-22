# Parser hardening and miscellaneous safe fixes in TS validation

**Date:** 2026-03-19 20:04
**Task:** Fix 7 files with BOM stripping, ID collisions, quote handling, pattern expansion, and skip-scope fixes.

## Goal
Harden parsers against BOM-prefixed files, fix check ID collisions, expand test file pattern detection, fix reason detection logic, and tighten R58 skip scope.

## Approach

### 1. tsconfig_check.rs — BOM + check ID collision
- Strip `\u{FEFF}` BOM before `serde_json::from_str`
- Rename tsconfig IDs: T60→T-TSC-60, T61→T-TSC-61 (jscpd_check.rs owns T60/T61 for content import/velite)

### 2. npmrc_check.rs — quoted values
- In `parse_npmrc_settings`, strip surrounding `"` from values after trimming

### 3. jscpd_check.rs — BOM + silent parse failure
- Strip BOM before JSON parsing
- On `Err(e)` from serde_json, emit Error CheckResult instead of silent return

### 4. source_scan.rs — file length help text + test file patterns
- T32 uses 400 threshold and help text says 400 — consistent, no change needed
- Expand `is_ts_test_file` to match: `__mocks__/`, `*.stories.ts`, `*.stories.tsx`, `*.e2e.ts`, `test/`, `tests/`, `.test.mjs`

### 5. ts_comment_checks.rs — v8 ignore + reason detection
- Add `v8 ignore` to T35 coverage suppression patterns in source_scan.rs (that's where check_comment_pattern is called)
- Fix reason detection: accept `--reason` (no space between -- and text), reject `-- ` followed by nothing

### 6. code_quality_checks.rs — R58 fs.rs skip
- Change `path.ends_with("fs.rs")` to `path.ends_with("src/fs.rs")`

### 7. allow_checks.rs — R36 case-insensitive + rust-toolchain.toml
- Make EXCEPTION match case-insensitive
- Add `rust-toolchain.toml` to config_files list

## Files to Modify
- `src/app/ts/validate/tsconfig_check.rs`
- `src/app/ts/validate/npmrc_check.rs`
- `src/app/ts/validate/jscpd_check.rs`
- `src/app/ts/validate/source_scan.rs`
- `src/app/ts/validate/ts_comment_checks.rs`
- `src/app/rs/validate/code_quality_checks.rs`
- `src/app/rs/validate/allow_checks.rs`
