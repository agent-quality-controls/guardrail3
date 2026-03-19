# Tighten all TS guardrails to maximum strictness

**Date:** 2026-03-19 17:15
**Scope:** eslint_check.rs, tsconfig_check.rs, package_check.rs, tool_config_checks.rs, jscpd_check.rs, adversarial_ts_plugins.rs, adversarial_ts_tools.rs

## Summary
Audited every TS guardrail check against steady-parent's actual configs. Upgraded ~50 checks: 35 severity bumps (Warn→Error), 12 new checks, 3 clippy fixes.

## Changes

### eslint_check.rs — 26 severity bumps
T3, T4, T42, T43, T46-T48, T60-T83: all Warn → Error. Every ESLint rule guardrail3 checks is now Error if missing.

### tsconfig_check.rs — 5 changes
- `isolatedModules`: moved from warn_bools to additional_required_bools (Warn→Error)
- `esModuleInterop`: added to additional_required_bools (was unchecked)
- T65 `target` must equal "ES2022" (Error)
- T66 `module` must equal "ESNext" (Error)
- T67 `moduleResolution` must equal "bundler" (Error)

### package_check.rs — 4 severity bumps + 10 new checks
Severity bumps: T18 (packageManager), T55 (preinstall), T57 (engines), T-PLUG-11 (knip script)
New: T-PKG-01 (private:true), T-PLUG-12..19 (eslint, typescript, typescript-eslint, eslint-plugin-import-x, eslint-import-resolver-typescript, eslint-plugin-boundaries, only-allow, jscpd)

### tool_config_checks.rs — 3 severity bumps
T-TOOL-08, T-TOOL-09, T-TOOL-10: Warn → Error

### jscpd_check.rs — 3 new checks
T-JSCPD-01: minTokens missing (Warn)
T-JSCPD-02: absolute:true missing (Warn)
T-JSCPD-03: required ignore patterns missing (Warn per pattern)

### Test fixes
- adversarial_ts_plugins.rs: updated T-PLUG-11 test to expect Error, added new core packages to content test fixture, removed unused helpers
- adversarial_ts_tools.rs: updated T-TOOL-08 test to expect Error
