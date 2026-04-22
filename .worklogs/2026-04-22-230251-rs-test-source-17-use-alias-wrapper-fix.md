Summary
- Fixed RS-TEST-SOURCE-17 so external harness wrappers reached through `use self::... as ...` aliases are detected as direct assertion use.

Decisions made
- Kept the change in `rule.rs` at the helper-resolution boundary instead of changing parser facts.
- Extended the existing local helper resolution to follow imported aliases recursively, while preserving the owned-assertions crate path check.
- Added one focused regression in `rule_tests/cases.rs` for the `use self::assert_demo as run; run();` wrapper case that previously escaped the rule.

Key files for context
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `.plans/2026-04-22-230115-rs-test-source-17-use-alias-wrapper-fix.md`

Next steps
- None for this fix. If the rule regresses again, start by checking imported alias resolution in `calls_local_assertion_helper()` and `local_assertion_helper_names()`.
