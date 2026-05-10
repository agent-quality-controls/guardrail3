## Goal

Create `packages/ts/package` as the `package.json` manifest-policy family.

End state for wave 1:

- fail-closed typed `package.json` loading
- explicit root-vs-local applicability
- root manifest policy checks
- local banned-dependency checks
- no tool/package leakage from sibling families

## Local Intent

The current local family contract says `ts/package` owns:

- `private`
- `packageManager`
- `engines`
- `preinstall`
- `prepare`
- `lint`
- `typecheck`
- `pnpm.overrides`
- `onlyBuiltDependencies`
- banned dependency surface

Current local notes also say the old implementation is boundary-mixed, especially relative to:

- sibling package/tool presence checks in `package_deps.rs`
- weak fail-closed behavior
- unresolved root-vs-leaf applicability

## External 2026 Delta

Several old assumptions need tightening:

- `packageManager` is now a real Node/Corepack control point.
- `private: true` still matters as the root publication guard.
- npm still documents `engines` as advisory unless `engine-strict` is enabled.
- `preinstall only-allow pnpm` still helps, but Corepack plus pinned `packageManager` is the more durable package-manager control point.

That means the family should keep `preinstall`, but not treat it as the only enforcement mechanism.

## Approach

1. Parse `package.json` once into typed manifest models.
2. Make missing/unparseable required root surfaces fail closed.
3. Separate applicability explicitly:
   - package-manager root
   - TS app/package roots
4. Split root policy from local dependency policy.
5. Keep tool/package presence checks out of this family unless the field truly belongs to `package.json` policy.

## Wave 1 Rule Cut

Wave 1 should include:

- required root `package.json` exists
- required root `package.json` parses
- root `private: true`
- root `packageManager` exists and is pinned
- root `engines.node`
- root `engines.pnpm`
- root `preinstall`
- root `prepare`
- root `lint`
- root `typecheck`
- root `pnpm.overrides`
- root `pnpm.onlyBuiltDependencies` inventory
- local banned dependency declarations

Wave 1 should not include:

- general tool/package presence checks from `package_deps.rs`
- `peerDependencies` scans
- `optionalDependencies` scans
- minimum version policy beyond the obvious root control points
- `type: "module"` policy

## Key Decisions

- Fail closed on required root manifest loading first.
  - Reason: the current silent skip is the clearest architecture bug in the legacy checker.

- Keep tool/package presence out of `ts/package`.
  - Reason: those checks belong to sibling families such as `ts/eslint`, not generic manifest policy.

- Separate root policy from local policy explicitly.
  - Reason: only some rules should apply at app/package roots, and the old checker mixed that boundary.

## Latest Ideas, Not Final Decisions

- `packageManager` plus Corepack is the durable package-manager control point.
- `preinstall only-allow pnpm` should remain defense-in-depth, not the primary enforcement story.
- Some package policy may later split again if TS gets a separate dependency family.

## Risks / Open Questions

- `engines` is advisory in npm by default, so error severity needs to match the real enforcement model.
- We still need a precise local applicability matrix for app/package manifests versus workspace root manifests.
- `devEngines.packageManager` may become relevant later, but it should not bloat wave 1.

## Files To Modify

- `packages/ts/package/**`
- `apps/guardrail3-ts/**` for wiring only after the family exists
- top-level workspace manifests as needed

## Source Inputs

- `.plans/todo/checks/ts/package.md`
- `.plans/todo/checks/ts/README.md`
- `.plans/by_family/ts/package.md`
- `.plans/todo/legacy/audit/08-package-json.md`
- `.plans/todo/typescript/ts/tsconfig_npmrc_package_plugins_audit.md`
- `legacy/apps/guardrail3-current/crates/app/ts/validate/packages/package_check.rs`
- `legacy/apps/guardrail3-current/crates/app/ts/validate/packages/package_deps.rs`
- Node docs:
  - packages
  - Corepack
- npm docs:
  - `package.json`
- pnpm docs:
  - homepage
  - package_json
