# Plan G3RS L70 Fixture Expansion

## Summary

Added a detailed implementation plan for expanding G3RS L70 behavior fixtures after read-only audits across source, architecture, package, dependency, Garde, topology, and release rule families.

## Decisions Made

- L70 must cover only valid-input, valid-tool, valid-delegated-policy project violations.
- The existing L70 fixture must first replace its constant assertion so delegated clippy does not dominate behavior replay.
- Source-only rows should stay in one dense fixture because parseable unreferenced files can coexist.
- Workspace/package, app-architecture, Garde, and release metadata rows need separate L70 fixtures because each changes activation shape.
- Input-failure rows, delegated execution rows, repo-root workflow rows, and Info-only rows are excluded from required L70 failures.

## Key Files For Context

- `.plans/2026-05-13-123846-expand-g3rs-l70-project-policy-fixtures.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs/L70-delegated-policy-valid-project-policy-violated`
- `behavior/baselines/g3rs/L70-delegated-policy-valid-project-policy-violated`
- `scripts/behavior/verify-all.sh`

## Next Steps

- Implement the L70 plan fixture by fixture.
- Regenerate and verify baselines after each fixture mutation.
- Send adversarial agents against the plan, manifest, fixtures, and baselines before reporting implementation complete.
