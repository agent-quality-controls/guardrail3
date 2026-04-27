Goal
- Confirm whether `g3rs-test/test-support-generic` falsely flags module-alias calls when the alias points at a different module that happens to expose the same function name as a disallowed helper elsewhere in the file.

Approach
- Add a focused regression in `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs` where:
  - one module defines a disallowed helper name like `fixture_path` or `any_rule`
  - a different module is aliased as `h`
  - the public function calls `h::fixture_path(...)` or `h::any_rule(...)`
- Run the targeted test to prove whether the current helper-resolution logic is a false positive.
- If the bug is real, fix it in `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/support.rs` by resolving module aliases against the module they actually target instead of matching on the leaf helper name alone.
- Verify with package tests and `g3rs validate --path packages/rs/test/g3rs-test-file-tree-checks`.

Key decisions
- Keep the fix in `support.rs`.
  - The parser is already preserving alias paths and call paths; the bug candidate is semantic helper resolution that loses module identity.
- Prefer a targeted alias-to-module map over broad name filtering.
  - The problem is not that helper names are wrong globally, but that module-qualified calls are flattened by name after alias lookup.

Files to modify
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/support.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
