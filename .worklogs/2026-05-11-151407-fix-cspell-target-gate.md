Summary
- Fixed the G3TS spelling toolchain gate so it runs `cspell . --no-progress --no-summary` instead of bare `cspell --no-progress --no-summary`.
- Tightened spelling package-script validation so `spellcheck` and direct `validate` cspell invocations must include an explicit file, directory, or glob target.

Decisions made
- Fixed the runnable gate in `g3ts-hooks-contract-types` because toolchain gates are generated from hook command requirements.
- Fixed package-script validation in `g3ts-spelling-config-checks` because G3TS previously accepted scripts that would fail the same way as the internal gate.
- Kept the check at argv-token level because package-script parsing is already delegated to the shared command parser.

Key files for context
- `packages/ts/hooks/g3ts-hooks-contract-types/src/contract.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates_tests/cases.rs`
- `packages/ts/spelling/g3ts-spelling-config-checks/crates/runtime/src/common.rs`
- `packages/ts/spelling/g3ts-spelling-config-checks/crates/runtime/src/spellcheck_fail_closed_tests/cases.rs`
- `packages/ts/spelling/g3ts-spelling-config-checks/crates/runtime/src/validate_runs_spellcheck_tests/cases.rs`

Verification
- `cargo test --manifest-path packages/ts/spelling/g3ts-spelling-config-checks/Cargo.toml --workspace`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `g3rs validate --path packages/ts/hooks/g3ts-hooks-contract-types`
- `g3rs validate --path packages/ts/spelling/g3ts-spelling-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --force`

Next steps
- None.
