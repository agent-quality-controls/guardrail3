# Fixture exclusion at FamilyView level + configurable placement exclusions

**Date:** 2026-04-02 20:24

## Summary
- Placement's root discovery now uses configurable excluded_paths from
  guardrail3.toml instead of hardcoded tests/fixtures patterns
- FamilyView::build applies the same exclusions when building the scoped view
- Legality carries ALL data (marks, doesn't erase)
- Fixture exclusion happens at two points: placement (root discovery) and
  FamilyView (view construction)

## Test results
- All tests pass except 3 hexarch tests and 1 pre-existing CLI test
- The 3 hexarch failures are pre-existing issues with specific test assertions
  (dependency inventory messages, nested workspace detection, absolute path handling)
