# Add tree-sitter dependency and TypeScript AST parsing helpers

**Date:** 2026-03-16 08:51
**Task:** Execute step 04 of migration plan — add tree-sitter deps and create ast_helpers module

## Goal
Add tree-sitter + tree-sitter-typescript to Cargo.toml and create `src/ts/validate/ast_helpers.rs` with helpers that use AST-based analysis (not regex) to find comments, eslint-disables, ts-directives, process.env accesses, and `: any` type annotations.

## Approach

### Step-by-step plan
1. Add `tree-sitter = "0.24"` and `tree-sitter-typescript = "0.23"` to `[dependencies]` in Cargo.toml
2. Create `src/ts/validate/ast_helpers.rs` with:
   - `parse_typescript(source) -> Option<Tree>` using tree-sitter-typescript
   - `find_comments(tree, source) -> Vec<(usize, String)>` — walks tree for comment nodes
   - `find_eslint_disables(tree, source) -> Vec<(usize, String)>` — filters comments for eslint-disable
   - `find_ts_directives(tree, source) -> Vec<(usize, String)>` — filters comments for @ts-ignore/@ts-expect-error
   - `find_process_env(tree, source) -> Vec<usize>` — finds process.env member expressions not in strings
   - `find_any_types(tree, source) -> Vec<usize>` — finds `: any` type annotations not in strings
3. Add `pub mod ast_helpers;` to `src/ts/validate/mod.rs`
4. Write unit tests: real violation vs string literal for each helper

### Key decisions
- **tree-sitter 0.24 (not 0.26):** Following the plan spec exactly. tree-sitter-typescript 0.23 is built against 0.24 API.
- **Return tuples not structs:** The plan signature uses `Vec<(usize, String)>` and `Vec<usize>`, keeping it simple.

## Files to Modify
- `Cargo.toml` — add two dependencies
- `src/ts/validate/ast_helpers.rs` — new file with all helpers + tests
- `src/ts/validate/mod.rs` — add `pub mod ast_helpers;`
