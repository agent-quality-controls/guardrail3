# Fix InlineStdFsVisitor to catch std::fs::File::open

**Date:** 2026-03-16 12:02
**Task:** Fix path_is_std_fs_call to accept all paths with 3+ segments starting with std::fs

## Goal
`std::fs::File::open(...)` should be caught by R58. Currently skipped because 4+ segment paths are rejected.

## Approach
1. Simplify `path_is_std_fs_call` to accept any path with 3+ segments starting with `std::fs`
2. Update `r58_allows_type_references` test — `std::fs::Permissions::from_mode` in expression context SHOULD be caught
3. Add test for `std::fs::File::open("x")`

## Files to Modify
- `src/rs/validate/ast_helpers.rs` — simplify `path_is_std_fs_call`
- `src/rs/validate/code_quality_checks.rs` — update and add tests
