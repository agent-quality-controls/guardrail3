# Summary

Added a generated disposition ledger for the 745 Rust tests that still had `kept_compile_contract` status after strict fixture coverage.

# Decisions Made

- Kept `behavior/migration/g3rs-test-fixture-ledger.toml` as the fixture coverage ledger.
- Added `behavior/migration/g3rs-kept-test-disposition.toml` as the next migration ledger for the remaining kept rows.
- Classified direct rule-sidecar rows separately from ingestion, family-runner, validate-command, CLI, renderer, and public API contract rows.
- Wired the disposition verifier into `scripts/behavior/verify-all.sh` so the broad kept bucket cannot grow without a visible category count change.

# Key Files

- `.plans/2026-05-15-143306-g3rs-kept-test-disposition-audit.md`
- `.plans/2026-05-15-143306-g3rs-kept-test-disposition-audit.md.manifest.toml`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- `scripts/behavior/classify-kept-test-dispositions.py`
- `scripts/behavior/verify-kept-test-dispositions.py`
- `scripts/behavior/verify-all.sh`

# Verification

- `python3 scripts/behavior/classify-kept-test-dispositions.py --check`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `scripts/behavior/verify-all.sh`

# Next Steps

- Backfill the `needs_fixture_or_golden_backfill` rows first because those are direct rule behavior and are not true compile/API contracts.
