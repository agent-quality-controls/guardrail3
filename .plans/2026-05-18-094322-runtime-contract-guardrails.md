# Runtime Contract Guardrails

## Goal

Make dependency-runtime compatibility fail before CI discovers it.

For TypeScript, G3TS must require the package manager to check dependency `engines` against the lowest Node version the project claims to support.

For Rust, G3RS must keep the existing split: Cargo owns manifest resolver/MSRV policy, toolchain owns local toolchain compatibility, and deps owns the external MSRV verification tool.

## Approach

- Add a TS package-family rule named `g3ts-package/node-engine-install-contract`.
- Parse `pnpm-workspace.yaml` in `g3ts-package-ingestion` using `serde_norway`.
- Add typed package facts for:
  - `pnpm-workspace.yaml` presence and parse state
  - `nodeVersion`
  - `engineStrict`
- In `g3ts-package-config-checks`, require:
  - `pnpm-workspace.yaml` exists at the package-manager root
  - `engineStrict: true`
  - `nodeVersion` is an exact semantic version
  - root `package.json` `engines.node` parses as a version requirement
  - root `engines.node` accepts `nodeVersion`
- Update the G3TS package fixtures so the clean fixture carries the new contract and the broken policy fixture emits the new rule.
- Add a Rust cargo-family rule named `g3rs-cargo/rust-version-aware-resolver`.
- In `g3rs-cargo-ingestion`, parse root `.cargo/config.toml` / `.cargo/config` with `cargo-config-toml-parser`.
- In `g3rs-cargo-config-checks`, require resolver 3, or resolver 2 plus `[resolver] incompatible-rust-versions = "fallback"`.
- Add a Rust deps-family rule named `g3rs-deps/cargo-msrv-verify-installed`.
- In `g3rs-deps-config-checks`, require `cargo-msrv` on PATH for workspace-tooling inputs.
- Extend `g3rs-deps-hook-contract` so Cargo manifest and lockfile changes require `cargo msrv verify --rust-version <workspace rust-version> -- cargo check --locked`.
- The in-binary G3RS cargo gate runner must read the adopted workspace `Cargo.toml` and pass that workspace's declared `rust-version` to `cargo-msrv`.

## Key Decisions

- TS does not reimplement npm dependency engine graph checks. pnpm owns that through `nodeVersion` plus `engineStrict`.
- TS package family owns the package-manager contract because it is `package.json` plus `pnpm-workspace.yaml` policy.
- TS hooks only need to ensure package/lockfile edits route through existing package validation. The package family owns the required settings.
- Rust does not use one combined rule because the ecosystem stores the contract in separate places.
- Cargo config parsing must use `cargo-config-toml-parser`; no raw string checks.
- pnpm workspace parsing must use `serde_norway`; no line scanning.
- Node semver range parsing must use `js-semver`; `nodejs-semver` pulls `miette` and duplicates `unicode-width` in the G3TS app lockfile.

## Files To Modify

- `packages/ts/package/g3ts-package-types/src/types.rs`
- `packages/ts/package/g3ts-package-types/src/lib.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run.rs`
- `packages/ts/package/g3ts-package-ingestion/Cargo.toml`
- `packages/ts/package/g3ts-package-ingestion/guardrail3-rs.toml`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/lib.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/run.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/node_engine_install_contract.rs`
- `packages/ts/package/g3ts-package-config-checks/Cargo.toml`
- `packages/ts/package/g3ts-package-config-checks/guardrail3-rs.toml`
- `behavior/fixtures/g3ts-rule/package/**`
- `behavior/golden/g3ts-rule/approved.normalized.json`
- `packages/rs/cargo/g3rs-cargo-types/src/types.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/guardrail3-rs.toml`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rust_version_aware_resolver.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/cargo_msrv_verify_installed.rs`
- `packages/rs/deps/g3rs-deps-hook-contract/crates/runtime/src/contract.rs`
- `packages/rs/hooks/g3rs-hooks-contract-types/src/types.rs`
- `behavior/fixtures/g3rs-rule/{cargo,deps}/**`
- `behavior/golden/g3rs-rule/approved.normalized.json`
