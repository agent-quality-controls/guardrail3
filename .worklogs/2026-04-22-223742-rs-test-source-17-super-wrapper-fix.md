## Summary

Fixed `g3rs-test/external-harnesses-use-assertions` so external harness wrappers that call local assertion helpers via `super::assert_demo()` are now promoted into `local_assertion_helpers`. The rule now treats `super` the same as `self` and `crate` in the recursive promotion loop, so a test function that calls such a wrapper is flagged as direct assertion use.

## Decisions made

- Fixed the recursive helper promotion branch in `rule.rs`.
  - Why: the miss was in propagation from wrapper helpers, not in the final external-harness classification.
- Added a red regression for `super::assert_demo()` wrappers.
  - Why: this was the concrete production-path miss and the exact missing coverage.
- Kept the change scoped to the source-check rule and its sidecar tests.
  - Why: the existing parser facts already contained everything needed.

## Key files for context

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `.plans/2026-04-22-223703-rs-test-source-17-super-wrapper-fix.md`

## Next steps

- None for this fix.
