Summary:
- Added manifest-backed G3TS family-rule fixtures for tsconfig, jscpd, and fmt.
- Classified g3ts-fmt/hook-contract as CLI-unreachable because the fmt family exports it for hook aggregation, not as a validate finding.

Decisions made:
- Kept one clean golden fixture per family.
- Split tsconfig extends-chain and strict-baseline failures because a broken extends chain prevents strict-baseline evaluation.
- Used Fixture3 approval output as the behavior source instead of hand-written expected output.

Key files for context:
- .plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml
- behavior/fixtures/g3ts-rule/tsconfig
- behavior/fixtures/g3ts-rule/jscpd
- behavior/fixtures/g3ts-rule/fmt
- behavior/golden/g3ts-rule/approved.normalized.json
- scripts/behavior/verify-g3ts-family-rule-fixtures.py
- scripts/behavior/verify-g3ts-rule-fixture-coverage.py

Verification:
- fixture3 check --suite g3ts-rule
- python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py
- python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py

Next steps:
- Continue adding manifest-backed G3TS family-rule fixtures for spelling, typecov, style, topology, arch, apparch, hooks, and Astro families.
