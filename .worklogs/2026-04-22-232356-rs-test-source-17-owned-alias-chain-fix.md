Summary
- Fixed `g3rs-test/external-harnesses-use-assertions` so external harnesses that call owned assertions through a local alias chain inventory correctly instead of false-firing as direct local assertions.
- The rule now distinguishes local helper aliases from aliases that still resolve back into the owned assertions crate.

Decisions made
- Fixed the resolution logic in the rule instead of adding a narrow exception for one alias shape.
- Kept the change local to `g3rs-test/external-harnesses-use-assertions` because the bug was in how this rule classified imported alias chains.

Key files for context
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs
- .plans/2026-04-22-231951-rs-test-source-17-owned-alias-chain-fix.md

Next steps
- Finish the hook parser bug fixes already in progress: function-tail brace parsing and escaped-hash comment handling.
- Finish the `g3rs-hooks/hook-rs-16-config-changes-trigger-validation` discarded-trigger fix.
