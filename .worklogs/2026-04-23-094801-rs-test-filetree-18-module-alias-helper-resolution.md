Summary
- Fixed `g3rs-test/test-support-generic` so sibling helpers imported through a module alias are detected as local canned and semantic helpers.
- Split helper-discovery and alias-resolution code into an owned support module so the rule file stays under the code-size guardrail.

Decisions made
- Moved helper discovery into `rs_test_18_test_support_generic/support.rs` instead of adding another shallow branch in `rule.rs`.
- Broadened sibling helper discovery to follow relative imports with local aliases like `use self::helpers as h;`; the previous `local_name.is_none()` filter was the real false-negative source.
- Kept the fix in the rule/support boundary. The parser already provides call paths and imports, so ingestion did not need to change.

Key files for context
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/support.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `.plans/2026-04-23-094230-rs-test-filetree-18-module-alias-helper-resolution.md`
- `.plans/2026-04-23-094007-rust-batch-cleanup-parallel.md`

Next steps
- Consume the active parallel attack reports for `rs/test`, `rs/code`, `hooks`, `rs/arch`, and `rs/apparch`.
- For each concrete finding, add a red regression first, fix it at the owning parser/support/rule boundary, verify with package tests and `g3rs validate`, and commit before moving to the next bug.
