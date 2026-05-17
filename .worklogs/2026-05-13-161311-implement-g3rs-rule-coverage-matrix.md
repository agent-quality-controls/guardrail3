# Implement G3RS Rule Coverage Matrix

## Summary

Implemented Stage 1 from `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`.

The change adds the plan manifest, a 276-row G3RS rule coverage matrix, a verifier that checks source IDs against replay baselines, and `verify-all.sh` wiring.

## Decisions Made

- Generated matrix rows from active source and current replay baselines instead of old test files.
- Treated `Error` and `Warn` baseline rows as covered.
- Treated info-only rows as planned decisions, not accidental coverage.
- Treated absent hook-contract rows as `planned_cli_surface`.
- Left future fixture names out of the `fixture` field until those fixtures exist in manifests; future fixture ownership is recorded in `reason`.
- Kept Stage 1 fixture-neutral: no replay fixtures or baselines were changed.

## Negative Controls

- Missing matrix: temp plan manifest pointed at `behavior/coverage/missing.toml`; `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py --manifest "$tmp_manifest"` returned exit 1.
- Missing source row: temp matrix deleted one `[[rule]]`; temp plan manifest pointed at that matrix; verifier returned exit 1 with `source rule ID missing from coverage matrix`.
- False current replay state: temp matrix changed one covered row from `error_or_warn` to `absent`; verifier returned exit 1 with current-state mismatch.

## Verification

- `python3 -m py_compile scripts/behavior/*.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `git diff --check`

## Key Files For Context

- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md.manifest.toml`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `scripts/behavior/verify-all.sh`

## Next Steps

- Implement Stage 2: hook-contract CLI-visible inventory and the missing hook replay fixtures.
- Update the coverage matrix after Stage 2 so hook planned rows become covered rows where applicable.
