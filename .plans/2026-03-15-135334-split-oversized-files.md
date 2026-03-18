# Split 4 oversized Rust files under 500 effective lines

**Date:** 2026-03-15 13:53
**Task:** Split 4 files exceeding 500 lines into sub-modules

## Goal
Each of the 4 files should be under 500 effective lines. No logic changes — pure code movement to new sub-module files.

## Approach

### 1. src/rs/validate/config_files.rs (564 lines → 4 files)
- `config_files.rs` — orchestrator `check()`, `check_per_crate_clippy()`, `check_per_crate_clippy_content()`, `check_clippy_thresholds()`
- `rustfmt_check.rs` — `check_rustfmt_settings()` (R21-R23)
- `toolchain_check.rs` — `check_toolchain_settings()` (R24-R25)
- `workspace_metadata.rs` — `check_workspace_metadata()` (R55-R57)

### 2. src/rs/validate/deny_audit.rs (651 lines → 4 files)
- `deny_audit.rs` — orchestrator `check()`, `check_advisory_values()`, deprecated fields (R8-R11)
- `deny_bans.rs` — `check_ban_list()`, `check_tokio_feature_ban()` (R10-R13, R17-R18)
- `deny_licenses.rs` — `check_licenses()`, `check_sources()` (R14-R16)
- `deny_inventory.rs` — `check_skip_entries()`, `check_advisory_ignores()` (R19-R20)

### 3. src/hooks/validate.rs (745 lines → 3 files)
- `validate.rs` — orchestrator `run()`, `has_railpack_files()`
- `hook_checks.rs` — `check_hooks()` + all hook helper functions (H1-H11)
- `deploy_checks.rs` — `check_deployment()` + all deploy helper functions (D1-D5)

### 4. src/ts/validate/config_files.rs (837 lines → 6 files)
- `config_files.rs` — orchestrator `check()`
- `eslint_check.rs` — `check_eslint_config()`, `check_eslint_rule()`, `check_eslint_rule_presence()` (T1-T8, T40-T51)
- `tsconfig_check.rs` — `check_tsconfig()` (T9-T10, T52-T54)
- `npmrc_check.rs` — `check_npmrc()` (T11-T14)
- `package_check.rs` — `check_package_json()` (T15-T18, T55-T58)
- `jscpd_check.rs` — `check_jscpd()`, `check_content_import_restriction()`, `check_velite_config()` (T19-T22, T60-T61)

### Also: Fix .unwrap() in package_check.rs
Replace `ov.as_object().unwrap()` with `let Some(ov_obj) = ov.as_object() else { continue; };`

## Files to Modify
- `src/rs/validate/mod.rs` — add new module declarations
- `src/rs/validate/config_files.rs` — trim to orchestrator
- New: `rustfmt_check.rs`, `toolchain_check.rs`, `workspace_metadata.rs`
- `src/rs/validate/deny_audit.rs` — trim to orchestrator
- New: `deny_bans.rs`, `deny_licenses.rs`, `deny_inventory.rs`
- `src/hooks/mod.rs` — add new module declarations
- `src/hooks/validate.rs` — trim to orchestrator
- New: `hook_checks.rs`, `deploy_checks.rs`
- `src/ts/validate/mod.rs` — add new module declarations
- `src/ts/validate/config_files.rs` — trim to orchestrator
- New: `eslint_check.rs`, `tsconfig_check.rs`, `npmrc_check.rs`, `package_check.rs`, `jscpd_check.rs`
