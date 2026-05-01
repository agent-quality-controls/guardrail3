# Style policy test-attack fixes

## Goal

Close the real gaps found by adversarial review of `g3ts-eslint-plugin-style-policy` and the G3TS style-family wiring.

## Approach

- Fix the ESLint rule so visible static denied tokens inside partially dynamic template literals and binary string concatenations are reported.
- Fix the ESLint rule so unconfigured function calls inside `class` or `className` expressions are not treated as class-helper calls.
- Strengthen plugin tests so they assert reported token data, partial dynamic strings, configured versus unconfigured helper calls, and broader Astro `class:list` shapes.
- Replace the broken static `recommended` export with a flat-config factory that registers the plugin namespace and requires explicit rule options.
- Move parser packages out of runtime dependencies because they are test-only dependencies.
- Stop tracking generated `dist/` output and add package-local ignores for `dist/` and `node_modules/`.
- Fix G3TS style ingestion so the style-policy package identity and rule effectiveness are proven per probe, not through aggregate namespace data.
- Fix G3TS style ingestion so `denyList` is accepted only from the first ESLint rule options object, matching ESLint runtime behavior.
- Make G3TS style config checks reject the old `eslint-plugin-tailwind-ban` package.
- Replace stale parser tests that used `eslint-plugin-tailwind-ban` as the ESM-only resolver fixture with `g3ts-eslint-plugin-style-policy`.

## Key decisions

- Keep source parsing inside ESLint. G3TS still verifies delegated wiring only.
- Keep denylist ownership in ESLint config. G3TS verifies that the effective rule receives a non-empty first options object.
- Do not publish source maps without sources. Disable package source maps and declaration maps for this plugin.
- Do not keep generated `dist/` in git. `prepack` builds it for npm publishing.

## Files to modify

- `packages/ts/g3ts-eslint-plugin-style-policy/src/**`
- `packages/ts/g3ts-eslint-plugin-style-policy/tests/**`
- `packages/ts/g3ts-eslint-plugin-style-policy/package.json`
- `packages/ts/g3ts-eslint-plugin-style-policy/package-lock.json`
- `packages/ts/g3ts-eslint-plugin-style-policy/tsconfig.json`
- `packages/ts/style/g3ts-style-types/src/lib.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/**/run_tests/**`
- `packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs`

## Verification

- `npm test` in `packages/ts/g3ts-eslint-plugin-style-policy`
- `npm pack --dry-run --ignore-scripts --json` in `packages/ts/g3ts-eslint-plugin-style-policy`
- style ingestion/config runtime tests
- parser runtime tests
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path` on changed Rust workspaces
- reinstall local `g3ts`
