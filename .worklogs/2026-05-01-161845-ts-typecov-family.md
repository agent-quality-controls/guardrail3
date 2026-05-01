Summary
- Added the G3TS `typecov` family for TypeScript package roots.
- The family delegates measurement to `type-coverage` and only enforces wiring: package installation, `typecov` script, `--at-least 100`, fail-closed script routing, validate routing, Syncpack pinning, and hook triggers.

Decisions made
- Extended the shared package-script command parser to treat `type-coverage` and `typecov` script names as guardrail-related, so unsupported shell syntax fails closed at ingestion instead of being ignored.
- Removed the scaffolded config-file lane because the plan did not require a local `type-coverage` config surface.
- Kept threshold policy explicit as split-form `type-coverage --at-least 100`; `--at-least=100` is rejected so the contract remains simple and unambiguous.
- Wired `typecov` into the G3TS CLI, default family selection, config runner, hook contract aggregation, report names, and app dependency allowlist.

Key files for context
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser.rs`
- `packages/ts/typecov/g3ts-typecov-types/src/types.rs`
- `packages/ts/typecov/g3ts-typecov-ingestion/crates/runtime/src/run.rs`
- `packages/ts/typecov/g3ts-typecov-config-checks/crates/runtime/src/run.rs`
- `packages/ts/typecov/g3ts-typecov-hook-contract/src/contract.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-config/src/run.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/src/run.rs`

Verification
- `cargo test --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/typecov/g3ts-typecov-types/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/typecov/g3ts-typecov-ingestion/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/typecov/g3ts-typecov-config-checks/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/typecov/g3ts-typecov-hook-contract/Cargo.toml --workspace --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/parsers/package-script-command-parser --inventory`
- `g3rs validate --path packages/ts/typecov/g3ts-typecov-types --inventory`
- `g3rs validate --path packages/ts/typecov/g3ts-typecov-ingestion --inventory`
- `g3rs validate --path packages/ts/typecov/g3ts-typecov-config-checks --inventory`
- `g3rs validate --path packages/ts/typecov/g3ts-typecov-hook-contract --inventory`
- `g3rs validate --path apps/guardrail3-ts --inventory`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force --offline`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family typecov --inventory`

Adversarial review
- First pass found parser fail-open behavior for `typecov` script names, missing threshold matrix tests, and missing banned Syncpack group coverage.
- Fixed those findings and reran the relevant tests.
- Second pass reported clean.

Next steps
- Landing should install and wire `type-coverage` when it is ready to satisfy the new `typecov` family.
