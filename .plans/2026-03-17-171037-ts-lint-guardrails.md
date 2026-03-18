# Implement TS lint guardrails — all plugins, rules, hook, modules

**Date:** 2026-03-17 17:10
**Task:** Implement all checks from ts_guardrails_implementation.md

## What guardrail3 does here

guardrail3 does NOT implement ESLint rules. It VALIDATES that the right ESLint plugins are installed, configured, and have the correct rules enabled. Same pattern as R26 (verify clippy lints configured) and T1-T8 (verify ESLint config exists).

## Check Inventory (new checks)

### Plugin presence in package.json devDependencies (T-PLUG-*)
| ID | What | File |
|---|---|---|
| T-PLUG-01 | eslint-plugin-unicorn in devDeps | package_check.rs |
| T-PLUG-02 | eslint-plugin-regexp in devDeps | package_check.rs |
| T-PLUG-03 | eslint-plugin-sonarjs in devDeps | package_check.rs |
| T-PLUG-04 | eslint-plugin-jsx-a11y in devDeps | package_check.rs |
| T-PLUG-05 | stylelint in devDeps | package_check.rs |
| T-PLUG-06 | @double-great/stylelint-a11y in devDeps | package_check.rs |
| T-PLUG-07 | stylelint-config-standard in devDeps | package_check.rs |
| T-PLUG-08 | stylelint-config-tailwindcss in devDeps | package_check.rs |
| T-PLUG-09 | eslint-plugin-tailwind-ban in devDeps | package_check.rs |
| T-PLUG-10 | knip in devDeps | package_check.rs |

### Plugin configuration in eslint.config.mjs (T-ESLP-*)
| ID | What | File |
|---|---|---|
| T-ESLP-01 | unicorn flat/recommended imported and spread | eslint_plugin_checks.rs (new) |
| T-ESLP-02 | unicorn disabled rules present (no-null, prevent-abbreviations, etc.) | eslint_plugin_checks.rs |
| T-ESLP-03 | unicorn extra enabled rules present (no-keyword-prefix, etc.) | eslint_plugin_checks.rs |
| T-ESLP-04 | regexp flat/recommended imported and spread | eslint_plugin_checks.rs |
| T-ESLP-05 | regexp extra rules present (require-unicode-regexp, etc.) | eslint_plugin_checks.rs |
| T-ESLP-06 | sonarjs cherry-picked rules present (all 24) | eslint_plugin_checks.rs |
| T-ESLP-07 | jsx-a11y strict config imported | eslint_plugin_checks.rs |
| T-ESLP-08 | jsx-a11y control-has-associated-label enabled | eslint_plugin_checks.rs |
| T-ESLP-09 | extra React rules present (10 rules) | eslint_plugin_checks.rs |
| T-ESLP-10 | built-in ESLint/TS rules present (17 rules) | eslint_plugin_checks.rs |
| T-ESLP-11 | test file relaxations present | eslint_plugin_checks.rs |
| T-ESLP-12 | tailwind-ban configured with denyList | eslint_plugin_checks.rs |

### Stylelint config (T-STYL-*)
| ID | What | File |
|---|---|---|
| T-STYL-01 | .stylelintrc.mjs exists | stylelint_check.rs (new) |
| T-STYL-02 | extends includes stylelint-config-standard | stylelint_check.rs |
| T-STYL-03 | extends includes stylelint-config-tailwindcss | stylelint_check.rs |
| T-STYL-04 | @double-great/stylelint-a11y in plugins | stylelint_check.rs |
| T-STYL-05 | a11y rules enabled (11 rules) | stylelint_check.rs |

### Hook (H-CSS-*)
| ID | What | File |
|---|---|---|
| H-CSS-01 | Pre-commit hook has stylelint step for .css files | hook_checks.rs or hook_script_checks.rs |

### Category gating
- T-PLUG-01/02/03/10, T-ESLP-01..06/09/10/11 → **core** (always run)
- T-PLUG-04/05/06/07/08/09, T-ESLP-07/08/12, T-STYL-01..05 → **content profile** (only on content-type apps)
- T-ESLP-09 (extra React rules) → **core** (React rules apply to all apps with .tsx)
- H-CSS-01 → hooks (always, but only flags if project has .css files)

## Files to create/modify

### New files
- `apps/guardrail3/src/app/ts/validate/eslint_plugin_checks.rs` — T-ESLP-01..12
- `apps/guardrail3/src/app/ts/validate/stylelint_check.rs` — T-STYL-01..05

### Modified files
- `apps/guardrail3/src/app/ts/validate/package_check.rs` — T-PLUG-01..10
- `apps/guardrail3/src/app/ts/validate/mod.rs` — wire new modules, content-profile gating
- `apps/guardrail3/src/app/ts/validate/config_files.rs` — call new checks
- `apps/guardrail3/src/app/hooks/hook_script_checks.rs` — H-CSS-01
- `apps/guardrail3/src/domain/modules/pre_commit.rs` — add stylelint hook section
- `apps/guardrail3/src/help_gen.rs` — document new checks
- `apps/guardrail3/src/domain/report.rs` — add `content` field to TsCheckCategories

### Test files
- `apps/guardrail3/tests/adversarial_ts_plugins.rs` — adversarial tests for plugin checks
