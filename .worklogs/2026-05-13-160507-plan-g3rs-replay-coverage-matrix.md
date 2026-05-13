# Plan G3RS Replay Coverage Matrix

## Summary

Added a plan for making G3RS behavior replay coverage explicit and complete.

The plan defines a rule coverage matrix, a verifier, the current measured coverage state, exact missing rule IDs, and staged fixture work to cover every active `g3rs-*/*` ID through public CLI replay boundaries.

## Decisions Made

- Planned coverage from source rule IDs and replay baselines, not from old test files.
- Rejected permanent `not_replay_suitable`; every active rule ID must become replayable.
- Required a CLI-visible inventory surface for hook-contract IDs because those IDs currently exist in source but do not appear through replay boundaries.
- Split planned work into matrix infrastructure first, then hook replay, deny advanced policy, source/filetree input failures, release workflow coverage, and info-only decisions.
- Kept fixture minimality as a verifier/reviewer requirement: merge where findings do not hide each other, split only on real hiding boundaries.

## Key Files For Context

- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `behavior/baselines/g3rs`
- `behavior/baselines/g3rs-validate-repo`
- `scripts/behavior/verify-all.sh`

## Next Steps

- Implement Stage 1 from the plan: create `behavior/coverage/g3rs-rule-coverage.toml` and `scripts/behavior/verify-rule-coverage.py`.
- Wire the verifier into `scripts/behavior/verify-all.sh`.
- Run adversarial review against the matrix and verifier before adding new fixtures.
