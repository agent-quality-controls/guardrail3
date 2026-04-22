## Goal

Fix `RS-TEST-FILETREE-18` so alias chains in `test_support` still resolve to local canned or semantic helpers, including cases like `let run = fixture_path; let again = run; again()`.

## Approach

- Add red regression tests in `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`.
- Extend the local-helper path check in `rule.rs` to follow alias chains instead of stopping after one hop.
- Keep the change inside the rule boundary because parser facts already expose `local_call_aliases`.
- Verify with the package test suite and the package validate command.

## Key decisions

- Fix recursive alias resolution in the shared helper matcher.
  - Why: both canned and semantic helper detection use the same local-helper check, so one recursion fix covers both.
- Cover both helper classes in tests if the current rule already treats them as real.
  - Why: the bug is shared by both branches and needs proof on each.

## Files to modify

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
