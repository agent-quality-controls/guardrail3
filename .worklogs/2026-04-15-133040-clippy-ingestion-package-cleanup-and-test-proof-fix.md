Summary
- Cleaned `packages/rs/clippy/g3rs-clippy-ingestion` until full validation returned `No findings.`
- Fixed a real `RS-TEST-SOURCE-07` bug so setup helpers are not mistaken for proof helpers, while shared assertion wrappers over other assertions crates still count as real proof.

Decisions made
- Removed the local `types` crate instead of widening arch rules. It only held the ingestion error enum, so the clean fix was to move that enum into runtime and delete the extra crate.
- Kept the ingestion assertions crate as a shared proof wrapper over the config-check and filetree-check assertions crates. That shape matches the test design better than duplicating proof logic locally.
- Fixed `RS-TEST-SOURCE-07` in the rule, not in the package. The bad behavior was in the rule: it treated any local helper call as proof and did not recognize wrappers over another assertions crate.
- Fixed the package deps errors by allowlisting the new shared-proof dependencies instead of hiding them. Those deps are real and intentional.

Key files for context
- `.plans/2026-04-15-125653-clippy-ingestion-package-fixes.md`
- `.plans/2026-04-15-132610-test-source-07-proof-order-fix.md`
- `packages/rs/clippy/g3rs-clippy-ingestion/guardrail3-rs.toml`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/error.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/assertions/src/run.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs`

Next steps
- Move to the next package audit and keep fixing the clearly valid findings until the next real rule contradiction shows up.
- If another test rule flags wrappers over shared assertions crates, check whether it has the same blind spot before changing package code.
