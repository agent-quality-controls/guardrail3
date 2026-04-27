## Summary

Fixed the rs/code source-checks std alias scope bug by restoring alias state when leaving a module visit in `parse/fs_visitors.rs`. Added sibling-module regression coverage to the existing `g3rs-code/ast-15-direct-fs-usage` and `g3rs-code/ast-21-fs-glob-import` rule-test suites.

## Decisions made

- Kept the production fix at the parser visitor boundary, where alias state is owned.
- Moved the regression into the package's existing rule-test suites instead of leaving a new inline test module in `src/`, because `g3rs validate` rejected that shape.
- Covered both direct `s::fs::*` calls and `s::fs::*` glob imports because the same alias leak affected both visitors.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/false_positives.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/false_positives.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/mod.rs`

## Next steps

- Keep future parser visitors module-scoped when they accumulate per-module state.
- If this bug class reappears, add the regression at the rule-test layer rather than introducing ad hoc runtime test sidecars.
