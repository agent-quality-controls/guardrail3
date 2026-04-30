# Goal

Finish the remaining Astro static content guardrail slice:

- G3TS requires Astro generator/auditor integrations for sitemap, robots, llms, Nuasite, and structured-data presence wiring.
- G3TS bans old app-facing checker CLI packages and old llms package names.
- G3TS stops accepting checker CLI script contracts as a substitute for Astro build lifecycle integrations.
- G3TS completes Astro i18n delegated-rule enforcement for public copy, date/number formatting, helper exceptions, and protected disables.

# Approach

## 1. Verify current package state

Read the existing package manifests and source before editing:

- `packages/ts/astro/integrations/g3ts-astro-llms-generator`
- `packages/ts/astro/llms/g3ts-astro-llms-auditor`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-auditor`
- `packages/ts/astro/robots/g3ts-astro-robots-auditor`
- `packages/ts/g3ts-astro-nuasite-checks`
- `packages/ts/astro/seo/g3ts-astro-seo-*`
- `packages/ts/astro/setup/g3ts-astro-setup-*`
- `packages/ts/astro/i18n/g3ts-astro-i18n-*`

Do not create new `*-checks` packages. Do not add artifact validation logic to Rust G3TS.

## 2. Astro SEO package and integration enforcement

Implement or tighten rules in `g3ts-astro-seo-config-checks` and ingestion so G3TS verifies only package/config wiring:

- package `@astrojs/sitemap` is installed and imported/called in `astro.config.*`
- package `astro-robots` is installed and imported/called in `astro.config.*`
- package `g3ts-astro-sitemap-auditor` is installed and imported/called in `astro.config.*`
- package `g3ts-astro-robots-auditor` is installed and imported/called in `astro.config.*`
- package `g3ts-astro-llms-generator` is installed and imported/called in `astro.config.*`
- package `g3ts-astro-llms-auditor` is installed and imported/called in `astro.config.*`
- package `@nuasite/checks` is installed and imported/called in `astro.config.*`
- Nuasite config has `failOnError: true`, `failOnWarning: true`, `reportJson` enabled, and does not disable `seo`, `performance`, `accessibility`, or `geo`
- package `g3ts-astro-nuasite-checks` is installed and its `structuredDataPresentCheck` is registered in Nuasite `customChecks` when strict structured-data policy is enabled

Expected rule naming must stay semantic, for example:

- `g3ts-astro-seo/sitemap-integration-present`
- `g3ts-astro-seo/robots-integration-present`
- `g3ts-astro-seo/llms-generator-integration-present`
- `g3ts-astro-seo/llms-auditor-integration-present`
- `g3ts-astro-seo/nuasite-checks`
- `g3ts-astro-seo/structured-data-check`

## 3. Ban old app-facing checker packages

In Astro setup package policy and/or SEO package policy, reject direct dependencies on:

- `g3ts-astro-sitemap-checks`
- `g3ts-astro-robots-checks`
- `g3ts-astro-llms-checks`
- `g3ts-astro-llms`

Also grep the repo and delete any old package directories or package script references if present. Do not keep private/internal compatibility packages.

## 4. Validate script policy

Keep app `validate` script requirements focused on standard app validation:

- `eslint`
- `syncpack lint`
- `astro check`
- `astro build`

Do not require `g3ts-astro-sitemap-checks`, `g3ts-astro-robots-checks`, `g3ts-astro-llms-checks`, or any artifact auditor CLI ordering. Auditors run inside `astro build`.

## 5. Astro i18n delegated rules

Finish `g3ts-astro-i18n` enforcement without source parsing in Rust:

- require `eslint-plugin-i18next` package
- require effective `i18next/no-literal-string: error` on configured public source probes
- require `g3ts-eslint-plugin-astro-i18n-policy` package and namespace
- require `astro-i18n-policy/no-unlocalized-internal-hrefs: error` with options matching `[ts.astro.i18n]`
- require core `no-restricted-syntax: error` on public source probes with exact selectors for raw date/number formatting
- require approved helper module probes do not have those formatting bans active
- require `@eslint-community/eslint-comments/no-restricted-disable` covers `i18next/no-literal-string`, `astro-i18n-policy/*`, and `no-restricted-syntax`

Do not resurrect i18n image fields or rules. Media owns image policy.

## 6. Tests

Add or update sidecar tests for each changed Rust rule package.

Required coverage:

- missing required SEO integration fails
- wrong package name fails
- missing Nuasite fail-closed option fails
- old checker package dependency fails
- old checker CLI script alone does not satisfy integration wiring
- missing i18next package fails
- missing i18next rule fails
- missing date/number selector fails
- helper module still banned fails
- missing protected-disable pattern fails

## 7. Verification

Run:

- package-local npm tests for changed TS packages
- cargo tests for changed Rust packages
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- install local G3TS CLI
- `g3rs validate` on changed Rust packages
- `g3ts validate` against landing app if present
- adversarial review against this plan and the original architecture plan

## Files to modify

Expected roots:

- `packages/ts/astro/seo/g3ts-astro-seo-types`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks`
- `packages/ts/astro/i18n/g3ts-astro-i18n-types`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion`
- `packages/ts/astro/i18n/g3ts-astro-i18n-config-checks`
- shared parsers only if a parser cannot expose the required typed facts
