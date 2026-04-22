Summary
- Fixed RS-TEST-SOURCE-17 so imported owned-assertions aliases are no longer misclassified as local helpers when a same-file helper has the same name.

Decisions made
- Narrowed imported helper promotion to local-bound aliases only, which preserves the earlier `self::` wrapper fix while excluding `demo_assertions::...` aliases from the local-helper path.
- Added a regression that combines a same-file `assert_demo` helper with `use demo_assertions::assert_demo as run;` to prove the false positive and keep it covered.

Key files for context
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `.plans/2026-04-22-230935-rs-test-source-17-owned-alias-fix.md`

Next steps
- None for this fix. If the rule regresses again, inspect the imported alias map in `imported_local_helper_names()` first.
