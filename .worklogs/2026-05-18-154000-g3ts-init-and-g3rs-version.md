Summary
- Wired `g3ts init repo` and `g3ts init workspace`.
- Added `g3rs --version`.
- Fixed the repo validation issues found while testing the new G3TS command surface.

Decisions made
- `g3ts init repo` writes a managed `.githooks/pre-commit.d/g3ts` file and inserts only a small managed block into `.githooks/pre-commit`.
- `g3ts init repo` refuses to overwrite an unmanaged `.githooks/pre-commit` unless `--force` is passed.
- `g3ts init workspace` writes `guardrail3-ts.toml` beside the target workspace `package.json`.
- G3TS hook validation now validates the effective managed G3TS hook body when the root pre-commit hook dispatches into `.githooks/pre-commit.d/g3ts`.
- Generic TS hook checks no longer require Drizzle migration commands. That policy belongs in a database or migration-specific family, not in every TS repo.
- Repo walking now ignores generated and fixture-output directories that should not be interpreted as real workspaces.

Key files for context
- `.plans/2026-05-18-151325-g3ts-init-and-g3rs-version.md`
- `.plans/2026-05-18-151325-g3ts-init-and-g3rs-version.md.manifest.toml`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/fs.rs`
- `packages/ts/hooks/g3ts-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `scripts/verify-g3ts-init-and-g3rs-version.py`
- `scripts/verify-g3ts-validate-command-surface.py`

Verification
- `cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml`
- `cargo test --workspace --manifest-path apps/guardrail3-rs/Cargo.toml`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-ts/Cargo.toml -- -D warnings`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-rs/Cargo.toml -- -D warnings`
- `python3 scripts/verify-g3ts-init-and-g3rs-version.py`
- `python3 scripts/verify-g3ts-validate-command-surface.py`
- `fixture3 check --all`
- `g3ts validate repo --path .`
- `g3rs validate repo --path .`
- Reinstalled local `g3ts` and `g3rs`, then reran both repo validators.

Next steps
- If database migration policy is needed, create a dedicated TS database or migrations family instead of adding it back to generic hooks.
