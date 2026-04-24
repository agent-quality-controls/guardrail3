## Goal

Fix the Astro config-check hole where `TS-ASTRO-CONFIG-07` treats `astro-pipeline` as effective when the plugin is enabled at `error` severity but the required rule options are missing, leaving route-scoped rules inert.

## Approach

1. Prove the bug with config-check tests:
   - one case where `astro-pipeline` rules are enabled but have no route-scope options
   - one case where the required route-scope options are present
2. Extend the Astro ESLint surface snapshot in `g3ts-astro-types` and Astro ingestion so the config-check layer receives typed facts about rule-option effectiveness, not only plugin names and error-rule names.
3. Tighten `TS-ASTRO-CONFIG-07` to require both:
   - required `astro-pipeline` rules at `error`
   - non-empty route or endpoint scope options for the route-scoped pipeline rules
4. Update the diagnostic text so it says exactly what is missing and why.
5. Run the targeted Rust tests and validate against the real landing app.
6. Run adversarial review against the plan and the changed code.

## Key decisions

- Fix this in `ts/astro`, not in the plugin.
  - Reason: the bug is that guardrails call the plugin "effective" without proving the config shape that makes the plugin effective.
- Normalize the option fact in Astro ingestion.
  - Reason: config checks should consume typed facts, not re-parse raw ESLint option JSON ad hoc.
- Keep this as part of `TS-ASTRO-CONFIG-07`, not a new rule ID.
  - Reason: the current rule already claims the pipeline plugin is "wired and effective". Missing required options means it is not effective.

## Files to modify

- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
- `.worklogs/<timestamp>-ts-astro-pipeline-options-bug.md`
