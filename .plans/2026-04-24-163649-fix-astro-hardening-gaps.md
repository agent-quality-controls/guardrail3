## Goal

Close the remaining real Astro enforcement gaps from the latest attack pass so the current Astro slice no longer reports green while allowing direct `*.data.*` route imports, nested Velite config/output surfaces under an Astro app root, or route-heavy apps with endpoint-only pipeline coverage.

## Approach

1. Add red tests for the three concrete misses.
   - `packages/ts/eslint-plugin-astro-pipeline/tests/no-content-data-modules-in-routes.test.ts`
     - prove direct route imports of configured `*.data.*` modules fail
   - `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
     - prove nested `velite.config.*` outside the route tree and nested `.velite/**` under an Astro app root are discovered
     - prove endpoint-only route-scope options do not count as effective when real route probes exist
   - `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
     - prove `TS-ASTRO-CONFIG-07` fails when route files exist but route coverage is not effective

2. Fix the plugin rule at the root.
   - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-content-data-modules-in-routes.ts`
     - remove the direct-import blind spot instead of widening globs or messaging

3. Fix Astro filetree detection at discovery time.
   - `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/select.rs`
     - detect `velite.config.*` recursively under the app root, except for route files in `src/pages/**`
     - detect `.velite/**` recursively under the app root

4. Tighten `TS-ASTRO-CONFIG-07` using typed ingestion facts, not message tweaks.
   - `packages/ts/astro/g3ts-astro-types/src/types.rs`
   - `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
   - `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
   - `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
   - Carry lane facts that distinguish:
     - route-scoped rules having some scope options
     - route-scoped rules actually covering route probes
   - Keep endpoint-only apps valid, but fail route-heavy apps that only configure endpoint globs.

5. Tighten the Velite import rule path matching if needed while touching this surface.
   - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-velite-imports.ts`
   - Avoid substring false positives such as `novelite.config.ts`.

6. Verify and then run adversarial review against the original attack claims and the changed code.

## Key decisions

- Fix route coverage in `ts/astro`, not in the ESLint plugin.
  - The bug is that guardrails claim the plugin is effective without proving the lane config actually covers route files.
- Fix direct `*.data.*` imports in the rule itself.
  - The rule already owns that policy; the import-chain length check is the bug.
- Keep the current contract scope.
  - This pass closes concrete false greens and missed detections.
  - It does not try to solve inline hardcoded literals or every dynamic import edge.

## Files to modify

- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-content-data-modules-in-routes.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-velite-imports.ts`
- `packages/ts/eslint-plugin-astro-pipeline/tests/no-content-data-modules-in-routes.test.ts`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/select.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
