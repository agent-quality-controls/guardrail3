# Style policy path test-attack fixes

## Summary

Fixed the real `g3ts-style/policy-paths-valid` bugs found by adversarial review. The style path predicate now rejects backslash paths, Windows-style path escapes, encoded separators, encoded parent traversal, and URI schemes without `://`.

## Decisions made

- Reused the stricter path-validation semantics already used by Astro media policy instead of inventing a looser style-specific path interpretation.
- Kept empty-list ownership in `g3ts-style/strict-policy-configured`; `policy-paths-valid` owns invalid values inside configured lists.
- Added parity tests across `source_globs` and `stylelint_css_globs`, plus a multi-invalid-value test proving all bad field/value pairs are reported together.
- Did not split the style runtime file in this bug-fix commit. That was a pattern-parity observation, not a correctness blocker for this rule.

## Key files for context

- `.plans/2026-04-30-211505-style-policy-path-test-attack-fixes.md`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- adversarial convergence review: no must-fix gaps
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`

## Next steps

- Continue style family hardening from the next planned rule after this path-validation split.
