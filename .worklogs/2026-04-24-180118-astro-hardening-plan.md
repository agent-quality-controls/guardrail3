## Summary

Wrote and adversarially revised the Astro content pipeline hardening implementation plan. The plan defines strict-local Astro content architecture, parser-owned facts, route classes, ESLint plugin rules, G3TS enforcement rules, waiver matching, CLI verification, packed npm release gates, and exact test assertions.

## Decisions made

- Kept source semantics in `eslint-plugin-astro-pipeline` and setup/effectiveness/bypass enforcement in G3TS.
- Added route classes instead of treating every Astro route as content-backed.
- Moved waiver matching into policy checks fed by typed findings instead of config checks.
- Required parser-owned facts for ESLint config, package dependency specs, guardrail policy, ESLint directives, and package scripts.
- Required final implementation rounds to include adversarial review and exact test assertions.

## Key files for context

- `.plans/2026-04-24-173946-astro-content-pipeline-hardening.md`
- `packages/ts/eslint-plugin-astro-pipeline`
- `packages/ts/astro`
- `packages/parsers/eslint-config-parser`
- `packages/parsers/package-json-parser`
- `packages/parsers/guardrail3-rs-toml-parser`

## Next steps

- Implement Round 1 parser and policy foundations.
- Run parser tests.
- Send adversarial reviewers against the implementation and plan.
- Fix review findings before committing Round 1.
