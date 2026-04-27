Goal
- Make g3rs-test/real-proof-site recognize the full exported proof-helper surface of `define_result_assertions!` so shared assertions calls are not false-negative.

Approach
- Update the test-family parser in `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/mod.rs`.
- Replace the stale short helper list for `define_result_assertions!` with the full known helper contract used across active Rust families.
- Add a regression in `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs`.
- The regression will use a synthetic assertions module with `define_result_assertions!` and prove that every exported `assert_*` helper name counts as shared proof.
- Re-run test-family tests, then re-run validation on `packages/rs/cargo/g3rs-cargo-config-checks`.

Key decisions
- Keep the known-macro contract approach.
- Reject cargo-only package rewrites. The failure is in the rule's stale helper surface, not in the package using the shared assertions crate.
- Keep the contract in one parser constant for now, with one regression that covers the whole helper set.

Files to modify
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs`
