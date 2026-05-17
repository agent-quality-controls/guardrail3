# Summary

Reduced all broken `g3rs-rules` family fixtures with `fixture3 reduce` while preserving CLI-visible behavior.

The reducer removed 101 fixture files and 1,814 fixture lines. The committed approved output changed only because fixture file hashes changed after reduction.

# Decisions Made

- Used `fixture3 reduce` for minimization instead of a custom reducer.
- Used generated one-fixture scratch suites because the committed `g3rs-rule-fixtures` suite covers all fixtures and includes fixture hashes.
- Normalized scratch comparison by removing `fixture_hash` and replacing `fixture_id` with `fixture-root`, so reduction proves stable behavior rather than stable file identity.
- Regenerated committed `g3rs-rule-fixtures` approval only after checking the aggregate diff without fixture hashes was identical.
- Kept clean `R00-clean-golden` fixture roots unchanged.

# Verification

- `fixture3 check --suite g3rs-rule-fixtures --json`
- `fixture3 check --all --json`
- `python3 scripts/behavior/verify-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-rule-coverage.py`
- `python3 -m py_compile scripts/behavior/reduce-broken-family-rule-fixtures.py scripts/behavior/reduce-g3rs-fixture-oracle.py`

Additional check:

- `g3rs validate repo --inventory` currently exits 1 because repo hook inventory rules emit positive hook facts as `Error`; this was observed but not changed in this fixture-reduction work.

# Key Files For Context

- `.plans/2026-05-17-142903-reduce-broken-fixtures.md`
- `scripts/behavior/reduce-broken-family-rule-fixtures.py`
- `scripts/behavior/reduce-g3rs-fixture-oracle.py`
- `behavior/fixtures/g3rs-rules`
- `behavior/golden/g3rs-rule-fixtures/approved.normalized.json`
- `behavior/golden/g3rs-rule-fixtures/approved.meta.json`

# Next Steps

- Decide whether the hook inventory findings that render positive facts as `Error` are intentional. If not, fix that as a separate G3RS hooks severity bug.
