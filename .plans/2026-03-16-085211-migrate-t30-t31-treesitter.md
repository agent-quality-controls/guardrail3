# Migrate T30 (process.env) and T31 (: any) to tree-sitter

**Date:** 2026-03-16 08:52
**Task:** Step 21 — replace grep-based checks with tree-sitter AST queries

## Goal
T30 and T31 use grep to find `process.env` and `: any` / `as any`. Migrate to tree-sitter so matches in strings/comments are structurally excluded.

## Approach

### Step-by-step plan
1. Add `tree-sitter = "0.26"` and `tree-sitter-typescript = "0.23"` to Cargo.toml
2. Create `src/ts/validate/ts_ast_helpers.rs` with:
   - `parse_ts(source) -> Option<Tree>` — parse TS/TSX
   - `find_process_env(tree, source) -> Vec<usize>` — find `process.env` member expressions
   - `find_any_type_annotations(tree, source) -> Vec<(usize, String)>` — find `: any` and `as any`
3. Rewrite `check_process_env` to call tree-sitter first, fall back to grep on parse failure
4. Rewrite `check_any_types` to call tree-sitter first, fall back to grep on parse failure
5. T32-T35 stay grep-based
6. Register module in mod.rs

## Key decisions
- **tree-sitter over swc/oxc**: tree-sitter is lightweight, no proc-macro compilation, works well for queries
- **Fallback to grep**: if tree-sitter parse fails, existing grep logic runs

## Files to Modify
- `Cargo.toml` — add tree-sitter deps
- `src/ts/validate/ts_ast_helpers.rs` — new file with tree-sitter helpers
- `src/ts/validate/mod.rs` — add module declaration
- `src/ts/validate/source_scan.rs` — rewrite check_process_env and check_any_types
