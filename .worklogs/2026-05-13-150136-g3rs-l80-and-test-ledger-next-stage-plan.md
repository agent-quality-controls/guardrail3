# G3RS L80 And Test Ledger Next Stage Plan

## Summary

Pushed the completed behavior fixture expansion commit and added the next-stage plan for G3RS behavior replay migration.

The plan moves from fixture creation to migration safety: harden the L80 clean fixture, add a G3RS test ledger, add ledger/deletion verifiers, and wire them into behavior verification before any old tests are deleted.

## Decisions Made

- Limited this next stage to G3RS because the active repo direction is Rust guardrails only.
- Kept L80 as a clean realistic fixture, not another failure fixture.
- Planned ledger infrastructure before deleting tests so deletion cannot happen without machine-readable coverage status.
- Chose `apps/guardrail3-rs` as the first ledger scope because the accepted migration plan says CLI behavior is first.
- Required conservative `unclassified` rows first instead of guessing migration status from filenames.

## Key Files For Context

- `.plans/2026-05-13-145758-g3rs-l80-and-test-ledger-next-stage.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `scripts/behavior/verify-all.sh`
- `scripts/behavior/verify-fixtures.py`
- `behavior/fixtures/g3rs/L80-project-policy-valid-clean/fixture.toml`

## Next Steps

- Implement the plan in `.plans/2026-05-13-145758-g3rs-l80-and-test-ledger-next-stage.md`.
- Run the required verification commands from the plan.
- Run adversarial reviewers against the plan and implementation before reporting completion.
