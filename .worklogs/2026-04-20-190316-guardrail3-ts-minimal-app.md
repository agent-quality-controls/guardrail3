Summary

Built `apps/guardrail3-ts` as a separate minimal validate-only CLI that mirrors the Rust app shape but wires only the `eslint` family. The new `g3ts` binary compiles, the app root is clean under `g3rs validate`, and the CLI already produces real ESLint findings against external TS app roots.

Decisions made

- Kept the app boundary minimal: one supported family (`eslint`) and one runner crate (`family-runner-config`) instead of copying all Rust family-runner groups.
- Reused the existing generic app layers from `guardrail3-rs` only where they were actually neutral: app types, validate command, package crawler adapter, plain-text report, and CLI shell.
- Renamed all copied crate identities to `guardrail3-ts-*` and the binary to `g3ts` so the app is structurally separate from `g3rs`.
- Wired `family-runner-config` directly to `g3ts-eslint-ingestion::ingest_for_config_checks` and `g3ts-eslint-config-checks::check`, with no placeholder families.
- Kept `g3rs` as the verifier for the Rust app package itself. `apps/guardrail3-ts` now validates clean under Rust guardrails.

Key files for context

- `.plans/2026-04-20-185039-guardrail3-ts-minimal-app-design.md`
- `apps/guardrail3-ts/Cargo.toml`
- `apps/guardrail3-ts/guardrail3-rs.toml`
- `apps/guardrail3-ts/crates/types/app-types/src/supported_family.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-config/src/run.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`

Verification

- `cargo fmt --all --check --manifest-path apps/guardrail3-ts/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- --help`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/web --family eslint`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family eslint`

Next steps

- Add the next TS family packages: `ts/package`, `ts/npmrc`, and `ts/tsconfig`.
- Decide whether `g3ts` should be installed globally now or kept repo-local until more families exist.
- Start bringing `/Users/tartakovsky/Projects/websmasher/websmasher/apps/web` and `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing` into shape by fixing the current `g3ts-eslint/exists` missing-config finding.
