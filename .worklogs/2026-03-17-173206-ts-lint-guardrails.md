# Implement TS lint guardrails — 28 new checks

**Date:** 2026-03-17 17:32
**Scope:** 5 new files, 6 modified files, 3 new test files

## Summary
Implemented 28 new TypeScript guardrail checks validating ESLint plugin configuration, Stylelint setup, and pre-commit hook coverage. These verify that the right lint plugins are installed and configured — guardrail3 does not implement the rules, it ensures they ARE configured.

## New Checks

### T-PLUG-01..10 (package_check.rs)
Plugin packages in devDependencies. Core: unicorn, regexp, sonarjs, knip. Content-profile: jsx-a11y, stylelint, stylelint-a11y, stylelint-tailwindcss, tailwind-ban.

### T-ESLP-01..12 (eslint_plugin_checks.rs — new file)
ESLint plugin configuration in eslint.config.mjs:
- Unicorn flat/recommended + disabled rules + extra rules
- Regexp flat/recommended + extra rules
- SonarJS 24 cherry-picked rules
- React 10 extra rules
- Built-in ESLint/TS 17 rules
- Test file relaxations
- jsx-a11y strict (content-profile)
- tailwind-ban (content-profile)

### T-STYL-01..05 (stylelint_check.rs — new file)
Stylelint config: existence, config-standard extends, config-tailwindcss extends, a11y plugin, 11 a11y rules. Content-profile only.

### H-CSS-01 (hook_script_checks.rs)
Pre-commit hook has stylelint step for .css files.

## Category Gating
- T-PLUG-01..03/10, T-ESLP-01..06/09..11: core (always run on TS projects)
- T-PLUG-04..09, T-ESLP-07/08/12, T-STYL-01..05: content profile (only when project has content-type apps)
- Content detection: global `[typescript.checks] content = true` OR any `[typescript.apps.*] type = "content"`

## Tests
- 6 adversarial T-PLUG tests (adversarial_ts_plugins.rs)
- 7 adversarial T-ESLP tests (adversarial_ts_eslint.rs)
- 5 adversarial T-STYL tests (adversarial_ts_stylelint.rs)
