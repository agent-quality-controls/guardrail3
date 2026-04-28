## Goal

Implement narrow G3TS Astro SEO wiring enforcement without artifact parsing in core.

## Approach

- Extend `astro-config-parser` typed output with `trailingSlash`.
- Extend `guardrail3-rs-toml-parser` typed SEO policy with `strict_ai_readable`.
- Carry those fields through `g3ts-astro-seo-types` and ingestion.
- Split existing coarse sitemap and robots checks into semantic package and integration rules.
- Add SEO rules for canonical site, static output, trailing slash, checker packages, validate script ordering, strict llms package/integration, and broad crawler generator absence.
- Keep sitemap XML, robots.txt, and llms.txt parsing out of G3TS core.

## Key Decisions

- Use existing package-json script parser facts for validate command ordering.
- Treat `strict_ai_readable = false` as the default so llms generator/checker requirements only apply when explicitly enabled.
- Enforce `@agentmarkup/astro` absence because no TS waiver evaluator is currently wired for rule suppression.

## Files To Modify

- `packages/parsers/astro-config-parser`
- `packages/parsers/guardrail3-rs-toml-parser`
- `packages/ts/astro/seo/g3ts-astro-seo-types`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks`
