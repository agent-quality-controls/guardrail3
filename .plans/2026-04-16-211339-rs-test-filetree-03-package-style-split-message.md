Goal
- Make RS-TEST-FILETREE-03 explain the package-vs-nested-assertions mistake directly.

Approach
- Add targeted tests in the RS-TEST-FILETREE-03 rule tests for the approved missing-assertions message.
- Check whether rule input already exposes enough structure to detect a nested `component/assertions/Cargo.toml` package separately.
- Update the rule message text without changing unrelated behavior.
- Run the targeted test workspace and confirm the new message appears.

Key decisions
- Keep this as a test-family bug fix, not an apparch change.
- Do not guess nested-package structure if the rule input does not expose it.
- Use the exact user-approved wording, with `package` not `workspace`.

Files to modify
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs
- packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/tests/mod.rs
