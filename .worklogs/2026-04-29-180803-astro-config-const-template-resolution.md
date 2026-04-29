## Summary

Fixed the Astro config parser so static template literals built from same-file unmutated const string bindings resolve as static values. This removes false G3TS Astro setup/SEO failures when an Astro config reuses `siteUrl` and derived sitemap URLs instead of duplicating literals.

## Decisions made

- Fixed the shared Astro config parser rather than patching Astro setup or SEO checks, because those families must consume typed parser facts and should not parse config source themselves.
- Accepted only string literals, unmutated same-file const chains, and template literals composed from those values.
- Kept dynamic calls, imported constants, environment reads, spreads, and non-string template expressions fail-closed.

## Key files for context

- `packages/parsers/astro-config-parser/crates/runtime/src/parser.rs`
- `packages/parsers/astro-config-parser/crates/runtime/src/parser_tests/cases.rs`
- `.plans/2026-04-29-180531-astro-config-const-template-resolution.md`

## Verification

- `cargo test --manifest-path packages/parsers/astro-config-parser/Cargo.toml --workspace --offline --locked parses_same_file_const_strings_and_static_templates` failed before the parser fix.
- `cargo test --manifest-path packages/parsers/astro-config-parser/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-integrate-static-railway/apps/landing --family astro --inventory`
- `g3rs validate --path packages/parsers/astro-config-parser --inventory`
- `git diff --check`

## Next steps

- Keep imported config object resolution unsupported unless a real app needs it; same-file static const/template resolution is the narrow bug fixed here.
