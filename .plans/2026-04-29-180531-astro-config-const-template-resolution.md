# Goal

Fix Astro config parsing so same-file static string constants and static template literals are accepted anywhere G3TS reads Astro config values.

# Bug

`/Users/tartakovsky/Projects/websmasher/websmasher-integrate-static-railway/apps/landing/astro.config.mjs` uses:

```js
const siteUrl = "https://seochecks.ai";
const sitemapIndexUrl = `${siteUrl}/sitemap-index.xml`;

export default defineConfig({
  site: siteUrl,
  integrations: [
    g3tsRobotsAuditor({
      site: siteUrl,
      sitemapUrls: [sitemapIndexUrl],
    }),
  ],
});
```

The shared Astro config parser already resolves simple const identifiers for root string properties, but it does not resolve template literals with static expressions. That makes integration option parsing fail, which invalidates the config snapshot and causes unrelated Astro setup/SEO rules to report missing `site`, `output`, integrations, Nuasite, sitemap, robots, and structured-data wiring.

# Approach

- Add parser tests before changing code:
  - same-file const `site` should satisfy the root `site` field.
  - template literal `${siteUrl}/sitemap-index.xml` should resolve inside nested integration options.
  - dynamic template expressions should still fail.
- Fix only `packages/parsers/astro-config-parser/crates/runtime/src/parser.rs`.
- Add one resolver for static string expressions:
  - string literals.
  - template literals whose expressions all resolve to strings through unmutated same-file const bindings.
  - const identifier chains.
- Reuse that resolver from:
  - `resolve_string_expr`.
  - `static_value` for template literals.
- Do not support imported constants, environment variables, function calls, spreads, or dynamic expressions.
- Do not patch Astro family checks. They must keep consuming typed parser facts.

# Files to modify

- `packages/parsers/astro-config-parser/crates/runtime/src/parser_tests/cases.rs`
- `packages/parsers/astro-config-parser/crates/runtime/src/parser.rs`
- `.worklogs/<timestamp>-astro-config-const-template-resolution.md`

# Verification

- `cargo test --manifest-path packages/parsers/astro-config-parser/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-integrate-static-railway/apps/landing --family astro --inventory`
- `git diff --check`
