# TS-PACKAGE — package.json checker (12 rules)

**Input:** package.json (JSON parsed)
**Current code:** `package_check.rs`

## Rules

| New ID | Old ID | Field | Description | Status |
|--------|--------|-------|-------------|--------|
| TS-PACKAGE-01 | T15 | pnpm.overrides | Override section with required pins (zod, @eslint/js) | Implemented |
| TS-PACKAGE-02 | T16 | pnpm.overrides (extra) | Inventory of non-baseline overrides | Implemented |
| TS-PACKAGE-03 | T17 | dependencies/devDependencies | Banned dependency found (axios, lodash, moment, uuid, etc.) | Implemented |
| TS-PACKAGE-04 | T18 | packageManager | `packageManager` field must exist (corepack pnpm pin) | Implemented |
| TS-PACKAGE-05 | T55 | scripts.preinstall | `only-allow pnpm` enforcement | Implemented |
| TS-PACKAGE-06 | T56 | scripts.prepare | Prepare script exists (husky/git hooks) | Implemented |
| TS-PACKAGE-07 | T57 | engines | `engines` field exists (minimum Node.js version) | Implemented |
| TS-PACKAGE-08 | T58 | pnpm.onlyBuiltDependencies | Inventory of post-install script restrictions | Implemented |
| TS-PACKAGE-09 | T-PKG-01 | private | `"private": true` in root package.json | Implemented |
| TS-PACKAGE-10 | T-PKG-02 | scripts.lint | `lint` script exists for CI | Implemented |
| TS-PACKAGE-11 | T-PKG-03 | scripts.typecheck | `typecheck` script exists for CI | Implemented |
| TS-PACKAGE-12 | T-PKG-04 | engines.pnpm | `engines` includes pnpm version constraint | Implemented |
