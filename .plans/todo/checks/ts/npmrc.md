# TS-NPMRC — Package-manager policy checker

**Input:** root `.npmrc`
**Parser:** line-based structured key/value parse
**Current code:** `app/ts/validate/npmrc_check.rs`
**Owned root:** package-manager root

## Owns

- root `.npmrc` existence
- duplicate-key detection
- required baseline settings
- weaker-than-baseline values
- extra settings inventory

## Does not own

- `package.json` policy
- `pnpm.overrides`
- dependency allow/ban policy

## Contract direction

This is the TS equivalent of a package-manager policy root family.
It should stay separate from `ts/package` because:
- `.npmrc` governs pnpm behavior
- `package.json` governs package metadata and dependency declarations
