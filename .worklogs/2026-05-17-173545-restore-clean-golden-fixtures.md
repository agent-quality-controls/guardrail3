# Summary

Restored all `R00-clean-golden` G3RS rule fixtures from the last state where clean fixtures were intentionally unreduced.

The reducer now only selects broken fixtures. The semantic `rule-set` oracle remains available for broken fixture minimization, but clean fixtures cannot be selected through the reducer script.

# Decisions Made

- Restored clean fixtures from commit `5ab595eb5`.
- Removed `--include-clean` and `--all-cases` from `scripts/behavior/reduce-broken-family-rule-fixtures.py`.
- Re-approved `g3rs-rule-fixtures` after restoring clean fixtures because fixture hashes and clean fixture stdout returned to the unreduced state.
- Did not consolidate to one global clean fixture in this change because `scripts/behavior/verify-family-rule-fixtures.py` currently enforces exactly one clean fixture per family.

# Verification

- `fixture3 check --suite g3rs-rule-fixtures --json`
- `fixture3 check --all --json`
- `python3 scripts/behavior/verify-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-rule-coverage.py`
- `python3 -m py_compile scripts/behavior/reduce-broken-family-rule-fixtures.py scripts/behavior/reduce-g3rs-fixture-oracle.py scripts/behavior/reduce-g3rs-fixture-rule-set-oracle.py`

# Reduction Measurement

Compared with commit `867e8902e`:

- Broken fixtures: 375 files to 243 files, 8077 lines to 5201 lines.
- Clean fixtures: 977 files to 977 files, 28541 lines to 28541 lines.
- Full suite: 1352 files to 1220 files, 36618 lines to 33742 lines.

The full-suite reduction is low because clean fixtures are intentionally restored and dominate the corpus.

# Key Files For Context

- `.plans/2026-05-17-173013-restore-clean-golden-fixtures.md`
- `scripts/behavior/reduce-broken-family-rule-fixtures.py`
- `behavior/fixtures/g3rs-rules`
- `behavior/golden/g3rs-rule-fixtures/approved.normalized.json`
- `scripts/behavior/verify-family-rule-fixtures.py`

# Next Steps

- If one global clean fixture should replace per-family clean fixtures, change the fixture manifest and `verify-family-rule-fixtures.py` contract in a separate change.
- If broken fixture line count still needs deeper reduction, the next reducer must edit file contents. `fixture3 reduce` currently removes only directories and files.
