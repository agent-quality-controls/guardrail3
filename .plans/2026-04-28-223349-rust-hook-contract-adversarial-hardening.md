# Rust hook contract adversarial hardening

## Goal

- Close the adversarial gaps found after commit 766611438.
- Keep hook contract enforcement fail-closed.
- Preserve structured parser/check ownership instead of adding hook-specific string hacks.

## Approach

- Fix .githooks/pre-commit so Rust-triggered validation fails if g3rs is missing and validates the active Rust CLI app without a family filter.
- Tighten hook shell facts for fail-open wrappers by representing return 0 and assignment/export command-substitution wrappers.
- Tighten source checks so critical commands inside fail-open availability guards are detected.
- Tighten modular hook aggregation so only real directory dispatch includes all pre-commit.d files; a single sourced file must not satisfy other modular files.
- Tighten alias handling in required contract command checks using parsed shell words instead of raw prefix matching.
- Add unit tests proving each adversarial gap exists before or with the fix.
- Re-run Rust parser/check/app validation, g3rs validation, install local g3rs, commit with a worklog.

## Files to modify

- .githooks/pre-commit
- packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs
- packages/parsers/hook-shell-parser/crates/runtime/src/support.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/contract_critical_command_not_fail_open/rule.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/required_contract_command_present/rule.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs
- relevant rule sidecar tests
