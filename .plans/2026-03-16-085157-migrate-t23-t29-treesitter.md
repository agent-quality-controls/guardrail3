# Migrate T23-T29 (eslint-disable + ts-ignore) to tree-sitter

**Date:** 2026-03-16 08:51
**Task:** Step 20 of migration plan: replace grep-based eslint-disable and ts-ignore checks with tree-sitter AST parsing

## Goal
T23-T26 (eslint-disable) and T27-T29 (ts-ignore/ts-expect-error) only fire on actual comments, not string/template literals. Falls back to grep on parse failure.

## Approach

### Step-by-step plan
1. Add `tree-sitter` + `tree-sitter-typescript` to Cargo.toml dependencies
2. Create `src/ts/validate/ast_helpers.rs` with:
   - `parse_ts(source)` — parse TS/TSX source, return tree
   - `find_comment_texts(tree, source)` — extract all comment node texts with line numbers
3. Rewrite `check_eslint_disable` in source_scan.rs to:
   - Try tree-sitter parse, walk Comment nodes only
   - Fall back to current grep logic on parse failure
4. Rewrite `check_ts_ignore` similarly
5. Register `ast_helpers` module in mod.rs
6. Run `cargo test`

### Key decisions
- **tree-sitter over regex:** Comments can contain eslint-disable patterns that are real directives. String/template literals containing these patterns are NOT directives. Tree-sitter distinguishes them structurally.
- **Fallback to grep:** If tree-sitter fails to parse (malformed TS), grep is better than nothing.

## Files to Modify
- `Cargo.toml` — add tree-sitter deps
- `src/ts/validate/ast_helpers.rs` — NEW: tree-sitter helpers
- `src/ts/validate/mod.rs` — register ast_helpers module
- `src/ts/validate/source_scan.rs` — rewrite check_eslint_disable and check_ts_ignore
