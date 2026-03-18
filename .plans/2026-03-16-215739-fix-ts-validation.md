# Fix ALL TypeScript validation issues

**Date:** 2026-03-16 21:57
**Task:** Fix 5 categories of TS validation issues

## Goal
Fix fixture exclusion, kill T33, make T-ARCH-01 skip Rust-only apps, rewrite all TS check messages to what/why/how format, add inventory markers on confirmations.

## Approach

### 1. Exclude tests/fixtures/ from TS source scan
In `source_scan.rs::collect_ts_files()`, add `tests/fixtures/` skip like Rust does.
Also in `test_checks.rs::collect_ts_tsx_files()`.

### 2. Kill T33
Delete the `else if effective_lines > 250` branch in `check_file_length()`.

### 3. T-ARCH-01 skip Rust-only apps
In `ts_arch_checks.rs::discover_ts_apps()`, after checking `has_package_json || has_src`, also walk the directory for .ts/.tsx files. If none found, skip.

### 4. Rewrite ALL TS check messages
Every CheckResult across all 11 files must answer what/why/how.

### 5. Add inventory markers
Any Info result confirming something is correct -> `.as_inventory()`.

## Files to Modify
- `source_scan.rs` - fixture exclusion, kill T33, rewrite messages
- `ts_arch_checks.rs` - Rust-only skip, rewrite messages
- `config_files.rs` - no changes needed (just dispatches)
- `eslint_check.rs` - rewrite messages, inventory markers
- `eslint_audit.rs` - rewrite messages, inventory markers
- `tsconfig_check.rs` - rewrite messages, inventory markers
- `npmrc_check.rs` - rewrite messages, inventory markers
- `package_check.rs` - rewrite messages, inventory markers
- `jscpd_check.rs` - rewrite messages, inventory markers
- `ts_comment_checks.rs` - rewrite messages, inventory markers
- `test_checks.rs` - fixture exclusion, rewrite messages, inventory markers
