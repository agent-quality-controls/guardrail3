Summary
- Fixed two surviving `hook_shared_13_no_unconditional_exit_zero` bypasses: same-line opener+closer+suffix scope leakage and function-definition tails hidden by later `}` characters in comments or strings.

Decisions made
- Kept the fix local to the rule instead of broadening the parser.
- Added red tests first for both shell-valid bypasses, including separate coverage for `if` and `while`.
- Replaced `rsplit_once('}')` with a quote/comment-aware brace scan so the function tail is found at the actual closing brace.
- Added same-line control-flow suffix scanning so `fi && ...`, `esac && ...`, and `done && ...` do not leak scope to later lines.

Key files for context
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`

Next steps
- Re-run the hooks validation path if the unrelated in-flight workspace edits are cleared.
- Keep the worktree changes isolated to the hook rule, its sidecar tests, and this worklog.
