# Astro media config resolution fixes

## Summary

Fixed the media guardrail false positives reported from the landing app. Astro config parsing now resolves bounded static constants, media source probing handles file paths correctly, and the media ESLint plugin compares helper imports by module identity instead of raw import text.

## Decisions made

- Resolved same-file and local relative imported constants in the Astro config parser because forcing literals duplicated config values and pushed apps toward worse code.
- Preserved unsupported dynamic values as `UnsupportedExpression` instead of invalidating the whole Astro config parse. One dynamic option should not erase unrelated static config facts.
- Kept computed member reads unsupported. `publicMedia.favicon` is a clean static contract; `publicMedia["favicon"]` adds no value and makes static analysis weaker.
- Split `g3ts-astro-media-ingestion` into facade, runtime, and assertions crates after G3RS flagged the package shape. Runtime owns ingestion; assertions own test helpers.
- Fixed media helper matching in the ESLint plugin by resolving relative imports against the linted file and comparing app-relative module identity.
- Published `g3ts-eslint-plugin-astro-media-policy@0.1.10` and updated G3TS required pins to that version.

## Key files for context

- `.plans/2026-04-30-130613-media-config-resolution-fixes.md`
- `packages/parsers/astro-config-parser/crates/runtime/src/parser.rs`
- `packages/parsers/astro-config-parser/crates/runtime/src/parser_tests/cases.rs`
- `packages/ts/astro/media/g3ts-astro-media-ingestion/crates/runtime/src/eslint/targets.rs`
- `packages/ts/astro/media/g3ts-astro-media-ingestion/crates/runtime/src/eslint/targets_tests/cases.rs`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/utils/module-identity.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/no-raw-public-image-paths.ts`
- `packages/ts/g3ts-eslint-plugin-astro-media-policy/src/rules/require-approved-media-helper.ts`

## Verification

- `npm test` in `packages/ts/g3ts-eslint-plugin-astro-media-policy`
- `cargo test -p astro-config-parser-runtime --manifest-path packages/parsers/astro-config-parser/Cargo.toml`
- `cargo test -p g3ts-astro-media-ingestion-runtime --manifest-path packages/ts/astro/media/g3ts-astro-media-ingestion/Cargo.toml`
- `cargo test -p g3ts-astro-media-ingestion --manifest-path packages/ts/astro/media/g3ts-astro-media-ingestion/Cargo.toml`
- `cargo test -p g3ts-astro-media-config-checks-runtime --manifest-path packages/ts/astro/media/g3ts-astro-media-config-checks/Cargo.toml`
- `cargo test -p g3ts-astro-setup-ingestion --manifest-path packages/ts/astro/setup/g3ts-astro-setup-ingestion/Cargo.toml`
- `cargo test -p g3ts-astro-seo-ingestion --manifest-path packages/ts/astro/seo/g3ts-astro-seo-ingestion/Cargo.toml`
- `cargo test -p g3ts-astro-seo-config-checks-runtime --manifest-path packages/ts/astro/seo/g3ts-astro-seo-config-checks/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate` on changed parser, media, setup, SEO, and app packages
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- `npm view g3ts-eslint-plugin-astro-media-policy version --registry=https://registry.npmjs.org/`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- Final adversarial test attack returned no blockers.

## Next steps

- Landing can rerun G3TS with the installed CLI and update its app-side media wiring to the released `0.1.10` plugin.
