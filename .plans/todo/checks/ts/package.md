# TS-PACKAGE — package.json policy checker

**Input:** workspace/package-manager `package.json`, TS package/app-root `package.json`
**Parser:** JSON
**Current code:** `app/ts/validate/package_check.rs`
**Owned root:** package-manager root for workspace policy, plus TS package/app roots for local manifest policy

## Owns

- package-manager-root `package.json` existence as a policy root
- local TS package/app-root `package.json` existence where a package root is required
- `"private": true` where required by root kind
- `packageManager` field
- `engines`
- `engines.pnpm`
- `preinstall` enforcement for pnpm-only installs
- `prepare` script policy
- generic required scripts
  - `lint`
  - `typecheck`
- `pnpm.overrides` baseline
- extra override inventory
- `pnpm.onlyBuiltDependencies`
- banned dependency/package surface declared in manifests

## Does not own

- `ESLint` plugin package presence
  - that belongs to `ts/eslint`
- formatter package/config
  - that belongs to `ts/fmt`
- spelling package/config
  - that belongs to `ts/spelling`
- type-coverage package/config
  - that belongs to `ts/typecov`
- size-budget package/config
  - that belongs to `ts/size`
- `.npmrc`
  - that belongs to `ts/npmrc`
- package-level dependency declarations that are architectural rather than generic policy
  - that belongs to `ts/libarch`

## Contract direction

This family should own root package metadata and package-policy constraints.
It should not grow into a dumping ground for every package-based tool check.

When a repo has many TS package/app roots, this family must distinguish:
- workspace/package-manager policy at the top root
- local package-manifest policy at package/app roots

The family must also keep key-level ownership explicit:
- `dependencies` / `devDependencies`
  - generic bans and required baseline packages here
  - tool/plugin-specific presence belongs to the owning family
- `scripts`
  - generic repo scripts here
  - tool-specific scripts belong to the owning tool family
- `pnpm.*`
  - workspace policy here
- `engines*`
  - package/runtime manager policy here
