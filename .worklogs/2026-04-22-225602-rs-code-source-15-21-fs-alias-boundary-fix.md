## Summary

Fixed the shared `RS-CODE-SOURCE-15` and `RS-CODE-SOURCE-21` alias-handling gap so `std::fs` aliases now resolve through the parser visitor boundary. The visitor now tracks both std-root aliases and std::fs subtree aliases, which covers direct-import, call, and glob detection for chained and reversed alias forms.

## Decisions made

- Fixed alias tracking in `parse/fs_visitors.rs`.
  - Why: the bug was in alias propagation, not in the rules that consume visitor output.
- Added red regressions for direct import, call, and glob surfaces.
  - Why: each surface was affected by the same root cause and needed coverage.
- Kept the change shared across all fs checks.
  - Why: `RS-CODE-SOURCE-15` and `RS-CODE-SOURCE-21` both depend on the same visitor-derived alias state.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs`
- `.plans/2026-04-22-225602-rs-code-source-15-21-fs-alias-boundary-fix.md`

## Next steps

- None for this fix.
