# Fix 48 clippy errors after service profile switch

**Date:** 2026-03-15 15:59
**Task:** Fix all clippy errors caused by service profile banning std::fs and std::process operations

## Goal
Zero clippy errors on `cargo clippy --all-targets`.

## Approach
1. Add `#[allow(clippy::disallowed_methods)]` with reason comments to functions using banned fs/process ops
2. Fix 3 type_complexity errors by adding type aliases
3. Fix 2 doc_markdown errors by adding backticks

## Files to Modify
- Multiple files across src/ — see detailed error list in task description
