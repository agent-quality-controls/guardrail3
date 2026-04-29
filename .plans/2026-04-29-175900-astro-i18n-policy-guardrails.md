# Goal

Add Astro i18n guardrails for static content sites without duplicating existing linters.

The first implementation must protect the source-level i18n mistakes that agents are most likely to create:

- internal content links that drop the locale prefix
- content image components that carry inline localized `alt` text
- content image components that use raw `src` instead of a locale-owned image key
- raw date/number formatting outside approved locale helper modules
- hidden `eslint-disable` bypasses around delegated i18n rules

G3TS must enforce installation and wiring. G3TS must not implement source AST linting itself.

# Architecture

## Package 1: ESLint plugin

Package:

- `g3ts-eslint-plugin-astro-i18n-policy`

Type:

- TypeScript npm package
- ESLint flat-config compatible plugin
- no CLI
- no Astro integration

Plugin namespace:

- `astro-i18n-policy`

Exports:

```ts
export default plugin;
export const rules = plugin.rules;
export const configs = plugin.configs;
```

Required rules:

- `astro-i18n-policy/no-unlocalized-internal-hrefs`
- `astro-i18n-policy/no-inline-image-alt`
- `astro-i18n-policy/require-content-image-key`

No custom rules for raw public copy, raw date formatting, or raw number formatting in this phase. Those must be delegated to existing ESLint rules.

## Package 2: G3TS Astro i18n family

Packages:

