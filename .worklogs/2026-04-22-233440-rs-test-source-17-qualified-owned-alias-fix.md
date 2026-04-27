Summary
- Fixed `g3rs-test/external-harnesses-use-assertions` so qualified calls to re-aliased owned assertions names like `self::again()` inventory as owned assertions usage.
- The rule now reuses the owned-assertion alias map for both bare and `crate/self/super`-qualified alias paths.

Decisions made
- Kept the fix local to the rule because the bug was incomplete alias classification, not a wider ingestion problem.
- Reused the existing owned-assertion alias map instead of inventing another resolution path.

Key files for context
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs
- .plans/2026-04-22-233440-rs-test-source-17-qualified-owned-alias-fix.md

Next steps
- Fix `g3rs-test/test-support-generic` for module-alias helper calls like `h::fixture_path()` and `h::any_rule()`.
- Re-run the `rs/test` attack pass after that file-tree fix lands.
