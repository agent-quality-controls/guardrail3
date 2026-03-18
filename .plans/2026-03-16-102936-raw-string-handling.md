# Fix raw string handling in grep helpers

**Date:** 2026-03-16 10:29
**Task:** Bug #27 — Raw string handling in compute_multiline_string_lines and strip_string_literals

## Goal
Both grep helper functions correctly handle Rust raw strings (r"...", r#"..."#, r##"..."##, etc.) so that raw string content doesn't break quote parity or leak into comment detection.

## Approach

### 1. `compute_multiline_string_lines` in allow_checks.rs (line 11)
- Currently iterates chars and toggles `in_string` on every `"`
- Fix: when encountering `"`, check if preceded by `r` or `r#...#` pattern
- When raw string opener detected, scan forward for matching closer (`"` followed by same number of `#`)
- Mark all lines spanned as string lines

### 2. `strip_string_literals` in source_scan.rs (line 209)
- Same issue — toggles on `"` without raw string awareness
- Fix: convert to index-based iteration, detect `r"` or `r#..."` patterns, skip to matching closer

### Tests
- allow_checks.rs: `multiline_string_lines_handles_raw_strings`
- source_scan.rs: `strip_string_literals_handles_raw_strings`

## Files to Modify
- `src/rs/validate/allow_checks.rs` — fix compute_multiline_string_lines, add test
- `src/rs/validate/source_scan.rs` — fix strip_string_literals, add test