- `packages/ts/astro/i18n/g3ts-astro-i18n-types`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion`
- `packages/ts/astro/i18n/g3ts-astro-i18n-config-checks`
- `packages/ts/astro/i18n/g3ts-astro-i18n-hook-contract`

No `g3ts-astro-i18n-checks` app-facing package.

Family name:

- `astro-i18n`

G3TS runner:

- add `astro-i18n` as an Astro child family in the structure runner
- include it in `--family astro`
- allow direct `--family astro-i18n` only if the current CLI family parser already supports child family selection; do not build new CLI selection plumbing just for this

# Delegation

## Public copy

Delegate to:

- `eslint-plugin-i18next`
- rule: `i18next/no-literal-string`

G3TS i18n config checks must enforce:

- package `eslint-plugin-i18next` is installed
- ESLint effective config has plugin namespace `i18next`
- `i18next/no-literal-string` is `error` on configured Astro public source lanes
- allowed machine strings are explicit in ESLint config
- `@eslint-community/eslint-comments/no-restricted-disable` protects `i18next/no-literal-string`

G3TS must not parse source strings.

## Generic image alt existence

Do not build new generic alt-existence logic.

Delegate to one approved existing rule:

- preferred for JSX/TSX/MDX: `eslint-plugin-jsx-a11y` rule `jsx-a11y/alt-text`
- for Astro template lanes, only enforce if existing parser/ESLint effective config proves the rule applies correctly; otherwise rely on Nuasite rendered-output checks for emitted `<img alt>`

G3TS i18n config checks must enforce `jsx-a11y/alt-text` only on lanes where ESLint proves it is effective. It must not require `jsx-a11y` for pure `.astro` files if the rule does not actually run there.

Nuasite remains the rendered-output backstop for emitted image alt presence.

## Raw date and number formatting

Delegate to core ESLint:

- `no-restricted-syntax`

Required banned selectors on public source lanes:

```js
{
  selector: "CallExpression[callee.property.name='toLocaleDateString']",
  message: "Use an approved locale date formatting helper instead of formatting dates inline."
}
{
  selector: "CallExpression[callee.property.name='toLocaleString']",
  message: "Use an approved locale date/number formatting helper instead of formatting values inline."
}
{
  selector: "NewExpression[callee.object.name='Intl'][callee.property.name='DateTimeFormat']",
  message: "Use an approved locale date formatting helper instead of constructing Intl.DateTimeFormat inline."
}
{
  selector: "NewExpression[callee.object.name='Intl'][callee.property.name='NumberFormat']",
  message: "Use an approved locale number formatting helper instead of constructing Intl.NumberFormat inline."
}
```

Allowed helper modules must be enforced by ESLint flat-config overrides:

- public source lanes: selectors above are `error`
- approved helper modules: selectors are absent or explicitly off

G3TS must verify both facts:

- public source probes have the required `no-restricted-syntax` selectors
- configured helper module probes do not have those bans

If the ESLint parser cannot expose selector options for a rule, add parser support in `eslint-config-parser`; do not approximate with raw string contains in family code.

# Custom ESLint Rules

## Shared config

Each custom rule accepts a single object:

```ts
interface AstroI18nPolicyRuleOptions {
  readonly locales: readonly string[];
  readonly defaultLocale?: string;
  readonly requireLocalePrefixForContentRoutes?: boolean;
  readonly allowedUnprefixedRoutes?: readonly string[];
  readonly contentRoutePrefixes?: readonly string[];
  readonly approvedInternalLinkHelpers?: readonly string[];
  readonly approvedLocalizedLinkComponents?: readonly string[];
  readonly contentImageComponents?: readonly string[];
  readonly contentImageKeyProps?: readonly string[];
  readonly bannedImageSourceProps?: readonly string[];
  readonly bannedImageAltProps?: readonly string[];
}
```

No hidden defaults. The rule must report a config error if a required option for that rule is absent or empty.

## `no-unlocalized-internal-hrefs`

Checks:

- JSX/TSX/MDX/Astro attribute literals:
  - `href`
  - `to`
- string literals passed as first argument to configured helper functions unless helper is approved as locale-aware

Fails when:

- value is an internal durable content route
- route starts with one of `contentRoutePrefixes`
- route is not prefixed by one of `locales`
- route is not listed in `allowedUnprefixedRoutes`

Allows:

- external URLs
- protocol URLs
- hash-only links
- `mailto:`
- `tel:`
- routes already prefixed with configured locale
- configured `allowedUnprefixedRoutes`
- configured approved localized link components
- configured approved internal link helpers

Report message must include:

- offending value
- configured locales
- configured content route prefixes
- exact suggested fix: use one of `approvedInternalLinkHelpers` or `approvedLocalizedLinkComponents`

Do not try to infer Astro route files.

## `no-inline-image-alt`

Checks:

- configured `contentImageComponents`
- props named in `bannedImageAltProps`

Fails when:

- content image component has inline non-empty string alt

Allows:

- no alt prop
- empty string only if explicitly allowed later; initial implementation does not add that option
- non-content image components

Report message must include:

- component name
- prop name
- suggested fix: move alt text into the locale-owned content image entry and pass an image key

## `require-content-image-key`

Checks:

- configured `contentImageComponents`

Fails when:

- component lacks at least one prop from `contentImageKeyProps`
- component has any prop from `bannedImageSourceProps`

Report message must include:

- component name
- required key prop names
- banned source prop name if present

Example valid config:

```js
{
  contentImageComponents: ["ArticleImage"],
  contentImageKeyProps: ["image"],
  bannedImageSourceProps: ["src", "url"],
  bannedImageAltProps: ["alt"]
}
```

# G3TS Astro i18n Config

Add a new section in `guardrail3-ts.toml`:

```toml
[ts.astro.i18n]
locales = ["en"]
default_locale = "en"
require_locale_prefix_for_content_routes = true
allowed_unprefixed_routes = ["/", "/robots.txt", "/llms.txt", "/sitemap-index.xml"]
content_route_prefixes = ["/blog"]
approved_internal_link_helpers = ["localizedHref", "buildLocalizedPath"]
approved_localized_link_components = ["LocalizedLink"]
approved_date_format_helpers = ["src/i18n/format-date.ts"]
approved_number_format_helpers = ["src/i18n/format-number.ts"]
content_image_components = ["ArticleImage"]
content_image_key_props = ["image"]
banned_image_source_props = ["src", "url"]
banned_image_alt_props = ["alt"]
public_source_globs = ["src/**/*.{astro,ts,tsx}", "content/**/*.mdx"]
helper_source_globs = ["src/i18n/**/*.ts"]
```

All paths are app-relative.

No default locale assumptions in G3TS. Missing config is an error when the Astro app declares strict i18n mode.

## G3TS checks

Rule IDs:

- `g3ts-astro-i18n/strict-policy-configured`
- `g3ts-astro-i18n/policy-paths-valid`
- `g3ts-astro-i18n/i18next-plugin-wired`
- `g3ts-astro-i18n/i18n-policy-plugin-wired`
- `g3ts-astro-i18n/no-unlocalized-internal-hrefs-rule`
- `g3ts-astro-i18n/no-inline-image-alt-rule`
- `g3ts-astro-i18n/require-content-image-key-rule`
- `g3ts-astro-i18n/raw-date-number-formatting-bans`
- `g3ts-astro-i18n/protected-i18n-rule-disables-restricted`

Each rule checks one thing only.

### `strict-policy-configured`

Fails when:

- `[ts.astro.i18n]` is missing
- `locales` is empty
- `content_route_prefixes` is empty while `require_locale_prefix_for_content_routes = true`
- `public_source_globs` is empty

### `policy-paths-valid`

Fails when:

- configured app-relative paths traverse out with `..`
- helper paths are absolute filesystem paths
- glob strings are empty

### `i18next-plugin-wired`

Fails when:

- `eslint-plugin-i18next` is missing from package facts
- ESLint effective config for public source probes lacks plugin namespace `i18next`
- `i18next/no-literal-string` is not `error`

### `i18n-policy-plugin-wired`

Fails when:

- `g3ts-eslint-plugin-astro-i18n-policy` is missing
- ESLint effective config for public source probes lacks plugin namespace `astro-i18n-policy`

### Rule-specific wiring checks

For each custom rule:

- verify the rule is `error` on public source probes
- verify required option arrays exactly match or are equivalent to `[ts.astro.i18n]`
- reject dynamic or missing rule options when parser cannot prove them

### `raw-date-number-formatting-bans`

Fails when:

- public source probes do not have `no-restricted-syntax` at `error`
- any required selector is missing
- approved helper module probes still have the bans active

### `protected-i18n-rule-disables-restricted`

Fails when:

- `@eslint-community/eslint-comments/no-restricted-disable` is not active on public source lanes
- protected patterns do not cover:
  - `i18next/no-literal-string`
  - `astro-i18n-policy/*`
  - `no-restricted-syntax`

# Hook Contract

`g3ts-astro-i18n-hook-contract` exposes:

```rust
pub fn hook_contract() -> G3TsHookContract
```

Required hook behavior:

- pre-commit must run app-level `validate`
- pre-commit must run G3TS validation for the app
- trigger files include:
  - `eslint.config.*`
  - `guardrail3-ts.toml`
  - `src/**/*.astro`
  - `src/**/*.ts`
  - `src/**/*.tsx`
  - `content/**/*.mdx`

Do not put hook requirements in ingestion or check packages.

# Package Pins

Astro setup Syncpack required pins must add:

- `g3ts-eslint-plugin-astro-i18n-policy`
- `eslint-plugin-i18next`
- `@eslint-community/eslint-plugin-eslint-comments`

Do not add `g3ts-astro-i18n-checks`; that app-facing package does not exist in this architecture.

# Tests

## ESLint plugin tests

Use ESLint `RuleTester` or the package's existing test convention if one exists.

Required tests:

- unlocalized `/blog/foo` fails
- `/en/blog/foo` passes
- `/` passes when configured as allowed
- external URL passes
- approved localized component passes
- inline `alt="English text"` on configured content image component fails
- content image component with `image="hero"` passes
- content image component with `src="..."` fails
- missing required config reports config error

## G3TS tests

Each G3TS check file gets sidecar tests following current package pattern.

Required tests:

- golden input reports exactly the expected i18n rule IDs
- missing `[ts.astro.i18n]` fails
- missing plugin package fails
- missing plugin namespace fails
- missing custom rule fails
- wrong custom rule options fail
- missing `i18next/no-literal-string` fails
- missing `no-restricted-syntax` selector fails
- helper module probe with formatting ban active fails
- restricted disables missing protected i18n patterns fails

# Non-goals

- Do not implement language detection in this phase.
- Do not implement built-output i18n artifact auditing in this phase.
- Do not inspect sitemap or HTML output for i18n in G3TS.
- Do not add app-facing `g3ts-astro-i18n-checks`.
- Do not hardcode seochecks route names, component names, helpers, or locales.
- Do not parse source AST in Rust G3TS.
