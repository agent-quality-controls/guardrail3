# Update `ts init` to auto-detect apps and generate project-specific config

**Date:** 2026-03-17 20:21
**Task:** Make `guardrail3 ts init` analyze the project, discover apps, auto-detect types, and generate a project-specific config instead of a generic template.

## Goal
Instead of generating a hardcoded `[typescript.apps.my-app] type = "service"` template, `ts init` should use `discover_ts_apps` and `auto_detect_app_type` to produce a config that reflects the actual project structure.

## Approach

### Step 1: Make `discover_ts_apps` and `auto_detect_app_type` publicly accessible
- In `ts_arch_checks.rs`: change `pub(super) fn discover_ts_apps` to `pub fn discover_ts_apps`
- In `mod.rs`: change `fn auto_detect_app_type` to `pub fn auto_detect_app_type`

### Step 2: Update `run_ts` in `init.rs`
- Replace hardcoded `ts_section` with dynamic generation via `generate_ts_section`
- Add `generate_ts_section` function that discovers apps and auto-detects types
- Add `detect_reason` function for human-readable comments

### Step 3: Add necessary imports to `init.rs`
- `FileSystem` trait, `RealFileSystem`, `TsAppType`

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/ts_arch_checks.rs` -- make `discover_ts_apps` pub
- `apps/guardrail3/src/app/ts/validate/mod.rs` -- make `auto_detect_app_type` pub
- `apps/guardrail3/src/commands/init.rs` -- replace hardcoded template with dynamic generation
