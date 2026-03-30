# TS-PACKAGE

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/package_check.rs`
- package/tool dependency ownership in `apps/guardrail3/crates/app/ts/validate/package_deps.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/package.md` as the detailed family ledger until the cutover is complete

Current state:

- package policy exists, but shares some tool/package surfaces with sibling families
- the current runtime already emits a meaningful package-policy rule surface, but only part of the old ledger is implemented today

Rule inventory:

- `T-PKG-01` — `package.json` must set `"private": true` where the root kind requires it. This rule exists to prevent accidental publication of roots that should stay internal.
- `T15` — `pnpm.overrides` must exist and include the baseline override pins. This rule exists to keep transitive dependency versions controlled and consistent across the workspace.
- `T16` — extra `pnpm.overrides` entries are inventoried. This rule exists to keep non-baseline transitive pinning visible without banning all local additions.
- `T17` — banned dependencies in `dependencies` or `devDependencies` are rejected. This rule exists to push the codebase toward approved alternatives instead of legacy or disallowed packages.
- `T18` — `packageManager` must be declared. This rule exists to pin the workspace package-manager version and avoid drift across developer and CI environments.
- `T55` — `preinstall` must enforce pnpm-only installs. This rule exists to prevent accidental `npm` or `yarn` usage from creating conflicting lockfiles.
- `T56` — `prepare` script presence is tracked. This rule exists to keep install-time setup such as git-hook bootstrapping visible, but it is currently only a warning-level policy.
- `T57` — `engines` must exist. This rule exists to declare the supported runtime floor explicitly.
- `T-PKG-04` — `engines.pnpm` must exist when `engines` is present. This rule exists to pin the supported pnpm version explicitly.
- `T58` — `pnpm.onlyBuiltDependencies` is inventoried. This rule exists to make install-script allowlisting visible as a supply-chain hardening measure.
- `T-PKG-02` — `lint` script must exist. This rule exists to ensure the root/package exposes a standard lint entrypoint.
- `T-PKG-03` — `typecheck` script must exist. This rule exists to ensure the root/package exposes a standard typecheck entrypoint.

Current implementation mapping:

- `apps/guardrail3/crates/app/ts/validate/package_check.rs`
  - `push_private_field_result(...)` implements `T-PKG-01`
  - `push_pnpm_override_results(...)` implements `T15` and `T16`
  - `push_banned_dependency_results(...)` implements `T17`
  - `push_package_manager_results(...)` implements `T18`
  - `push_script_results(...)` implements `T55`, `T56`, `T-PKG-02`, and `T-PKG-03`
  - `push_engine_results(...)` implements `T57` and `T-PKG-04`
  - `push_only_built_dependencies_result(...)` implements `T58`
  - `check_package_json(...)` keeps root-level versus per-package handling explicit, with per-app manifests currently only getting `T17`
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - still contains tool-package checks that the old ledger assigns to sibling families rather than `ts/package`

Implementation status:

- `T-PKG-01`: implemented
- `T15`: implemented
- `T16`: implemented
- `T17`: implemented
- `T18`: implemented
- `T55`: implemented
- `T56`: implemented
- `T57`: implemented
- `T-PKG-04`: implemented
- `T58`: implemented
- `T-PKG-02`: implemented
- `T-PKG-03`: implemented
- tool-specific package/config/script checks listed in the old ledger: intentionally not owned here

Known reconciliation notes:

- the old ledger is directionally right about ownership, but the current code still mixes several sibling-family package/tool checks in `package_deps.rs`
- current runtime behavior is stricter at the package-manager root than at per-app/package roots; per-app manifests currently only get banned-dependency enforcement
- `T56` is implemented in code even though the older placeholder did not spell it out yet
- the current runtime is a mix of real package-policy rules and older IDs that still need family-boundary cleanup

Historical/supplemental references:

- `.plans/todo/checks/ts/package.md`

Rule inventory:

- `T-PKG-01` — root `package.json` is private.
  What it should do: require `"private": true` on the package-manager root manifest.
  What it is for: prevent accidental publication of the workspace root.
- `T15` — baseline `pnpm.overrides` entries exist.
  What it should do: require the baseline override pins currently expected by policy.
  What it is for: keep transitive dependency versions pinned and avoid silent version drift.
- `T16` — extra `pnpm.overrides` entries are inventoried.
  What it should do: inventory non-baseline override entries.
  What it is for: make custom transitive pins visible without automatically forbidding them.
- `T17` — banned dependencies are forbidden in manifests.
  What it should do: reject banned package names and banned prefixes in `dependencies` and `devDependencies`, both at the root and in local app/package manifests.
  What it is for: enforce approved dependency choices and prevent backsliding to deprecated or disallowed packages.
- `T18` — `packageManager` is pinned.
  What it should do: require a `packageManager` field on the root manifest.
  What it is for: pin the package-manager version and keep installs reproducible through corepack.
- `T55` — `preinstall` enforces pnpm.
  What it should do: require a `preinstall` script that contains `only-allow pnpm`.
  What it is for: stop accidental `npm` or `yarn` installs from creating conflicting lockfiles.
- `T56` — `prepare` script presence is inventoried or warned.
  What it should do: warn when `prepare` is missing and inventory success when present.
  What it is for: encourage consistent local bootstrap behavior, especially hook/setup wiring.
- `T-PKG-02` — `lint` script exists.
  What it should do: require a generic `lint` script in `package.json`.
  What it is for: ensure CI and local lint entrypoints are standardized.
- `T-PKG-03` — `typecheck` script exists.
  What it should do: require a generic `typecheck` script in `package.json`.
  What it is for: ensure CI and local type-check entrypoints are standardized.
- `T57` — `engines` exists.
  What it should do: require a root `engines` field.
  What it is for: pin the supported Node runtime contract instead of letting deployment/runtime drift silently.
- `T-PKG-04` — `engines.pnpm` exists.
  What it should do: require a pnpm version constraint inside `engines`.
  What it is for: make the pnpm runtime contract explicit alongside `packageManager`.
- `T58` — `pnpm.onlyBuiltDependencies` is inventoried.
  What it should do: inventory `pnpm.onlyBuiltDependencies` when present.
  What it is for: surface supply-chain hardening of post-install script execution.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/package_check.rs`
  - `push_private_field_result(...)` implements `T-PKG-01`.
  - `push_pnpm_override_results(...)` implements `T15` and `T16`.
  - `push_banned_dependency_results(...)` and `check_banned_deps_in_package(...)` implement `T17`.
  - `push_package_manager_results(...)` implements `T18`.
  - `push_script_results(...)` implements `T55`, `T56`, `T-PKG-02`, and `T-PKG-03`.
  - `push_engine_results(...)` implements `T57` and `T-PKG-04`.
  - `push_only_built_dependencies_result(...)` implements `T58`.
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - still owns some tool/package presence checks that the plan says belong to sibling families, not to `ts/package`.

Current doc/code reconciliation notes:

- the old ledger is directionally right on key ownership, but the live code is narrower than the plan in one important way:
  - it does not currently emit a dedicated “missing local app/package `package.json`” rule for required local roots
- several mixed tool/package checks still live in `package_deps.rs`; those should stay out of the final `ts/package` family contract
- this family needs a future explicit distinction between:
  - root workspace/package-manager manifest policy
  - local app/package manifest policy

Next planning focus:

- clarify `package.json` key ownership across package, eslint, fmt, spelling, typecov, size, css, and tests
- decide whether local TS package/app roots should get a broader subset of `ts/package` rules beyond `T17`
- decide whether local TS package/app-root manifest existence becomes a concrete package-family rule or stays part of future TS root discovery/arch ownership
