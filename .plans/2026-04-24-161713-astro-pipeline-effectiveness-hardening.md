## Goal

Close the remaining inert-rule gap in the Astro pipeline contract and, if the source graph supports it cleanly, add the next concrete Astro-native source rule for raw content imports.

Desired end state:

- `TS-ASTRO-CONFIG-07` does not report green if the authored/spec content rules are enabled without non-empty authored/spec globs.
- The ESLint plugin and Astro ingestion tests prove that gap directly.
- If architecturally clean in the existing import-closure model, route closures also fail on direct raw content imports instead of only fs/glob reads.

## Approach

1. Prove the current bug with red tests.
   - `ts/astro` config-check test where pipeline rules are enabled but `authoredContentGlobs` and `specContentGlobs` are absent.
   - `ts/astro` ingestion test proving those rules are currently counted as effective without content-source scope.

2. Read the current plugin/import-closure surface before choosing the second rule.
   - Check whether direct raw content imports can be detected without bolting on special cases.
   - If the answer is no, stop after the effectiveness fix.
   - If the answer is yes, add that rule with its own red tests.

3. Implement the effectiveness fix at the orchestrator/config layer.
   - Extend typed ESLint surface facts to distinguish:
     - route-scoped rule effectiveness
     - content-data rule effectiveness
     - authored/spec content-source rule effectiveness
   - Update `TS-ASTRO-CONFIG-07` to require the new content-source effectiveness facts.

4. If direct raw content imports are cleanly representable:
   - add plugin rule
   - add README/docs
   - extend `TS-ASTRO-CONFIG-07` required rules and effectiveness checks

5. Verify.
   - plugin tests
   - `ts/astro` ingestion/config-check tests
   - `apps/guardrail3-ts` workspace tests

6. Run adversarial review against:
   - the new plan
   - the new tests
   - the final `TS-ASTRO-CONFIG-07` effectivity contract

## Key decisions

- Fix the existing effectivity bug before adding broader policy.
  - A green contract with inert rules is worse than a missing rule.
- Do not add a raw-content-import rule unless it fits the existing import-closure architecture without brittle path hacks.
  - If it needs a special parser branch for one file type, reject it for this pass.

## Files to modify

- `.plans/2026-04-24-161713-astro-pipeline-effectiveness-hardening.md`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- maybe:
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/*`
  - `packages/ts/eslint-plugin-astro-pipeline/tests/*`
  - `packages/ts/eslint-plugin-astro-pipeline/README.md`
