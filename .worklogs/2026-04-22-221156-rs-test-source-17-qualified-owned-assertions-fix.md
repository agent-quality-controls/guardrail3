## Summary

Fixed the `g3rs-test/external-harnesses-use-assertions` false positive where `demo_assertions::assert_demo()` was treated as a local proof helper when the file also defined a local `assert_demo()`.

## Decisions made

- Kept the fix inside `rule.rs` instead of changing parser facts.
- Added a red-first regression that reproduces the qualified owned-call collision.
- Taught the rule to recognize package-root-qualified owned assertions calls before the local-helper branch can claim them.

## Key files for context

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`

## Next steps

- None.
