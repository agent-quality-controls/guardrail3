## Summary

Fixed `RS-TEST-SOURCE-17` so local assertion helpers reached through `self::helper()` or `crate::helper()` now promote wrapper functions into `local_assertion_helpers`. External harnesses that call those wrappers are now classified as direct assertion use instead of slipping through as clean.

## Decisions made

- Fixed the recursive helper promotion loop in `rule.rs`.
  - Why: the bug was in proof propagation, not in the final external-harness classification branch.
- Added one regression for each qualified local-call form.
  - Why: `self::` and `crate::` are the same rebinding class here, and both needed coverage.
- Kept the change inside the source-check rule and its sidecar tests.
  - Why: the parser already exposes the call-path facts needed by the rule.

## Key files for context

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `.plans/2026-04-22-222726-rs-test-source-17-wrapper-promotion-fix.md`

## Next steps

- None for this fix.
