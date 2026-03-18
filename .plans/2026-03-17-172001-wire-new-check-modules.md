# Wire new check modules into TS validate orchestrator

**Date:** 2026-03-17 17:20
**Task:** Add eslint_plugin_checks, stylelint_check, and check_lint_plugins to the TS validate mod.rs orchestrator

## Goal
The new check modules (eslint_plugin_checks, stylelint_check, and the new check_lint_plugins in package_check) need to be called from the TS validate orchestrator (mod.rs). Content-profile checks are gated on a helper function that checks if any app is content-type.

## Approach

### Step-by-step plan
1. Add `mod eslint_plugin_checks;` and `mod stylelint_check;` declarations
2. After config_files::check call, add core ESLint plugin checks (read eslint.config.mjs, call check_core_plugins)
3. Add plugin package checks (check_lint_plugins with content_enabled flag)
4. Add content-profile gated section (check_content_plugins + check_stylelint)
5. Add `has_content_app` helper function

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/mod.rs` — all changes here
