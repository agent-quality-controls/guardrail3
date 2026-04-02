# Legality marks, doesn't erase — remove filter_to_roots

**Date:** 2026-04-02 19:51

## Summary
Removed filter_to_roots from legality::collect. Legality now carries ALL
structure data through with markers. It doesn't erase roots or directories.
The mapper and runner decide what each family sees.

## Impact
- Hexarch tests: 5 failures → 2 failures (3 fixed by having all roots visible)
- Full workspace: only 2 test failures remain (+ 1 pre-existing CLI)
- Fixture data is now in legality output (test fixtures visible to families)

## Remaining 2 failures
- rs_hexarch_14: dependency inventory exact message mismatch
- rs_hexarch_11: absolute root workspace app member test

## Next steps
- Fixture exclusion needs to happen at walker or FamilyView level, not legality
- The 2 remaining failures need investigation
