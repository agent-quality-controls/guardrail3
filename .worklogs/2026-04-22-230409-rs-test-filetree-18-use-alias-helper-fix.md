Summary
- Fixed `RS-TEST-FILETREE-18` so `test_support` now resolves local helper calls reached through `use self::... as ...` aliases.
- Added red regressions for both the canned fixture path and semantic-helper path, then verified the package runtime tests and `g3rs validate` for the touched package.

Decisions made
- Kept the fix at the rule boundary by building a local-import alias map from existing `TestSupportFileInput` facts instead of widening parser output.
- Reused the same alias-resolution path for canned and semantic helper detection so both branches share the same behavior.
- Added use-alias regressions that force the helper-call-only shape, because the zero-arg string-return case was already covered by the existing rule.

Key files for context
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/parse/body.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`

Next steps
- If a follow-up attack is needed, compare the same import-alias resolution shape against other rs/test rules that still rely on function-body alias facts only.
