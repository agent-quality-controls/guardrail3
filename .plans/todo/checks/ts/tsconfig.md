# TS-TSCONFIG — TypeScript compiler config checker

**Input:** `tsconfig.json`, `tsconfig.base.json`
**Parser:** JSON
**Current code:** `app/ts/validate/tsconfig_check.rs`
**Owned root:** nearest TS package/app root with a `tsconfig` surface

## Owns

- TypeScript config existence
- parseability
- strict baseline compiler options
- extra compiler-option inventory

## Does not own

- `ESLint` rules
- package-manager policy
- source scanning

## Current rule surface

The current implementation already carries a dense rule surface here:
- strictness
- `noImplicitReturns`
- unused locals/parameters
- `noUncheckedIndexedAccess`
- `exactOptionalPropertyTypes`
- `isolatedModules`
- switch/unreachable/unused-label controls
- target/module/moduleResolution
- `noPropertyAccessFromIndexSignature`
- `noImplicitOverride`
- extra compiler option inventory

## Contract direction

This is a clean standalone family.
It should stay separate from `ts/eslint` because compiler policy and lint policy are different layers and different files.

This family must also model inheritance and local overrides correctly; checking only one root/base config is not enough in a monorepo.
