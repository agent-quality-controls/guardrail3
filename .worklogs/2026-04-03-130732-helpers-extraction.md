# Move test helpers from rule facades to tests/helpers.rs

**Date:** 2026-04-03 13:07

## Summary
Extracted #[cfg(test)] helper functions from 127 split-rule mod.rs facades
into tests/helpers.rs. This makes mod.rs facade-only (only mod/pub use
declarations + #[cfg(test)] mod tests;). ARCH-04 violations: 2344 → 1870.

## Decisions
- Helpers use `crate::` paths instead of `super::super::super::` for
  reaching sibling modules — cleaner and stable.
- `pub(super)` for helpers only used locally, `pub(crate)` for helpers
  re-exported by tests/mod.rs.
- Test files updated: `super::super::X` → `super::helpers::X`.
- `super::super::check()` kept as-is (references the rule's pub export).

## Key files
- `/tmp/move_test_helpers.py` — automation script
- Any `*/tests/helpers.rs` — new helper locations
- Any `*/mod.rs` in rule dirs — cleaned facades
