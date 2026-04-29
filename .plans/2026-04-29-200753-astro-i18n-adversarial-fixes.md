# Astro i18n adversarial fixes

## Goal
Fix the i18n guardrail bugs found by adversarial review without broadening scope or adding ad hoc validation inside G3TS.

## Approach
- Prove ESLint plugin bugs with rule tests:
  - missing `requireLocalePrefixForContentRoutes` must report missing config
  - arbitrary calls like `t('/blog/x')` must not be treated as href/link helpers unless explicitly configured as checked link helpers
  - static empty or whitespace image alt strings must be forbidden on configured content image components
- Fix `g3ts-eslint-plugin-astro-i18n-policy`:
  - preserve omitted boolean detection for required options
  - add explicit `checkedInternalLinkHelpers` option so call-expression href checking is opt-in and not heuristic
  - report any static alt string, including empty strings
- Fix G3TS i18n ESLint ingestion:
  - only count `no-restricted-syntax` selectors when the rule is `error`
  - only count `@eslint-community/eslint-comments/no-restricted-disable` patterns when the rule is `error`
  - probe configured Astro, TS, TSX, and MDX lanes instead of one TSX and one TS file
  - require rules/plugins to be effective on every configured public probe
- Fix G3TS i18n policy config detection:
  - parser must distinguish missing `[ts.astro.i18n]` from an empty default struct
- Fix hook contract triggers:
  - include package manifests, lockfiles, and Syncpack config because i18n enforcement depends on packages and pins
- Verify:
  - npm tests/build for ESLint plugin
  - cargo tests for changed Rust packages
  - g3rs on changed Rust packages
  - g3ts on the landing app after installing the new CLI locally
- Commit with a worklog.

## Files to modify
- `packages/ts/g3ts-eslint-plugin-astro-i18n-policy/src/**`
- `packages/ts/g3ts-eslint-plugin-astro-i18n-policy/tests/**`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/src/**`
- `packages/ts/astro/i18n/g3ts-astro-i18n-config-checks/crates/runtime/src/**`
- `packages/ts/astro/i18n/g3ts-astro-i18n-hook-contract/src/**`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/**`
- `.worklogs/*`
