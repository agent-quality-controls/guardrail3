Summary
- Renamed active fixture and golden roots to use `g3rs-rule` and `g3ts-rule` instead of plural or `-fixtures` directory names.
- Renamed behavior scripts so fixture-specific scripts include `fixture` or `fixtures` in their names.
- Updated Fixture3 suite names, golden metadata, plans, worklogs, and behavior verifier references.

Decisions made
- Fixture suite folder names now describe the domain only: `g3rs-rule` and `g3ts-rule`.
- Script names keep the fixture wording because scripts are executable behavior around fixture replay, fixture coverage, or fixture reduction.
- Historical plan and worklog references were updated so repository search does not keep stale naming contracts alive.

Key files for context
- `fixture3.yaml`
- `scripts/behavior/fixture3-g3rs-fixture-replay.py`
- `scripts/behavior/fixture3-g3ts-fixture-replay.py`
- `scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `scripts/behavior/verify-g3ts-rule-fixture-coverage.py`
- `behavior/fixtures/g3rs-rule`
- `behavior/fixtures/g3ts-rule`
- `behavior/golden/g3rs-rule`
- `behavior/golden/g3ts-rule`

Verification
- `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`
- `fixture3 check --all --json`
- `git diff --check`
- `scripts/behavior/verify-all.sh`

Next steps
- Continue building G3TS family fixture sets using the same folder naming convention.
