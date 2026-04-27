Summary
- Fixed the shared `rs/test` source-check false negative for owned assertions imported through a package-root alias like `use demo_assertions::{self as da}; da::assert_demo()`.
- `g3rs-test/real-proof-site` and `g3rs-test/external-harnesses-use-assertions` now both classify that import shape through one normalization step instead of duplicating `self as` handling.

Decisions made
- Kept the parser unchanged. The parser was preserving the syntactic import correctly; the missing step was semantic normalization back to the assertions crate root.
- Added `normalized_owned_assertion_relative_segments(...)` in source-check support so the root-alias fix is shared instead of reimplemented in both rules.
- Kept the runtime change small: `g3rs-test/external-harnesses-use-assertions` already delegates to `has_owned_assertion_proof`, so its new regression is satisfied by the shared normalization fix plus coverage.

Key files for context
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `.plans/2026-04-23-095328-rs-test-source-root-alias-fix.md`

Next steps
- Land the separate `g3rs-test/runtime-assertions-split` import-alias chain fix already validated in `g3rs-test-file-tree-checks`.
- Continue with the in-flight hooks engine fix and the `rs/apparch` private-module public-surface ingestion fix.
