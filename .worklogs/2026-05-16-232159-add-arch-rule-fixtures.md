Summary:
- Added arch family rule fixtures under `behavior/fixtures/g3rs-rule/arch`.
- The arch fixture set has one clean golden fixture and three broken fixtures.
- Approved the updated `g3rs-rule` golden output.

Decisions made:
- Kept config-graph violations in `arch-R10-package-contract-violations` because dependency count, feature contract, boundary crossing, and shared flag all emit together without hiding each other.
- Kept file-tree violations separate from source violations because missing facades can hide facade-source checks.
- Kept the broad replay coverage matrix unchanged because `verify-g3rs-rule-fixture-coverage.py` still validates the older broad suites; arch family-rule coverage is enforced by `verify-g3rs-family-rule-fixtures.py`.

Key files for context:
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3rs-rule/arch`
- `behavior/golden/g3rs-rule/approved.normalized.json`

Verification:
- `fixture3 check --suite g3rs-rule`
- `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-test-deletion.py`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate repo --path "$PWD"`
- `git diff --check`

Next steps:
- Build the next planned family-rule fixture set.
