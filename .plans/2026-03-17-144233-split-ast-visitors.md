# Split ast_visitors.rs to get under 500 lines

**Date:** 2026-03-17 14:42
**Task:** Extract InlineStdFsVisitor into fs_usage_visitor.rs

## Goal
Get ast_visitors.rs under 500 lines by extracting InlineStdFsVisitor (lines 526-586) into a new file.

## Approach
1. Create `fs_usage_visitor.rs` with InlineStdFsVisitor and its impl blocks
2. Remove InlineStdFsVisitor from ast_visitors.rs
3. Update mod.rs to add `pub mod fs_usage_visitor`
4. Update ast_helpers.rs import to pull from fs_usage_visitor instead of ast_visitors

## Files to Modify
- `apps/guardrail3/src/app/rs/validate/fs_usage_visitor.rs` — new file
- `apps/guardrail3/src/app/rs/validate/ast_visitors.rs` — remove InlineStdFsVisitor
- `apps/guardrail3/src/app/rs/validate/mod.rs` — add module
- `apps/guardrail3/src/app/rs/validate/ast_helpers.rs` — update import path
