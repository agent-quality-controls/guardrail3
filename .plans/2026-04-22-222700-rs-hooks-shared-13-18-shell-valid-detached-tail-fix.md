## Goal

Fix the remaining shell-valid production-path misses in `packages/rs/hooks/g3rs-hooks-source-checks` without widening the slice:
- `hook_shared_13_no_unconditional_exit_zero` must treat same-line control tails like `fi && exit 0` and `done && exit 0` as scope closures followed by an executable `exit 0`
- `hook_shared_18_executable_command_context_only` must not false-positive on detached or piped executable commands such as `cargo test --workspace &` and `cargo test --workspace | tee /tmp/log`

## Approach

1. Add red-first regressions in the rule-specific golden tests for the exact shell-valid cases.
2. Fix `hook_shared_13` at the scope-boundary layer so the rule recognizes shell-valid same-line closures instead of skipping the line entirely.
3. Fix `hook_shared_18` by using the parser's detached-aware command traversal rather than default traversal.
4. Keep the changes local to the two rule files and their tests.

## Key Decisions

- Do not expand parser behavior.
  - The parser/query layer already has the needed traversal; the bug is in how the hook rules consume it.
- Treat shell-valid detached and piped commands as executable.
  - They are not inert text just because they are backgrounded or piped.
- Keep the hook_shared_13 fix scoped to shell keyword closure handling.
  - The remaining bug is the scope heuristic, not the executable-line detection path.

## Files to Modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule_tests/golden.rs`
- `.worklogs/2026-04-22-222700-rs-hooks-shared-13-18-shell-valid-detached-tail-fix.md`
