Summary:
- Added manifest-backed G3TS family-rule fixtures for style.
- Covered style policy, package/tooling, Stylelint stack, ESLint style-policy wiring, protected-disable wiring, Syncpack pinning, and ESLint disable inventory.

Decisions made:
- Added tiny local Node module stubs inside the clean fixture so the existing ESLint and Stylelint parser helpers can resolve config without installing real dependencies.
- Kept `[style]` in fixture config because the current parser schema maps `style` at the root level, even though current messages still say `[ts.style]`.
- Classified g3ts-style/hook-contract as CLI-unreachable for the style family validate runner.

Key files for context:
- .plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml
- behavior/fixtures/g3ts-rule/style
- behavior/golden/g3ts-rule/approved.normalized.json

Verification:
- fixture3 check --suite g3ts-rule
- python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py
- python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py

Next steps:
- Continue with topology, arch, apparch, hooks, and Astro family fixtures.
