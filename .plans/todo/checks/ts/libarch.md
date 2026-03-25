# TS-LIBARCH — TypeScript library/package architecture checker

**Input:** discovered TS library/package roots, package manifests, source/import structure
**Parser:** package discovery + JSON + structured TS import analysis
**Current code:** no cohesive family yet; today library concerns are scattered across `package_check.rs`, `package_deps.rs`, and app-type handling in `app/ts/validate/mod.rs`
**Owned root:** TS package/app roots with `type = "library"` or library auto-detection

## Owns

- library/package discovery and library-type-aware checks
- library structural contract
  - canonical public entrypoints
  - internal vs exported module layout
  - package boundary visibility
- canonical library root detection and library-root ownership
- package dependency-shape checks that are architectural, not generic package policy
  - `peerDependencies` vs `dependencies` where the library surface requires it
- import-boundary checks for internal library layering when the library contract is explicit
- escalation from flat package to more structured library shape when complexity requires it

## Does not own

- generic root `package.json` policy
  - that belongs to `ts/package`
- service-app hex architecture
  - that belongs to `ts/hexarch`
- content-site structure
  - that belongs to `ts/content`
- generic source-scan rules
  - that belongs to `ts/code`

## Contract direction

This is the missing library architecture family on the TypeScript side.

Right now the TS validator knows about `type = "library"` but does not have a real library-family contract.

This family should answer:
- what a TypeScript library/package is allowed to look like
- how its public surface is organized
- when dependency declarations are architecturally wrong for a shared package
- how a TS library root is identified distinctly from service/content apps

Without this family, TS libraries remain a policy gap.
