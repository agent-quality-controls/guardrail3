# Goal

Remove the duplicate broad `behavior/fixtures/g3rs` fixture suite after family-specific rule fixtures have replaced its rule coverage.

# Current State

- `behavior/fixtures/g3rs` is the old broad layered fixture suite.
- `behavior/fixtures/g3rs-rules` is the family-specific rule fixture suite.
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml` marks `behavior/fixtures/g3rs` as `transitional_broad_composite` with action `reduce_or_remove_after_family_rule_coverage_replaces_it`.
- `scripts/behavior/verify-family-rule-fixtures.py` and `scripts/behavior/verify-rule-coverage.py` now prove family rule coverage through `g3rs-rules`.

# Approach

1. Remove the `g3rs-validate` suite from `fixture3.yaml`.
2. Remove `behavior/fixtures/g3rs`.
3. Keep `behavior/golden/g3rs-validate` for the migration ledger scripts that still audit deleted test rows against historical fixture output.
4. Update the family-rule fixture manifest so `behavior/fixtures/g3rs` is no longer listed as transitional active corpus.
5. Keep `g3rs-validate-repo`, `g3rs-cli-output`, and `g3rs-rule-fixtures`.
6. Run:
   - `fixture3 check --all --json`
   - `python3 scripts/behavior/verify-family-rule-fixtures.py`
   - `python3 scripts/behavior/verify-rule-coverage.py`
   - `rg "behavior/fixtures/g3rs"` to catch stale references that still matter.

# Key Decisions

- Do not delete `behavior/fixtures/g3rs-rules`.
- Do not delete `behavior/fixtures/g3rs-validate-repo`.
- Do not delete `behavior/fixtures/g3rs-cli-output`.
- Do not delete parser/shared fixtures in this change.
- Stale historical plan references can remain unless an active verifier reads them.
- `behavior/golden/g3rs-validate` can remain because it is a migration-audit artifact, not active fixture source code.

# Files To Modify

- `fixture3.yaml`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `behavior/fixtures/g3rs`
