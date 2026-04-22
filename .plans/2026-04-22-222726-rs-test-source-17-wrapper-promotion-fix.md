## Goal

Fix `RS-TEST-SOURCE-17` so a local assertion helper called through `self::helper()` or `crate::helper()` still promotes its wrapper function into `local_assertion_helpers`, preventing external harnesses that call the wrapper from being misclassified as clean.

## Approach

- Add red-first regressions in `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`.
- Update the recursive helper promotion loop in `rule.rs` to recognize qualified local wrapper calls as local proof propagation.
- Keep the change inside the rule boundary and avoid touching ingestion or broader source analysis.
- Verify with the package test suite and the package validate run.

## Key decisions

- Fix helper promotion in `local_assertion_helper_names` rather than patching the outer check.
  - Why: the bug is in recursive proof propagation, and the helper set is the shared source of truth for later classification.
- Cover both `self::` and `crate::` forms.
  - Why: both are the same local rebinding class and the user report names both paths.

## Files to modify

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
