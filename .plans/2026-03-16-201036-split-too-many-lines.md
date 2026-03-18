# Split functions with #[allow(clippy::too_many_lines)]

**Date:** 2026-03-16 20:10
**Task:** Find and split all functions with `#[allow(clippy::too_many_lines)]` in rs/validate/, hooks/, and discover.rs

## Goal
Remove all `#[allow(clippy::too_many_lines)]` by splitting long functions into smaller sub-functions (each under 100 lines).

## Approach
For each function, extract logical sections into helper functions. The original function becomes an orchestrator that calls the helpers.

## Analysis of each function

Most of these functions are already under or near 100 lines. Several are validation functions that check multiple fields sequentially - these split naturally into per-field or per-section helpers.

Functions that are genuinely long and need splitting:
1. `hook_checks.rs:check_hooks` (~200 lines) - split into H1-H4, H5-H6, H7-H11
2. `hook_checks.rs:check_monolithic_patterns` (~120 lines) - data-driven, mostly const array
3. `discover.rs:detect_rust` (~100 lines) - split workspace parsing from exclude parsing
4. `release_repo_checks.rs:check_release_plz_toml` (~110 lines) - split file reading from content validation
5. `rustfmt_check.rs:check_rustfmt_settings` (~158 lines) - split by value type checking
6. `cargo_lints.rs:check_lint_level` (~100 lines) - borderline but can extract match arms
7. `deny_audit.rs:check_advisory_values` (~110 lines) - split unmaintained/yanked checks
8. `deny_licenses.rs:check_licenses` (~108 lines) - split license list from private/confidence
9. `deny_licenses.rs:check_sources` (~87 lines) - split registry/git checks
10. `dependency_scan.rs:check_cargo_lock` (~89 lines) - split read/parse from scan
11. `deny_bans.rs:check_ban_list` (~132 lines) - split settings from ban list check
12. `toolchain_check.rs:check_toolchain_settings` (~122 lines) - split channel from components
13. `workspace_metadata.rs:check_workspace_metadata` (~77 lines) - borderline
14. `config_files.rs:check_clippy_thresholds` (~88 lines) - borderline
15. `mod.rs:run` (~173 lines) - split into per-domain helpers

## Files to Modify
- `apps/guardrail3/src/app/rs/validate/dependency_scan.rs`
- `apps/guardrail3/src/app/rs/validate/deny_licenses.rs`
- `apps/guardrail3/src/app/rs/validate/release_repo_checks.rs`
- `apps/guardrail3/src/app/hooks/hook_checks.rs`
- `apps/guardrail3/src/app/rs/validate/deny_bans.rs`
- `apps/guardrail3/src/app/rs/validate/deny_audit.rs`
- `apps/guardrail3/src/app/rs/validate/mod.rs`
- `apps/guardrail3/src/app/rs/validate/toolchain_check.rs`
- `apps/guardrail3/src/app/rs/validate/workspace_metadata.rs`
- `apps/guardrail3/src/app/rs/validate/rustfmt_check.rs`
- `apps/guardrail3/src/app/rs/validate/cargo_lints.rs`
- `apps/guardrail3/src/app/discover.rs`
- `apps/guardrail3/src/app/rs/validate/config_files.rs`
