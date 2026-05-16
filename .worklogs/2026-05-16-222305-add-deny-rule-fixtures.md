Summary:
- Added deny family rule fixtures under `behavior/fixtures/g3rs-rules/deny`.
- The deny fixture set has one clean golden fixture and six broken fixtures.
- Approved the updated `g3rs-rule-fixtures` golden output.

Decisions made:
- Split missing deny config from shadowing because missing config prevents config checks from running, while shadowing requires a valid selected config.
- Kept advisory, license, ban, and source/ignore violations as separate config fixtures because each section can fail independently without causing deny TOML parse failure.
- Marked `g3rs-deny/extra-feature-bans-inventory`, `g3rs-deny/highlight-inventory`, and `g3rs-deny/stricter-advisories-inventory` as Info-only inventory rules because the implementation has no `Error` or `Warn` branch for them.

Key files for context:
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3rs-rules/deny`
- `behavior/golden/g3rs-rule-fixtures/approved.normalized.json`

Verification:
- `fixture3 check --suite g3rs-rule-fixtures`
- `python3 scripts/behavior/verify-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-rule-coverage.py`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-test-deletion.py`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate repo --path "$PWD"`
- `git diff --check`

Next steps:
- Build the next planned family-rule fixture set.
