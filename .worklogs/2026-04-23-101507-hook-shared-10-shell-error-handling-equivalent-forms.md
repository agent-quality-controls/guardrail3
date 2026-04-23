Summary
- Fixed `HOOK-SHARED-10` so equivalent fail-closed `set` spellings no longer warn when they semantically enable `errexit`.
- Added red-first regressions for split short flags, reordered short flags, and the long-form `-o errexit` spelling, then verified the hook and parser runtimes plus `g3rs validate` scopes.

Decisions made
- Moved the rule from raw source-line string matching to semantic matching on executable `set` commands.
  - Why: the bug was at the rule boundary, and executable command matching is the architecturally correct surface for shell option semantics.
- Treated `-e` and `-o errexit` as equivalent enable/disable forms, while preserving sequential option semantics within one `set` command.
  - Why: those are the same shell option, and the false warnings came from spelling/order differences, not from missing parser data.
- Limited the fix to this rule and its rule-specific tests.
  - Why: the parser and shared command-query runtime were already sufficient; widening the change would add unnecessary risk.

Key files for context
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_10_shell_error_handling/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_10_shell_error_handling/rule_tests/golden.rs`
- `.plans/2026-04-23-101128-hook-shared-10-shell-error-handling-equivalent-forms.md`

Next steps
- None for this bug fix.
