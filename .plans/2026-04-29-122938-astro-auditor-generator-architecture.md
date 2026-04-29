# Goal

Move Astro artifact validation out of G3TS CLI script-order contracts and into Astro build lifecycle integrations, while delegating rendered-output checks to Nuasite.

The app contract must be:

- generators create standard site artifacts during `astro build`
- auditors validate generated artifacts during `astro build`
- Nuasite validates rendered HTML output during `astro build`
- G3TS verifies generators/auditors are installed and wired in `astro.config.*`
- G3TS does not parse or validate generated sitemap, robots, llms, media, icon, structured-data, OpenGraph, Twitter-card, accessibility, performance, or rendered HTML artifacts
- old `*-checks` packages are removed from this repository
- old checker CLI binaries are removed
- no backwards compatibility for old package names, old CLI entrypoints, or old script contracts

# Naming

Use these words consistently:

- `generator`: creates an artifact
- `auditor`: validates an artifact or rendered output
- no new app-facing package should use `checks`

Existing `*-checks` packages must be deleted. Their useful validation code must be moved into the corresponding `*-auditor` package. No `*-checks` package may remain as a private dependency, workspace package, app dependency, package.json script target, or published package in this repository.

Exception:

- `g3ts-astro-nuasite-checks` is not an artifact checker CLI package.
- It is a Nuasite custom-check package.
- It may remain only for checks that Nuasite does not provide itself.
- It must not become a separate rendered-output validation framework.
- It must not parse rendered HTML directly; it must consume Nuasite `PageCheckContext` or `SiteCheckContext`.

# Final Package Set

## Sitemap

### Generator

Use the upstream Astro generator:

- `@astrojs/sitemap`

Do not write a `g3ts-astro-sitemap-generator` unless `@astrojs/sitemap` cannot satisfy a specific required policy.

### Auditor

Package:

- `g3ts-astro-sitemap-auditor`

Type:

- TypeScript npm package
- Astro integration

Export:

```ts
export default function g3tsSitemapAuditor(config: G3TsSitemapAuditorConfig): AstroIntegration;
```

Astro lifecycle:

- run in `astro:build:done`
- use Astro-provided build output directory
- use emitted output files and available build route/page data from Astro hook context
- throw on audit failure so `astro build` fails

Parser dependencies:

- use maintained third-party sitemap/XML parser libraries
- current implementation may reuse `fast-xml-parser`
- do not create a G3TS sitemap parser package

Required audit behavior:

- generated sitemap index exists at configured filename, default `sitemap-index.xml`
- sitemap XML parses
- sitemap index targets exist in output
- every sitemap `loc` is absolute HTTPS
- every sitemap `loc` uses the canonical Astro `site` host
- no duplicate `loc`
- no slash/non-slash pair duplicate
- every publishable built Astro HTML page appears in sitemap
- every sitemap URL maps to a publishable built Astro HTML page unless explicitly configured as an allowed non-page URL
- generated artifact URLs follow Astro trailing slash policy

Config:

```ts
interface G3TsSitemapAuditorConfig {
  readonly site: string;
  readonly indexFilename?: string;
  readonly trailingSlash: "always" | "never";
  readonly allowedMissingRoutes?: readonly string[];
  readonly allowedExtraUrls?: readonly string[];
  readonly ignoredHtmlFiles?: readonly string[];
}
```

Strict config requirements:

- `site` required and must be HTTPS
- `trailingSlash` required
- arrays default to empty only inside the integration after validation
- no hidden default exceptions for app routes

Migration:

- move useful logic from `g3ts-astro-sitemap-checks` into `g3ts-astro-sitemap-auditor`
- delete `g3ts-astro-sitemap-checks`
- remove its CLI binary
- G3TS must stop requiring `g3ts-astro-sitemap-checks` CLI invocation
- G3TS must require `g3ts-astro-sitemap-auditor` integration

## Robots

### Generator

Use the approved Astro robots generator:

- `astro-robots`

Do not write a `g3ts-astro-robots-generator` unless `astro-robots` cannot satisfy a specific required policy.

### Auditor

Package:

- `g3ts-astro-robots-auditor`

Type:

- TypeScript npm package
- Astro integration

Export:

