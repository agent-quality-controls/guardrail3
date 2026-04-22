Summary
- Fixed hook_shared_13 so tab-delimited scoped openers like `if\t...` and `case\t...` no longer false-positive when they contain unconditional `exit 0` inside the scope.

Decisions made
- Kept the fix at the rule boundary by reusing `starts_shell_keyword()` instead of adding parser complexity.
- Expanded the existing golden tests with tab-delimited same-line `if` and `case` cases, plus a tab-delimited `case` opener regression, so the opener and same-line scope paths are both covered.
- Rejected any broader shell parsing change because the current helper already models shell keyword whitespace correctly and the bug was only in the opener checks.

Key files for context
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `.plans/2026-04-22-225236-rs-hooks-shared-13-tab-opener-fix.md`

Next steps
- None for this fix. If the tab-delimited opener path regresses again, start from `starts_shell_keyword()` and the same-line scope helper in `rule.rs`.
