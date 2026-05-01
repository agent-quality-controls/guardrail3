# Style Test Attack Fixes

## Summary

Fixed the blocking gaps found by adversarial review of the style enforcement closure. The style family now checks protected ESLint disable policy per probe, fails closed on invalid directive inventory globs and package parse failures, keeps Syncpack policy verdicts in checks instead of ingestion, and covers more package-script and Syncpack edge cases.

## Decisions Made

- Kept Syncpack ingestion as normalization only: it now returns source and version-group facts, while config checks decide whether the required style plugin pin is present.
- Kept the app-local canonical Syncpack source requirement as `source: ["package.json"]` because this style family validates one app package boundary at a time.
- Kept fail-closed package script behavior: any `||` in the reachable validation chain invalidates the style validation contract.
- Restored style directive DTOs to the public-field type-package pattern used by Astro DTO packages.

## Key Files

- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/package_scripts.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/eslint_suppression.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/syncpack_policy.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint_directives.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/package.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/syncpack.rs`
- `packages/ts/style/g3ts-style-types/src/types.rs`

## Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-types/Cargo.toml`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/crates/runtime/Cargo.toml`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/Cargo.toml`
- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml`
- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml`
- `g3rs validate --path packages/ts/style/g3ts-style-types --inventory`
- `g3rs validate --path packages/ts/style/g3ts-style-ingestion --inventory`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks --inventory`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-landing/apps/landing --family style --inventory`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family style --inventory`

## Next Steps

- No blocking follow-up from the convergence attack.