```ts
export default function g3tsRobotsAuditor(config: G3TsRobotsAuditorConfig): AstroIntegration;
```

Astro lifecycle:

- run in `astro:build:done`
- read emitted `robots.txt`
- throw on audit failure so `astro build` fails

Parser dependencies:

- use `robots-parser`
- do not create a G3TS robots parser package

Required audit behavior:

- `robots.txt` exists
- `robots.txt` parses
- contains exactly the configured sitemap URLs
- sitemap URLs are absolute HTTPS
- sitemap URLs use canonical Astro `site` host
- no duplicate sitemap URLs
- no bare/www host mixing
- every configured sitemap URL is present
- no unconfigured sitemap URL is present

Config:

```ts
interface G3TsRobotsAuditorConfig {
  readonly site: string;
  readonly sitemapUrls: readonly string[];
}
```

Strict config requirements:

- `site` required and must be HTTPS
- `sitemapUrls` required and non-empty

Migration:

- move useful logic from `g3ts-astro-robots-checks` into `g3ts-astro-robots-auditor`
- delete `g3ts-astro-robots-checks`
- remove its CLI binary
- G3TS must stop requiring `g3ts-astro-robots-checks` CLI invocation
- G3TS must require `g3ts-astro-robots-auditor` integration

## LLMs

Keep generator and auditor split.

Reason:

- generator proves we create `llms.txt`
- auditor proves the emitted `llms.txt` is exactly structurally acceptable after generation
- if generation or output mutation breaks, the auditor failure points at the emitted artifact

### Generator

Rename existing generator package:

- from `g3ts-astro-llms`
- to `g3ts-astro-llms-generator`

Type:

- TypeScript npm package
- Astro integration

Export:

```ts
export default function g3tsLlmsGenerator(config: G3TsLlmsGeneratorConfig): AstroIntegration;
```

Astro lifecycle:

- run in `astro:build:done`
- write `llms.txt`

Generation config:

Keep current strict config shape, renamed:

```ts
interface G3TsLlmsGeneratorConfig {
  readonly title: string;
  readonly site: string;
  readonly sections: readonly {
    readonly heading: string;
    readonly links: readonly {
      readonly title: string;
      readonly href: string;
      readonly description?: string;
    }[];
  }[];
}
```

### Auditor

Package:

- `g3ts-astro-llms-auditor`

Type:

- TypeScript npm package
- Astro integration

Export:

```ts
export default function g3tsLlmsAuditor(config: G3TsLlmsAuditorConfig): AstroIntegration;
```

Astro lifecycle:

- run in `astro:build:done`
- read emitted `llms.txt`
- throw on audit failure so `astro build` fails

Parser dependencies:

- use `parse-llms-txt`
- use maintained Markdown parser where needed
- do not create a G3TS llms parser package

Required audit behavior:

- `llms.txt` exists
- parses as llms.txt
- has non-empty title
- configured required sections exist
- every generated `llms.txt` link is absolute HTTPS
- every generated `llms.txt` link uses the canonical Astro `site` host unless explicitly configured as an allowed external URL
- every generated `llms.txt` link maps to a publishable built Astro HTML page unless explicitly configured as an allowed non-page URL
- every configured required route pattern has at least one matching generated `llms.txt` link
- no generated `llms.txt` link points at a missing built page
- section list items start with valid Markdown links
- malformed Markdown link text is rejected

Config:

```ts
interface G3TsLlmsAuditorConfig {
  readonly site: string;
  readonly requiredSections: readonly string[];
  readonly requiredRoutePatterns: readonly string[];
  readonly allowedExternalUrls: readonly string[];
  readonly allowedNonPageUrls: readonly string[];
  readonly ignoredHtmlFiles: readonly string[];
}
```

Strict config requirements:

- `site` required and must be HTTPS
- `requiredSections` required
- `requiredRoutePatterns` required
- `allowedExternalUrls` required
- `allowedNonPageUrls` required
- `ignoredHtmlFiles` required
- empty arrays are allowed only when the app intentionally has no policy for that field; config keys must still be present
- do not require enumerating every page URL in config
- do not make apps maintain huge per-page link lists

Migration:

