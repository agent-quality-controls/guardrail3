# Fix R43 to handle path-qualified macros (std::todo!, core::todo!, etc.)

**Date:** 2026-03-16 12:02
**Task:** R43 check misses `std::todo!()`, `core::todo!()`, etc. because `path_to_string` returns `"std::todo"` but the match checks bare `"todo"`.

## Goal
R43 should flag `std::todo!()`, `core::unimplemented!()`, etc. the same as bare `todo!()`.

## Approach

Two places need fixing:

1. **`ForbiddenMacroVisitor::visit_macro`** in `ast_helpers.rs` (line 429) — the match that decides whether to collect the macro. Extract last segment via `rsplit("::").next()`.

2. **`check_todo_macros`** in `code_quality_checks.rs` (line 25) — the match that decides severity. Same fix: extract last segment.

3. Add test in `code_quality_checks.rs` for `std::todo!()`.

4. Add test in `ast_helpers.rs` for path-qualified forbidden macros.

## Files to Modify
- `src/rs/validate/ast_helpers.rs` — fix visitor match + add test
- `src/rs/validate/code_quality_checks.rs` — fix severity match + add test
