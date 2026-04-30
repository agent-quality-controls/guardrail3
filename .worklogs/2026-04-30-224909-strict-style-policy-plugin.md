# Strict style policy plugin

## Summary

Added `g3ts-eslint-plugin-style-policy` and switched the G3TS style family from the weak third-party `eslint-plugin-tailwind-ban` rule to `style-policy/no-denied-class-tokens`. The new ESLint rule catches denied static class tokens in JSX/Astro class attributes, Astro `class:list`, class arrays/objects, conditionals, logical expressions, and configured helper calls.

## Decisions made

- Kept source parsing in ESLint. The new package is an ESLint plugin; G3TS only checks package presence and effective rule wiring.
- Replaced the third-party dependency instead of wrapping it because its implementation only covers JSX `className` string literals.
- Kept denylist ownership in ESLint config. G3TS only requires the rule to be active at `error` with a non-empty ESLint-owned `denyList`.
- Published `g3ts-eslint-plugin-style-policy@0.1.1` so app agents can install the package from npm.

## Key files for context

- `.plans/2026-04-30-224222-strict-style-policy-plugin.md`
- `packages/ts/g3ts-eslint-plugin-style-policy/src/rules/no-denied-class-tokens.ts`
- `packages/ts/g3ts-eslint-plugin-style-policy/tests/no-denied-class-tokens.test.ts`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-types/src/lib.rs`

## Verification

- `npm test` in `packages/ts/g3ts-eslint-plugin-style-policy`
- `npm publish --access public` published `g3ts-eslint-plugin-style-policy@0.1.1`
- `npm view g3ts-eslint-plugin-style-policy@0.1.1 name version --json`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/ts/style/g3ts-style-types`
- `g3rs validate --path packages/ts/style/g3ts-style-ingestion`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-landing/apps/landing --family style --inventory`

## Next steps

- Landing app should replace `eslint-plugin-tailwind-ban` with `g3ts-eslint-plugin-style-policy` and wire `style-policy/no-denied-class-tokens`.
