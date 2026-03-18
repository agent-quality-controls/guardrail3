# Fix adversarial review gaps (easy fixes)

**Date:** 2026-03-17 18:07
**Task:** Fix 5 low/medium gaps found by adversarial review. Defer threshold verification.

## Fixes

1. **knip script check** — Add T-PLUG-11 checking `"knip"` in package.json scripts section
2. **Stylelint architecture exceptions** — Add T-STYL-06 checking a11y/media-prefers-color-scheme is null and no-duplicate-selectors is null
3. **tailwind-ban denyList presence** — Update T-ESLP-12 to also check for `denyList` or `deny-list` substring
4. **naming-convention selector check** — Update T-ESLP-10 to additionally verify `naming-convention` has `selector` substring nearby (weak but catches empty rule)
5. **jsx-no-leaked-render validStrategies** — Update BUILTIN_RULES check to also verify `validStrategies` substring for this specific rule

## Deferred
- Rule threshold verification (HIGH) — discuss separately
