# Adversarial fixtures for gap fixes

**Date:** 2026-03-17 18:20
**Scope:** adversarial_ts_plugins.rs, adversarial_ts_eslint.rs, adversarial_ts_stylelint.rs

## Summary
Added 7 adversarial tests that would FAIL if the 5 gap fixes were reverted. Each test exercises a specific fix.

## Tests
- T-PLUG-11 knip script: missing vs present
- T-ESLP-12 tailwind-ban: plugin present but no denyList
- T-ESLP-10 naming-convention: rule present but no selector config
- T-ESLP-10 jsx-no-leaked-render: rule present but no validStrategies
- T-STYL-06 architecture exceptions: missing vs present
