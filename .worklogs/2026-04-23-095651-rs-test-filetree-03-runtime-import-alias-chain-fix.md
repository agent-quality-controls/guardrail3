Summary
- Fixed `g3rs-test/runtime-assertions-split` so assertions modules that reach `check_test_tree()` through chained import aliases are still reported.
- Added a red regression for `use demo_runtime::{self as rt}; use self::rt as again; again::check_test_tree()` and verified the package workspace plus `g3rs validate`.

Decisions made
- Kept the fix in `helpers.rs` because the parser already supplies import bindings and call paths; the bug was incomplete alias classification, not missing ingestion data.
- Resolved runtime-root and `check_test_tree` imports with a fixpoint over `UseBinding`s so local alias chains like `self::rt as again` inherit the runtime-root classification without relying on import order.
- Restricted local runtime-root rebinding to direct alias-chain shapes after stripping `crate/self/super` prefixes.
  - Why: the goal is to follow alias chains, not to infer arbitrary deeper local module semantics.

Key files for context
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
- `.plans/2026-04-23-095241-rs-test-filetree-03-runtime-import-alias-chain-fix.md`

Next steps
- None for this fix.
