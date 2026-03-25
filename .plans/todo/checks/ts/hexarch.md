# TS-HEXARCH — TypeScript architecture checker

**Input:** discovered TS apps, `eslint.config.*`, TypeScript source imports
**Parser:** directory discovery + structured `ESLint` parse + TS source import analysis
**Current code:** `app/ts/validate/ts_arch_checks.rs`, architecture portion of `eslint_audit.rs`
**Owned root:** TS package/app roots with `type = "service"` or `type = "extension"` or matching auto-detection

## Owns

- service/extension-app discovery and type-aware architecture checks
- service hex structure presence
- import boundary enforcement between TS layers
- `ESLint` boundaries-zone config
- `ESLint` import-direction / entry-point / external-boundary policy
- service route-entry enforcement such as canonical wrapper usage for HTTP handlers
- boundary-evasion checks:
  - dynamic import boundary escapes
  - re-export/barrel laundering across zones
  - alias/path normalization before layer classification
  - files outside canonical `modules/` roots when they participate in service architecture

## Does not own

- general `ESLint` baseline unrelated to boundaries
- content-site structure, static-generation, or locale concerns
  - that belongs to `ts/content`
- library/package architecture
  - that belongs to `ts/libarch`
- test quality

## Current known problem

This family is one of the messiest current areas:
- old architecture checks and boundary-config checks are split awkwardly
- the audit notes call out several gaps in dynamic imports, alias coverage, and structure depth
- older runtime checks still rely on string matching in places where the target contract requires structured import analysis

So this family plan should be treated as architecture-first, not “current implementation is fine”.

## Contract direction

The family should own:
- structural service-app shape
- layer/import semantics
- boundary configuration that makes those semantics enforceable
- route-entry policy that is part of the service architecture contract
- false-positive control for tests and non-service files

That keeps architecture in one place instead of half in `eslint` and half in `architecture`.

`extension` is not a separate family.
It is a `ts/hexarch` variant with a different rule subset where needed.
