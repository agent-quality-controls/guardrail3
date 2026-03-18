# Step 03: Add syn dependency and AST helper module

**Date:** 2026-03-15 22:35
**Task:** Migration step 03 — add syn crate and create ast_helpers.rs

## Goal
Add `syn` dependency and create `src/rs/validate/ast_helpers.rs` with AST-based source analysis helpers that will eventually replace the grep-based checks in allow_checks.rs, structure_checks.rs, and code_quality_checks.rs.

## Input Information
- Existing checks use `filter_non_comment_lines()` + string matching (grep-style)
- Patterns to detect: `#![allow(...)]`, `#[allow(...)]`, `#[cfg_attr(..., allow(...))]`, `#[garde(skip)]`, `unsafe {}`, `unsafe fn`, `todo!()`, `unimplemented!()`, `panic!()`, `.unwrap()`, `.expect()`, `use std::fs`
- Return types should be simple tuples: `(usize, String)` for (line, name) or `Vec<usize>` for line-only
- Use `syn::visit::Visit` trait for AST walking
- Project forbids `unsafe`, `unwrap`, `expect`, `todo!`, `panic!` via workspace lints

## Approach

### Step-by-step
1. Add `syn = { version = "2", features = ["full", "parsing", "visit"] }` to Cargo.toml
2. Create `src/rs/validate/ast_helpers.rs` implementing 10 helpers using syn::visit::Visit
3. Add `pub mod ast_helpers;` to mod.rs
4. Write unit tests — each helper gets a positive test and a string-literal-false-positive test

### Key decisions
- **Simple tuple return types** instead of named structs: the plan spec uses `Vec<(usize, String)>` — simpler, matches the task description
- **syn::visit::Visit** over manual recursion: cleaner, handles all node types automatically
- **span.start().line** for line numbers: syn provides this directly, no manual tracking needed

## Files to Modify
- `Cargo.toml` — add syn dependency
- `src/rs/validate/ast_helpers.rs` — new file with all helpers
- `src/rs/validate/mod.rs` — add pub mod ast_helpers
