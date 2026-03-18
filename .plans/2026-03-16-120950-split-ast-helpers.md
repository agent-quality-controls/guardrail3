# Split ast_helpers.rs into two files

**Date:** 2026-03-16 12:09
**Task:** Split 956-line ast_helpers.rs into ast_helpers.rs (~450 lines) and ast_visitors.rs (~450 lines)

## Goal
Both files under 500 lines. All 361 tests pass. Zero clippy warnings. No caller changes needed.

## Approach

### What moves to ast_visitors.rs
All visitor structs and their Visit impls (lines ~330-677):
- ItemAllowVisitor, CfgAttrAllowVisitor, GardeSkipVisitor, UnsafeVisitor
- ForbiddenMacroVisitor, UnwrapExpectVisitor, DeriveVisitor, InlineStdFsVisitor
- TestAttrVisitor, PubFnVisitor, TestCountVisitor, IgnoreVisitor
- Helper functions used ONLY by visitors: has_garde_skip, collect_outer_allows, collect_cfg_attr_allows, has_test_or_tokio_test, trait_item_attrs
- LintList struct (used only by CfgAttrAllowVisitor pipeline)
- The ignore_meta_tests module

### What stays in ast_helpers.rs
- Module doc, imports, Located type, DeriveInfo struct
- parse_file, all public find_*/count_*/has_* functions
- Helper functions shared across modules: span_line, extract_allow_lints, extract_cfg_attr_allow_lints, path_to_string, item_attrs, impl_item_attrs
- use_tree_matches_std_fs, use_subtree_is_fs (used by find_std_fs_imports inline)
- is_cfg_test_attr (used by both find_std_fs_imports and InlineStdFsVisitor)
- The main tests module

### Wiring
- Add `mod ast_visitors;` to mod.rs
- In ast_helpers.rs: add `mod ast_visitors; pub use ast_visitors::*;` — wait, that won't work since mod.rs owns the module tree
- Better: ast_visitors.rs is a sibling module. ast_helpers.rs uses `use super::ast_visitors::*;` or `use crate::rs::validate::ast_visitors::*;`
- Actually simplest: make visitor structs pub(crate) in ast_visitors, import them in ast_helpers where the public functions use them

## Files to Modify
- `src/rs/validate/ast_helpers.rs` — remove visitor structs, add import of ast_visitors
- `src/rs/validate/ast_visitors.rs` — new file with all visitors
- `src/rs/validate/mod.rs` — add `mod ast_visitors;`
