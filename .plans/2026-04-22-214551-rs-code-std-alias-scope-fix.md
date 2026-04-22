## Goal

Fix the `rs/code` source-check parser so `std` alias bindings do not leak across sibling modules.

## Approach

- Add a red-first unit test in `parse/fs_visitors.rs` that proves a `use std as s;` binding in one sibling module does not authorize `s::fs` in another sibling module.
- Scope the alias set to each module visit in `StdFsImportVisitor` and `StdFsGlobImportVisitor` by snapshotting and restoring the alias set around `visit_item_mod`.
- Keep the change inside `packages/rs/code/g3rs-code-source-checks/crates/runtime`.
- Run the package tests and `g3rs validate` for the touched package.

## Key decisions

- Fix at the parser visitor boundary, not in the rule layer.
- Use a focused regression test on the visitor helpers so the alias leak is proven at the exact layer that owns the state.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs` test module added in place
