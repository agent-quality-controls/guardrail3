Summary:
Fixed two concrete shell-valid production-path gaps in `g3rs-hooks-source-checks`: `hook_shared_13` now recognizes same-line `fi && exit 0` / `done && exit 0` tails, and `hook_shared_18` now walks relaxed resolved commands so detached and piped executable commands are still seen.

Decisions made:
- Kept the `hook_shared_13` fix inside the existing scope-closure helpers instead of adding a separate parser pass.
- Switched `hook_shared_18` from strict resolved-command traversal to relaxed traversal, which matches the shell-valid detached/pipeline cases without changing the rule predicate.
- Added red-first regression tests for each fixed shape before patching behavior.

Key files for context:
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule_tests/golden.rs`
- `.plans/2026-04-22-222700-rs-hooks-shared-13-18-shell-valid-detached-tail-fix.md`

Next steps:
- If more hook regressions are needed, attack the remaining shell-safety slices with the same red-first approach.
- Re-run package validation after any further hook-source edits.
