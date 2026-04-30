# Style config check tests

## Summary

Added focused sidecar tests for the new G3TS style config rules. The tests prove each owned rule reports the expected semantic ID and fails when its delegated Stylelint, ESLint, package, or policy contract is broken.

## Decisions made

- Moved result-shape assertions into `g3ts-style-config-checks-assertions` because G3RS requires runtime sidecar tests to use the shared assertions crate instead of inspecting `G3CheckResult` directly.
- Kept the tests at the runtime crate boundary with typed `g3ts-style-types` inputs, so they exercise rule behavior without filesystem or parser setup.
- Rejected parser-level fixture tests in this change because the immediate missing proof was rule behavior, and parser execution depends on Node package fixtures.

## Key files for context

- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/lib_tests/cases.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/lib_tests/helpers.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/assertions/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml`

## Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`

## Next steps

- Wire the landing app to the new style contract and confirm `g3ts validate --family style` moves from expected errors to clean inventory.
