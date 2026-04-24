# Astro Syncpack Stack Policy

## Goal

Stop hand-rolling dependency version semantics in G3TS. Make Astro package-policy enforcement delegate to Syncpack, while G3TS only verifies that the Astro app has Syncpack installed, configured, and wired.

## Approach

1. Revert the rejected custom Astro stack-version slice.
   - Keep no custom SemVer/range parser changes from the aborted work.
   - Keep no `TS-ASTRO-CONFIG-08` custom package floor rule from that slice.

2. Add shared Syncpack config parsing.
   - Add a parser package under `packages/parsers` if no existing parser covers Syncpack config.
   - Parse only structural Syncpack config facts G3TS needs:
     - config exists and parses as JSON.
     - `source` entries as strings.
     - `versionGroups` fields used by the Astro contract: `dependencies`, `dependencyTypes`, `packages`, `specifierTypes`, `pinVersion`, `isBanned`, and `isIgnored`.
   - Do not put Astro-specific canonical contract matching in the Syncpack parser. The parser must not know which packages Astro requires or forbids.
   - Do not parse package dependency version strings in G3TS.
   - Do not emulate arbitrary Syncpack selector semantics. The accepted Astro contract is stricter than generic Syncpack:
     - selected config path is `.syncpackrc` at the Astro app root or repo root.
     - `source` must contain the exact literal entry for the app manifest relative to the selected `.syncpackrc`; aliases such as `./package.json`, absolute-style paths, `..`, and globs do not satisfy the Astro contract.
     - required pins live in the first policy prefix as one explicit group per dependency.
     - forbidden bans live in the same first policy prefix as one explicit group per dependency.
     - canonical groups use exact `dependencies` and exact `dependencyTypes`; no glob, negated selector, catch-all, package-scoped, ignored, or app-specific group satisfies the Astro family contract.

3. Add Astro family input facts.
   - Astro ingestion reads the shared Syncpack parser output.
   - Astro ingestion compares Syncpack `source` entries literally against the expected manifest path relative to the selected `.syncpackrc`.
   - Astro ingestion decides whether the app manifest is covered, which required pins are missing, and which forbidden bans are missing.
   - Astro ingestion supplies the complete required pin list and forbidden dependency list as typed facts for diagnostics, so config checks do not duplicate package policy.
   - Astro ingestion reads package scripts through the shared `package-script-command-parser`.
   - Astro ingestion maps parser-owned safe tool invocation facts into `safely_runs_astro_check` and `safely_runs_syncpack_lint`.
   - Astro checks consume typed Syncpack policy facts and safe-tool booleans.
   - Astro checks do not parse `.syncpackrc` directly.
   - Astro checks do not interpret Syncpack source, canonical-group, `isIgnored`, `pinVersion`, or `isBanned` semantics.
   - Astro checks do not split shell-like package scripts locally.
   - Astro checks do not decide fail-open shell semantics for package scripts.
   - A safe `syncpack lint` script does not satisfy the contract if another script in the same package contains an unsafe `syncpack lint` invocation.

4. Add Astro config checks.
   - `TS-ASTRO-CONFIG-08`: `package.json` includes `syncpack` and a script that invokes `syncpack lint`.
   - `TS-ASTRO-CONFIG-09`: Syncpack config pins the required Astro stack packages to exact versions.
   - `TS-ASTRO-CONFIG-10`: Syncpack config bans forbidden direct Astro landing deps.
   - Remove the old direct `velite` package scan from Astro config checks; the direct dependency ban is now owned by Syncpack.

5. Required pinned stack groups.
   - Pin exact versions rather than "at least" floors because Syncpack is strongest as a pin/mismatch validator and agent-managed apps should drift only through explicit config bumps.
   - Each required pin must be an explicit canonical version group:
     - `dependencies` is exactly one required package.
     - `dependencyTypes` is exactly `["prod", "dev"]`.
     - `packages` is omitted or empty.
     - `specifierTypes` is omitted or empty.
     - `isIgnored` is absent or false.
     - `isBanned` is absent or false.
     - `pinVersion` is the exact required version.
   - Required pins:
     - `astro`: `6.1.9`
     - `@astrojs/node`: `10.0.6`
     - `@astrojs/react`: `5.0.4`
     - `@astrojs/mdx`: `5.0.4`
     - `@astrojs/check`: `0.9.8`
     - `react`: `19.2.5`
     - `react-dom`: `19.2.5`
     - `@types/react`: `19.2.14`
     - `@types/react-dom`: `19.2.3`
     - `typescript`: `5.9.3`
     - `eslint-plugin-astro`: `1.7.0`
     - `eslint-plugin-astro-pipeline`: `0.1.2`
     - `tailwindcss`: `4.2.4`
     - `@tailwindcss/postcss`: `4.2.4`
     - `class-variance-authority`: `0.7.1`
     - `clsx`: `2.1.1`
     - `tailwind-merge`: `3.5.0`
     - `lucide-react`: `0.577.0`
     - `zod`: `4.3.6`
     - `@types/node`: `25.6.0`

