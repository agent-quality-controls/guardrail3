# Fix all remaining guardrail3 issues

**Date:** 2026-03-15 15:48
**Task:** Fix 9 categories of issues in guardrail3

## Goal
Remove minimal profile, fix pre-commit hook duplication sections, add hook validator warnings, add missing ESLint rule checks, add missing banned TS packages, add missing tsconfig strict settings, exempt test files from R30, and update guardrail3's own config.

## Approach

### 1. Remove minimal profile
- clippy.rs: remove `minimal_profile_methods()`, `minimal_profile_types()`
- deny.rs: remove `minimal_profile_ban_entries()`
- clippy_coverage.rs: remove MINIMAL constants, update match arms to default to service
- deny_bans.rs: remove "minimal" match arm
- generate.rs: remove "minimal" match arm in `build_deny_for_profile`
- init.rs: remove "minimal" match arm in `generate_config_content`
- cli.rs: no explicit profile list in help text, but default is "service" already

### 2. Fix pre-commit hook
- Split PRE_COMMIT_SCRIPT into base + duplication sections
- Add DUPLICATION_CARGO_DUPES and DUPLICATION_JSCPD consts
- In generate.rs, build hook content based on profile/config

### 3. Fix hook validator
- Add duplication tool checks in hook_checks.rs

### 4. Add missing ESLint rule checks
- Add 21 rules to eslint_check.rs using check_eslint_rule_presence

### 5. Add missing banned TS packages
- Add request-promise, postgres, cross-fetch to package_check.rs and source_scan.rs

### 6. Add missing tsconfig strict settings
- Add 5 checks to tsconfig_check.rs

### 7. Exempt test files from R30
- Add is_test check in check_crate_level_allow, demote to Info for test files

### 8. Fix guardrail3.toml
- Change profile from minimal to service

## Files to Modify
- src/modules/clippy.rs, deny.rs, pre_commit.rs
- src/commands/generate.rs, init.rs
- src/rs/validate/clippy_coverage.rs, deny_bans.rs
- src/hooks/hook_checks.rs
- src/ts/validate/eslint_check.rs, package_check.rs, source_scan.rs, tsconfig_check.rs
- src/rs/validate/source_scan.rs
- guardrail3.toml
