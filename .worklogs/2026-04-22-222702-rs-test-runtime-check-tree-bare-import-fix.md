## Summary

Fixed the remaining `g3rs-test/runtime-assertions-split` gap where assertions modules could call runtime `check_test_tree()` through a bare import and escape detection. Added a red test for `use demo_runtime::check_test_tree; check_test_tree();` and fixed the helper import collector to record bare imports of `check_test_tree`.

## Decisions made

- Fixed the import collector instead of the call matcher.
  - Why: the direct `check_test_tree()` call path was already visible; the helper just never marked that bare import as special.
- Kept this as a separate bug-fix commit from the alias-chain change.
  - Why: it is a different root cause on the same rule surface.

## Key files for context

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
- `.plans/2026-04-22-222629-rs-test-runtime-check-tree-bare-import-fix.md`

## Next steps

- Continue through the remaining `rs/test` helper-wrapper bugs (`self::helper()` / `crate::helper()` forms) and the pending hooks fixes.
