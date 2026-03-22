# Fix 18 clippy errors in eslint_parser.rs

**Date:** 2026-03-20 08:02
**Task:** Fix all clippy warnings in eslint_parser.rs

## Goal
Clean clippy output for eslint_parser.rs — zero warnings.

## Approach
1. `doc_markdown` (5): Add backticks around `ESLint` in doc comments
2. `struct_excessive_bools` (1): Allow on `EslintConfig`
3. `string_slice` (1): line 163 — use `.get()` instead of direct slice
4. `type_complexity` (2): Add type alias for `(String, Option<u32>)`
5. `inefficient_to_owned` (3): Restructure parse_array_rule_value to avoid reassignment pattern
6. `indexing_slicing` (5+): Replace `chars[i]` with `.get(i)` patterns in extract_first_number

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/eslint_parser.rs`
