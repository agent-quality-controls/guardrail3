Goal
- Make RS-TEST-SOURCE-07 detect the real proof step instead of blaming harmless setup helpers.
- Keep tests that end in shared assertions clean.
- Keep tests that hide result checks in local helpers failing.

Approach
- Add a failing rule test for a test function that calls local setup helpers first and shared assertions last.
- Extend the parser to keep function call order for test functions.
- Update RS-TEST-SOURCE-07 to inspect the last relevant proof-like call instead of the first local call.
- Keep shared assertions wrapper calls valid.
- Re-run rule tests, the full test-source package tests, and then the clippy ingestion package validation.

Key decisions
- Fix the parser once instead of trying to guess setup helper names inside the rule.
- Use call order from the current file only. Do not attempt cross-file control-flow analysis.
- Stop if another rule contradiction appears after this fix.

Files to modify
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/types.rs
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/mod.rs
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs
- packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs
