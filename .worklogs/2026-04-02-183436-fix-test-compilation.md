# Fix test compilation after restructuring

**Date:** 2026-04-02 18:34

## Summary
Fixed ~231 test compilation errors from the module restructuring. Test binaries
all compile. Tests run with 3 failures to investigate:

1. ts_validate_help_contains_check_ids — PRE-EXISTING (fails before our changes)
2. extra_visitors::ignore_with_name_value_reason_not_flagged — needs investigation
3. code family: 2 scoping tests — needs investigation

## Changes
- FamilyMapper::new → from_legality in all test helpers
- structure::collect 1-arg → 2-arg in test helpers
- FamilyView::from_tree → FamilyView::build in test helpers
- Fixed test module paths after sidecar migration
- Added root_path() to FamilyView for test infrastructure
- Added dev-dependencies on legality/project-tree to 14 crates
