# Fix all 11 guardrail3 issues

**Date:** 2026-03-15 13:41
**Task:** Fix 11 issues across the guardrail3 codebase

## Goal
Fix all 11 reported issues: monorepo config false positives, hardcoded paths, profile-aware validation, single-crate support, directory exclusions, npmrc validation, process.env suppression, monorepo profile, init differentiation, eslint-disable-line, and unreachable! noise.

## Approach
Apply fixes sequentially to each file, then verify with cargo build and runtime tests.

## Files to Modify
- `src/rs/validate/mod.rs` — load guardrail3.toml for profile, pass to checks
- `src/rs/validate/clippy_coverage.rs` — profile-aware expected bans
- `src/rs/validate/deny_audit.rs` — profile-aware expected bans
- `src/rs/validate/source_scan.rs` — .claude exclusion, unreachable! in tests
- `src/rs/validate/cargo_lints.rs` — (already uses workspace_root correctly)
- `src/discover.rs` — single-crate workspace_member_dirs fix
- `src/modules/pre_commit.rs` — RUST_WORKSPACE variable
- `src/commands/generate.rs` — pre-commit hook workspace root replacement, monorepo deny
- `src/commands/init.rs` — profile-differentiated init
- `src/modules/clippy.rs` — monorepo match arm
- `src/modules/deny.rs` — (already handled)
- `src/ts/validate/source_scan.rs` — .claude exclusion, eslint-disable-line, process.env suppression
- `src/ts/validate/config_files.rs` — public-hoist-pattern in npmrc
