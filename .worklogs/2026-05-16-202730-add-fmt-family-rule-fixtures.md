Summary:
Added the `fmt` family-rule fixture set. Fmt now has one clean golden fixture, two broken fixtures, and verifier-backed coverage for all active `g3rs-fmt/*` rules.

Decisions made:
- Kept `fmt-R00-clean-golden` output empty because the fmt family emits no success inventory rows for a fully clean setup.
- Split missing `rustfmt.toml` into `fmt-R10-filetree-violations` because missing config prevents parsed-config policy rules from running.
- Combined wrong rustfmt settings, nightly-only stable setting, edition mismatch, ignore waiver failure, nested rustfmt config, and dual root config in `fmt-R20-policy-violations` because those findings coexist without hiding each other.
- Kept `g3rs-fmt/rustfmt-extra-settings-inventory` as explicit Info-only inventory. The implementation has no Error or Warn branch for that rule.
- Strengthened `verify-g3rs-family-rule-fixtures.py` so completed families must have broken coverage for every active rule except manifest-listed inventory-only rules.

Key files for context:
- `behavior/fixtures/g3rs-rule/fmt/fmt-R00-clean-golden/fixture.toml`
- `behavior/fixtures/g3rs-rule/fmt/fmt-R10-filetree-violations/fixture.toml`
- `behavior/fixtures/g3rs-rule/fmt/fmt-R20-policy-violations/fixture.toml`
- `scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md.manifest.toml`

Verification:
- `fixture3 check --suite g3rs-validate`
- `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-test-deletion.py`
- `python3 -m py_compile scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate repo --path "$PWD"`
- `git diff --check`

Next steps:
- Implement the next family in the all-family fixture plan: `toolchain`.
