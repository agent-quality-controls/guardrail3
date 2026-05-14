# Migrate Behavior Replay To Fixture3

## Goal

Use `fixture3` as the only active fixture approval CLI for G3RS behavior replay.

The migration must preserve current approved replay output.

## Active Scope

Migrate active runtime files only:

- `goldencheck.yaml`
- `scripts/behavior/goldencheck-g3rs-replay.py`
- `scripts/behavior/verify-goldencheck-migration.py`
- `scripts/behavior/verify-all.sh`
- `.gitignore`
- `behavior/golden/g3rs-validate/approved.meta.json`
- `behavior/golden/g3rs-validate-repo/approved.meta.json`

Do not rewrite historical worklogs or old plan prose only because they mention `goldencheck`.

## Required Changes

1. Rename `goldencheck.yaml` to `fixture3.yaml`.
2. Rename `scripts/behavior/goldencheck-g3rs-replay.py` to `scripts/behavior/fixture3-g3rs-replay.py`.
3. Rename `scripts/behavior/verify-goldencheck-migration.py` to `scripts/behavior/verify-fixture3-migration.py`.
4. Update `fixture3.yaml` command argv to call `scripts/behavior/fixture3-g3rs-replay.py`.
5. Update `fixture3.yaml` storage directories:
   - approved output stays under `behavior/golden/<suite>`
   - received output moves to `.fixture3/<suite>`
   - diff output moves to `.fixture3/<suite>`
6. Update `scripts/behavior/verify-all.sh` to run:
   - `fixture3 check --all`
   - `python3 "$HERE/verify-fixture3-migration.py"`
7. Update the migration verifier so it checks `fixture3`, `fixture3.yaml`, and `.fixture3`.
8. Remove active `.goldencheck` runtime output.
9. Ignore `.fixture3/` runtime output instead of `.goldencheck/`.
10. Keep approved normalized output byte-equivalent except for metadata that names the approving tool.

## Non-Goals

- Do not change fixture design.
- Do not change suite names.
- Do not change approved normalized replay records.
- Do not replace `fixture3` with project-specific comparison logic.
- Do not restore `behavior/baselines`.

## Verification

Required commands:

```sh
python3 -m py_compile scripts/behavior/*.py
python3 scripts/behavior/verify-fixture3-migration.py
fixture3 check --all
scripts/behavior/verify-all.sh
g3rs validate --path apps/guardrail3-rs --inventory
g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
git diff --check
```

## Done State

- `fixture3 check --all` passes.
- `scripts/behavior/verify-all.sh` passes.
- No active script or root config file invokes `goldencheck`.
- No `.goldencheck` directory remains.
