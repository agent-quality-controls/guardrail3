# Fix clippy errors in source_scan.rs strip_string_literals

**Date:** 2026-03-16 12:02
**Task:** Fix clippy warnings in `strip_string_literals` — unchecked indexing and `as char` conversion. Also fix other clippy errors across codebase to reach 0 warnings.

## Goal
`cargo clippy --all-targets -- -D warnings` passes with 0 warnings.

## Approach

### strip_string_literals rewrite
Rewrite the function to iterate over characters with `.char_indices()` instead of byte indexing. This eliminates:
- `bytes[i]` indexing (clippy::indexing_slicing)
- `bytes[i] as char` (clippy::as_conversions / clippy::cast_possible_truncation)

The function only deals with ASCII delimiters (`r`, `#`, `"`, `\\`) so char iteration is safe and equivalent.

### Other clippy errors
- `garde_checks.rs:525` — needless raw string hashes
- `ast_helpers.rs:100` — redundant closure
- `ast_helpers.rs:131` — wildcard enum match arm
- `ast_helpers.rs:609` — doc_markdown (missing backticks)

## Files to Modify
- `src/rs/validate/source_scan.rs` — rewrite `strip_string_literals`
- `src/rs/validate/garde_checks.rs` — fix raw string hashes
- `src/rs/validate/ast_helpers.rs` — fix closure, match arm, doc
