# Fix 5 output/consistency issues

**Date:** 2026-03-17 10:41
**Task:** Fix 5 issues: relative paths, JSON counts, T32 severity, RS gitignore, TS test file skip

## Goal
Fix 5 output and consistency issues across report formatting, check severity, and file collection logic.

## Approach

### 1. Absolute paths to relative (text.rs, markdown.rs, json.rs)
- `Report.project_path` holds the project root string
- In text.rs and markdown.rs: when displaying file paths, strip project_path prefix
- In json.rs: keep `file` field absolute, add `file_relative` field
- Helper: create a `strip_project_root` function or do inline

### 2. JSON summary counts mismatch
- json.rs currently filters results by `show_inventory || !r.inventory`
- Remove this filter — always include ALL results in JSON
- The `inventory` boolean on each result lets consumers filter

### 3. T32 severity Warn→Error
- In ts/validate/source_scan.rs, change T32 severity from `Severity::Warn` to `Severity::Error`

### 4. RS collect_rs_files needs gitignore
- Port pattern from TS: call `load_gitignore_dirs(fs, root)` and use gitignore-aware filtering
- Problem: `collect_rs_files` doesn't take `fs` parameter — need to add it
- Add `is_excluded_dir_with_gitignore` variant like TS has

### 5. TS skip test files from source scan
- Add `is_ts_test_file()` helper
- In the source scan loop, skip T23-T31 checks (eslint-disable, ts-ignore, process.env, any types) for test files
- Test files: *.test.ts, *.spec.ts, *.test.tsx, *.spec.tsx, files in __tests__/

## Files to Modify
- `apps/guardrail3/src/report/text.rs` — strip project root from file paths
- `apps/guardrail3/src/report/markdown.rs` — strip project root from file paths
- `apps/guardrail3/src/report/json.rs` — remove inventory filter, add file_relative
- `apps/guardrail3/src/app/ts/validate/source_scan.rs` — T32 severity, test file skip
- `apps/guardrail3/src/app/rs/validate/source_scan.rs` — add gitignore support
