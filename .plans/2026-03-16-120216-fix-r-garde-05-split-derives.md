# Fix R-GARDE-05: Split derive attributes miscounted

**Date:** 2026-03-16 12:02
**Task:** Fix `find_derive_attributes` to merge all derive macros per item, not per attribute.

## Goal
`#[derive(Deserialize)] #[derive(Validate)] struct Foo {}` should produce ONE `DeriveInfo` with macros `["Deserialize", "Validate"]`, not two separate entries.

## Approach

### Step-by-step plan
1. In `src/rs/validate/ast_helpers.rs`, change `DeriveVisitor::collect_derives` to collect all derive macros from all attributes into a single `Vec<String>`, then push one `DeriveInfo` with all macros merged. Use the line of the first derive attribute.
2. Add a test in `ast_helpers.rs` tests: split derives produce 1 `DeriveInfo` with 2 macros.

### Key decisions
- **Merge per item, not per attribute:** The visitor already calls `collect_derives` once per item with all its attrs. We just need to accumulate macros across multiple `#[derive(...)]` attrs and emit one entry.

## Files to Modify
- `src/rs/validate/ast_helpers.rs` — change `collect_derives` to merge, add test
