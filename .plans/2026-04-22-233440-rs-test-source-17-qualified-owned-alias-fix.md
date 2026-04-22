Goal
- Make `RS-TEST-SOURCE-17` recognize qualified calls to re-aliased owned assertions names such as `self::again()`.

Approach
- Add a red regression for a qualified call through a local alias chain that still resolves into the owned assertions crate.
- Extend the owned-assertion alias check so it handles both bare alias calls and `crate/self/super`-qualified alias calls.
- Verify with source-check package tests and `g3rs validate`.

Key decisions
- Reuse the existing owned-assertion alias map instead of creating a second resolution path.
- Keep the fix local to `RS-TEST-SOURCE-17`, because the bug is incomplete alias classification inside this rule.

Files to modify
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs
