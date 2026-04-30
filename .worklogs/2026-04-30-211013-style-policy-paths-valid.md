# Style policy paths rule

## Summary

Split style policy path validation out of `g3ts-style/strict-policy-configured` into the dedicated semantic rule `g3ts-style/policy-paths-valid`. The strict policy rule now only checks that the style policy exists with non-empty source and CSS lane lists.

## Decisions made

- Empty lists belong to `g3ts-style/strict-policy-configured`.
- Empty string values inside non-empty lists belong to `g3ts-style/policy-paths-valid`.
- The path rule reports exact bad field/value pairs for absolute paths, parent traversal, external URLs, and empty values.
- Kept all style source validation delegated to Stylelint and ESLint; this rule only validates the configured enforcement lanes.

## Key files for context

- `.plans/2026-04-30-210642-style-policy-paths-valid.md`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/assertions/src/run.rs`

## Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family style --inventory`

## Next steps

- Continue with the remaining style family hardening items only after the app proves the current style contract against real wiring.
