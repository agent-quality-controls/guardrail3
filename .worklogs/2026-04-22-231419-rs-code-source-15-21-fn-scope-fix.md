Summary
- Fixed the fs visitor alias-scope leak across sibling function boundaries and split the parser implementation across smaller visitor modules so `parse/fs_visitors.rs` no longer trips the code-family line-count guardrail.
- Added sibling-function regressions for direct call, direct import, and glob-import surfaces, then verified the code-source package test suite passes.

Decisions made
- Moved the three visitor implementations into parser-owned submodules under `parse/fs_visitors/` and kept `fs_visitors.rs` as the shared alias-matching facade.
- Preserved the shared alias collection and matching helpers in the facade file so the behavior stayed the same while the implementation budget was split across smaller modules.
- Kept the regression tests focused on sibling-function leakage because that is the actual scope boundary that was missing.

Key files for context
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/inline_std_fs.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_import.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_glob_import.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs`

Next steps
- The code-source package is green.
- `g3rs validate --path packages/rs/code/g3rs-code-source-checks --family code` is still blocked by unrelated compile errors in `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`, so rerun validation after that in-flight work is resolved.
