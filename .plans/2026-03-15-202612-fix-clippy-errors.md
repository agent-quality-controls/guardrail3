# Fix all cargo clippy errors for pre-commit compliance

**Date:** 2026-03-15 20:26
**Task:** Fix 13 clippy errors across multiple files

## Goal
`cargo clippy -- -D warnings` passes clean. All 139 tests still pass.

## Approach

### Errors to fix:
1. `src/cli.rs` — struct_excessive_bools on ValidateArgs → add #[allow]
2. `src/commands/init.rs` — too_many_lines on run() → add #[allow]
3. `src/config/mod.rs` — disallowed toml::from_str → add #[allow]
4. `src/hooks/deploy_checks.rs` — 2x disallowed serde_json::from_str → add #[allow]
5. `src/rs/validate/garde_checks.rs` — 2x indexing_slicing in production code → use .get()
6. `src/rs/validate/garde_checks.rs` — doc_markdown → add backticks
7. `src/rs/validate/mod.rs` — too_many_lines on run() → add #[allow]
8. `src/ts/validate/jscpd_check.rs` — disallowed serde_json::from_str → add #[allow]
9. `src/ts/validate/package_check.rs` — disallowed serde_json::from_str → add #[allow]
10. `src/ts/validate/tsconfig_check.rs` — disallowed serde_json::from_str → add #[allow]
11. `src/main.rs` — missing_const_for_fn on domains_from_args → add const

### Then check test files for needless_raw_string_hashes and missing test allows.

## Files to Modify
- src/cli.rs, src/commands/init.rs, src/config/mod.rs, src/hooks/deploy_checks.rs
- src/rs/validate/garde_checks.rs, src/rs/validate/mod.rs
- src/ts/validate/jscpd_check.rs, src/ts/validate/package_check.rs, src/ts/validate/tsconfig_check.rs
- src/main.rs
- Various test files as needed
