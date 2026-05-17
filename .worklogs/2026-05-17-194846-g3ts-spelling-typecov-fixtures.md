Summary:
- Added manifest-backed G3TS family-rule fixtures for spelling and typecov.
- Marked spelling and typecov hook-contract rule IDs as CLI-unreachable family contracts.

Decisions made:
- Kept one clean golden fixture per family.
- Split missing package roots from policy/script violations because missing package.json stops deeper checks.
- Used separate script-not-wired fixtures to expose validate wiring failures without hiding package/config presence checks.

Key files for context:
- .plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml
- behavior/fixtures/g3ts-rule/spelling
- behavior/fixtures/g3ts-rule/typecov
- behavior/golden/g3ts-rule/approved.normalized.json

Verification:
- fixture3 check --suite g3ts-rule
- python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py
- python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py

Next steps:
- Continue with style, topology, arch, apparch, hooks, and Astro family fixtures.
