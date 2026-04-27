Goal
- Fix g3rs-test/external-harnesses-use-assertions so owned-assertions aliases are still classified as owned assertions even when a same-file helper shares the same name.

Approach
- Add a focused regression covering `fn assert_demo() { ... } use demo_assertions::assert_demo as run; #[test] fn harness() { run(); }`.
- Narrow the imported-alias helper resolution in `rule.rs` so only local-bound aliases (`crate` / `self` / `super`) can promote local helpers.
- Preserve the earlier `use self::assert_demo as run;` local-wrapper fix by leaving local-bound imports in the resolution map.

Key decisions
- Fix the rule helper-resolution boundary rather than changing ingestion facts.
- Keep owned-assertions alias detection separate from local-helper promotion, because the two cases have opposite classifications.

Files to modify
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
