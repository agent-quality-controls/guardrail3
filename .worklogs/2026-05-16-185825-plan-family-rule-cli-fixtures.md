# Plan Family Rule CLI Fixtures

## Summary

Planned the next fixture architecture step: family-scoped CLI fixtures for rule coverage. The plan keeps the external CLI as the only durable product boundary while organizing fixtures by rule family and declaring target rules in fixture metadata.

## Decisions Made

- Use `behavior/fixtures/g3rs-rule/<family>/<fixture-id>/fixture.toml`.
- Keep one `g3rs-validate` suite instead of one suite per family.
- Require `rule_family`, `target_rules`, and `expected_findings` in every family-rule fixture.
- Group rules into minimal fixtures per family when output is not hidden.
- Start implementation with `cargo` because it has the largest remaining rule count.

## Key Files

- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `fixture3.yaml`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- `scripts/behavior/verify-g3rs-family-rule-fixtures.py`

## Next Steps

- Build the cargo family rule fixture inventory.
- Add `scripts/behavior/verify-g3rs-family-rule-fixtures.py`.
- Add cargo fixtures under `behavior/fixtures/g3rs-rule/cargo`.
- Wire the new fixture glob into `fixture3.yaml`.
- Update ledger rows only after approved CLI output proves the target rules.
