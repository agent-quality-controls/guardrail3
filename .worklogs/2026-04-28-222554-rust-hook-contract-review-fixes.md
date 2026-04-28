Summary

- Fixed the adversarial hook-contract findings and made the new hook-contract packages self-validating under G3RS.
- Hardened hook command parsing for all-family validation, alias spoofing, fail-open wrappers, modular hook dispatch, concrete lockfile commands, and facade-reexported assertion proof.
- Split `guardrail3-rs-family-runner-process` into runtime/assertions crates because the new process-runner tests must follow the same assertions split as the rest of the app.

Decisions made

- Pre-commit now runs full app validation instead of `--family hooks`; a family-filtered validate command cannot satisfy the hook contract.
- Hook-contract package runtime crates expose contract code through facade-only `lib.rs`, and their tests call sibling assertions crates.
- Assertion facade reexports are recognized as proof-bearing, because `pub use contract::assert_*` is a valid package boundary and should not force tests into private modules.
- Kept a dependency-count waiver for the process runner because it is the deliberate aggregator of independent Rust family packages.

Key files for context

- `.plans/2026-04-28-215223-rust-hook-contract-review-fixes.md`
- `.githooks/pre-commit`
- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/assertions/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/required_contract_command_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/contract_critical_command_not_fail_open/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis/pipeline.rs`

Next steps

- Run adversarial review against the plan and code after this commit.
- Convert any remaining adversarial findings into follow-up fixes before considering the hook-contract migration complete.
