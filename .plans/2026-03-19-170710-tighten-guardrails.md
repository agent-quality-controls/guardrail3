# Tighten all TS guardrails to maximum strictness

**Date:** 2026-03-19 17:07
**Task:** Implement ~50 changes across 5 validation files to tighten guardrails

## Changes per file

### 1. eslint_check.rs — 26 severity bumps
- T3 max-lines-per-function: Warn → Error
- T4 complexity: Warn → Error
- T42 no-console: Warn → Error
- T43 eqeqeq: Warn → Error
- T46 max-dependencies: Warn → Error
- T47 explicit-function-return-type: Warn → Error
- T48 strict-boolean-expressions: Warn → Error
- T60-T83 (24 rules): all Warn → Error

### 2. tsconfig_check.rs — 3 changes
- Move `isolatedModules` from warn_bools to additional_required_bools (Error)
- Add `esModuleInterop` to additional_required_bools (Error)
- Add string-value checks for `target`, `module`, `moduleResolution`

### 3. package_check.rs — 7 severity bumps + 9 new checks
Severity: T18, T55, T57 Warn→Error, T-PLUG-11 Warn→Error
New: private:true, eslint, typescript, typescript-eslint, eslint-plugin-import-x, eslint-import-resolver-typescript, eslint-plugin-boundaries, only-allow, jscpd in devDeps

### 4. tool_config_checks.rs — 3 severity bumps
T-TOOL-08, T-TOOL-09, T-TOOL-10: Warn → Error

### 5. jscpd_check.rs — 3 new checks
- minTokens missing → Warn
- Required ignore patterns → Warn
- absolute:true → Warn
