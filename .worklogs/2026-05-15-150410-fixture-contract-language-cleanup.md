# Summary

Documented the corrected fixture contract and removed invented behavior-replay language from the kept-test disposition migration. The migration now says ingestion coverage needs serialized owned Rust output, not custom adapters, exporters, suites, record maps, or normalized fact layers.

# Decisions Made

- Kept `fixture3.yaml` `suites` vocabulary because it belongs to fixture3's external schema.
- Renamed vague disposition names to output-boundary names so the ledger says what is missing.
- Added `verify-fixture-contract-language.py` so forbidden fixture-contract terms cannot return to the active behavior migration files.
- Left the generic fixture3 command runner intact because it only executes fixtures and emits command stdout/stderr/exit JSON.

# Key Files

- `.plans/2026-05-15-145757-fixture-contract-and-replay-audit.md`
- `.plans/2026-05-15-145757-fixture-contract-and-replay-audit.md.manifest.toml`
- `.plans/2026-05-15-143306-g3rs-kept-test-disposition-audit.md`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- `scripts/behavior/classify-kept-test-dispositions.py`
- `scripts/behavior/verify-fixture-contract-language.py`
- `scripts/behavior/verify-all.sh`

# Verification

- `python3 scripts/behavior/classify-kept-test-dispositions.py --check`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-fixture-contract-language.py`
- `scripts/behavior/verify-all.sh`

# Next Steps

- When ingestion fixture coverage is built, derive `serde::Serialize` on the owned output structs first.
- Only add custom conversion code after documenting the exact type that cannot derive `Serialize`.
