# Add T-PLUG-01..10 lint plugin checks

**Date:** 2026-03-17 17:16
**Task:** Add check_lint_plugins function to package_check.rs

## Goal
New public function `check_lint_plugins` that validates ESLint plugin packages exist in devDependencies, with core plugins always checked and content-profile plugins gated by `content_enabled` flag.

## Approach
Single edit to append the function at end of package_check.rs. Function follows existing patterns in the file (CheckResult construction, severity levels, as_inventory for passing checks).

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/package_check.rs` — append check_lint_plugins function
