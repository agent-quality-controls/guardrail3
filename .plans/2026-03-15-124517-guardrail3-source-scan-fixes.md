# Fix 4 remaining issues in source_scan.rs

**Date:** 2026-03-15 12:45
**Task:** Apply block comment filtering to R37/R38-R39/R40-R41, fix filter_non_comment_lines re-entry, add unsafe{ pattern, add -repo/-ports suffixes to R51

## Goal
All four checks properly filter block comments, the comment filter handles `*/code/*` patterns, unsafe detection catches no-space variant, and dependency direction catches -repo/-ports suffixes.

## Approach
1. R37 (check_cfg_attr_allow): switch from content.lines() to filter_non_comment_lines()
2. R38-R39 (check_file_length): use filter_non_comment_lines().len() for effective line count
3. R40-R41 (check_use_count): use filter_non_comment_lines() instead of content.lines()
4. filter_non_comment_lines: after finding `*/`, check remaining content for new `/*`
5. check_unsafe: add "unsafe{" to patterns
6. check_dependency_direction: add "-repo" and "-ports" to banned_suffixes

## Files to Modify
- `src/rs/validate/source_scan.rs` — all changes in this single file
