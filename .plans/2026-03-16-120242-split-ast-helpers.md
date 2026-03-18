# Split ast_helpers.rs (686 lines) into two files under 500

**Date:** 2026-03-16 12:02
**Task:** Split oversized ast_helpers.rs

## Approach

Split by functionality:
1. `ast_helpers.rs` — parsing (parse_typescript, parse_tsx, parse_ts_file) + comment helpers (CommentInfo, find_comments, find_eslint_disables, find_ts_directives, walk_comments) + their tests (~260 lines)
2. `ts_code_analysis.rs` — code analysis (find_process_env, find_any_types, find_test_method_calls) + internal walkers + node_text + their tests (~425 lines)

Update `mod.rs` to add `pub mod ts_code_analysis` and re-export from `ast_helpers` so callers don't change.

## Files to Modify
- `src/ts/validate/ast_helpers.rs` — keep parsing + comments only
- `src/ts/validate/ts_code_analysis.rs` — new file with code analysis
- `src/ts/validate/mod.rs` — add new module
