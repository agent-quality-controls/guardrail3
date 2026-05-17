Summary:
Completed Cargo family-rule fixture coverage so every active Cargo rule ID is broken by at least one Cargo fixture, while keeping exactly one clean Cargo golden fixture.

Decisions made:
- Added one additional broken fixture instead of three because missing root Rust lints, a missing declared member, and a malformed declared member Cargo.toml do not hide each other in the Cargo-only CLI path.
- Strengthened `verify-g3rs-family-rule-fixtures.py` so non-clean family-rule fixtures must make every `target_rules` entry emit `Error` or `Warn`, not merely appear as `Info` inventory.
- Kept `cargo-R00-clean-golden` as the only clean Cargo fixture.

Key files for context:
- `behavior/fixtures/g3rs-rule/cargo/cargo-R30-structure-and-input-violations/fixture.toml`
- `scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `behavior/golden/g3rs-validate/approved.normalized.json`

Verification:
- `fixture3 check --suite g3rs-validate`
- `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-test-deletion.py`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate repo --path "$PWD"`
- `git diff --check`

Next steps:
- Apply the same family-rule fixture pattern to the next Rust family: one clean golden fixture, then the minimal broken fixtures needed to make every active rule in that family emit `Error` or `Warn`.
