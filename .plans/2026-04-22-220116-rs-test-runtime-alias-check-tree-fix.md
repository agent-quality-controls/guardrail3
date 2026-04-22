## Goal

Fix `RS-TEST-FILETREE-03` so assertions modules that import the runtime crate as `self as <alias>` still trip the "assertions module orchestrates family execution" check when they call `<alias>::check_test_tree(...)`.

## Approach

- Add a red test that proves `use demo_runtime::{self as rt, check_test_tree}; rt::check_test_tree()` is currently missed.
- Fix the runtime alias collector in the `rs_test_03_runtime_assertions_split` helper so `self as <alias>` is treated as a runtime root alias.
- Re-run the `rs/test` file-tree slice tests and `g3rs validate`.

## Key decisions

- Fix the alias collector, not the violation rule.
  - Why: the wrong result comes from incomplete import binding, not from the rule output layer.
- Keep the fix narrow to the proven alias form.
  - Rejected: broad import normalization churn without another concrete failing case.

## Files to modify

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
- `.worklogs/<timestamp>-rs-test-runtime-alias-check-tree-fix.md`
