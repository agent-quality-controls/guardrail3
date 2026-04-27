Goal
- Fix g3rs-test/external-harnesses-use-assertions so an owned assertions import aliased into a local name and then re-aliased again is still recognized as owned assertions, not as a local assertion helper.

Approach
- Add a regression test in the source-checks sidecar for `use demo_assertions::assert_demo as run; use self::run as again;`.
- Update the helper-resolution boundary in `rs_test_17_external_harnesses_use_assertions/rule.rs` so local alias chaining preserves owned-assertions provenance instead of collapsing into a direct-local helper match.
- Keep the change minimal and confined to the rule plus its tests.
- Verify with the targeted regression, full package tests, and `g3rs validate` for the touched package.

Key decisions
- Fix at the rule helper-resolution layer rather than changing parser facts because the current facts already expose the alias chain.
- Preserve the earlier `use self::assert_demo as run;` local wrapper behavior while avoiding false positives for owned-assertions aliases.

Files to modify
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs
