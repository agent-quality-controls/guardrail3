# Restore RS-TEST Inventory Contract

**Date:** 2026-03-29 20:05
**Scope:** `.plans/todo/checks/rs/test.md`, `apps/guardrail3/crates/app/rs/families/test/README.md`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_11_cargo_mutants_installed.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_12_mutants_toml_exists.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_13_mutants_profile_present.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_14_mutation_hook_present.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_15_mutants_config_sane.rs`, corresponding mutation-rule sidecar tests`

## Summary
Restored positive inventory reporting for `RS-TEST-11` through `RS-TEST-15`. The previous `RS-TEST` zero sweep preserved failure behavior but incorrectly changed the `--inventory` contract by hiding pass-side mutation findings.

## Context & Problem
After the `RS-TEST` cleanup, repo-root `RS-TEST` reached `0/0/0` under `--inventory` because mutation-adoption rules had been changed to stay quiet on compliant setups. That matched a “quiet baseline” interpretation, but it contradicted the intended inventory semantics: `--inventory` is supposed to show which checks actually ran, including pass-side observations. The user explicitly called this out as wrong.

## Decisions Made

### Restore pass-side inventory for mutation adoption rules
- **Chose:** Reintroduce `Info` inventory emissions for healthy mutation setups in `RS-TEST-11` through `RS-TEST-15`.
- **Why:** `--inventory` is meant to display evaluated checks, not only failures. Mutation rules were the only `RS-TEST` rules where I had diverged from that contract.
- **Alternatives considered:**
  - Keep healthy mutation setups quiet and rely on non-`--inventory` for clean status — rejected because it changes the meaning of inventory.
  - Remove inventory from all positive observations repo-wide — rejected because that is a broader policy change and not what was requested.

### Keep the app-local hook fix
- **Chose:** Leave `apps/guardrail3/.githooks/pre-commit` in place from the prior commit.
- **Why:** That was the correct fix for `RS-TEST-14`; only the reporting contract was wrong.
- **Alternatives considered:**
  - Revert the hook addition too — rejected because it would reintroduce a genuine missing-hook warning.

## Architectural Notes
- `RS-TEST` now has two useful views again:
  - ordinary validation: no warnings/errors on the app root
  - `--inventory`: five positive mutation-adoption infos showing that the setup is present and checked
- This restores consistency with the family’s intended mutation-adoption reporting model without widening any escape hatch.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `.plans/todo/checks/rs/test.md`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_11_cargo_mutants_installed.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_12_mutants_toml_exists.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_13_mutants_profile_present.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_14_mutation_hook_present.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_15_mutants_config_sane.rs`
- `.worklogs/2026-03-29-191808-finish-rs-test-zero.md`

## Open Questions / Future Considerations
- Inventory behavior is still not unified across all finished families; this commit only restores the intended `RS-TEST` contract.
- The repo still has unrelated dirty work in `deps`, `release`, `Cargo.lock`, and `project-tree`. None of that is part of this commit.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_11_cargo_mutants_installed.rs` — tool-availability inventory/warn behavior
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_14_mutation_hook_present.rs` — hook-step inventory/warn behavior
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_15_mutants_config_sane.rs` — sane-config inventory/warn behavior
- `apps/guardrail3/crates/app/rs/families/test/README.md` — family contract including mutation rules
- `.worklogs/2026-03-29-191808-finish-rs-test-zero.md` — prior checkpoint that introduced the contract mistake

## Next Steps / Continuation Plan
1. Commit only the `RS-TEST` inventory-contract restore files and this worklog.
2. Re-run repo-root `RS-TEST` both with and without `--inventory` after the commit to confirm:
   - failures remain clean
   - `--inventory` shows the five mutation infos again
3. Continue the requested audit of finished-family inventory behavior across `cargo`, `clippy`, `arch`, and `hexarch`.
