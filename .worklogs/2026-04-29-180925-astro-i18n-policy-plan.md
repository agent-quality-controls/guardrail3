## Summary

Added the implementation plan for Astro i18n policy guardrails. The plan keeps framework-specific i18n checks under Astro, delegates generic lintable behavior to existing ESLint plugins and core rules, and limits new custom code to Astro-specific source policy gaps.

## Decisions made

- Planned `g3ts-eslint-plugin-astro-i18n-policy` for custom Astro source rules instead of adding i18n validation logic to the G3TS CLI.
- Delegated public-copy detection to `eslint-plugin-i18next`.
- Delegated raw date and number formatting bans to core ESLint `no-restricted-syntax`.
- Kept language detection and rendered-output i18n audits out of this first implementation plan.

## Key files for context

- `.plans/2026-04-29-175900-astro-i18n-policy-guardrails.md`

## Next steps

- Implement the planned ESLint plugin and Astro i18n family packages when i18n work resumes.
