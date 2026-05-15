# Summary

Implemented the new `g3rs init` and nested `g3rs validate` CLI contract.
Added repo and workspace initialization, managed hook generation, managed hook chain validation, docs, manifest verification, behavior fixture updates, and local behavior ledgers.
Migrated the tracked Git hook to the new thin adapter plus `.githooks/pre-commit.d/g3rs` managed script so commits no longer call the deleted legacy CLI.

# Decisions Made

- Replaced the flat `validate-repo` and `validate --path` surfaces with `validate repo` and `validate workspace --path`.
- Kept `--path` as the single path option across repo and workspace commands.
- Made `.githooks/pre-commit.d/g3rs` the only managed modular hook file that G3RS validates.
- Kept the adapter thin and moved merge-conflict, secret, file-size, repo, and staged workspace validation into the managed G3RS hook script.
- Made `init repo --force` insert or replace only a bounded managed block in project-owned hooks.
- Marked obsolete monolithic-hook rule tests as replaced by the managed hook chain coverage.

# Key Files For Context

- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/init.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs`
- `docs/cli.md`
- `docs/rs/project-shape.md`
- `.plans/2026-05-15-163150-g3rs-cli-contract-init-validate.md.manifest.toml`
- `scripts/verify-g3rs-cli-contract.py`

# Verification

- `scripts/verify-g3rs-cli-contract.py`
- `cargo test --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-rs/crates/types/app-types/Cargo.toml`
- `cargo test --workspace --all-targets --all-features` in `packages/rs/hooks/g3rs-hooks-ingestion`
- `cargo test --workspace --all-targets --all-features` in `packages/rs/hooks/g3rs-hooks-source-checks`
- `cargo clippy --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo clippy --manifest-path apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` in both changed hook workspaces
- `scripts/behavior/verify-all.sh`
- `g3rs validate repo --path .`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --force`

# Next Steps

- Implement the topology and apparch project-shape enforcement rules described in `docs/rs/project-shape.md`.
- Add direct fixture coverage for fixture3-style root workspace shape rejection when those families are updated.
