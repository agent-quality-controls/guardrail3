Summary

Implemented Rust hook contracts as owner-family packages and wired the hooks family to consume those contracts. Added contract-derived hook checks for required commands, critical fail-open wrappers, trigger coverage, and required tools while preserving the existing hook rules.

Decisions made

- Hook requirements live in `g3rs-<family>-hook-contract` packages, not ingestion or checks, because they are policy contracts owned by each family.
- `g3rs-hooks-contract-types` owns the shared contract structs so owner families and hooks checks do not depend on each other directly.
- `contract-trigger-coverage` warns instead of reporting success because the current shell parser cannot fully prove staged-file condition coverage without falling back to raw text matching.
- Fixed shell parser regressions at the parser layer: command substitutions, quoted shell payloads, env split-string assignment payloads, and negated command text handling now support existing hook checks without weakening tests.

Key files for context

- `packages/rs/hooks/g3rs-hooks-contract-types/src/lib.rs`
- `packages/rs/*/g3rs-*-hook-contract/crates/runtime/src/lib.rs`
- `apps/guardrail3-rs/crates/logic/family-runner-process/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/required_contract_command_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/contract_critical_command_not_fail_open/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/contract_trigger_coverage/rule.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/contract_required_tools_installed/rule.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/wrappers.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`

Verification

- `cargo test --manifest-path packages/parsers/hook-shell-parser/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/rs/hooks/g3rs-hooks-config-checks/Cargo.toml --workspace --offline --locked`
- `cargo check --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --offline`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --offline --locked`
- Contract package tests for all Rust hook-contract packages with shared `CARGO_TARGET_DIR=/tmp/guardrail3-contract-target`
- Final adversarial review returned `CLEAN`

Next steps

- The repository hook itself still needs to be updated later to satisfy the new hook contract findings. The current commit implements the guardrails and tests, not the repo's hook script remediation.
