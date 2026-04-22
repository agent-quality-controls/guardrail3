## Goal

Fix `RS-CODE-SOURCE-33` so nested public modules that alias `anyhow` still trigger weak-public-error detection for `Result<_, anyhow::Error>` forms.

## Approach

- Add a red unit test proving nested-module aliases like `use anyhow as ah;` and `use anyhow::Error as AppError;` are currently missed.
- Move `anyhow` alias binding to the public-surface visitor's module scope instead of one file-root bag.
- Keep the change limited to the `public_surface` parser slice and the rule test that proves the bug.
- Verify with the full `rs/code` source-checks package and `g3rs validate`.

## Key decisions

- Fix this in `parse/attrs/public_surface.rs`.
  - Why: that is where module reachability and public-surface traversal already live.
  - Rejected: patching `analysis_helpers.rs` alone, because it has no module-scope binding context.
- Merge parent and nested-module `anyhow` bindings.
  - Why: nested modules inherit outer imports and can add their own aliases.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_33_public_weak_error_forms/rule_tests/direct.rs`
- `.worklogs/<timestamp>-rs-code-nested-anyhow-alias-fix.md`
