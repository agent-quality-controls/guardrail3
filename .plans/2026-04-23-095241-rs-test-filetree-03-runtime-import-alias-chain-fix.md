Goal
- Fix `RS-TEST-FILETREE-03` so assertions modules that reach `check_test_tree()` through a chained import alias are still reported.

Approach
- Add a rule regression in `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs` for `use demo_runtime::{self as rt}; use self::rt as again; again::check_test_tree()`.
- Update `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs` to close over import alias chains when deriving runtime roots instead of only accepting bindings whose first segment is already the package name.
- Verify with targeted package tests first, then `g3rs validate --path packages/rs/test/g3rs-test-file-tree-checks`.

Key decisions
- Keep the fix in the rule helper boundary.
  - The parser already exposes import bindings and call paths, so the missing behavior is alias classification, not ingestion.
- Resolve import aliases transitively with cycle protection.
  - A chained `use self::rt as again;` should inherit the runtime-root classification from `rt`, and the same mechanism should cover longer local alias chains without adding special cases for a single pattern.

Files to modify
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
