# TS-TEST — TypeScript test quality checker (5 rules)

**Input:** Test configs + source files
**Current code:** `ts/validate/test_checks.rs`

## Rules

| New ID | Old ID | What | Description | Status |
|--------|--------|------|-------------|--------|
| TS-TEST-01 | T-TEST-01 | stryker config | Stryker mutation testing config exists | Implemented |
| TS-TEST-02 | T-TEST-02 | test files | TypeScript test files found | Implemented |
| TS-TEST-03 | T-TEST-03 | test runner | Vitest/Jest configured in devDeps | Implemented |
| TS-TEST-04 | T-TEST-04 | .skip() | `.skip()` without documented reason | Implemented |
| TS-TEST-05 | T-TEST-05 | .only() | `.only()` in committed code | Implemented |
