# Suppress cargo metadata JSON in G3RS validation output

## Goal

`g3rs validate --path <workspace>` must not print raw `cargo metadata` JSON during normal validation.

The lockfile gate must still run and still fail when `cargo metadata --locked` fails.

## Approach

- Fix the command execution boundary in `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates.rs`.
- Keep stderr inherited for all cargo gates so real command errors stay visible.
- Suppress stdout only for the lockfile metadata command because its successful stdout is machine JSON, not user-facing validation output.
- Do not change family hook contracts and do not remove the lockfile gate.

## Test

- Add a unit test in `cargo_gates_tests/cases.rs` proving `cargo metadata --locked` is classified as stdout-suppressed.
- Keep existing command-sequence tests unchanged so the gate still runs.

## Files To Modify

- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates_tests/cases.rs`

## Verification

- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-validate-command-runtime cargo_gates`
- `cargo build --release --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/parsers/g3rs-toml-parser` output must not contain JSON metadata beginning with `{"packages"`.
- `.githooks/pre-commit`
