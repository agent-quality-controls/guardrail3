Summary

- Fixed the `rs/test` source-check false negative for owned assertions imported through package-root aliases like `use demo_assertions::{self as da}; da::assert_demo()`.
- Added red regressions for both `RS-TEST-SOURCE-07` and `RS-TEST-SOURCE-17`, then normalized grouped `self as` imports back to the assertions package root before owned-assertion matching.

Decisions made

- Fixed the bug in shared source-check support instead of changing the parser.
  - Why: ingestion is accurately preserving the syntactic `self` import segment; the missing step was semantic normalization for owned-assertion resolution.
- Reused `RS-TEST-SOURCE-07`'s owned-assertion proof path as the production fix surface.
  - Why: `RS-TEST-SOURCE-17` already delegates owned-assertion detection through `has_owned_assertion_proof`, so fixing the shared semantic boundary removes the false negative for both rules without duplicating logic.
- Limited the change to the requested source-check package files plus the plan/worklog.
  - Why: there were unrelated in-progress edits elsewhere in the tree.

Key files for context

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `.plans/2026-04-23-095328-rs-test-source-root-alias-fix.md`

Next steps

- None for this bug fix.
