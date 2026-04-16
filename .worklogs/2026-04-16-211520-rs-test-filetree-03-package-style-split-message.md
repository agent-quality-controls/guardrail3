Summary
- Updated RS-TEST-FILETREE-03 so the missing-assertions error explains the real package-shape mistake instead of telling users to add a vague sibling assertions crate.
- Added a regression test that proves the new message text and fixed the component path the rule reports for package-style fixtures.

Decisions made
- Kept this as a test-family bug fix, not an apparch change, because the bad guidance starts in RS-TEST-FILETREE-03.
- Used the component package path in the message, falling back to `runtime_rel_dir` when `rel_dir` is empty, because package-style fixtures model the root component that way.
- Did not add the separate "nested assertions package is the wrong shape" case in this pass because the current rule input does not expose a direct fact for a found nested `component/assertions/Cargo.toml` package.

Key files for context
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/tests/mod.rs
- .plans/2026-04-16-211339-rs-test-filetree-03-package-style-split-message.md

Next steps
- If we want the second message, extend test-file-tree ingestion so the rule knows when a nested `component/assertions/Cargo.toml` package actually exists.
- After that, add a dedicated RS-TEST-FILETREE-03 case that tells users to move it to `component/crates/assertions` and `component/crates/runtime`.
