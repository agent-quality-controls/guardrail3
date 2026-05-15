# Summary

Added a function-level Rust test-to-fixture ledger so fixture migration can prove which active tests are covered by replay fixtures, which tests remain unclassified, and which expected fixture findings are missing or forbidden.

# Decisions Made

- The ledger is per `#[test] fn`, not per file, because file-level coverage can hide individual behaviors that still depend on unit tests.
- Default verification allows `unclassified` rows so the migration can proceed incrementally while still proving inventory completeness.
- Strict verification fails on `unclassified` rows and is intentionally not wired into the default verifier yet.
- The test extractor skips multi-line attributes between `#[test]` and `fn`, because existing tests use multi-line `#[expect(...)]` attributes.

# Key Files

- `scripts/behavior/list-rust-tests.py`
- `scripts/behavior/verify-test-fixture-ledger.py`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
- `scripts/behavior/verify-all.sh`
- `.plans/2026-05-15-111315-g3rs-test-to-fixture-ledger.md`

# Verification

- `python3 -m py_compile scripts/behavior/list-rust-tests.py scripts/behavior/verify-test-fixture-ledger.py scripts/behavior/verify-ledger.py scripts/behavior/verify-fixtures.py scripts/behavior/verify-rule-coverage.py`
- `python3 scripts/behavior/verify-test-fixture-ledger.py`
- `python3 scripts/behavior/verify-test-fixture-ledger.py --strict`
- `fixture3 check --all`
- `scripts/behavior/verify-all.sh`
- `git diff --check`

# Next Steps

- Classify ledger rows into `covered_hit`, `covered_non_hit`, `not_cli_visible`, `kept_compile_contract`, or `kept_replay_system`.
- Turn on strict verification only after the ledger has no `unclassified` rows.
