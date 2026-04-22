## Summary

Fixed `RS-CODE-SOURCE-31` so nested-module `use` bindings now count as inherent impls of the underlying struct in shared crates. The rule now resolves local `use` imports and aliases before matching impl self types, which covers both `use super::Input; impl Input` and `use super::Input as Alias; impl Alias`.

## Decisions made

- Fixed the matcher in `rs_code_ast_31_public_struct_named_fields/rule.rs` instead of touching ingestion.
  - Why: the bug is local inherent-impl identity reconstruction inside the rule.
- Broadened the regression beyond the exact alias form.
  - Why: the direct imported-name form is the same rebinding class and would have stayed broken without coverage.
- Kept the change scoped to the rule file and its shared tests plus the required plan/worklog.
  - Why: the fix belongs at the rule boundary and did not require broader package changes.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.plans/2026-04-22-222256-rs-code-public-struct-use-alias-fix.md`

## Next steps

- None for this fix.
