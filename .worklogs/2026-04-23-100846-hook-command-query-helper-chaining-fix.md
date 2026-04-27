Summary
- Fixed shared hook command-query helper resolution so nested helper chains can reach root-scope helpers using the visible call-site line, and later helper redefinitions win over earlier definitions.
- Landed the dependent hook regressions for config-trigger matching, fail-open wrapper detection, and unconditional `exit 0` detection.

Decisions made
- Kept the main semantic fix in the shared shell command-query runtime.
  - Why: helper chaining and redefinition are parser/runtime concerns, and multiple hook rules depend on the same traversal semantics.
- Added a context-aware line query API instead of teaching each hook rule to re-implement shared command traversal.
  - Why: nested helper-body checks in hook rules need access to both the local parsed body and the root helper inventory.
- Kept `g3rs-hooks/hook-shared-10-shell-error-handling` trigger recursion in hook support, but aligned it to the same "latest visible definition" rule.
  - Why: that rule matches trigger-like text, not just resolved commands, so it still owns its text-side recursion.
- Fixed shell-safety absolute line reporting at the same time.
  - Why: once nested helper chains can jump back to root helpers, line mapping must distinguish local-function vs root-function lookups or reports drift.

Key files for context
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/wrappers.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/text.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_21_no_fail_open_wrappers/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `.plans/2026-04-23-095618-hook-command-query-helper-chaining-fix.md`

Next steps
- If later hook rules need nested helper-body command checks, reuse `any_resolved_command_on_line_in_context` instead of adding more ad hoc root/local traversal code.
- If command-query callers ever need absolute nested command line numbers from the shared runtime, add that at the parser API boundary rather than rebuilding offsets in each rule.
