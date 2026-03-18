# Fix R53 to support single-crate `[lints.rust]` fallback

**Date:** 2026-03-16 12:02
**Task:** R53 check only looks at `[workspace.lints.rust].unsafe_code`. Add fallback to `[lints.rust].unsafe_code` for single-crate projects.

## Goal
R53 correctly detects `unsafe_code = "forbid"` or `"deny"` in both workspace and single-crate Cargo.toml layouts.

## Approach
1. In `check_unsafe_code_forbid`, after the workspace path lookup returns None, try `table["lints"]["rust"]["unsafe_code"]`.
2. Add a test with single-crate Cargo.toml content (no `[workspace]` section).

## Files to Modify
- `src/rs/validate/structure_checks.rs` — add fallback path + test
