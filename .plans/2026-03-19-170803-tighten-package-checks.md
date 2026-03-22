# Tighten package.json guardrails

**Date:** 2026-03-19 17:08
**Task:** Bump severities and add new checks to package_check.rs

## Goal
Strengthen package.json validation by bumping 4 checks from Warn to Error and adding 9 new devDependency checks plus 1 private field check.

## Approach

### Severity bumps (Warn -> Error)
1. T18 (packageManager missing) - line 167
2. T55 (preinstall pnpm enforcement) - line 203
3. T57 (engines field missing) - line 268
4. T-PLUG-11 (knip script missing) - line 385

### New checks in check_lint_plugins
Add T-PLUG-12 through T-PLUG-19 using existing check_pkg closure pattern:
- T-PLUG-12: eslint
- T-PLUG-13: typescript
- T-PLUG-14: typescript-eslint
- T-PLUG-15: eslint-plugin-import-x
- T-PLUG-16: eslint-import-resolver-typescript
- T-PLUG-17: eslint-plugin-boundaries
- T-PLUG-18: only-allow
- T-PLUG-19: jscpd

### New check in check_package_json
Add T-PKG-01: private field must be true. Parse JSON, check json.get("private") == Some(true).

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/package_check.rs` - all changes
