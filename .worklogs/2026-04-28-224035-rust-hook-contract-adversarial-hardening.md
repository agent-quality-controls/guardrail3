# Rust hook contract adversarial hardening

## Summary

Fixed the first adversarial review findings against Rust hook contracts. The hook parser now models more fail-open wrappers, hook checks reject critical command availability guards that skip on missing tools, modular hook aggregation only trusts directory dispatch, and the root pre-commit fails closed when Rust validation needs g3rs.

## Decisions made

- Kept fail-open detection in the shared hook shell parser and hooks source checks instead of patching one hook script shape.
- Kept pre-commit validation scoped to the active Rust app because package-local validation currently exposes separate migration debt in packages that are not clean package roots yet.
- Reused parsed shell words for alias and dispatcher analysis instead of adding more raw substring checks.

## Key files for context

- .githooks/pre-commit
- packages/parsers/hook-shell-parser/crates/runtime/src/support.rs
- packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/contract_critical_command_not_fail_open/rule.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/required_contract_command_present/rule.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs

## Verification

- cargo test --manifest-path packages/parsers/hook-shell-parser/Cargo.toml --workspace --offline --locked
- cargo test --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml --workspace --offline --locked
- cargo test --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --offline --locked
- cargo clippy --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --all-targets --all-features -- -D warnings
- g3rs validate --path apps/guardrail3-rs --family hooks --inventory
- g3rs validate --path apps/guardrail3-rs
- cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --locked --force

## Next steps

- Run another adversarial review after TypeScript hook contracts are implemented so the Rust and TypeScript hook contract systems are checked together.
