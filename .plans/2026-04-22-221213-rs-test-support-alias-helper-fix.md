## Goal

Fix `g3rs-test/test-support-generic` so `test_support` cannot hide canned helper or semantic helper usage behind a local call alias such as `let run = fixture_path; run()`.

## Approach

1. Add red tests in `rs_test_18_test_support_generic/rule_tests/cases.rs` for:
   - a canned fixture helper routed through a local alias
   - a semantic helper routed through a local alias
2. Update `rs_test_18_test_support_generic/rule.rs` to treat `local_call_aliases` the same way it already treats direct bare calls for local helper detection.
3. Run the `g3rs-test-file-tree-checks` package tests, the `g3rs-test-ingestion` package tests, and `g3rs validate` for the touched package.

## Key decisions

- Fix the rule, not ingestion.
  - Why: the parser already exposes `local_call_aliases`; the rule is simply not consuming an existing fact.
- Cover both canned and semantic helper paths.
  - Why: they share the same alias-blind detection pattern inside the same rule.

## Files to modify

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `.worklogs/2026-04-22-*.md`
