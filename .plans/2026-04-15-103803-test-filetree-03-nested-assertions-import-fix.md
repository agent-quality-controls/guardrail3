# Goal
Fix `RS-TEST-FILETREE-03` so nested sidecars like `x/rule_tests` may call the exact shared assertions module path the same rule already requires: `assertions/x/rule.rs`.

# Approach
1. Add a direct failing rule test for the nested shape.
   - Sidecar: `crates/runtime/src/foo/rule_tests/mod.rs`
   - Shared assertions file: `crates/assertions/src/foo/rule.rs`
   - Import from sidecar: `use demo_assertions::foo::rule::assert_runtime;`
   - Expected result: no "sibling assertions module" error.
2. Change only the path check in `RS-TEST-FILETREE-03`.
   - Compare the full expected assertions module path, not only the short owner name.
3. Run the `g3rs-test-file-tree-checks` workspace tests.

# Key Decisions
- Fix the rule, not the package. The rule already requires nested assertions files, so the import check must allow that same nested path.
- Keep the change narrow. Do not relax unrelated sidecar boundary checks.

# Files To Modify
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/tests/mod.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs`
