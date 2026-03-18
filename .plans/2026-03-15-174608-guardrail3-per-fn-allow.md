# Fix test modules to use per-function #[allow] instead of per-module #![allow]

**Date:** 2026-03-15 17:46
**Task:** Replace module-level `#![allow(...)]` in test modules with per-function `#[allow(...)]` attributes

## Goal
All test modules use per-function `#[allow]` instead of `#![allow]` so the pre-commit hook passes.

## Approach
For each of the 6 files, remove `#![allow(...)]` lines from the test module and add the specific `#[allow(...)]` attributes each test function actually needs.

## Files to Modify
- `src/discover.rs` — 5 module allows, 2 test fns
- `src/rs/validate/allow_checks.rs` — 4 module allows, 10 test fns
- `src/rs/validate/code_quality_checks.rs` — 3 module allows, 7 test fns
- `src/rs/validate/deny_inventory.rs` — 3 module allows, 5 test fns
- `src/rs/validate/source_scan.rs` — 3 module allows, 14 test fns
- `src/rs/validate/structure_checks.rs` — 3 module allows, 4 test fns
