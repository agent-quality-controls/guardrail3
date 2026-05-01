## Summary

Added the generic TS package `validate` script contract. The package family now requires a standard root `validate` script and rejects fail-open validate scripts while still leaving tool-specific checks to their owning families.

## Decisions Made

- Added `validate_script` to the package root snapshot so checks consume parsed package facts instead of reading package.json directly.
- Added `g3ts-package/validate-script-present` to require the standard script name.
- Added `g3ts-package/validate-script-fail-closed` to reject unsupported parser output or reachable `||` fallbacks.
- Kept formatter, spelling, style, and type coverage ownership out of `ts/package`; those families own their own tools and packages.
- Kept selected app roots that are not pnpm workspace roots in local-only mode, matching the existing package-family boundary.

## Key Files

- `packages/ts/package/g3ts-package-types/src/types.rs`
- `packages/ts/package/g3ts-package-types/src/convert.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/validate_script_present.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/validate_script_fail_closed.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/assertions/src/validate_script_present.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/assertions/src/validate_script_fail_closed.rs`

## Verification

- `cargo test --manifest-path packages/ts/package/g3ts-package-types/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/package/g3ts-package-config-checks/Cargo.toml --workspace --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/ts/package/g3ts-package-types --inventory`
- `g3rs validate --path packages/ts/package/g3ts-package-config-checks --inventory`
- `g3rs validate --path apps/guardrail3-ts --inventory`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force --offline`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family package --inventory`
- Adversarial review: no blocking findings.

## Next Steps

- Continue with the next planned TS tooling family that delegates actual validation to an external tool, likely type coverage.
