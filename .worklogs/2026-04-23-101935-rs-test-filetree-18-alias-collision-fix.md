Summary
- Fixed `RS-TEST-FILETREE-18` so module-qualified helper calls keep module identity instead of flattening on the leaf helper name.
- Added regressions proving sibling-module alias collisions stay quiet while real module-alias helper calls still fire.

Decisions made
- Kept the fix in `rs_test_18_test_support_generic/support.rs`.
  - The parser already exposes alias paths and sibling files. The bug was local helper resolution losing module identity after alias lookup.
- Kept the new tests focused on canned and semantic helper collisions.
  - Those were the two concrete false-positive surfaces for `h::fixture_path(...)` and `h::any_rule(...)`.

Key files for context
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/support.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `.plans/2026-04-23-101133-rs-test-filetree-18-alias-collision-fix.md`

Next steps
- Re-run the broader adversarial pass across `rs/test`, `rs/hooks`, `rs/code`, and `rs/apparch` after the remaining local cleanup commits land.
