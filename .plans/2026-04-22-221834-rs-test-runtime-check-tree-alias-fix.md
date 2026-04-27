## Goal

Fix `g3rs-test/runtime-assertions-split` so assertions modules cannot hide runtime `check_test_tree()` calls behind local aliases such as `let run = rt::check_test_tree; run()`.

## Approach

1. Add red tests for assertions modules that route `check_test_tree()` through:
   - an imported runtime alias
   - a directly imported bare `check_test_tree`
2. Extend the helper in `rs_test_03_runtime_assertions_split/helpers.rs` to inspect per-function `local_call_aliases` in addition to direct `file_call_paths`.
3. Run the `g3rs-test-file-tree-checks` package tests and `g3rs validate` for the touched package.

## Key decisions

- Fix the helper instead of ingestion.
  - Why: the parser already records `local_call_aliases`; the rule helper is simply not consuming that fact.
- Keep the detection shared.
  - Why: `assertions_call_runtime_check_test_tree(...)` is the single rule surface for this specific orchestration prohibition.

## Files to modify

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/assertions_violations.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
- `.worklogs/2026-04-22-*.md`
