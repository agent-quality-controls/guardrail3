Summary

- Migrated G3RS behavior replay from custom per-command baseline JSON files to `goldencheck 0.1.4`.
- Preserved the current fixture output by proving old `behavior/baselines` records matched the new `behavior/golden` approved records before deleting the old baseline tree.

Decisions made

- Kept guardrail3-specific fixture semantics in `scripts/behavior/*` verifiers.
- Moved only generic approved/received/diff/approval mechanics to `goldencheck`.
- Kept a small `g3rs` replay harness because `goldencheck` should not know how to run or normalize `g3rs validate`.
- Removed old baseline scripts and old baseline directories instead of keeping compatibility paths.

Key files for context

- `goldencheck.yaml`
- `scripts/behavior/goldencheck-g3rs-replay.py`
- `scripts/behavior/replay_common.py`
- `scripts/behavior/verify-goldencheck-migration.py`
- `scripts/behavior/verify-all.sh`
- `behavior/golden/g3rs-validate/approved.normalized.json`
- `behavior/golden/g3rs-validate-repo/approved.normalized.json`
- `.plans/2026-05-13-215648-migrate-behavior-replay-to-goldencheck.md`
- `.plans/2026-05-13-215648-migrate-behavior-replay-to-goldencheck.md.manifest.toml`

Verification

- Pre-migration old baselines:
  - `python3 scripts/behavior/verify-baselines.py` -> `behavior-baselines: PASS records:46`
  - `python3 scripts/behavior/verify-baselines.py --manifest .plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml` -> `behavior-baselines: PASS records:9`
- Migration equivalence:
  - old `behavior/baselines/g3rs` matched new `behavior/golden/g3rs-validate/approved.normalized.json` for 46 records
  - old `behavior/baselines/g3rs-validate-repo` matched new `behavior/golden/g3rs-validate-repo/approved.normalized.json` for 9 records
- Current verification:
  - `python3 -m py_compile scripts/behavior/*.py`
  - `scripts/behavior/verify-all.sh`
  - `goldencheck check --all`
  - `g3rs validate --path apps/guardrail3-rs --inventory`
  - `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
  - `git diff --check`

Next steps

- Use `goldencheck check --all` as the behavior replay gate.
- Approve intentional fixture-output changes with `goldencheck approve --suite <suite> --change <path>`.
