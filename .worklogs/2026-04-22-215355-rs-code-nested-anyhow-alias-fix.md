## Summary

Fixed `RS-CODE-SOURCE-33` so nested public modules no longer hide `anyhow` aliases from weak-public-error detection. The public-surface visitor now carries `anyhow` alias bindings by module scope instead of using only one file-root binding bag.

## Decisions made

- Moved the fix into `parse/attrs/public_surface.rs`.
  - Why: that visitor already owns module visibility and public-surface traversal.
  - Rejected: patching `analysis_helpers.rs` in isolation, because it cannot know nested import scope.
- Merged parent and nested-module `anyhow` bindings.
  - Why: nested modules can use outer aliases and add their own.

## Key files for context

- `.plans/2026-04-22-215251-rs-code-nested-anyhow-alias-fix.md`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_33_public_weak_error_forms/rule_tests/direct.rs`

## Next steps

- Finish the remaining queued `rs/hooks` and `rs/code` bug fixes from the parallel wave.
- Specifically, review the still-open worker result for `hook_rs_16` and close out any remaining uncommitted worker state.
