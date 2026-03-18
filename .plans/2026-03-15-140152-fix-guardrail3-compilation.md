# Fix 21 compilation errors in guardrail3

**Date:** 2026-03-15 14:01
**Task:** Fix all 21 compilation errors caused by strict workspace lints (dead_code, unused_results)

## Goal
Zero compilation errors from `cargo build`.

## Approach

### 1. Dead code — 2 unused functions
- `discover_workspace_crates` in generate.rs — remove it (not called anywhere)
- `tokio_feature_ban` in deny.rs — remove it (not called anywhere)

### 2. Dead code — unused struct fields in config/types.rs
- Add `#[allow(dead_code)]` with reason comment to structs with unused fields:
  - `GuardrailConfig` (version, hooks)
  - `RustConfig` (workspaces)
  - `TypeScriptConfig` (apps, migrations)
  - `HooksConfig` (extra_dir)

### 3. Unused results — BTreeSet::insert() and Map::insert() return values
- Wrap all `.insert()` calls on BTreeSet/Map in `let _ = ...`
- Files: discover.rs, report/json.rs, clippy_coverage.rs, deny_bans.rs, rustfmt_check.rs

## Files to Modify
- `src/config/types.rs` — add dead_code allows
- `src/commands/generate.rs` — remove unused function
- `src/modules/deny.rs` — remove unused function
- `src/discover.rs` — wrap 2 inserts
- `src/report/json.rs` — wrap 6 inserts
- `src/rs/validate/clippy_coverage.rs` — wrap 2 inserts
- `src/rs/validate/deny_bans.rs` — wrap 2 inserts
- `src/rs/validate/rustfmt_check.rs` — wrap 3 inserts
