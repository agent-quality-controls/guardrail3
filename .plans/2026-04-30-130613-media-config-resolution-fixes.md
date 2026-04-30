# Goal

Fix three landing-reported G3TS/media issues cleanly at the owning layer:

- media ESLint probes must not append synthetic files under literal file paths
- media helper import checks must compare normalized module identity, not raw import text
- Astro integration option checks must resolve bounded static constants through the shared Astro config parser

# Decisions

- Fix probe generation in `g3ts-astro-media-ingestion`; this is a media ingestion bug.
- Fix helper module normalization in `g3ts-eslint-plugin-astro-media-policy`; source import semantics belong to the delegated ESLint rule.
- Fix imported/static constant resolution in `astro_config_parser`; every Astro family consumes that shared parser and should benefit from the same bounded static evaluator.
- Do not execute app config code.
- Do not support env vars, function calls, spreads, computed members, or arbitrary package imports.
- If static resolution fails, preserve explicit unresolved values so G3TS can emit clear messages.

# Implementation

## Media public_source_globs probes

Modify:

- `packages/ts/astro/media/g3ts-astro-media-ingestion/src/eslint/targets.rs`

Rules:

- literal file path ending in `.astro`, `.ts`, `.tsx`, or `.mdx` probes that file directly
- directory path probes `path/__g3ts_media_probe__.<ext>` only when extension is inferable or fallback applies
- glob path uses stable prefix before first glob metacharacter
- generated probe must never contain `.<ext>/__g3ts_media_probe__`

Add unit tests in the media ingestion package for:

- `src/mdx-components.tsx` -> exact file probe
- `src/pages/**/*.{astro,ts,tsx}` -> probes under `src/pages`
- `content/**/*.mdx` -> probe under `content`
- duplicate inputs dedupe

## Media helper module normalization

Modify:

- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/no-raw-public-image-paths.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/require-approved-media-helper.ts`
- shared utility file if useful

Rules:

- configured `mediaHelperModules` are app-relative module identities
- import source strings are normalized before matching
- relative imports are resolved against the current linted filename
- extension variants `.ts`, `.tsx`, `.js`, `.jsx`, `.mjs`, `.cjs` match the same identity
- aliases are not invented; raw alias strings still match only if explicitly configured
- local shadowing protection stays

Tests:

- helper imported from `../lib/media-assets` matches configured `src/lib/media-assets`
- helper imported from `../lib/media-assets.ts` matches configured `src/lib/media-assets`
- raw unmatched alias still fails unless configured
- local shadowing still fails

## Astro config static constants

Modify shared parser packages under `packages/parsers/astro-config-parser`.

Required support:

- same-file `const favicon = "/favicon.ico"`
- same-file `export const favicon = "/favicon.ico"`
- object constant: `const publicMedia = { favicon: "/favicon.ico" }`
- member read: `publicMedia.favicon`
- local imported constants from relative app files
- imported object constants plus member reads
- arrays and objects containing resolved values

Reject:

- env vars
- function calls
- computed properties
- spread properties
- package imports
- non-local imports

Tests:

- Astro config integration first argument resolves same-file consts
- resolves local imported consts
- resolves imported object member values
- unresolved dynamic values remain unresolved and do not panic

# Verification

Run:

- npm tests for `g3ts-eslint-plugin-astro-media-policy`
- cargo tests for `astro-config-parser`
- cargo tests for `g3ts-astro-media-ingestion`
- cargo tests for `g3ts-astro-media-config-checks`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- install local G3TS CLI
- `g3rs validate` on changed Rust packages
- `g3ts validate` against landing
- adversarial review against this plan and changed code

# Release

If the ESLint plugin changes, publish a new patch version and update required pins in G3TS setup.
