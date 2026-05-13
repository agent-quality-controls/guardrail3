# Fix G3RS Rule Coverage Active Source Scan

## Summary

Fixed the G3RS behavior coverage scanner so it counts runtime rule source only.

The coverage matrix now has `266` active source rule IDs instead of `276`; the removed IDs only appeared in test/assertion code and could not be emitted by runtime validation.

## Decisions Made

- Excluded `target`, `.cargo-target`, `tests`, `rule_tests`, `contract_tests`, `*_tests`, and `assertions` paths from active source scanning.
- Removed ten test-only hook rows from `behavior/coverage/g3rs-rule-coverage.toml`.
- Updated the matrix plan and manifest counts to match the corrected scanner.
- Kept replay baseline parsing unchanged because baselines are still the current behavior source.

## Verification

- `python3 -m py_compile scripts/behavior/*.py`
- `python3 scripts/behavior/verify-rule-coverage.py`
- scanner negative controls:
  - fake ID under `rule_tests` is ignored
  - fake ID under runtime source fails as missing from the coverage matrix
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `git diff --check`

## Key Files For Context

- `scripts/behavior/verify-rule-coverage.py`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md.manifest.toml`
- `.plans/2026-05-13-162201-fix-g3rs-rule-coverage-active-source-scan.md`

## Next Steps

- Recompute the Stage 2 hook fixture list from the corrected 39 absent runtime IDs.
- Build only fixtures for IDs that runtime code can emit.
