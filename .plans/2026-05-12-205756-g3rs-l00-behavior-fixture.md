# G3RS L00 Behavior Fixture

## Goal

Make the lowest G3RS behavior fixture prove real CLI behavior, not only filesystem shape.

`L00-workspace-root-not-found` must fail when `g3rs validate --path repo --inventory` is run against a directory with no root `Cargo.toml`.

## Approach

- Extend the fixture manifest with command-output expectations for L00.
- Extend `scripts/behavior/verify-fixtures.py` so it can execute manifest-listed commands for fixtures that define expected command output.
- Keep command verification narrow for this step: only fixtures with explicit command assertions are executed.
- Assert exact command exit polarity and required stdout/stderr fragments.
- Keep the existing tree-shape checks unchanged.

## Files To Modify

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `scripts/behavior/verify-fixtures.py`
- `.worklogs/<timestamp>-g3rs-l00-behavior-fixture.md`

## Done

- `scripts/behavior/verify-all.sh` runs L00 command verification and exits 0.
- `git diff --check` exits 0.
