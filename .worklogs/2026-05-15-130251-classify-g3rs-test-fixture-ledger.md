# Summary

Classified the Rust test-to-fixture ledger with a deterministic classifier. The ledger now separates fixture-backed hits, fixture-backed non-hits, compile/API contracts, and remaining fixture coverage gaps.

# Decisions Made

- Added `scripts/behavior/classify-test-fixture-ledger.py` instead of hand-editing 1735 TOML rows.
- Classified only rows supported by rule IDs, test names, and approved fixture replay output.
- Left 46 rows as `unclassified` with explicit reasons because current fixtures do not prove those scenario-specific rule branches.
- Tightened `verify-test-fixture-ledger.py` so kept compile/replay rows require a reason.
- Wired `classify-test-fixture-ledger.py --check` into `scripts/behavior/verify-all.sh` so generated ledger state cannot drift silently.

# Key Files

- `scripts/behavior/classify-test-fixture-ledger.py`
- `scripts/behavior/verify-test-fixture-ledger.py`
- `scripts/behavior/verify-all.sh`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
- `.plans/2026-05-15-125105-classify-g3rs-test-fixture-ledger.md`

# Verification

- `python3 -m py_compile scripts/behavior/classify-test-fixture-ledger.py scripts/behavior/verify-test-fixture-ledger.py scripts/behavior/list-rust-tests.py`
- `python3 scripts/behavior/classify-test-fixture-ledger.py --check`
- `python3 scripts/behavior/verify-test-fixture-ledger.py`
- `python3 scripts/behavior/verify-test-fixture-ledger.py --strict`
- `scripts/behavior/verify-all.sh`
- `git diff --check`

# Next Steps

- Build or extend fixtures for the 46 remaining unclassified scenario-specific rows.
- Turn on strict ledger verification only after the unclassified count reaches zero.
