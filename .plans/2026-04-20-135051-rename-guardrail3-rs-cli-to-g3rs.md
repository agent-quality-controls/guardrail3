## Goal

Rename the active Rust CLI from `guardrail3-rs` to `g3rs` so the live binary, clap command name, and local tests all use `g3rs`.

## Approach

1. Rename the CLI runtime package from `guardrail3-rs` to `g3rs`.
2. Update direct crate references in the CLI runtime `main.rs`.
3. Update any path dependencies that still point at the old package name.
4. Update clap command naming and CLI argument tests.
5. Regenerate the app lockfile via Cargo if needed.
6. Run focused verification on `apps/guardrail3-rs`.

## Key Decisions

- Rename the actual CLI package, not just the clap display name.
  - Reason: the user asked for the command to be `g3rs`, and keeping the package name as `guardrail3-rs` would keep `cargo run -p ...` and `cargo install` inconsistent.
- Keep internal library crate names like `guardrail3-rs-app-types` unchanged for now.
  - Reason: the user asked only for the CLI command rename, not a full app crate rename sweep.

## Files To Modify

- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/main.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli_tests/cases.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/assertions/Cargo.toml`
- `apps/guardrail3-rs/guardrail3-rs.toml`
- `apps/guardrail3-rs/Cargo.lock`

## Verification

- `cargo test -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs --workspace`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- --help`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --help`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path apps/guardrail3-rs`
