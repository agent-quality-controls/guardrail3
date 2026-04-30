# Style denylist boundary

## Summary

Removed the Tailwind denylist from the G3TS style policy contract. G3TS now verifies that `eslint-plugin-tailwind-ban` is installed and active with a non-empty ESLint-owned `denyList`, without mirroring the app-specific class list in `guardrail3-ts.toml`.

## Decisions made

- Moved Tailwind denied-class ownership fully to ESLint because ESLint is the delegated source validator.
- Kept no-op prevention in G3TS: an empty or blank-only ESLint `denyList` is still ineffective and fails.
- Removed `tailwind_denylist` from the shared TOML parser type so G3TS no longer treats it as a known policy field.
- Added ingestion-side tests for empty, blank, and non-empty ESLint denylist options using the required owned sidecar plus assertions-crate shape.

## Key files for context

- `.plans/2026-04-30-204239-style-denylist-boundary.md`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/ts/style/g3ts-style-types/src/lib.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/policy.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`

## Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/parsers/guardrail3-rs-toml-parser/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/parsers/guardrail3-rs-toml-parser`
- `g3rs validate --path packages/ts/style/g3ts-style-types`
- `g3rs validate --path packages/ts/style/g3ts-style-ingestion`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family style --inventory`

## Next steps

- Wire the landing app style stack when that app is ready: `[ts.style]` should define only source and CSS lanes; ESLint should own the Tailwind denyList.
