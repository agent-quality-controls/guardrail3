# Astro i18n adversarial fixes

## Summary
Fixed the adversarially reported Astro i18n guardrail gaps. The ESLint plugin now requires explicit locale-prefix policy, does not treat arbitrary function calls as internal links, and bans all static alt strings on configured content image components.

## Decisions made
- Added explicit `checkedInternalLinkHelpers` instead of guessing every call expression with a string argument. Arbitrary calls are not href surfaces.
- G3TS now treats delegated ESLint rules as effective only when they are active at `error` severity across all configured public i18n probe lanes.
- Missing `[ts.astro.i18n]` is represented as missing in the TOML model instead of a default empty config.
- Split i18n ESLint ingestion into `eslint/mod.rs`, `eslint/run.rs`, `eslint/settings.rs`, and `eslint/targets.rs` after G3RS flagged the old file as too large.
- Fixed the pre-commit hook path for standalone npm TypeScript packages. The hook no longer calls removed `guardrail3 ts validate --staged` and no longer assumes a root pnpm workspace for standalone package commits.
- Published `g3ts-eslint-plugin-astro-i18n-policy@0.1.1` and updated the required Astro stack pin.

## Key files
- `packages/ts/g3ts-eslint-plugin-astro-i18n-policy/src/rules/no-unlocalized-internal-hrefs.ts`
- `packages/ts/g3ts-eslint-plugin-astro-i18n-policy/src/rules/no-inline-image-alt.ts`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/src/eslint/run.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/src/eslint/settings.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/src/eslint/targets.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`
- `.githooks/pre-commit`

## Verification
- `npm test` in `packages/ts/g3ts-eslint-plugin-astro-i18n-policy`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- package-level `cargo test` for changed i18n/parser/setup packages
- `g3rs validate` on changed Rust packages and `apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory` now reports real landing configuration gaps, including missing i18n policy/package wiring.

## Next steps
- Landing app must install `g3ts-eslint-plugin-astro-i18n-policy@0.1.1`, add `[ts.astro.i18n]`, and wire the delegated ESLint policy rules.
- Existing G3RS warnings on large i18n policy structs remain warnings, not failures.
