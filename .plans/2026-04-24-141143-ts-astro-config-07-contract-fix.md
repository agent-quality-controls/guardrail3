## Goal

Prove whether `TS-ASTRO-CONFIG-07` is enforcing a good Astro pipeline contract against the real landing app, then fix the checker at the correct layer. If the enforced contract is good, keep the enforcement and fix the misleading diagnostic so it describes the real requirement.

## Approach

1. Reproduce `TS-ASTRO-CONFIG-07` on the real Astro landing app under `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing`.
2. Read the live app ESLint config and the plugin rules to determine whether the app is actually protected or whether the checker is overreaching.
3. Add a config-check test that models the landing shape: `astro-pipeline` active on Astro route files but missing on generic TS and TSX source lanes.
4. Fix `TS-ASTRO-CONFIG-07` at the smallest correct place:
   - if the contract is bad, change the rule logic
   - if the contract is good, keep the logic and change the diagnostic text to state the real source-lane requirement
5. Run the affected Rust tests and validate the real landing app again.
6. Run an adversarial review against the original Astro family plan and the implemented rule surface.

## Key decisions

- Do not change the landing app until the checker contract is proven.
- Prefer fixing the checker message over weakening the contract if the app is in fact escaping the plugin.
- Keep the fix inside the `ts/astro` config-check package unless the evidence shows the parser or ingestion model is wrong.

## Files to modify

- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
- `.worklogs/<timestamp>-ts-astro-config-07-contract-fix.md`
