# Runtime Contract Guardrails

## Summary

Added runtime compatibility guardrails for both TypeScript and Rust so package/toolchain contracts cannot claim a lower runtime than the installed dependency/tooling surface requires.

TypeScript now checks the root Node engine against `pnpm-workspace.yaml` `nodeVersion` and `engineStrict`. Rust now checks Cargo resolver behavior against MSRV-aware resolution and requires `cargo msrv verify --rust-version <workspace rust-version> -- cargo check --locked` through the Rust hook contract.

## Decisions Made

- Used `serde_norway` for `pnpm-workspace.yaml` parsing.
- Rejected `yaml-rust2` because it introduced duplicate dependency versions rejected by G3RS.
- Rejected `serde_yml` because `cargo deny` reports it as unsound and archived.
- Used `js-semver` for npm semver range parsing.
- Rejected `nodejs-semver` because it pulled `miette` and introduced a duplicate `unicode-width` version in the G3TS app lockfile.
- Kept TypeScript runtime compatibility in `g3ts-package`, because the failure is a package/install contract issue.
- Kept Rust resolver compatibility in `g3rs-cargo`, because resolver semantics are Cargo configuration.
- Kept Rust dependency MSRV verification in `g3rs-deps`, because `cargo-msrv` is dependency/tool compatibility verification.
- Extended the Rust hook contract instead of hardcoding hook source checks, so hook validation still derives from family-owned requirements.
- The G3RS app declares Rust 1.85 and the G3TS app declares Rust 1.88. `apps/guardrail3-ts` cannot honestly claim 1.86 because its locked dependency graph includes `ar_archive_writer 0.5.0`, which fails under 1.86 and 1.87.
- The G3RS in-binary cargo gate derives the concrete `--rust-version` value from the adopted workspace `Cargo.toml`, instead of using one hardcoded Rust version for every workspace.

## Key Files

- `.plans/2026-05-18-094322-runtime-contract-guardrails.md`
- `.plans/2026-05-18-094322-runtime-contract-guardrails.md.manifest.toml`
- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/node_engine_install_contract.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rust_version_aware_resolver.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/cargo_msrv_verify_installed/rule.rs`
- `packages/rs/deps/g3rs-deps-hook-contract/crates/runtime/src/contract.rs`
- `packages/rs/hooks/g3rs-hooks-contract-types/src/types.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/required_contract_command_present/rule.rs`
- `scripts/behavior/verify-runtime-contract-guardrails.py`

## Verification

- `fixture3 check --all`
- `python3 scripts/behavior/verify-runtime-contract-guardrails.py`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`
- `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `cargo check --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --all-targets --all-features`
- `cargo check --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --all-targets --all-features`
- `cargo clippy --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo clippy --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo msrv verify --rust-version 1.85 -- cargo check --locked` in `apps/guardrail3-rs`
- `cargo msrv verify --rust-version 1.88 -- cargo check --locked` in `apps/guardrail3-ts`
- `g3rs validate workspace --path apps/guardrail3-rs --staged`
- `g3rs validate workspace --path apps/guardrail3-ts --staged`
- `g3rs validate workspace --path packages/ts/package/g3ts-package-ingestion`
- `g3rs validate workspace --path packages/ts/package/g3ts-package-config-checks`
- `g3rs validate workspace --path packages/rs/cargo/g3rs-cargo-ingestion`
- `g3rs validate workspace --path packages/rs/cargo/g3rs-cargo-config-checks`
- `g3rs validate workspace --path packages/rs/deps/g3rs-deps-ingestion`
- `g3rs validate workspace --path packages/rs/deps/g3rs-deps-config-checks`
- `g3rs validate workspace --path packages/rs/deps/g3rs-deps-hook-contract`
- `g3rs validate workspace --path packages/rs/hooks/g3rs-hooks-contract-types`
- `g3rs validate workspace --path packages/rs/hooks/g3rs-hooks-config-checks`
- `g3rs validate workspace --path packages/rs/hooks/g3rs-hooks-source-checks`

## Next Steps

- Run the new G3TS package rule against a real package that declares `engines.node` below its pinned `pnpm-workspace.yaml` `nodeVersion`.
- Decide separately whether G3TS should grow a repo setup generator for current hook contracts; the unrelated untracked debug plan in this worktree records that question.
