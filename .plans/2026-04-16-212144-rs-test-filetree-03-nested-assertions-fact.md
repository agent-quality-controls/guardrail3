Goal
- Let RS-TEST-FILETREE-03 distinguish "expected assertions crate missing" from "wrong nested assertions package found".

Approach
- Extend test ingestion component facts with an optional nested assertions manifest path.
- Add a regression test in test ingestion that proves the nested `component/assertions/Cargo.toml` path is discovered.
- Add a regression test in RS-TEST-FILETREE-03 for the approved wrong-shape message.
- Update the rule to emit the wrong-shape case before the generic missing-assertions case.
- Run the affected workspaces.

Key decisions
- Detect only the concrete wrong nested manifest path under `root_rel_dir/assertions/Cargo.toml`.
- Treat this as a test-family bug fix, not an apparch rule change.
- Keep the existing generic missing-assertions message for cases where the wrong nested manifest is not present.

Files to modify
- packages/rs/test/g3rs-test-types/src/types.rs
- packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components.rs
- packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests/file_tree.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/test_helpers.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/tests/mod.rs
