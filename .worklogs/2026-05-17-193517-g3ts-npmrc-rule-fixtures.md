Summary
- Added CLI-visible G3TS rule fixtures for the `npmrc` family.
- Covered root `.npmrc` existence, parseability, duplicate keys, required baseline settings, weakened values, and extra-settings inventory.
- Marked `npmrc` complete in the G3TS fixture manifest.

Decisions made
- Kept `g3ts-npmrc/extra-settings-inventory` as inventory-only because the production rule intentionally emits `Info`.
- Used four npmrc fixtures total: one clean fixture, one missing-root fixture, one parse-error fixture, and one policy-violation fixture.
- Did not change npmrc rule code; this slice only records and verifies existing external CLI behavior.

Key files for context
- `.plans/2026-05-17-193123-g3ts-npmrc-rule-fixtures.md`
- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3ts-rule/npmrc`
- `behavior/golden/g3ts-rule/approved.normalized.json`
- `scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `scripts/behavior/verify-g3ts-rule-fixture-coverage.py`

Verification
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`
- `fixture3 check --suite g3ts-rule --json`
- `fixture3 check --all --json`
- `scripts/behavior/verify-all.sh`
- `git diff --check`

Next steps
- Continue the G3TS family fixture sequence with `tsconfig`, then `eslint`, `jscpd`, `fmt`, `spelling`, `typecov`, and `style`.
