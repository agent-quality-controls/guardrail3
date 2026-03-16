# Step 04: Add tree-sitter Dependency + Create TypeScript Parsing Helper Module

## Goal
Add `tree-sitter` + `tree-sitter-typescript` and create a helper module for TS source analysis.

## Task (1 agent)

### 1. Add dependencies
In `Cargo.toml`:
```toml
[dependencies]
tree-sitter = "0.24"
tree-sitter-typescript = "0.23"
```

### 2. Create `src/ts/validate/ast_helpers.rs`

```rust
//! AST-based TypeScript source analysis using tree-sitter.

/// Parse TypeScript source into a tree-sitter tree.
pub fn parse_typescript(source: &str) -> Option<tree_sitter::Tree> { ... }

/// Find all single-line and block comments.
pub fn find_comments(tree: &tree_sitter::Tree, source: &str) -> Vec<Comment> { ... }

/// Find eslint-disable directives (only in actual comments, not strings).
pub fn find_eslint_disables(tree: &tree_sitter::Tree, source: &str) -> Vec<EslintDisable> { ... }

/// Find @ts-ignore and @ts-expect-error (only in comments).
pub fn find_ts_directives(tree: &tree_sitter::Tree, source: &str) -> Vec<TsDirective> { ... }

/// Find process.env member accesses (not in strings or comments).
pub fn find_process_env(tree: &tree_sitter::Tree, source: &str) -> Vec<ProcessEnvAccess> { ... }

/// Find `: any` type annotations (not in strings or comments).
pub fn find_any_types(tree: &tree_sitter::Tree, source: &str) -> Vec<AnyType> { ... }
```

### 3. Wire into mod.rs

Add `pub mod ast_helpers;` to `src/ts/validate/mod.rs`.

### 4. Write unit tests

Same pattern: real violation vs string literal containing the pattern.

## Verification

```bash
cargo check
cargo test
```

## On Failure
tree-sitter requires a C compiler for the grammar. If CI doesn't have one, add `cc` to the build dependencies or use a pre-compiled grammar.
