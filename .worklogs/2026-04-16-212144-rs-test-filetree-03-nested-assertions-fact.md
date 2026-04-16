Summary
- Extended test file-tree ingestion so RS-TEST-FILETREE-03 can see a wrong nested `component/assertions/Cargo.toml` manifest separately from the expected `component/crates/assertions/Cargo.toml` path.
- Added regression tests for both ingestion and the new wrong-shape rule message, then updated the rule to report that case before the generic missing-assertions error.

Decisions made
- Added one concrete fact: `nested_assertions_cargo_rel_path: Option<String>`, because the rule needed the exact wrong path to produce a truthful message.
- Detected only `root_rel_dir/assertions/Cargo.toml`, because that is the specific wrong fix we wanted to name.
- Kept the old generic missing-assertions path for cases where no wrong nested manifest is present.

Key files for context
- packages/rs/test/g3rs-test-types/src/types.rs
- packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components.rs
- packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests/file_tree.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/tests/mod.rs
- .plans/2026-04-16-212144-rs-test-filetree-03-nested-assertions-fact.md

Next steps
- If we want this to fire in `apps/guardrail3-rs`, keep the app in the wrong nested shape long enough to validate, or continue the app restructure toward `component/crates/runtime` and `component/crates/assertions`.
- After the app package layout is settled, rerun full app validation and fix the next real contradiction.
