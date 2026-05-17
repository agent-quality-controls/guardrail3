Summary:
- Added G3TS family-rule fixtures for the `hooks` family.
- Marked `hooks` completed in the G3TS family fixture manifest.

Decisions:
- Used `git_init = true` fixture metadata for hook fixtures that need `core.hooksPath = .githooks`.
- Classified hook inventory-only rules in the manifest because they report durable hook facts but do not fail.
- Kept targeted source fixtures separate where one hook defect would otherwise add unrelated failures.

Key files:
- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3ts-rule/hooks`
- `behavior/golden/g3ts-rule/approved.normalized.json`

Verification:
- `fixture3 check --suite g3ts-rule`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`

Next steps:
- Continue G3TS family-rule fixture coverage with the Astro families.
