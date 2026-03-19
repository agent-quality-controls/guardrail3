# Close remaining gaps from parsed config audit

**Date:** 2026-03-19 17:31
**Scope:** eslint_check.rs, tsconfig_check.rs, jscpd_check.rs, package_check.rs → package_deps.rs, mod.rs

## Summary
Closed all remaining gaps found by parsing actual configs with `eslint --print-config` and `tsc --showConfig`.

## Changes

### ESLint preset checks (T-ESLP-13, T-ESLP-14)
Added presence checks for `strictTypeChecked` and `stylisticTypeChecked` strings. Closes the 54-rule gap — these two presets provide 57+ rules. If removed, guardrail3 now catches it.

### tsconfig known_keys (14 new entries)
Added 9 strict-expansion flags (noImplicitAny, strictNullChecks, etc.) and 5 resolved settings (allowSyntheticDefaultImports, resolvePackageJsonExports, etc.) to known_keys. These won't be falsely flagged as T10 "extra option" anymore.

### jscpd format check (T-JSCPD-04)
Added Warn if `format` field is missing from .jscpd.json.

### package.json script checks (T-PKG-02, T-PKG-03)
Added Error checks for `lint` and `typecheck` scripts.

### File split: package_check.rs → package_deps.rs
Moved `check_lint_plugins` and `check_additional_tools` to new `package_deps.rs` to fix R38 (512 effective lines, max 500). Shared `check_dev_dep` helper eliminates duplicated closure. Both files now under 400 lines.
