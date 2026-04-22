## Goal

Fix `RS-TEST-FILETREE-03` so assertions modules cannot hide runtime orchestration behind a bare imported `check_test_tree`.

## Approach

1. Add a red test for `use demo_runtime::check_test_tree; check_test_tree();`.
2. Fix the helper import collector so a bare imported `check_test_tree` is treated as a runtime orchestration entrypoint the same way an aliased import is.
3. Run the file-tree package tests and `g3rs validate`.

## Key decisions

- Fix the import collector, not the call matcher.
  - Why: the direct call path `check_test_tree()` is already visible; the bug is that the helper never records that bare import as special.

## Files to modify

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
- `.worklogs/2026-04-22-*.md`
