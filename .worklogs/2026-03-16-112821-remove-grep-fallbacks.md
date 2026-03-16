# Remove all grep fallbacks — AST-only source scanning

**Date:** 2026-03-16 11:28
**Scope:** All source scan modules (rs + ts)

## Summary
Deleted all 15 grep fallback functions, the grep supplement for macro bodies, and all grep helper functions (compute_multiline_string_lines, strip_string_literals, etc.). Source scanning is now 100% AST-based (syn for Rust, tree-sitter for TypeScript). If a file can't be parsed, it gets zero source scan results — which is correct since unparseable files can't compile.

## Decisions Made

### No grep fallback at all
- **Chose:** Delete all fallback paths. If syn/tree-sitter can't parse → skip file.
- **Why:** The grep paths were the source of nearly all bugs found by adversarial agents (string tracking, comment tracking, brace counting, raw strings, BOM). AST paths had zero bugs. Grep fallbacks added complexity and bugs while handling a case (unparseable files) that doesn't matter (they can't compile anyway).

### No macro body supplement
- **Chose:** Delete `check_item_level_allow_grep_supplement` entirely.
- **Why:** It was the buggiest component (compute_multiline_string_lines, starts_with limitations). `#[allow]` inside `macro_rules!` bodies is a known AST limitation — accepting it is better than maintaining a buggy partial parser.

### Expression-level attributes not detected
- **Chose:** Accept that `#[allow]` on let bindings, match arms, loop bodies is invisible.
- **Why:** syn's Visit trait doesn't walk expression-level attributes through visit_item. Fixing this requires a custom expression visitor, which is out of scope for this migration.

### BOM handling
- **Chose:** Strip UTF-8 BOM in `parse_file` before passing to syn.
- **Why:** BOM bytes cause syn parse failure on otherwise valid Rust.

### cfg(test) imports
- **Chose:** Added `is_cfg_test_attr` check in `find_std_fs_imports` to skip `#[cfg(test)] use std::fs`.
- **Why:** This logic was previously in the grep path. Moving to AST ensures test-only imports aren't flagged.

## Files Modified
- `src/rs/validate/allow_checks.rs` — deleted 5 grep functions, compute_multiline_string_lines
- `src/rs/validate/structure_checks.rs` — deleted check_unsafe_grep, use-count grep branch
- `src/rs/validate/code_quality_checks.rs` — deleted 5 grep functions, cfg_test brace tracking
- `src/rs/validate/garde_checks.rs` — deleted count_deserialize_structs grep
- `src/rs/validate/test_checks.rs` — deleted content_has_test_grep
- `src/rs/validate/test_quality_checks.rs` — deleted 3 grep functions
- `src/ts/validate/source_scan.rs` — deleted 2 grep functions
- `src/ts/validate/ts_comment_checks.rs` — deleted 2 grep functions
- `src/ts/validate/test_checks.rs` — deleted 2 grep functions
- `src/rs/validate/ast_helpers.rs` — added BOM stripping, cfg(test) import skip, DeriveInfo/DeriveVisitor
- `src/rs/validate/ast_visitors.rs` — DELETED (merged into ast_helpers.rs)
- `tests/adversarial_grep_attacks.rs` — updated 3 tests for AST-only behavior
- `tests/adversarial_fixtures.rs` — updated macro body test for AST-only behavior
- `tests/fixtures/grep-attacks/edge-cases/*.rs` — fixed println\! syntax errors
