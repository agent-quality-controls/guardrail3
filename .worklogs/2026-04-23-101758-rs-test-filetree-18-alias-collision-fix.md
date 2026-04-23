Summary
- Fixed `RS-TEST-FILETREE-18` sibling-module alias collisions so `h::fixture_path(...)` and `h::any_rule(...)` are only treated as disallowed helpers when alias `h` actually targets a sibling module exporting those helper names.
- Added red regressions proving the false positive when a glob-imported sibling exposes the same helper name as a different sibling module aliased as `h`.

Decisions made
- Fixed the bug in `support.rs`.
  - Why: the parser already preserves local import bindings and call paths; the broken step was helper resolution flattening sibling-module helper names before matching alias-qualified calls.
- Added alias-specific sibling helper maps and consulted them before falling back to the existing name-only local helper check.
  - Why: sibling module aliases can be resolved precisely from the analyzed sibling files, while inline module aliases still need the old fallback because current parsed function facts do not carry module ownership.
- Only register alias-specific maps when an import resolves to an actual sibling file.
  - Why: inserting empty sibling maps for inline module aliases broke existing true-positive inline module alias cases.

Key files for context
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/support.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `.plans/2026-04-23-101133-rs-test-filetree-18-alias-collision-fix.md`

Next steps
- None for this fix.
