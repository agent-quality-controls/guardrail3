Summary:
Added the first minimized family-rule fixture set under `behavior/fixtures/g3rs-rule/cargo`. Cargo now has one clean golden fixture and two broken-layer fixtures that expose CLI-visible cargo rule behavior without relying on internal ingestion structs.

Decisions made:
- Kept exactly one cargo clean golden fixture: `cargo-R00-clean-golden`.
- Added `cargo-R10-policy-violations` for parse-valid cargo policy failures across root lint policy, approved/unapproved allow handling, member lint inheritance, member weakened overrides, member edition drift, and member-local allows.
- Added `cargo-R21-root-metadata-missing` because missing root edition, missing `rust-version`, and missing resolver are mutually compatible and cannot be represented by the clean fixture.
- Removed the attempted wrong-type metadata fixture because wrong-type `edition` and `rust-version` fail Cargo TOML parsing before cargo rules run through the CLI. Those rows remain internal unit tests.
- Added `scripts/behavior/verify-g3rs-family-rule-fixtures.py` to enforce fixture metadata, one clean golden per family, known rule IDs, approved-output presence, and completed-family ledger coverage.
- Updated disposition generation so 26 cargo rows are `covered_by_cli_output` and 16 cargo rows remain `keep_internal_unit_test`.
- Refreshed fixture3 approved metadata for all suites because changing `fixture3.yaml` changes the fixture3 manifest hash.

Key files for context:
- `behavior/fixtures/g3rs-rule/cargo/cargo-R00-clean-golden/fixture.toml`
- `behavior/fixtures/g3rs-rule/cargo/cargo-R10-policy-violations/fixture.toml`
- `behavior/fixtures/g3rs-rule/cargo/cargo-R21-root-metadata-missing/fixture.toml`
- `scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `scripts/behavior/classify-kept-test-dispositions.py`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `behavior/migration/g3rs-kept-test-disposition.toml`

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
- Build the next family-rule fixture set from the remaining `needs_rule_fixture_or_golden_output` rows.
- Do not add a second clean golden fixture for a family. Add broken-layer fixtures only when the clean fixture cannot expose the behavior.
