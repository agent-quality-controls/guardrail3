Goal
- Fix `RS-TEST-FILETREE-18` so `test_support` no longer misses local helper wrappers reached through `use self::fixture_path as run;`.

Approach
- Add a red regression in `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs` that proves a canned fixture helper wrapped through a `use` alias is missed today.
- Add a parallel regression for the semantic-helper path so the same resolution gap is covered on both branches of the rule.
- Fix the rule in `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs` by resolving local imports at the helper-detection boundary, reusing the existing import facts instead of widening parser output.
- Keep the change minimal: do not refactor unrelated checks, and do not broaden the rule surface beyond local helper resolution.

Key decisions
- Use the rule boundary rather than parser changes because the needed import facts already exist on `TestSupportFileInput`.
- Prefer a small import-resolution helper over duplicating alias logic in each call site.
- Cover both canned and semantic helper paths because they share the same resolution shape and the bug is likely not limited to one branch.

Files to modify
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
