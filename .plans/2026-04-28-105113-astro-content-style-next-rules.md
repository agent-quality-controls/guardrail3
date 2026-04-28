# Goal

Define the next G3TS implementation source of truth for strict Astro content apps after the semantic rule ID migration.

The target is a guardrail setup that:

- keeps delegated validators delegated
- makes escape hatches visible
- keeps Astro-specific content rules inside Astro families
- does not create cross-family dependency chains
- starts a separate style family for Tailwind/CSS policy

# Current Baseline

Already implemented:

- Astro setup/content/MDX/SEO/state are separate packages under `packages/ts/astro`.
- G3TS requires Astro static content apps to use Astro, not Next/Velite/Contentlayer.
- G3TS requires Syncpack package pins and forbidden dependency policy.
- G3TS requires Astro integrations for React, MDX, sitemap, robots, and Nuasite checks.
- G3TS requires content routes to use approved content adapters.
- G3TS requires inline public-copy lint through ESLint.
- G3TS requires MDX lanes, approved component maps, Zod wrapper parsing, and raw image bans.
- G3TS requires Nuasite rendered-page checks and JSON-LD presence check wiring.
- Current emitted rule IDs are semantic.

# Architectural Decisions

## Escape Hatches Are Allowed But Must Be Visible

Inline suppressions are allowed only as explicit escape hatches.

They must never be hidden.

Policy:

- `eslint-disable`, `eslint-disable-next-line`, and `eslint-disable-line` may exist.
- Every disable directive must include a description.
- Every disable directive must be inventoried or reported as a warning by the guardrail system.
- Disabling protected delegated rules should require an explicit guardrail waiver, not only an inline ESLint directive.
- Unused disable directives must fail.

Do not use blanket "no disable comments anywhere" as the default policy.

## Delegate Suppression Parsing To ESLint

Require the third-party package `@eslint-community/eslint-plugin-eslint-comments`.

This is not our plugin. G3TS only enforces that the third-party plugin is installed, active, and configured to the contract below.

Required delegated rules:

- `@eslint-community/eslint-comments/require-description`
- `@eslint-community/eslint-comments/no-unused-disable`
- `@eslint-community/eslint-comments/no-restricted-disable`

Do not implement suppression parsing in G3TS with tree-sitter unless ESLint cannot cover a required lane.

If parser-backed extraction becomes necessary, put it in a shared parser/ingestion layer. Do not hand-roll comment regex in check rules.

## Whole-App Validation Script Belongs To Astro Setup

Astro setup owns the app-level validation command because it owns the Astro app validator stack.

Hooks/CI families own whether hooks or workflows call that command.

Split:

- Astro setup requires a fail-closed app script named `validate`.
- The standard app script name is `validate`.
- That script must safely run lint, package lint, typecheck, and build.
- Hooks later require pre-commit/CI to invoke the app validation script.

Do not put the app script contract only in hooks. Hooks cannot prove the app exposes a correct validation command for agents to run.

## URL And Crawler Artifacts Belong To Astro SEO

The crawler-surface rules are necessary:

- every route output visible to crawlers must have exactly one canonical public URL path.
- sitemap output must not expose duplicate URLs, slash variants, `http`, foreign hosts, or both bare and `www` hosts.
- robots output must point to the one approved sitemap URL.
- llms output must exist and parse if the app is configured as an AI-readable content app.

These rules are Astro SEO/output rules, not Astro content rules and not generic `TS-CONTENT` rules.

Do not require a route manifest for this. A route manifest duplicates Astro routes and sitemap output, and it creates another generated artifact agents must wire correctly.

The source of truth is the final static output that crawlers see:

- generated sitemap XML
- generated robots.txt
- generated llms.txt
- rendered page output checked by Nuasite and existing JSON-LD checks

Do not create a generic content family that depends on Astro extraction. That creates cross-family orchestration and fragile dependency ordering.

Future `TS-CONTENT` is allowed only if applications themselves produce a framework-independent content manifest artifact as part of normal build/tooling.

## Style/Tailwind Is Not Astro

Tailwind and CSS policy belongs to a separate TS style family.

Astro must not own:

- arbitrary Tailwind value bans
- required design-token utilities
- class ordering
- CSS file policy
- component styling constraints that would still apply in Next/Vite/React

# Implementation Plan

## 1. Astro Suppression Visibility And Restriction

### Owner Packages

