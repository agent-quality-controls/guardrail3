## Goal

Make the Astro family fail closed against the current bad Astro content shape:

- no Velite in Astro apps
- no route import closures that source page content from `*.data.ts`-style modules
- Astro pipeline ESLint wiring must include the new Astro-native content rules

## Approach

1. Extend `eslint-plugin-astro-pipeline`.
   - Add source-level rule to ban route import-closure modules matching configured content-data globs.
   - Add source-level rule to ban Velite imports in Astro route import closures.
   - Extend plugin options with the minimum new surface needed for those rules.
   - Update recommended config and README.

2. Add red tests before implementation.
   - Plugin tests:
     - route -> component -> `homepage-v2.data.ts` must fail
     - normal component/helper imports must still pass
     - route closure importing `.velite` or `velite` must fail
   - Astro family tests:
     - missing new plugin rules must fail `TS-ASTRO-CONFIG-07`
     - Astro app with `velite` package must fail a new config rule
     - Astro app with `velite.config.*` or `.velite/**` must fail new filetree rules

3. Extend `ts/astro`.
   - Add config rule for `velite` package absence in Astro apps.
   - Add filetree rules for:
     - no `velite.config.*`
     - no `.velite/**`
   - Update ingestion/types as needed.
   - Extend `TS-ASTRO-CONFIG-07` required rule set to include the new plugin rules.

4. Verify with:
   - plugin tests
   - Astro config/filetree tests
   - `apps/guardrail3-ts` workspace tests

5. Run an adversarial pass against:
   - the new plugin rules
   - the new Astro config/filetree rules
   - the plan itself

## Key decisions

- Keep the contract plugin-first.
  - Source-policy belongs in ESLint.
  - `g3ts` should enforce package/config/filetree presence and required rule wiring.
- Do not add a fuzzy "all public pages must have an adapter import" heuristic in this pass.
  - It is easy to make noisy.
  - The current real failure mode is `*.data.ts` plus Velite, and those are concretely enforceable.
- Ban Velite at multiple surfaces.
  - Package-only is not enough.
  - Filetree-only is not enough.
  - Source-only is not enough.

## Files to modify

- `.plans/2026-04-24-153434-astro-native-content-contract.md`
- `packages/ts/eslint-plugin-astro-pipeline/src/index.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/configs/recommended.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/options.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-content-data-modules-in-routes.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-velite-imports.ts`
- `packages/ts/eslint-plugin-astro-pipeline/tests/no-content-data-modules-in-routes.test.ts`
- `packages/ts/eslint-plugin-astro-pipeline/tests/no-velite-imports.test.ts`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/select.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_05_no_velite_config.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_06_no_velite_output.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_04_no_velite_package.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
