# Split ast_helpers.rs into two files under 500 lines

**Date:** 2026-03-16 10:18
**Task:** Split rs/validate/ast_helpers.rs (916 lines) into two files, both under 500 lines

## Goal
Both resulting files under 500 effective lines. All callers unchanged.

## Approach

Split into:
1. `ast_helpers.rs` (~460 lines) — Public API functions, DeriveInfo struct, internal helper functions (extract_*, has_*, use_tree_*, path_to_string, item_attrs, etc.), and re-export of visitors module. No tests in this file.
2. `ast_visitors.rs` (~470 lines) — All 11 visitor structs + their Visit impls + all 36 unit tests.

The visitors need access to internal helpers (span_line, collect_outer_allows, etc.) so those helpers become `pub(super)` or we put them in the main file and make visitors use `super::`.

Tests stay with visitors since tests exercise the public API (which re-delegates to visitors) and the test module is 376 lines.

## Files to Modify
- `src/rs/validate/ast_helpers.rs` — Remove visitors + tests, keep public API + internals
- `src/rs/validate/ast_visitors.rs` — New file with visitors + tests
- `src/rs/validate/mod.rs` — Add `mod ast_visitors;`
