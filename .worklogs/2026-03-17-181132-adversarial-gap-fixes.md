# Fix 5 adversarial review gaps

**Date:** 2026-03-17 18:11
**Scope:** eslint_plugin_checks.rs, package_check.rs, stylelint_check.rs

## Summary
Fixed 5 gaps found by adversarial review agents comparing implementation against the plan.

## Fixes
1. T-PLUG-11: knip script check — verifies "knip" in package.json scripts
2. T-STYL-06: stylelint architecture exceptions — verifies a11y/media-prefers-color-scheme and no-duplicate-selectors are disabled
3. T-ESLP-12: tailwind-ban denyList — verifies denyList is configured, not just plugin present
4. T-ESLP-10: naming-convention selector — warns if rule present without selector config
5. T-ESLP-10: jsx-no-leaked-render validStrategies — warns if rule present without validStrategies

## Deferred
Rule threshold verification (e.g., complexity=15 vs complexity=999) — ESLint configs are JS files, parsing numeric thresholds from JS source via string matching is fragile. Discuss separately.
