# Summary

Removed the old broad `behavior/fixtures/g3rs` fixture suite after family-specific `g3rs-rule` fixtures replaced its rule coverage.

The active fixture3 suites now cover G3RS through `g3rs-rule`, `g3rs-validate-repo`, and `g3rs-cli-output`. The old `g3rs-validate` suite is no longer in `fixture3.yaml`.

# Decisions Made

- Deleted `behavior/fixtures/g3rs` because it was marked as transitional broad composite coverage.
- Kept `behavior/golden/g3rs-validate` because migration ledger scripts still use it as historical deleted-test evidence.
- Removed the `g3rs-validate` active suite from `fixture3.yaml`.
- Removed old default `verify-fixtures.py` and `verify-compaction.py` calls from `scripts/behavior/verify-all.sh`; those checks target the deleted layered fixture suite.
- Kept `g3rs-validate-repo`, `g3rs-cli-output`, and `g3rs-rule`.

# Verification

- `fixture3 check --all --json`
- `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `python3 scripts/behavior/verify-test-fixture-ledger.py --strict`
- `scripts/behavior/verify-all.sh`

# Current Fixture Size

- `behavior/fixtures/g3rs-rule`: 1220 files, 33742 lines.
- `behavior/fixtures/g3rs-validate-repo`: 26 files, 321 lines.
- `behavior/fixtures/g3rs-cli-output`: 14 files, 90 lines.

# Key Files For Context

- `fixture3.yaml`
- `scripts/behavior/verify-all.sh`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `.plans/2026-05-14-113549-migrate-behavior-replay-to-fixture3.md.manifest.toml`
- `behavior/fixtures/g3rs-rule`
- `behavior/golden/g3rs-validate`

# Next Steps

- If we want to remove `behavior/golden/g3rs-validate`, first migrate `behavior/migration/g3rs-test-fixture-ledger.toml` rows from old layered fixture IDs to current `g3rs-rule` fixture IDs.
