# Style policy test attack fixes

## Summary

Fixed the real issues found by adversarial review of the strict style-policy plugin and G3TS style wiring. Published `g3ts-eslint-plugin-style-policy@0.1.2` with the corrected rule behavior and package surface.

## Decisions made

- Kept static class-source analysis inside the ESLint plugin, not G3TS.
- Treated partially dynamic templates and string concatenations as reportable when a denied token is visible in a static segment.
- Restricted recursive call analysis to configured class helpers so arbitrary calls in `className` do not false-positive.
- Made `recommended` a flat-config factory because a static config could not register the plugin and also carry the required app-owned options.
- Moved parser packages to dev dependencies and stopped tracking generated `dist/`; npm `prepack` now owns build artifacts.
- Tightened G3TS style wiring to require per-probe `style-policy` package identity, first-options-object `denyList`, and removal of legacy `eslint-plugin-tailwind-ban`.

## Key files for context

- `.plans/2026-05-01-085305-style-policy-test-attack-fixes.md`
- `packages/ts/g3ts-eslint-plugin-style-policy/src/utils/class-extract.ts`
- `packages/ts/g3ts-eslint-plugin-style-policy/tests/no-denied-class-tokens.test.ts`
- `packages/ts/g3ts-eslint-plugin-style-policy/src/configs/recommended.ts`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs`

## Verification

- `npm test` in `packages/ts/g3ts-eslint-plugin-style-policy`
- `npx tsc -p tsconfig.json --noEmit` in `packages/ts/g3ts-eslint-plugin-style-policy`
- `npm pack --dry-run --ignore-scripts --json` in `packages/ts/g3ts-eslint-plugin-style-policy`
- `npm publish --access public` published `g3ts-eslint-plugin-style-policy@0.1.2`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/parsers/eslint-config-parser/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/ts/style/g3ts-style-types`
- `g3rs validate --path packages/ts/style/g3ts-style-ingestion`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `g3rs validate --path packages/parsers/eslint-config-parser`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-landing/apps/landing --family style --inventory`
- Final adversarial agent returned `No unresolved findings`.

## Next steps

- Landing app agents should install `g3ts-eslint-plugin-style-policy@0.1.2` or newer.
