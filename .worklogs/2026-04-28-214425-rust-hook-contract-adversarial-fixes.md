Summary

- Hardened the Rust hook contract implementation after adversarial review.
- Fixed command matching, fail-open detection, modular hook aggregation, config trigger handling, release README triggers, and local hook validation.

Decisions made

- `G3RsValidatePath` no longer accepts `--family ...`, because a family-filtered validate command cannot satisfy all-family hook contracts.
- `ConcreteLockfileCommand` now means `cargo metadata --locked`, not any Node or package-manager lockfile command.
- Modular `.githooks/pre-commit.d/*` files are aggregated only for hook contract command presence; per-file source checks still run on each hook file.
- Compact one-line shell conditionals are normalized before trigger coverage is evaluated so an `else` validation branch does not satisfy a config-changing `then` branch.
- Critical `if ! command; then ... fi` wrappers now fail if the failure branch does not terminate non-zero.

Key files for context

- `.plans/2026-04-28-210833-rust-hook-contract-adversarial-fixes.md`
- `.githooks/pre-commit`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/required_contract_command_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/guardrail_validate_staged_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/config_changes_trigger_validation/support/logic.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/logic/family-runner-process/src/run.rs`

Next steps

- Run adversarial review against the plan and implementation.
- Decide whether the remaining hook-family warnings should become hard errors after full repo validation is clean enough for pre-commit.
