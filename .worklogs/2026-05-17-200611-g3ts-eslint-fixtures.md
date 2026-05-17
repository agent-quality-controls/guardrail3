Summary:
- Added G3TS family-rule fixtures for the `eslint` family.
- Marked the `eslint` family completed in the G3TS fixture manifest.

Decisions:
- Used fixture-local fake `eslint` modules so the CLI exercises the real ESLint parser boundary without installing external packages inside fixtures.
- Kept one weak-baseline fixture for the grouped TS/TSX policy rules because those effective-config failures do not hide each other.
- Kept a separate broken-carveout fixture because JS carve-out failure requires the TS baseline to stay valid.

Key files:
- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3ts-rule/eslint`
- `behavior/golden/g3ts-rule/approved.normalized.json`

Verification:
- `fixture3 check --suite g3ts-rule`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`

Next steps:
- Continue G3TS family-rule fixture coverage with `hooks`.
