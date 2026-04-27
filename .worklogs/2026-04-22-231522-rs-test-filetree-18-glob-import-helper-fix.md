Summary
- Fixed the g3rs-test/test-support-generic gap where `test_support` could miss helpers imported from sibling module files via `use self::helpers::*;`.
- Added red regressions for both canned fixture helpers and semantic result helpers, then wired sibling analyzed files into the rule input so the rule can resolve glob imports without tree crawling.

Decisions made
- Kept the fix in the file-tree test-support boundary by passing sibling analyzed files from the runner instead of adding filesystem discovery inside the rule.
- Reused the existing helper-classification heuristics for sibling files so glob-imported helper names are treated consistently with local aliases.
- Expanded the canned regression to use a non-string return type so it proves the glob-import path rather than the existing zero-arg canned helper heuristic.

Key files for context
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/support.rs`

Next steps
- None for this fix. If the rule is expanded again, check whether other glob-import shapes in `test_support` need the same sibling-file resolution path.
