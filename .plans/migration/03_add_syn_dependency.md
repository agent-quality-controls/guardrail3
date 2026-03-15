# Step 03: Add syn Dependency + Create Rust Parsing Helper Module

## Goal
Add `syn` to guardrail3 and create a helper module that the migrated checks will use.

## Task (1 agent)

### 1. Add dependency
In `Cargo.toml`:
```toml
[dependencies]
syn = { version = "2", features = ["full", "parsing", "visit"] }
```

### 2. Create `src/rs/validate/ast_helpers.rs`

A module providing convenience functions for common AST inspection patterns:

```rust
//! AST-based source analysis helpers using syn.
//!
//! These functions parse Rust source into an AST and inspect it
//! structurally — no grep, no false positives from strings/comments.

use syn::{visit::Visit, File, Item, Attribute, Meta};

/// Parse a Rust source file. Returns None if parsing fails (non-Rust content).
pub fn parse_file(source: &str) -> Option<syn::File> {
    syn::parse_file(source).ok()
}

/// Find all #![allow(...)] crate-level attributes.
pub fn find_crate_level_allows(file: &syn::File) -> Vec<CrateAllow> { ... }

/// Find all #[allow(...)] item-level attributes with their locations.
pub fn find_item_allows(file: &syn::File) -> Vec<ItemAllow> { ... }

/// Find all #[cfg_attr(..., allow(...))] attributes.
pub fn find_cfg_attr_allows(file: &syn::File) -> Vec<CfgAttrAllow> { ... }

/// Find all #[garde(skip)] attributes.
pub fn find_garde_skips(file: &syn::File) -> Vec<GardeSkip> { ... }

/// Find all unsafe blocks and unsafe fn declarations.
pub fn find_unsafe_usage(file: &syn::File) -> Vec<UnsafeUsage> { ... }

/// Find all todo!(), unimplemented!(), panic!() macro invocations.
pub fn find_forbidden_macros(file: &syn::File) -> Vec<ForbiddenMacro> { ... }

/// Find all .unwrap() and .expect() method calls.
pub fn find_unwrap_expect(file: &syn::File) -> Vec<UnwrapExpect> { ... }

/// Find all `use std::fs` imports.
pub fn find_std_fs_imports(file: &syn::File) -> Vec<StdFsImport> { ... }

/// Count use statements.
pub fn count_use_statements(file: &syn::File) -> usize { ... }
```

Each return type includes the line number (from syn spans) for reporting.

### 3. Wire into mod.rs

Add `pub mod ast_helpers;` to `src/rs/validate/mod.rs`.

### 4. Write unit tests

For each helper function, write at least 2 tests:
- One with a real violation (should find it)
- One with the same text in a string literal (should NOT find it)

## Verification

```bash
cargo check  # compiles
cargo test   # all existing tests pass + new helper tests
```

## On Failure
If syn conflicts with existing deps, check version compatibility. syn 2.x should work with any recent Rust.