6. Required banned groups.
   - Each forbidden dependency must be an explicit canonical version group:
     - `dependencies` is exactly one forbidden dependency.
     - `dependencyTypes` is exactly `["prod", "dev", "optional", "peer"]`.
     - `packages` is omitted or empty.
     - `specifierTypes` is omitted or empty.
     - `isIgnored` is absent or false.
     - `isBanned` is true.
     - `pinVersion` is absent.
   - Forbidden dependencies:
     - `next`
     - `velite`
     - `eslint-mdx`

7. Verification.
   - Add parser tests for valid/missing/malformed Syncpack config facts.
   - Add Syncpack parser tests for structural `source`, `versionGroups`, `dependencyTypes`, `packages`, `specifierTypes`, `pinVersion`, `isBanned`, and `isIgnored` parsing.
   - Add Astro ingestion tests for root/app-local exact source coverage, protected prefix order, canonical pin groups, canonical banned groups, package-scoped group rejection, `specifierTypes` rejection, wrong dependencyTypes rejection, catch-all rejection, app-local source entries resolved relative to the app-local config, source alias rejection, and source glob rejection.
   - Add package script parser tests for fake text, wrapper invocations, leading `||` fail-open chains, later `&& ... || ...` fail-open chains, and safe-plus-unsafe duplicate script surfaces.
   - Add Astro config-check tests for missing Syncpack package/script/config, missing pin, wrong pin, shadowed pin, scoped-away pin, specifier-scoped pin, noncanonical banned groups, source coverage, exact missing-config pin lists, and valid golden config.
   - Run focused parser and Astro checks.
   - Run `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`.
   - Install local `g3ts` and run Astro validation against the real landing app if the downstream app is available.

## Key Decisions

- Syncpack owns dependency policy. G3TS owns enforcement that Syncpack is present and configured for the Astro contract.
- Exact pins are preferable to minimum floors for agent-managed repos. Minimum-floor semantics require npm range reasoning; exact Syncpack pins avoid that entire class of bugs and are intentionally bumpable in one config file.
- G3TS should accept package-manager wrappers for `syncpack lint` only when `package-script-command-parser` normalizes them into a safe tool invocation. Raw script text, fake `echo syncpack lint`, pipes, semicolons, command substitutions, `||` fail-open chains, and unsafe duplicate `syncpack lint` scripts do not satisfy the contract.
- Astro checks should not know Syncpack config shape. Astro ingestion owns exact source coverage and canonical group matching from shared parser facts, then converts those answers into Astro-family facts.
- This slice intentionally enforces a strict Syncpack shape instead of accepting all valid Syncpack config forms. That keeps G3TS from reimplementing Syncpack while still making the package policy auditable and hard for agents to bypass.

## Files To Modify

- `packages/parsers/syncpack-config-parser`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/*`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/lib.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/*`
- `packages/parsers/package-script-command-parser`
- `packages/parsers/package-json-parser`
- new Astro Syncpack config check files
- `.worklogs/<timestamp>-astro-syncpack-stack-policy.md`

## Delegation Audit

- Keep ESLint source-shape checks in `eslint-plugin-astro-pipeline`.
- Keep package dependency policy in Syncpack.
- Remove direct Astro package dependency validation once Syncpack owns the same ban; G3TS should enforce the validator contract, not duplicate the validator.
- Keep package script invocation parsing and fail-open command safety in `package-script-command-parser`; it parses shell-like command facts and now preserves non-ESLint command facts for `astro check` and `syncpack lint`.
- Keep Syncpack config parsing in `syncpack-config-parser`.
- Keep canonical Astro-contract matching in Astro ingestion, because the contract is Astro-family policy rather than Syncpack syntax.
- Do not accept catch-all Syncpack groups, selector globs, negated selectors, package-scoped groups, `specifierTypes`, or non-prefix groups as satisfying Astro dependency policy. These may be valid Syncpack, but they are not the canonical Astro contract.
- Remove the dead npm SemVer/range parser from `package-json-parser`; it was unused outside parser tests and belongs to Syncpack or a dedicated semver crate if ever needed.
- Keep package.json structural facts in `package-json-parser`, but do not grow npm SemVer/range policy there.
- Revisit custom duplicate/version checks in package families after this slice; anything checking dependency policy should prefer Syncpack unless it is strictly about G3TS setup.
- Pre-existing follow-up candidates:
  - `TS-PACKAGE-CONFIG-08` hard-codes banned dependencies and scans local manifests directly; migrate that dependency policy to Syncpack contract enforcement.
  - `TS-PACKAGE-CONFIG-06` raw-checks `scripts.preinstall` for `only-allow pnpm`; migrate that script invocation detection to `package-script-command-parser`.
  - `TS-PACKAGE-CONFIG-04` hand-rolls `packageManager` pnpm version/range validation with string checks; delegate to `package-json-parser` backed by a real SemVer parser if the rule stays.
