# Fix is_cfg_test_attr to handle compound cfg predicates

**Date:** 2026-03-16 12:01
**Task:** Fix `is_cfg_test_attr` to handle `#[cfg(all(test, ...))]` and similar compound predicates

## Goal
The function should return true for any `#[cfg(...)]` containing `test` as a standalone token, except when `test` only appears inside `not(...)`.

## Approach
1. Replace the single-ident parse with a token string scan
2. Convert attr meta to string, check for "test" as whole word
3. Exclude `not(test)` pattern
4. Add 5 tests covering bare, all(), any(), not(), and substring cases

## Files to Modify
- `src/rs/validate/ast_helpers.rs` -- replace is_cfg_test_attr + add tests
