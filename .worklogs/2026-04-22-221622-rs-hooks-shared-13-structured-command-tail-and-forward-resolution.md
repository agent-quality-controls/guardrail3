## Summary

Fixed `hook_shared_13` so unconditional `exit 0` is still caught when it appears after a same-line function definition, after a same-line loop terminator, or inside `sh -c` / `bash -c` wrappers. Corrected function resolution to use the latest definition that exists before the call, and replaced the invalid forward-definition test with a real redefinition case.

## Decisions made

- Used `hook_shell_parser::command_query::any_resolved_command_on_line(...)` for tail and wrapper detection.
  - Why: wrapper descent and same-line segment handling already belong to the parser command-query layer.
- Kept function-body skipping for ordinary lines, but carved out function-definition-line tails.
  - Why: the bug was not the body itself; it was that the rule skipped executable segments that share the definition line after `}`.
- Rejected forward function resolution.
  - Why: shell execution does not make a later definition available to an earlier top-level call. The right model is the latest definition whose line is at or before the call site.

## Key files for context

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `.plans/2026-04-22-221236-rs-hooks-shared-13-structured-command-tail-and-forward-resolution.md`

## Next steps

- Continue attacking `hooks` for any remaining command-context false negatives, especially around shell syntax that can hide later executable segments.
- Keep shell execution semantics owned by the shared parser/query layer wherever possible.
