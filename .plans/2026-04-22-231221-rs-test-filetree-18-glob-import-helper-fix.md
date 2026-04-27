Goal
- Fix the g3rs-test/test-support-generic miss where a public helper in `test_support` can call a helper imported through `use self::helpers::*;` from a sibling module file and escape detection.

Approach
- Add red regressions for sibling-module glob-imported canned and semantic helpers in `rs_test_18_test_support_generic/rule_tests/cases.rs`.
- Extend the file-tree test-support rule input so the rule can see sibling analyzed files from the same file tree input.
- Resolve glob-imported sibling helper names from the provided analyzed files, then reuse the existing canned/semantic helper checks with minimal new logic.
- Run the touched package tests and `g3rs validate` for the file-tree test package.

Key decisions
- Pass sibling analyzed files through the test-support rule input instead of adding ad hoc path crawling inside the rule.
- Keep the fix local to the file-tree test-support surface and avoid changing unrelated test families.
- Cover both canned and semantic helper variants because they share the same import-resolution gap.

Files to modify
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
