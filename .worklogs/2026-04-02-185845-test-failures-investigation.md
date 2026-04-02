# Test failures from restructuring — investigation needed

**Date:** 2026-04-02 18:58

## Status
- Test compilation: FIXED (all test binaries compile)
- CLI test: 1 failure (pre-existing)
- AST test: 1 failure (fixed — test was stale)
- Code tests: 2 failures (fixed — scoped_files/validation_scope bugs)
- Hexarch tests: 171 FAILURES — critical regression

## Hexarch investigation
The hexarch family gets empty results in all tests. The family_route_for_tests
builds a route from FamilyView data via ProjectTree reconstruction, but the
route produces no roots (empty). This means either:
1. The legality pipeline filters out the test fixture roots
2. The FamilyView doesn't contain the fixture data correctly
3. The ProjectTree reconstruction from FamilyView is incomplete

The test_support walk() function builds FamilyView with scope_roots=[""],
which should include everything. The issue is likely in the pipeline
(structure → legality → mapper) when processing the reconstructed ProjectTree.

## Next step
Debug the hexarch family_route_for_tests to find where roots get lost.
