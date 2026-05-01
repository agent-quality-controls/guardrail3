# Style Enforcement Closure

## Summary

Implemented the remaining style-family wiring checks so G3TS proves that delegated style validators run through `validate`, protected ESLint disable policy is active, disable directives are visible inventory, and the owned style ESLint plugin floor is pinned through Syncpack.

## Decisions Made

- Kept source scanning delegated to ESLint and Stylelint. G3TS only validates package/config/script wiring and inventories disable directives.
- Used `package-script-command-parser` facts for `validate` closure instead of substring checks.
- Used `eslint-directive-parser` for disable inventory instead of hand parsing comments.
- Used `syncpack-config-parser` for package floor policy instead of parsing dependency ranges from `package.json`.
- Split new style config checks into semantic modules instead of growing `run.rs`.
- Moved style type definitions out of `src/lib.rs` into `src/types.rs` so the type crate matches the facade-only package structure.

## Key Files

- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/package_scripts.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/eslint_suppression.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/syncpack_policy.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint_directives.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/syncpack.rs`
- `packages/ts/style/g3ts-style-types/src/types.rs`
- `.plans/2026-05-01-113425-style-enforcement-closure.md`

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

- Let the landing app wire the newly reported style policy gaps: protected ESLint disable restriction and Syncpack style plugin pin.
