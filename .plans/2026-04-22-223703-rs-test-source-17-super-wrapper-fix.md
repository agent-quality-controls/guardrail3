## Goal

Fix `g3rs-test/external-harnesses-use-assertions` so external harness wrappers that call local assertion helpers via `super::assert_demo()` are promoted the same way as `self::` and `crate::` wrappers.

## Approach

- Add a regression in the external-harness sidecar that proves a `super::assert_demo()` wrapper is currently missed.
- Extend the local helper promotion loop in `rule.rs` to treat `super`-qualified local calls as promotable wrapper evidence.
- Keep the change limited to the rule and its tests.
- Verify with the runtime package tests and `g3rs validate` for the touched package.

## Key decisions

- Fix this in the recursive helper promotion loop.
  - Why: the rule already treats the final external-harness call site correctly; the miss is in the wrapper promotion step.
- Reuse the existing qualified-call shape handling.
  - Why: `super::` is the same local rebinding class as `self::` and `crate::` here.

## Files to modify

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `.worklogs/2026-04-22-223703-rs-test-source-17-super-wrapper-fix.md`