- move useful logic from `g3ts-astro-llms-checks` into `g3ts-astro-llms-auditor`
- rename existing `g3ts-astro-llms` package to `g3ts-astro-llms-generator`
- delete `g3ts-astro-llms-checks`
- remove its CLI binary
- G3TS must stop requiring `g3ts-astro-llms-checks` CLI invocation
- G3TS must require both generator and auditor integrations

## Media

### Auditor

Package:

- no G3TS media auditor in this migration

Decision:

- do not implement `g3ts-astro-media-auditor` now
- Nuasite already owns rendered HTML image checks for missing alt, weak alt, image format, image size, lazy loading, external request count, and related performance/accessibility signals
- G3TS must enforce Nuasite integration and strict Nuasite failure settings instead of building a second rendered-output media checker
- future media checks that require URL graph, emitted asset cross-reference, image dimensions, or large artifact stores belong to the future Web Smasher audit pipeline, not to G3TS TypeScript

Immediate G3TS behavior:

- require `@nuasite/checks` integration
- require `failOnError: true`
- require `failOnWarning: true`
- require SEO, performance, accessibility, and geo checks enabled unless explicitly waived by guardrail config
- do not require a G3TS media auditor package

## Nuasite Rendered Output

Package:

- `@nuasite/checks`

Type:

- upstream Astro integration

Required behavior delegated to Nuasite:

- rendered title checks
- rendered meta description checks
- rendered canonical checks
- rendered viewport checks
- rendered heading hierarchy checks
- rendered image alt checks
- rendered OpenGraph checks
- rendered Twitter-card checks
- rendered JSON-LD validity checks when JSON-LD exists
- rendered robots/sitemap existence checks
- rendered `llms.txt` existence check
- rendered accessibility checks
- rendered performance checks

Required G3TS enforcement:

- `@nuasite/checks` is installed at the required Syncpack-pinned version
- `@nuasite/checks` is imported in `astro.config.*`
- `checks(...)` appears in Astro `integrations`
- `failOnError: true`
- `failOnWarning: true`
- `seo` is not `false`
- `performance` is not `false`
- `accessibility` is not `false`
- `geo` is not `false`
- `reportJson` is enabled

## G3TS Nuasite Custom Checks

Package:

- `g3ts-astro-nuasite-checks`

Type:

- Nuasite custom-check package
- not a CLI
- not an Astro integration
- not a standalone checker

Current custom check:

- `g3/structured-data-present`

Reason:

- Nuasite has `seo/json-ld-invalid`.
- Nuasite does not fail when a page has no JSON-LD.
- `g3/structured-data-present` adds only the missing presence policy.

Required G3TS enforcement when strict structured data is enabled:

- `g3ts-astro-nuasite-checks` is installed
- `structuredDataPresentCheck` is imported in `astro.config.*`
- `checks({ customChecks: [structuredDataPresentCheck], ... })` includes the custom check

Required limitation:

- do not add JSON-LD semantic validation to `g3ts-astro-nuasite-checks`
- do not parse HTML in `g3ts-astro-nuasite-checks`
- do not duplicate Nuasite checks in `g3ts-astro-nuasite-checks`
- delete the package if Nuasite adds an equivalent built-in required structured-data presence check that G3TS can enforce by Nuasite config

Migration:

- keep `g3ts-astro-nuasite-checks` only as a Nuasite custom-check package
- keep custom check scope limited to missing Nuasite policies

# G3TS Enforcement Changes

G3TS Rust CLI must enforce only wiring and strict configuration.

It must not:

- parse sitemap XML
- parse robots.txt
- parse llms.txt
- parse HTML/media artifacts
- parse rendered JSON-LD
- validate rendered structured-data semantics
- duplicate Nuasite rendered-output checks
- execute post-build artifact auditor CLIs
- inspect package scripts for artifact auditor CLI ordering
- validate artifact contents itself

It must enforce:

- required packages are present in package policy
- required Astro integrations are present in `astro.config.*`
- integration source modules match approved package names
- integration calls include statically visible config object or imported static config that existing Astro config parser can resolve
- strict required config keys are present when statically visible
- old app-facing `*-checks` package/script requirements are removed
- `validate` still runs `astro check` and `astro build`

Required Astro integrations:

- `@astrojs/sitemap`
- `astro-robots`
- `g3ts-astro-sitemap-auditor`
- `g3ts-astro-robots-auditor`
- `g3ts-astro-llms-generator`
- `g3ts-astro-llms-auditor`
- `@nuasite/checks`

Required Nuasite custom checks when strict structured data is enabled:

- `g3ts-astro-nuasite-checks`
- `structuredDataPresentCheck` registered through Nuasite `customChecks`

Required package bans for Astro apps:

- old removed checker packages:
  - `g3ts-astro-sitemap-checks`
  - `g3ts-astro-robots-checks`
  - `g3ts-astro-llms-checks`
- old generator package:
  - `g3ts-astro-llms`

These packages must not remain in app direct dependencies, workspace package manifests, generated plans as implementation targets, or G3TS required package lists.

# Astro Config Parser Requirements

The existing Astro config parser must support enough static inspection to verify:

- imported integration source module
- integration call presence
- first argument object keys
- inline first argument object keys

Imported config objects are not accepted in this migration. They require parser-level static import resolution first; until that exists, app config must use inline object literals so the guardrail can prove the contract without executing code.

If parser cannot prove config strictness:

- emit a G3TS error saying exactly which integration config cannot be statically verified
- do not silently accept dynamic config

# App Validate Script Policy

After migration, app `validate` script must still run:

- `astro check`
- `astro build`

It must not need to run:

- `g3ts-astro-sitemap-checks`
- `g3ts-astro-robots-checks`
- `g3ts-astro-llms-checks`
- future `g3ts-astro-media-checks`

Reason:

- auditors run inside `astro build`
- script-order enforcement for artifact auditors is replaced by Astro integration wiring enforcement
- artifact auditor CLIs do not exist after this migration

# Migration Order

1. Implement `g3ts-astro-sitemap-auditor`.
2. Move sitemap validation logic into `g3ts-astro-sitemap-auditor`.
3. Delete `g3ts-astro-sitemap-checks`.
4. Implement `g3ts-astro-robots-auditor`.
5. Move robots validation logic into `g3ts-astro-robots-auditor`.
6. Delete `g3ts-astro-robots-checks`.
7. Rename `g3ts-astro-llms` to `g3ts-astro-llms-generator`.
8. Implement `g3ts-astro-llms-auditor`.
9. Move llms validation logic into `g3ts-astro-llms-auditor`.
10. Delete `g3ts-astro-llms-checks`.
11. Update G3TS Astro setup/SEO rules to require generator/auditor integrations and Nuasite rendered-output wiring.
12. Remove old G3TS requirements for `*-checks` CLI scripts.
13. Add G3TS bans for removed `*-checks` package names and old `g3ts-astro-llms`.
14. Keep `g3ts-astro-nuasite-checks` only as a Nuasite custom-check package for missing Nuasite policies.
15. Update landing app to use integrations only.
16. Run `astro build` on landing and prove Nuasite/custom checks fail on intentionally broken fixtures before accepting clean output.
17. Release npm packages.
18. Install released packages in landing and rerun G3TS.

# Test Requirements

Each auditor package must have:

- unit tests for pure validation helpers
- integration tests using temporary output directories
- at least one test proving the Astro integration throws on invalid output
- at least one test proving clean output passes

G3TS must have contract tests proving:

- missing auditor integration fails
- wrong auditor package name fails
- missing strict config key fails when statically visible
- old CLI script invocation alone does not satisfy the rule
- old `*-checks` direct dependency is rejected where banned
- old `*-checks` workspace package directories no longer exist
- no package.json in the repository exposes `g3ts-astro-*-checks` binaries
- Nuasite integration missing fails
- Nuasite `failOnError: false` fails
- Nuasite `failOnWarning: false` fails
- strict structured-data mode without `structuredDataPresentCheck` fails

# Non-goals

- Do not build a generic sitemap parser package.
- Do not build a generic HTML parser package.
- Do not move artifact validation into Rust G3TS.
- Do not build a G3TS structured-data validator.
- Do not duplicate Nuasite rendered-output checks.
- Do not build `g3ts-astro-media-auditor` in this migration.
- Do not keep backwards compatibility for old package names.
- Do not keep app-facing CLI checker contracts.
- Do not keep private/internal `*-checks` packages.
- Do not make source ESLint media/i18n policy part of this migration.
