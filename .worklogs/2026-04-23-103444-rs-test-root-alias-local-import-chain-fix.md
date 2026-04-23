Summary
- Fixed `RS-TEST-SOURCE-07` and `RS-TEST-SOURCE-17` so owned assertions are preserved through local import chains rooted in a crate alias.
- Added regressions for `use demo_assertions::{self as da}; use self::da::assert_demo as prove;`.

Decisions made
- Fixed the bug in shared owned-assertion normalization and the two owning rules.
  - The parser already exposed the needed import paths; the missing piece was resolving `self::da::...` against previously known owned prefixes.
- Kept the import-map construction iterative.
  - Local alias chains can depend on earlier owned roots and should stabilize by repeated passes rather than one linear scan.

Key files for context
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule_tests/cases.rs`
- `.plans/2026-04-23-103432-rs-test-root-alias-local-import-chain-fix.md`

Next steps
- Continue the broader attack follow-up on the remaining `rs/test` file-tree, hooks/parser, and apparch ingestion bugs.
