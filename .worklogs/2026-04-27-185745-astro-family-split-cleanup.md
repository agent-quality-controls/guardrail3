Summary:
- Replaced the fake aggregate Astro split with per-family Astro setup, content, MDX, SEO, and state packages.
- Removed shared semantic Astro type and ingestion surfaces so family ingestion owns package/config parsing into its own DTOs.
- Added regression coverage preventing aggregate Astro packages, shared Astro type packages, and non-ingestion check-support dependencies from returning.

Decisions made:
- `g3ts-astro-check-support` now stays neutral: ESLint raw config reading and path helpers only.
- Setup owns Astro setup and Syncpack policy DTOs.
- Content owns Astro content pipeline DTOs and live content config checks.
- MDX owns MDX policy DTOs.
- SEO owns rendered SEO/helper policy DTOs and its Astro config DTOs.
- State no longer reads shared Astro package DTOs; it only uses package parsing for app-root discovery.
- Public DTO field waivers stay in type crates because those fields are the explicit package contract between ingestion and checks.

Key files for context:
- `packages/ts/astro/setup/g3ts-astro-setup-types`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion`
- `packages/ts/astro/content/g3ts-astro-content-types`
- `packages/ts/astro/content/g3ts-astro-content-ingestion`
- `packages/ts/astro/mdx/g3ts-astro-mdx-types`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion`
- `packages/ts/astro/seo/g3ts-astro-seo-types`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion`
- `packages/ts/astro/state/g3ts-astro-state-ingestion`
- `packages/ts/astro/g3ts-astro-check-support`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`

Next steps:
- If future TS families are split, start with family-local types and ingestion packages instead of creating aggregate/shared semantic DTO packages.
- Add local ingestion unit tests for setup/content/MDX/SEO/state once fixture helpers are extracted.
