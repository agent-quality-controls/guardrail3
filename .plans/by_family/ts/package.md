# TS-PACKAGE

Status: current family contract, legacy-grouped implementation, boundary-mixed relative to `RS-CARGO` and `RS-DEPS`.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/package_check.rs`
- package/tool dependency ownership in `apps/guardrail3/crates/app/ts/validate/package_deps.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/package.md` as the detailed family ledger until the cutover is complete

Current state:

- package policy exists, but shares some tool/package surfaces with sibling families
- the current runtime already emits a meaningful package-policy rule surface, but only part of the old ledger is implemented today
- compared with Rust, this family still lacks explicit fail-closed behavior and a clean root-kind applicability matrix

Rule inventory:

- `T-PKG-01` root `package.json` is private
  - Should require `"private": true` at the package-manager root.
  - It is for preventing accidental publication of the workspace root.
- `T15` `pnpm.overrides` baseline
  - Should require the baseline override entries the policy cares about.
  - It is for pinning key transitive dependencies and reducing version drift.
- `T16` extra `pnpm.overrides` inventory
  - Should inventory non-baseline override entries.
  - It is for surfacing extra transitive pins that deserve justification.
- `T17` banned dependency declarations
  - Should error on banned packages in `dependencies` or `devDependencies`.
  - It is for keeping the TS stack on approved libraries and avoiding known bad choices.
- `T18` `packageManager` field present
  - Should require a pinned `packageManager` declaration.
  - It is for locking the workspace onto a known package-manager version.
- `T55` `preinstall` enforces pnpm
  - Should require a `preinstall` script that blocks `npm`/`yarn` installs.
  - It is for preventing conflicting lockfiles and mixed package-manager usage.
- `T56` `prepare` script presence
  - Should inventory or warn on missing `prepare`.
  - It is for making hook/bootstrap setup happen automatically after install.
- `T-PKG-02` `lint` script present
  - Should require a generic `lint` script.
  - It is for standard CI/editor entrypoints.
- `T-PKG-03` `typecheck` script present
  - Should require a generic `typecheck` script.
  - It is for giving CI and developers a stable compiler-check entrypoint.
- `T57` `engines` field present
  - Should require the `engines` field.
  - It is for pinning the supported runtime version range.
- `T-PKG-04` `engines.pnpm` present
  - Should require a pnpm engine constraint when `engines` is present.
  - It is for keeping tool-version policy explicit.
- `T58` `pnpm.onlyBuiltDependencies` inventory
  - Should surface the `onlyBuiltDependencies` allowlist when configured.
  - It is for making post-install script allowlisting explicit.

Current implementation mapping:

- `package_check.rs` owns all of the rules above
- `check_package_json(...)` keeps root-level versus per-package handling explicit, with per-app manifests currently only getting `T17`
- `package_deps.rs` still contains tool-package checks that the old ledger assigns to sibling families rather than `ts/package`

Known reconciliation notes:

- the old ledger says this family owns root and local package-manifest existence, but current code silently does nothing if the root `package.json` is missing
- current runtime behavior is stricter at the package-manager root than at per-app/package roots; per-app manifests currently only get banned-dependency enforcement
- some tool/package ownership is still mixed into sibling families through `package_deps.rs`
- compared with Rust:
  - fail-closed manifest/input integrity is weaker than `RS-CARGO` and `RS-DEPS`
  - dependency-policy ownership is less explicit
  - root policy and local package policy are not cleanly separated yet
- `T17` currently behaves like a narrow manifest-level dependency rule, not a full TS dependency family; that needs to stay explicit unless a future `TS-DEPS` family is introduced

Historical/supplemental references:

- `.plans/todo/checks/ts/package.md`
- `.plans/by_family/rs/cargo.md`
- `.plans/by_family/rs/deps.md`

Next planning focus:

- add fail-closed family-owned handling for missing, unreadable, or malformed required `package.json` surfaces
- define an applicability matrix:
  - root-only rules
  - root-and-local rules
  - local-only rules
- separate root package policy from per-app/package manifest policy cleanly
- move tool-specific dependency checks out to sibling families so this family only owns generic `package.json` policy
- decide whether dependency-policy remains a narrow manifest-level concern here or becomes the seed of a future TS dependency family
