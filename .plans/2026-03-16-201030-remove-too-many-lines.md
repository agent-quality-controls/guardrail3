# Remove #[allow(clippy::too_many_lines)] from ts/validate + commands + main.rs

**Date:** 2026-03-16 20:10
**Task:** Find and remove all `#[allow(clippy::too_many_lines)]` by splitting long functions into sub-functions.

## Goal
Remove all 5 instances of `#[allow(clippy::too_many_lines)]` in the specified directories by extracting helper functions so each is under 100 lines.

## Approach

### 1. main.rs (line 33) — already short enough
The `main()` function is only ~20 lines. The dispatch is already split into `handle_rs`/`handle_ts`. Just remove the allow.

### 2. eslint_check.rs (line 9) — split check_eslint_config into sub-functions
The function is ~274 lines. Split into:
- `check_eslint_config` — orchestrator (reads file, calls helpers)
- `check_eslint_boundary_enforcement` — T6
- `check_relaxed_rules` — T7
- `check_file_overrides` — T8
- `check_test_relaxations` — T49
- `check_route_wrappers` — T50
- `check_process_env_ban` — T51

### 3. eslint_audit.rs (line 6) — split check into sub-functions
The function is ~117 lines. Split into:
- `check` — orchestrator
- `check_zone_definitions` — T36
- `check_import_direction` — T37
- `check_entry_point` — T38
- `check_external_deps` — T39

### 4. npmrc_check.rs (line 8) — split check_npmrc into sub-functions
The function is ~120 lines. Split into:
- `check_npmrc` — orchestrator (reads file, parses settings, calls helpers)
- `parse_npmrc_settings` — parse key=value pairs
- `check_expected_settings` — T12/T13
- `check_extra_settings` — T14

### 5. commands/init.rs (line 8) — split run_rs into sub-functions
The function is ~108 lines. Split into:
- `run_rs` — orchestrator
- `scaffold_config` — write guardrail3.toml
- `scaffold_local_dir` — write local/ override files
- `scaffold_release_files` — write release config for service profile

## Files to Modify
- `apps/guardrail3/src/main.rs` — remove allow (function already short)
- `apps/guardrail3/src/app/ts/validate/eslint_check.rs` — split + remove allow
- `apps/guardrail3/src/app/ts/validate/eslint_audit.rs` — split + remove allow
- `apps/guardrail3/src/app/ts/validate/npmrc_check.rs` — split + remove allow
- `apps/guardrail3/src/commands/init.rs` — split + remove allow
