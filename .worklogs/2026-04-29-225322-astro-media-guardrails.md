## Summary

Built the Astro media guardrail family and published the delegated TypeScript packages.

The new `astro-media` family enforces explicit media policy configuration, required package installation, Astro media asset integration wiring, media ESLint rule wiring, protected media rule disables, and fail-closed `astro build` validation.

## Decisions made

- Put asset existence checks in `g3ts-astro-media-assets`, an Astro integration that runs during `astro build`, because G3TS should verify wiring instead of checking output files itself.
- Put source misuse checks in `g3ts-eslint-plugin-astro-media-policy`, because raw public image paths and content image prop misuse are source-level lint rules.
- Added a separate `astro-media` G3TS family with its own types, ingestion, config checks, and hook contract packages instead of adding more checks to the existing Astro setup/content/SEO families.
- Required Syncpack pins for `g3ts-astro-media-assets@0.1.0` and `g3ts-eslint-plugin-astro-media-policy@0.1.2`.
- Kept media hook triggers to config/source files only. Package and Syncpack triggers remain Astro setup ownership; adding `public/**/*` to the media hook contract currently creates a hook routing failure in the real landing app.

## Key files for context

- `.plans/2026-04-29-211020-astro-media-guardrails.md`
- `packages/ts/astro/media/g3ts-astro-media-assets/src/index.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/index.ts`
- `packages/ts/astro/media/g3ts-astro-media-types/src/types.rs`
- `packages/ts/astro/media/g3ts-astro-media-ingestion/src/run.rs`
- `packages/ts/astro/media/g3ts-astro-media-config-checks/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`

## Verification

- `npm test` in `packages/ts/astro/media/g3ts-astro-media-assets`
- `npm test` in `packages/ts/g3ts-eslint-plugin-astro-media-policy`
- `cargo test --manifest-path packages/ts/astro/media/g3ts-astro-media-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/astro/media/g3ts-astro-media-ingestion/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/astro/media/g3ts-astro-media-hook-contract/Cargo.toml --offline`
- `cargo test --manifest-path packages/parsers/guardrail3-rs-toml-parser/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro-media --inventory` reports the expected missing landing media setup.
- `g3rs validate` passed for the new media Rust packages, the TOML parser, and `apps/guardrail3-ts`; remaining output is warning/info inventory only.
- `git diff --check`

## Next steps

- Landing should install and configure `g3ts-astro-media-assets` and `g3ts-eslint-plugin-astro-media-policy`, add `[ts.astro.media]`, wire the Astro integration, and enable the media ESLint rules.
- After landing is fixed, rerun `g3ts validate --path apps/landing --family astro --inventory` from the landing repo.
