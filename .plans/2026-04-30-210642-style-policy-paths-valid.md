# Goal

Split style policy path validation into its own semantic rule: `g3ts-style/policy-paths-valid`.

# Problem

`g3ts-style/strict-policy-configured` currently checks two different assertions:

- `[ts.style]` required lists exist and are non-empty
- configured style globs are safe app-relative paths

That violates the semantic rule-ID model because one rule name hides two independent failures.

# Approach

- Keep `g3ts-style/strict-policy-configured` focused on presence/non-empty list checks:
  - `source_globs`
  - `stylelint_css_globs`
- Add `g3ts-style/policy-paths-valid`:
  - reject empty path values
  - reject absolute paths
  - reject parent traversal
  - reject external URLs
  - report the exact bad field/value pairs
- Add focused sidecar tests for:
  - valid policy emits both strict-policy and policy-paths findings as Info
  - empty source glob list keeps strict policy as Error and path rule as Info
  - empty source glob value keeps strict policy as Info and path rule as Error
  - absolute source glob fails path rule
  - parent traversal CSS glob fails path rule
  - external URL CSS glob fails path rule

# Files To Modify

- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/assertions/src/run.rs`

# Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
