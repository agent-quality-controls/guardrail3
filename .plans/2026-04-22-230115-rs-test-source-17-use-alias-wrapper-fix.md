Goal
- Fix g3rs-test/external-harnesses-use-assertions so external harness wrappers reached through `use ... as ...` aliases still count as local assertion wrappers and are reported as direct assertion use.

Approach
- Add a focused red regression in `rule_tests/cases.rs` for `fn assert_demo() { assert_eq!(1, 1); } use self::assert_demo as run; #[test] fn harness() { run(); }`.
- Update `rule.rs` at the helper-resolution boundary so local assertion helper detection can follow imported local aliases in addition to existing bare alias chains.
- Keep the change minimal and local to the rule, using existing parser facts rather than widening ingestion or test fixtures.

Key decisions
- Fix in rule logic, not ingestion, because the parser already exposes the import bindings needed to resolve `use` aliases.
- Reuse the current helper discovery and add only the missing alias-resolution path, rather than broadening the direct assertion heuristic.

Files to modify
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
