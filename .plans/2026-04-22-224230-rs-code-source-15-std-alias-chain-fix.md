## Goal

Fix `g3rs-code/ast-15-direct-fs-usage` so chained std alias rebinding like `use std as s; use s as t; t::fs::read_to_string(...)` is detected at the parser visitor boundary.

## Approach

- Add red regression tests in `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`.
- Extend `fs_visitors.rs` to propagate std aliases through alias chains instead of only recording aliases directly from `std`.
- Keep the change in the parser visitor layer so rule code stays simple and does not gain string-based alias hacks.
- Verify with the package test suite and the package validate command.

## Key decisions

- Fix alias propagation in the shared visitor state.
  - Why: both std-fs import and inline-call detection consume the same alias set, so the parser boundary is the right place.
- Cover both `use` alias chains and `extern crate` roots where the same alias chain root cause applies.
  - Why: both are valid Rust and both exercise the same alias-resolution bug.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`
