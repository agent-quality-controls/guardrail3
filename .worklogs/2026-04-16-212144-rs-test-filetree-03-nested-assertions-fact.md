Summary
- Moved the nested-assertions detection earlier in test ingestion so single-crate packages with `component/assertions/Cargo.toml` are treated as wrong package shape instead of as valid assertions layout.
- Added regression tests for both file-tree and source ingestion, and verified that `apps/guardrail3-rs --family test` now reports the intended wrong-shape errors before any package restructure.

Decisions made
- Fixed the bug at component normalization time, because the old normalization was already accepting `component/assertions` as expected and hiding the real problem from downstream rules.
- Reused the same package-style expectation for both source and file-tree ingestion so the test family stays internally consistent.
- Kept the wrong-shape message in RS-TEST-FILETREE-03, but made ingestion feed it the right expected path: `component/crates/assertions/Cargo.toml`.

Key files for context
- packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components.rs
- packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests/file_tree.rs
- packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests/source.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/tests/mod.rs
- .plans/2026-04-16-212144-rs-test-filetree-03-nested-assertions-fact.md

Next steps
- Reshape `apps/guardrail3-rs` components that now fail with the new message into `component/crates/runtime` and `component/crates/assertions`, or remove the separate assertions crate if the component should stay single-crate.
- Then rerun full app validation and fix the remaining real test issues, starting with the RS-TEST-SOURCE-07 proof-step failures already surfaced by the new structure check.
