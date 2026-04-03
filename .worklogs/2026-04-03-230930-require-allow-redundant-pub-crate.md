# Guardrails: require redundant_pub_crate = "allow"

**Date:** 2026-04-03 23:09

## Summary
CARGO-01 now requires `redundant_pub_crate = "allow"` in clippy lints.
Error message explains: conflicts with rustc `unreachable_pub`, both
cannot be denied (clippy#5369). Added `RequiredAllowLint` struct and
`EXPECTED_CLIPPY_REQUIRED_ALLOW` constant. Applied to app Cargo.toml
with escape hatch.

## Key files
- lint_support.rs: EXPECTED_CLIPPY_REQUIRED_ALLOW constant
- rs_cargo_01 rule.rs: check loop for required allows
- apps/guardrail3/Cargo.toml: added the allow
- guardrail3.toml: escape hatch for app's allow
