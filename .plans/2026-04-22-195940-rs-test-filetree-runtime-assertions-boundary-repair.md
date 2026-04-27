Goal

Repair the remaining `rs/test` file-tree boundary defect in `g3rs-test/runtime-assertions-split`. The rule should consume ingestion-owned component file-tree facts instead of rebuilding cross-file lookup state from the top-level analyzed file bag.

Approach

- Extend the file-tree family types with prebound component facts needed by `g3rs-test/runtime-assertions-split`:
  - source module names
  - sidecar files
  - external harness files
  - assertions module files
  - top-level existing file paths for sidecar assertions-module presence checks
- Add a red rule test that proves `g3rs-test/runtime-assertions-split` still depends on `input.files` instead of the prebound component facts.
- Build those new facts in `g3rs-test-ingestion` during file-tree input construction.
- Rewrite `rs_test_03_runtime_assertions_split/violations.rs` and its helpers to consume the prebound component facts directly.
- Update fixtures, package tests, and run `g3rs validate` on the touched packages.

Key decisions

- Keep the top-level `files` bag for the other file-tree rules.
  - Reason: this repair is scoped to `g3rs-test/runtime-assertions-split`, not a full file-tree family reshape.
- Bind component-local file groups in ingestion instead of adding another generic map to the check package.
  - Reason: the defect is local rebinding from a bag. Replacing one local index with another would not improve the boundary.
- Keep the non-component harness scan in the rule.
  - Reason: that is a direct top-level file classification pass, not a recovered cross-file index.

Files to modify

- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/file_tree_analysis.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/fixtures.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/violations.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/assertions_violations.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
