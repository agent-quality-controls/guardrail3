# Goal

Fix the `g3ts-style/policy-paths-valid` defects found by adversarial review.

# Findings To Fix

- Windows-style absolute paths and backslash traversal pass.
- External URI schemes without `://` pass.
- Encoded separators and encoded parent traversal pass.
- Tests do not cover `stylelint_css_globs` required-list parity.
- Tests do not prove path validation parity across both configured fields.
- Tests do not prove multiple invalid field/value pairs are reported in one finding.

# Approach

- Add failing sidecar tests first for:
  - empty `stylelint_css_globs` list
  - empty CSS glob value
  - absolute CSS glob
  - source parent traversal
  - source external URL
  - Windows absolute path
  - backslash traversal
  - encoded separators / encoded parent traversal
  - multiple invalid values reported together
- Update style path validation to match the stricter Astro media path predicate:
  - trim before validation
  - reject `\\`
  - reject `%2f` and `%5c`
  - reject encoded parent segment variants
  - reject `/`
  - reject `..` path segments
  - reject `://`
  - reject alpha URI schemes like `data:`, `file:`, `mailto:`, `npm:`
- Keep `strict-policy-configured` limited to list presence.
- Do not split the style runtime file in this bug fix. That broader module-shape cleanup belongs in a separate architecture pass if needed.

# Files To Modify

- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/assertions/src/run.rs` only if the new assertions need it

# Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
