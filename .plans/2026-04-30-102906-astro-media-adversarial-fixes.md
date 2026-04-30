# Astro media adversarial fixes

## Goal
Fix the real media guardrail bugs found by adversarial review without moving delegated validation into the G3TS CLI.

## Approach
- Add failing tests before fixes for the reported false negatives and false positives.
- Keep asset existence/type/path checks inside `g3ts-astro-media-assets`.
- Keep source misuse checks inside `g3ts-eslint-plugin-astro-media-policy`.
- Keep G3TS responsible only for package/config/integration/ESLint wiring.
- Remove the media hook contract's hardcoded package-manager binary.
- Remove i18n ownership of image component rules after media owns them.

## Fixes
- `g3ts-astro-media-assets`
  - aggregate invalid/missing asset errors into one message
  - reject backslashes and encoded path separators
  - require configured assets to be files, not directories
  - validate runtime option types with actionable errors
  - add tests for all of the above
- `g3ts-eslint-plugin-astro-media-policy`
  - require explicit shared options used by the rule behavior
  - reject meaningless content image key props
  - make metadata helper checks extension-aware
  - stop allowing approved helper names when they are local/shadowed or member calls from arbitrary objects
  - add false-positive tests for non-image route strings
- `g3ts-astro-media-config-checks`
  - enforce output asset URL paths separately from app-relative paths/globs
  - reject external URLs and root-relative app paths
  - add missing broken-case tests for path validation and key wiring rules
- `g3ts-astro-media-hook-contract`
  - remove `pnpm` from critical commands
- `g3ts-astro-i18n-config-checks`
  - stop requiring image component rules that now belong to media

## Files to modify
- `packages/ts/astro/media/g3ts-astro-media-assets/src/index.ts`
- `packages/ts/astro/media/g3ts-astro-media-assets/tests/check-media-assets.test.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/*.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/utils/options.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/tests/*.ts`
- `packages/ts/astro/media/g3ts-astro-media-config-checks/crates/runtime/src/*.rs`
- `packages/ts/astro/media/g3ts-astro-media-config-checks/crates/runtime/src/lib_tests/*`
- `packages/ts/astro/media/g3ts-astro-media-hook-contract/src/contract.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-config-checks/crates/runtime/src/*`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`

## Verification
- npm tests for changed TypeScript packages
- cargo tests for changed Rust packages and `apps/guardrail3-ts`
- publish changed npm packages and update required pins
- reinstall local `g3ts`
- run `g3ts` against landing for media signal
- run `g3rs` on changed Rust packages and `apps/guardrail3-ts`
