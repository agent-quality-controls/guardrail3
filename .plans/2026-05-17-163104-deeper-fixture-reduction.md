# Goal

Reduce the `g3rs-rule` fixture corpus substantially while preserving CLI-visible behavior for every fixture.

# Current Finding

The previous reduction was conservative:

- It processed only broken fixtures.
- It excluded `R00-clean-golden` fixtures.
- Three clean fixtures each still contain 9,167 text lines:
  - `code/code-R00-clean-golden`
  - `garde/garde-R00-clean-golden`
  - `release/release-R00-clean-golden`
- Those three fixtures alone account for 27,501 lines.
- The prior run used only one default `fixture3 reduce` pass with `dirs,files`.
- Default `fixture3 reduce` only removes directory and file candidates. It does not shrink lines inside a file.

# Approach

1. Extend `scripts/behavior/reduce-g3rs-broken-family-rule-fixtures.py` into a reusable family-rule fixture reducer:
   - support `--include-clean`
   - support `--all-cases`
   - keep the default as broken-only
   - keep one-fixture scratch approved output
   - keep rollback on behavior drift
2. Run exact-output reduction on:
   - all clean fixtures
   - the three previous incomplete fixtures
   - all broken fixtures again if the larger budget exposes more removable files
3. Use a larger explicit oracle budget on copied large fixtures:
   - `--max-oracle-calls 2000`
4. If a fixture remains large after exact-output reduction, switch to a semantic reducer oracle.
5. Semantic reducer oracle preserves:
   - zero vs nonzero exit class
   - sorted set of `Error` and `Warn` rule IDs emitted by the fixture
6. Semantic reducer oracle intentionally does not preserve:
   - duplicate finding counts
   - `Info` findings
   - exact diagnostic message text
   - exact numeric counts in diagnostic messages
7. Do not hand-edit fixture behavior.
8. Regenerate committed approved output only after proving every fixture still satisfies the family fixture manifest:
   - expected exit class
   - every `expected_findings` rule ID appears
   - every broken fixture's `target_rules` emit `Error` or `Warn`

# Verification

- `fixture3 check --suite g3rs-rule --json`
- `fixture3 check --all --json`
- `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `python3 -m py_compile scripts/behavior/reduce-g3rs-broken-family-rule-fixtures.py scripts/behavior/reduce-g3rs-fixture-oracle.py`

# Rollback Rule

If normalized per-fixture behavior changes after applying a reducer report, restore that fixture root from scratch backup and stop.

# Files To Modify

- `scripts/behavior/reduce-g3rs-broken-family-rule-fixtures.py`
- `behavior/fixtures/g3rs-rule/**`
- `behavior/golden/g3rs-rule/approved.normalized.json`
- `behavior/golden/g3rs-rule/approved.meta.json`
- `.worklogs/<timestamp>-deeper-fixture-reduction.md`
