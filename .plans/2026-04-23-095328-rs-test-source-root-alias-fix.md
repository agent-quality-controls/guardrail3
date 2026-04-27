Goal

- Fix the `rs/test` source-check false negative for owned assertions imported through a package-root alias such as `use demo_assertions::{self as da}; da::assert_demo()`.
- Prove the bug with red regressions first.
- Normalize `self as` package-root imports at the owned-assertion resolution boundary so both `g3rs-test/real-proof-site` and `g3rs-test/external-harnesses-use-assertions` classify the call correctly.

Approach

- Add regression tests in:
  - `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule_tests/cases.rs`
  - `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- Run targeted package tests and confirm both new cases fail before changing runtime logic.
- Add a shared owned-assertion import normalization helper in `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs` if that keeps the fix at the semantic boundary and avoids duplicating `self as` handling across rules.
- Update:
  - `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
  - `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
  so they consume the normalized root alias behavior.
- Verify with package tests and `g3rs validate --path packages/rs/test/g3rs-test-source-checks`.

Key decisions

- Fix at owned-assertion import normalization, not by adding ad hoc call-path exceptions in each rule.
- Keep the parser unchanged because it is accurately preserving the syntactic `self` import shape; the missing step is semantic normalization back to the package root.
- Limit edits to the requested rule and test files plus `support.rs` only if shared normalization is needed.

Files to modify

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs` if shared normalization is added
