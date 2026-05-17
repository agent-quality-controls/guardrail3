Summary:
- Added G3TS family-rule fixtures for topology, arch, and apparch.
- Updated the G3TS fixture manifest so those structure families are marked completed.

Decisions:
- Topology nested config coverage uses `validate-repo` because the topology rule is repo-wide and is not emitted by `validate --family topology`.
- Arch uses separate fixtures for missing manifest, invalid manifest, semantic facade violations, and facade parse failure because each layer can hide the next one.
- Apparch uses one clean fixture and one broken fixture because one invalid layer graph exposes all current apparch rules without hiding another apparch rule.

Key files:
- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3ts-rule/topology`
- `behavior/fixtures/g3ts-rule/arch`
- `behavior/fixtures/g3ts-rule/apparch`
- `behavior/golden/g3ts-rule/approved.normalized.json`

Verification:
- `fixture3 check --suite g3ts-rule`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`

Next steps:
- Continue the G3TS fixture build with the remaining planned families, starting with `eslint` or `hooks`.
