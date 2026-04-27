# Summary

Implemented strict Astro MDX wrapper guardrails and released `g3ts-eslint-plugin-astro-pipeline@0.1.8`. G3TS now requires ESLint to enforce approved MDX import names, raw UI export bans in component maps, Zod prop parsing for component-map wrappers, and raw MDX image bans.

# Decisions Made

- Kept MDX source validation in `g3ts-eslint-plugin-astro-pipeline`; G3TS only checks package pins, effective ESLint rules, and fail-closed options.
- Required explicit `approvedMdxComponentNames`, `approvedMdxImageComponents`, `mdxPropsParserName`, and `rawUiModuleGlobs`; rejected hidden defaults.
- Added explicit `allowedMdxComponentMapExports` for non-component exports like `mdxComponents`, instead of forcing apps into fake component-only modules.
- Published through a temporary npm user config sourced from root `.env.local` because npm does not read `NPM_TOKEN` directly and the existing user npm config can override auth.

# Key Files

- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/mdx-imports-only-approved-components.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/mdx-component-map-no-raw-ui-exports.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/mdx-component-wrapper-requires-zod-parse.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/no-raw-mdx-images.ts`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/eslint.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/strict_component_rules`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`
- `.plans/2026-04-27-212213-astro-mdx-zod-wrapper-hardening.md`

# Verification

- `npm test --prefix packages/ts/g3ts-eslint-plugin-astro-pipeline`
- `cargo test --manifest-path packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/Cargo.toml --offline --locked`
- `cargo test --manifest-path packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/Cargo.toml --offline --locked`
- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-ingestion/Cargo.toml --offline --locked`
- `cargo test --workspace --offline --locked` in `apps/guardrail3-ts`
- `g3rs validate` on touched Rust packages: Astro MDX types, ingestion, config checks, and Astro setup ingestion.
- `npm publish --userconfig "$tmp_config" --access public` from `packages/ts/g3ts-eslint-plugin-astro-pipeline`
- `npm view g3ts-eslint-plugin-astro-pipeline version`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

# Next Steps

- Landing now needs to install `g3ts-eslint-plugin-astro-pipeline@0.1.8`, update its Syncpack pin, and wire the four new MDX ESLint rules.
- Landing component-map wrappers then need local Zod schemas and `parseMdxComponentProps` calls for each approved MDX component export.
