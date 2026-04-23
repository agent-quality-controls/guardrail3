Goal
- Fix `RS-TEST-SOURCE-07` and `RS-TEST-SOURCE-17` so owned assertions remain recognized through root-alias local import chains like `use demo_assertions::{self as da}; use self::da::assert_demo as prove;`.

Approach
- Add the red regressions in both rule sidecar test suites.
- Fix owned-assertion import normalization to resolve `crate/self/super` aliases through already-known owned root prefixes.
- Make the two rules build their owned import maps iteratively so dependent aliases resolve even when expressed through local path chains.
- Verify `g3rs-test-source-checks` with `cargo test` and `g3rs validate`.

Key decisions
- Keep the fix in shared source-check support plus the two owning rules.
- Do not move this into ingestion. The bug is local import rebinding inside rule-time owned-assertion resolution.

Files to modify
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