- `packages/ts/astro/setup/g3ts-astro-setup-config-checks`
- `packages/ts/astro/content/g3ts-astro-content-config-checks`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks`
- shared ESLint ingestion facts under the relevant Astro ingestion packages

### Package Policy

Astro setup must require package:

- `@eslint-community/eslint-plugin-eslint-comments`

The version floor belongs in Syncpack Astro stack policy.

Add it to the required Syncpack version group for strict Astro content apps.

### Effective ESLint Requirements

G3TS must prove the ESLint plugin is active on these lanes:

- Astro route/source lane
- TS route/source lane
- TSX route/source lane
- MDX content lane
- approved MDX component-map lane

The plugin namespace must be:

- `@eslint-community/eslint-comments`

The app must enable:

- `@eslint-community/eslint-comments/require-description` at `error`
- `@eslint-community/eslint-comments/no-unused-disable` at `error`, or ESLint core unused-disable reporting at an equivalent fail-closed level
- `@eslint-community/eslint-comments/no-restricted-disable` at `warn` or `error`

### Protected Rules

`no-restricted-disable` must cover all delegated Astro rules that enforce architecture:

- `astro/valid-compile`
- `astro-pipeline/*`
- `i18next/no-literal-string`
- `mdx/remark`
- `@eslint-community/eslint-comments/*`

The exact config shape must be checked from effective ESLint config facts, not raw source text.

If the plugin cannot express wildcard restriction reliably through effective config facts, implement exact required entries for the concrete delegated rules currently enforced by G3TS.

### Warning Inventory

G3TS should report a warning inventory when disable directives exist.

Preferred implementation:

- delegate inventory to ESLint output if the plugin can expose/report all directive comments
- otherwise add parser-backed comment extraction in ingestion

Do not use raw substring checks in rule code.

### New Semantic Rule IDs

Use semantic IDs. Do not introduce numeric IDs.

Astro setup:

- `g3ts-astro-setup/eslint-comments-plugin-package-present`
- `g3ts-astro-setup/eslint-disable-descriptions-required`
- `g3ts-astro-setup/unused-eslint-disables-fail`

Astro content:

- `g3ts-astro-content/protected-content-rule-disables-restricted`
- `g3ts-astro-content/eslint-disable-inventory`

Astro MDX:

- `g3ts-astro-mdx/protected-mdx-rule-disables-restricted`
- `g3ts-astro-mdx/eslint-disable-inventory`

Astro SEO:

- `g3ts-astro-seo/protected-seo-rule-disables-restricted`
- `g3ts-astro-seo/eslint-disable-inventory`

If one shared setup rule can prove all lanes cleanly, keep package/wiring checks in setup and lane-specific protected-rule checks in the owning subfamily.

### Tests

Add tests proving:

- missing plugin package fails
- missing plugin namespace fails
- `require-description` missing fails
- `require-description` warning severity fails
- `no-unused-disable` missing fails unless core unused-disable reporting is fail-closed
- `no-restricted-disable` missing fails
- `no-restricted-disable` not covering `astro-pipeline/*` fails
- `eslint-disable-next-line astro-pipeline/... -- reason` is warned/inventoried, not hidden
- `eslint-disable-next-line astro-pipeline/...` without description fails through ESLint
- valid config with described disable inventory passes with warning/inventory only

## 2. Astro Whole-App Validation Script

### Owner Package

- `packages/ts/astro/setup/g3ts-astro-setup-config-checks`

### Rule

Add:

- `g3ts-astro-setup/validate-script`

Rule behavior:

- `package.json` must define `validate`.
- `check` is not an accepted alternative name.
- The script must fail closed.
- The script must invoke all required local validators:
  - ESLint lint script or direct `eslint`
  - Syncpack package lint script or direct `syncpack lint`
  - Astro typecheck script or direct `astro check`
  - Astro build script or direct `astro build`
- The script must not use `|| true`, ignored parser blockers, unsupported shell syntax, or command forms the package-script parser cannot understand.
- The script may call existing scripts through package-manager run commands if the command parser proves those scripts exist and are safe.
- If `package.json` defines extra validation-like scripts such as `check`, `verify`, `ci`, `precommit`, or `lint:all`, they must either be safe and fail-closed or be removed.

Use the existing package-script command parser.

Do not check raw script substrings.

### Tests

Add tests proving:

- missing `validate` fails
- `validate` that runs only lint fails
- `validate` with `|| true` fails
- `validate` with parser blocker fails
- `validate` invoking safe child scripts passes
- `check` without `validate` fails
- extra validation-like script with unsafe command fails
- extra validation-like script that is safe passes
- deleting the unsafe extra script passes

## 3. Astro Crawler Artifact Generation And Validation

### Owner Packages

- `packages/ts/astro/seo/g3ts-astro-seo-config-checks`
- `packages/ts/astro/seo/g3ts-astro-seo-file-tree-checks`
- new Astro integration package `packages/ts/astro/integrations/g3ts-astro-llms`
- new delegated sitemap checker package `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks`
- new delegated robots checker package `packages/ts/astro/robots/g3ts-astro-robots-checks`
- new delegated llms checker package `packages/ts/astro/llms/g3ts-astro-llms-checks`

### Package Policy

Use existing generators where they are narrow and canonical:

- `@astrojs/sitemap` generates sitemap XML.
- `astro-robots` generates robots.txt.

Do not require `@agentmarkup/astro` as the standard.

Reason:

- it overlaps with JSON-LD, robots, markdown mirrors, headers, and validation.
- it is useful as an optional app choice, but too broad to be the guardrail standard.
- G3TS should require narrow tools with one responsibility.

Use narrow G3TS-owned packages where the existing ecosystem does not provide a focused package:

- `g3ts-astro-llms` generates llms.txt.
- `g3ts-astro-sitemap-checks` validates generated sitemap XML. Current required floor: `0.1.2`.
- `g3ts-astro-robots-checks` validates generated robots.txt. Current required floor: `0.1.2`.
- `g3ts-astro-llms-checks` validates generated llms.txt.

### Astro Integration: `g3ts-astro-llms`

This is an Astro integration, not a G3TS check.

Responsibilities:

- generate `dist/llms.txt` during Astro build.
- accept explicit config only. No hidden defaults.
- emit deterministic output.
- optionally read approved Astro content adapters only if explicitly configured.
- never generate sitemap XML.
- never generate robots.txt.
- never inject JSON-LD.
- never patch headers.

Required config shape:

```ts
import g3tsLlms from "g3ts-astro-llms";

export default defineConfig({
  integrations: [
    g3tsLlms({
      title: "Site title",
      site: "https://example.com",
      sections: [
        {
          heading: "Docs",
          links: [
            { title: "Home", href: "/" }
          ]
        }
      ]
    })
  ]
});
```

Do not infer content adapters from folder names. If content-derived link generation is ever added, it must be a real implemented option with tests proving the generated links, not a no-op config field.

### Delegated Checker: `g3ts-astro-sitemap-checks`

This is a post-build checker package, not a G3TS core rule.

It must use a real XML parser.

It must check final sitemap files under the configured Astro output directory.

Required checks:

- sitemap XML parses.
- sitemap index recursion is followed when sitemap index files are present.
- every `<loc>` uses the configured HTTPS host exactly.
- no `<loc>` uses `http`.
- no `<loc>` uses a foreign host.
- no `<loc>` uses both bare and `www` host variants.
- no duplicate `<loc>`.
- no slash/no-slash pair for the same path.

It must not:

- infer routes from source files.
- inspect Astro collections.
- generate sitemap XML.
- mutate final output.
- validate page links.

### Delegated Checker: `g3ts-astro-robots-checks`

This is a post-build checker package, not a G3TS core rule.

It must parse final `robots.txt` with `robots-parser` or `robots-txt-parse`.

Required checks:

- `robots.txt` exists.
- `robots.txt` parses.
- `robots.txt` contains exactly one approved `Sitemap:` URL unless config explicitly lists more than one.
- every robots `Sitemap:` URL uses the configured HTTPS host exactly.
- no robots `Sitemap:` URL uses `http`.
- no robots `Sitemap:` URL uses a foreign host.
- no robots `Sitemap:` URL uses the non-canonical `www` or bare host variant.

It must not:

- generate robots.txt.
- mutate final output.
- validate sitemap XML contents.

### Delegated Checker: `g3ts-astro-llms-checks`

This is a post-build checker package, not a G3TS core rule.

It must parse final `llms.txt` with `parse-llms-txt`.

Required checks:

- `llms.txt` exists when strict AI-readable content policy is enabled.
- `llms.txt` parses with `parse-llms-txt`.
- every configured required section exists.
- every configured required link is present.

It must not:

- generate llms.txt.
- mutate final output.
- validate sitemap XML or robots.txt.

### Shared Output Checker Rules

It must use real parsers:

- sitemap XML: parse with an XML parser.
- robots.txt: parse with `robots-parser` or `robots-txt-parse`.
- llms.txt: parse with `parse-llms-txt`.

Shared parser helpers may live in a shared support package if needed, but the executable packages must remain split by artifact.

### Astro Config Contract

Strict Astro content apps must set:

- `site` to one canonical `https://` URL.
- `output: "static"`.
- `trailingSlash: "always"`.

Rejected config:

- missing `site`.
- `site` using `http`.
- missing `trailingSlash`.
- `trailingSlash: "ignore"`.
- `output: "server"`.

`trailingSlash: "never"` is not accepted by default. It requires a guardrail waiver plus output-check proof that sitemap and canonical URLs expose only the no-slash shape.

### G3TS Responsibilities

G3TS should check:

- `@astrojs/sitemap` is installed.
- `@astrojs/sitemap` integration is present in `astro.config`.
- `astro-robots` is installed.
- `astro-robots` integration is present in `astro.config`.
- `g3ts-astro-llms` is installed when strict AI-readable content policy is enabled.
- `g3ts-astro-llms` integration is present in `astro.config` when required.
- `g3ts-astro-sitemap-checks` is installed.
- `g3ts-astro-robots-checks` is installed.
- `g3ts-astro-llms-checks` is installed when strict AI-readable content policy is enabled.
- `astro.config` has `site`, `output: "static"`, and `trailingSlash: "always"`.
- `validate` runs `astro build` before `g3ts-astro-sitemap-checks`.
- `validate` runs `astro build` before `g3ts-astro-robots-checks`.
- `validate` runs `astro build` before `g3ts-astro-llms-checks` when strict AI-readable content policy is enabled.
- `validate` fails closed if any artifact checker fails.
- `@agentmarkup/astro` is absent unless explicitly waived as an app-specific replacement.

G3TS should not:

- parse sitemap XML in core.
- parse robots.txt in core.
- parse llms.txt in core.
- read Astro collections for URL uniqueness.
- compute slugs from filenames.
- require a route manifest.
- require a fixed content folder layout.
- require a fixed collection taxonomy.

### New Semantic Rule IDs

Astro SEO:

- `g3ts-astro-seo/canonical-site-config`
- `g3ts-astro-seo/static-output-config`
- `g3ts-astro-seo/trailing-slash-policy`
- `g3ts-astro-seo/sitemap-integration-present`
- `g3ts-astro-seo/robots-integration-present`
- `g3ts-astro-seo/llms-integration-present`
- `g3ts-astro-seo/sitemap-generator-package-present`
- `g3ts-astro-seo/robots-generator-package-present`
- `g3ts-astro-seo/llms-generator-package-present`
- `g3ts-astro-seo/sitemap-checks-package-present`
- `g3ts-astro-seo/robots-checks-package-present`
- `g3ts-astro-seo/llms-checks-package-present`
- `g3ts-astro-seo/sitemap-checks-validate-script`
- `g3ts-astro-seo/robots-checks-validate-script`
- `g3ts-astro-seo/llms-checks-validate-script`
- `g3ts-astro-seo/broad-crawler-generator-absent`

### Tests

Add tests proving:

- missing `site` fails.
- `site: "http://..."` fails.
- generated artifacts using any host other than `astro.config.site` fail in the delegated artifact checker packages.
- missing `trailingSlash` fails.
- `trailingSlash: "ignore"` fails.
- missing `@astrojs/sitemap` package fails.
- missing sitemap integration fails.
- missing `astro-robots` package fails.
- missing robots integration fails.
- strict AI-readable policy without `g3ts-astro-llms` fails.
- strict AI-readable policy without `g3ts-astro-llms` package fails.
- strict AI-readable policy without `g3ts-astro-llms` integration fails.
- missing `g3ts-astro-sitemap-checks` package fails.
- missing `g3ts-astro-robots-checks` package fails.
- missing `g3ts-astro-llms-checks` package fails when strict AI-readable policy is enabled.
- `validate` that builds but does not run sitemap checks fails.
- `validate` that builds but does not run robots checks fails.
- `validate` that builds but does not run llms checks fails when strict AI-readable policy is enabled.
- `validate` that runs artifact checks before build fails.
- `@agentmarkup/astro` fails unless waived.
- valid config passes.

For `g3ts-astro-sitemap-checks`, add package-local tests proving:

- duplicate sitemap `<loc>` fails.
- slash/no-slash sitemap pair fails.
- `http` sitemap URL fails.
- wrong-host sitemap URL fails.
- `www`/bare mixed sitemap URLs fail.

For `g3ts-astro-robots-checks`, add package-local tests proving:

- robots with wrong sitemap host fails.
- robots with duplicate sitemap URLs fails.

For `g3ts-astro-llms-checks`, add package-local tests proving:

- malformed llms.txt fails.
- valid llms.txt passes.

## 4. Astro Draft And Future Publication Policy

### Owner Package

- `packages/ts/astro/content/g3ts-astro-content-config-checks`

Do not implement this in the current slice.

Do not introduce a route manifest to support this rule.

This rule needs a separate design because Astro content collections can express draft/date policy inside collection schemas and content adapter modules.

Potential rules:

- `g3ts-astro-content/no-published-drafts`
- `g3ts-astro-content/no-future-published-content`

Rule behavior:

- `published = true` and `draft = true` fails unless policy explicitly allows it.
- `published = true` with date after current date fails unless policy explicitly allows future publishing.
- Current date must come from deterministic runtime input in tests.

Preferred future direction:

- require Astro content schemas to define explicit `draft` and `publishedAt` fields for configured public collections.
- require approved content adapters to filter drafts and future-dated content before route generation.
- use ESLint/TypeScript checks around adapters if the adapter contract can be expressed without parsing content files in G3TS core.

Non-goal:

- link checking.

## 5. Astro Content Schema Depth

### Owner Package

- `packages/ts/astro/content/g3ts-astro-content-config-checks`

This rule is still less settled than URL path uniqueness.

Target:

- ensure public content schemas are not shallow placeholders.

Possible rule:

- `g3ts-astro-content/content-schema-required-fields`

Policy source:

- `guardrail3-ts.toml` explicitly lists required field paths per configured Astro content collection or content adapter contract.

Do not hardcode blog-specific fields into G3TS.

Do not inspect arbitrary Zod source with regex.

Implementation options:

1. Prefer Astro schema or adapter-level TypeScript contracts if they can be checked through existing parsers.
2. If schema source inspection is required, delegate to ESLint or TypeScript tooling rather than Rust string matching.

Decision needed before coding:

- whether schema depth should be enforced from Astro `src/content.config.ts`, adapter return types, or a delegated ESLint rule.

Preferred direction:

- keep this inside Astro content because Astro content collections are the framework feature.
- do not create a generic content family.
- do not create a route manifest only to prove schema depth.

## 6. Style/Tailwind Family

### New Family

Create a separate TS style family. Do not put style rules in Astro.

Target package structure:

- `packages/ts/style/g3ts-style-types`
- `packages/ts/style/g3ts-style-ingestion`
- `packages/ts/style/g3ts-style-config-checks`
- `packages/ts/style/g3ts-style-source-checks`
- `packages/ts/style/g3ts-style-file-tree-checks` only if needed

### Delegation First

Research current packages before implementation.

Candidate delegated tools:

- Stylelint for CSS files and CSS modules.
- Tailwind-specific ESLint plugin for class usage if mature and compatible.
- Prettier or class sorting plugin only if project policy requires deterministic ordering.
- Syncpack for required style package pins and banned style packages.

Do not hand-parse Tailwind class strings until mature plugin options have been evaluated.

### Candidate Style Rules

Possible rules after tool research:

- required Stylelint package and config
- safe `lint:styles` script
- Tailwind version floor through Syncpack
- arbitrary Tailwind value ban
- design token utility policy
- class ordering policy
- CSS module/global CSS placement policy

### Boundary

Style family rules must apply to Astro, Next, Vite, and React packages when configured.

Style family must not require:

- Astro config
- Astro routes
- Astro content collections
- Astro integrations

# Non-Goals

- Do not build generic `TS-CONTENT` now.
- Do not make `TS-CONTENT` depend on Astro extraction.
- Do not do link checking now.
- Do not move rendered SEO validation out of Astro until there is a framework-independent rendered artifact validator family.
- Do not add numeric rule IDs.
- Do not parse ESLint disable comments with regex in rules.
- Do not infer public URLs from filenames for Astro content.
- Do not require an app-owned route manifest for crawler URL uniqueness.
- Do not require `@agentmarkup/astro` as the standard llms.txt generator.

# Implementation Order

1. Astro suppression visibility and restriction.
2. Astro whole-app validation script.
3. `g3ts-astro-llms` Astro integration package.
4. `g3ts-astro-sitemap-checks` delegated sitemap checker package.
5. `g3ts-astro-robots-checks` delegated robots checker package.
6. `g3ts-astro-llms-checks` delegated llms checker package.
7. Astro SEO G3TS checks requiring sitemap, robots, llms, and artifact-check wiring.
8. Draft/future publication policy after a separate Astro content design.
9. Content schema depth after a separate Astro content design.
10. Style/Tailwind family after package/tool research.

# Verification Standard

For every implementation slice:

- add rule-level tests before fixes where possible
- run package tests for touched crates
- run `cargo test --workspace --offline --locked` in `apps/guardrail3-ts`
- run `g3rs validate --path` on touched G3TS packages
- install local `g3ts`
- run `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- run adversarial review against this plan and the code
