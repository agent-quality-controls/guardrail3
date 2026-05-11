Summary
- Suppressed successful `cargo metadata` stdout in `g3rs validate` so raw package JSON no longer appears in validator output.
- Updated the lockfile gate command to `cargo metadata --locked --format-version 1` so Cargo does not emit the format-version warning.

Decisions made
- Fixed the output leak at the cargo gate runner boundary instead of filtering rendered validator output.
- Kept stderr inherited so real cargo failures still show actionable errors.
- Suppressed stdout only for the concrete lockfile metadata gate because its successful stdout is machine JSON.

Key files for context
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates_tests/cases.rs`
- `packages/rs/hooks/g3rs-hooks-contract-types/src/types.rs`

Verification
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-validate-command cargo_gates`
- `cargo build --release --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --force`
- `g3rs validate --path packages/parsers/g3rs-toml-parser` exits 0 and output contains neither raw metadata JSON nor the Cargo format-version warning.

Next steps
- None.
