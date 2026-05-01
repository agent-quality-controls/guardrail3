# Style Policy Prefix And Pattern Bans

## Summary

Added prefix and regex matching to `g3ts-eslint-plugin-style-policy` so apps can ban Tailwind class families such as `text-[...]` without enumerating every exact class token. Updated G3TS style ingestion so the rule is considered effective when ESLint owns any non-empty exact, prefix, or pattern policy.

## Decisions Made

- ESLint remains responsible for source scanning.
- G3TS only verifies the style policy rule is active at `error`, comes from `g3ts-eslint-plugin-style-policy`, and has a non-empty ESLint-owned policy mechanism.
- Prefix bans are first-class because they are simpler and safer for Tailwind arbitrary-value families than regex.
- Regex bans are supported for cases prefixes cannot express.
- Invalid regex values are reported by the ESLint rule as configuration errors.

## Key Files

- `packages/ts/g3ts-eslint-plugin-style-policy/src/rules/no-denied-class-tokens.ts`
- `packages/ts/g3ts-eslint-plugin-style-policy/src/utils/options.ts`
- `packages/ts/g3ts-eslint-plugin-style-policy/tests/no-denied-class-tokens.test.ts`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`

## Verification

- `npm test` in `packages/ts/g3ts-eslint-plugin-style-policy`
- `npm pack --dry-run` in `packages/ts/g3ts-eslint-plugin-style-policy`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/crates/runtime/Cargo.toml`
- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml`
- `g3rs validate --path packages/ts/style/g3ts-style-ingestion --inventory`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks --inventory`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --force`
- `npm publish --access public` for `g3ts-eslint-plugin-style-policy@0.1.3`

## Next Steps

- Landing/app agents should install `g3ts-eslint-plugin-style-policy@0.1.3` and configure `denyPrefixes` or `denyPatterns` in ESLint where they want class-family bans.
