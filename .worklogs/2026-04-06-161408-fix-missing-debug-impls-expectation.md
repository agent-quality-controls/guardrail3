# Fix missing_debug_implementations expected level

**Date:** 2026-04-06 16:14
**Scope:** g3rs-cargo-config-checks support.rs + golden fixture

## Summary
The check expected `missing_debug_implementations = "warn"` but every Cargo.toml in the repo uses `"deny"`. Fixed the expected level and golden fixture to match reality. Verified all conforming packages now produce clean (all-INFO) output.

## What was wrong
- `support.rs` EXPECTED_RUST_LINTS had `expected_level: "warn"` for `missing_debug_implementations`
- The golden fixture `golden_workspace.toml` also had `"warn"`
- The `missing_clippy.rs` test fixture also had `"warn"`
- Every real Cargo.toml (21 files across packages and apps) uses `"deny"`
- Result: every package falsely flagged with "deviates from policy"

## Remaining legitimate findings
- `reason-policy`: missing rust and clippy lint tables (real — needs config)
- `apps/guardrail3`: missing `unreachable_pub` in workspace rust lints (real — needs adding)

## Key Files
- `packages/g3rs-cargo-config-checks/crates/runtime/src/support.rs` — expected level fix
- `packages/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml` — fixture fix
