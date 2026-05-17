## Summary

Migrated active G3RS behavior replay from `goldencheck` to `fixture3`.

The approved normalized replay output is unchanged; only active config, scripts, runtime ignore path, and approved metadata moved to the new tool identity.

## Decisions

- Renamed the root replay manifest to `fixture3.yaml` because `fixture3` defaults to that filename.
- Renamed the replay harness to `scripts/behavior/fixture3-g3rs-fixture-replay.py`.
- Renamed the migration verifier to `scripts/behavior/verify-fixture3-migration.py`.
- Kept approved replay outputs under `behavior/golden/<suite>` because `fixture3` uses the same approved-output contract.
- Moved received/diff runtime output from `.goldencheck/<suite>` to `.fixture3/<suite>`.
- Updated approved metadata manifest hash and tool version because `fixture3` refuses approved metadata tied to the old manifest hash.

## Key Files

- `fixture3.yaml`
- `scripts/behavior/fixture3-g3rs-fixture-replay.py`
- `scripts/behavior/verify-fixture3-migration.py`
- `scripts/behavior/verify-all.sh`
- `.gitignore`
- `behavior/golden/g3rs-validate/approved.meta.json`
- `behavior/golden/g3rs-validate-repo/approved.meta.json`
- `.plans/2026-05-14-113549-migrate-behavior-replay-to-fixture3.md`

## Verification

- `python3 -m py_compile scripts/behavior/*.py`
- `python3 scripts/behavior/verify-fixture3-migration.py`
- `fixture3 check --all`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `git diff --check`
- `git diff -- behavior/golden/g3rs-validate/approved.normalized.json behavior/golden/g3rs-validate-repo/approved.normalized.json` produced no diff.

## Next Steps

- Continue fixture coverage closure with Stage 2 from `.plans/2026-05-14-110900-g3rs-fixture-coverage-closure.md`.
