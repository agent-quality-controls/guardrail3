# Fix R-TEST-07 to accept `#[ignore = "reason"]` attribute syntax

**Date:** 2026-03-16 12:02
**Task:** Fix false positive in R-TEST-07 when `#[ignore = "reason"]` is used

## Goal
`#[ignore = "reason"]` should not be flagged by R-TEST-07. The reason is provided inline in the attribute itself.

## Approach

### Step-by-step plan
1. In `src/rs/validate/ast_helpers.rs`, modify `IgnoreVisitor::check_ignore_attrs` to inspect `attr.meta`:
   - `Meta::Path` -> bare `#[ignore]`, needs reason comment (current behavior)
   - `Meta::NameValue` -> `#[ignore = "reason"]`, reason provided, skip
   - `Meta::List` -> `#[ignore(...)]`, treat as having reason, skip
2. Add test in `ast_helpers.rs` tests for `#[ignore = "reason"]` not being flagged
3. Add test in `test_quality_checks.rs` tests for the same

### Key decisions
- **Check meta variant instead of string parsing:** syn already parses the attribute meta, so we use the typed enum directly
- **`Meta::List` treated as having reason:** unusual syntax but if someone uses it, they've clearly been intentional

## Files to Modify
- `src/rs/validate/ast_helpers.rs` — modify `check_ignore_attrs` to check `attr.meta` variant, add unit test
- `src/rs/validate/test_quality_checks.rs` — add integration test for `#[ignore = "reason"]`
