# Astro media adversarial fixes

## Summary
Hardened Astro media guardrails after adversarial review. Media now owns image policy end to end, i18n no longer carries image rules, and the delegated media ESLint/Astro packages reject the bypasses reviewers found.

## Decisions
- Kept validation delegated: source misuse stays in `g3ts-eslint-plugin-astro-media-policy`, build asset checks stay in `g3ts-astro-media-assets`, and G3TS verifies package/config/integration/ESLint wiring.
- Required explicit media policy fields instead of hidden defaults: `allow_svg_icons` and `allowed_public_image_paths` must be configured, and unknown `[ts.astro.media]` fields fail.
- Removed i18n ownership of media fields from the shared TOML parser and i18n family contracts.
- Published `g3ts-eslint-plugin-astro-media-policy@0.1.8` and `g3ts-astro-media-assets@0.1.2`; setup Syncpack pins now require those versions.

## Key Files
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/no-raw-public-image-paths.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/no-inline-image-alt.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/require-approved-media-helper.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/require-content-image-key.ts`
- `packages/ts/astro/media/g3ts-astro-media-assets/src/index.ts`
- `packages/ts/astro/media/g3ts-astro-media-config-checks/crates/runtime/src/policy_rules.rs`
- `packages/ts/astro/media/g3ts-astro-media-ingestion/src/eslint/settings.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`

## Verification
- `npm test` in `packages/ts/g3ts-eslint-plugin-astro-media-policy`
- `npm test` in `packages/ts/astro/media/g3ts-astro-media-assets`
- `npm test` in `packages/ts/g3ts-eslint-plugin-astro-i18n-policy`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/parsers/guardrail3-rs-toml-parser/crates/runtime/Cargo.toml --offline`
- `g3rs validate` on changed parser, media, i18n, and setup Rust packages
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- Final adversarial convergence review returned no blockers after the last `allowed_public_image_paths` fix.

## Next Steps
- Landing agent should install the new media packages, add `[ts.astro.media]`, wire `g3tsAstroMediaAssets(...)`, and enable the `astro-media-policy/*` ESLint rules from the G3TS error messages.
