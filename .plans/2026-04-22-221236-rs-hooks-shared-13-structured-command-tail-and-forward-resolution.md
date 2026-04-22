## Goal

Fix `hook_shared_13` so unconditional `exit 0` is still detected when it is hidden behind same-line shell tails or shell wrappers, while keeping function resolution aligned with actual shell execution order.

## Approach

1. Add red tests for:
   - same-line function definition followed by `; exit 0`
   - same-line loop terminator followed by `; exit 0`
   - shell wrapper forms like `sh -c 'exit 0'` and `bash -c 'exit 0'`
   - later function redefinition overriding an earlier safe definition
2. Use the parser command-query layer to inspect resolved commands on a specific line so wrappers and same-line segments are handled by shared shell semantics instead of ad hoc line parsing.
3. Resolve function calls to the latest definition whose line is at or before the call site.
4. Run the hooks source-checks package tests and `g3rs validate` for the touched package.

## Key decisions

- Reused `hook_shell_parser::command_query::any_resolved_command_on_line(...)`.
  - Why: wrapper expansion and segment splitting already belong to the parser/query layer.
- Rejected forward function resolution.
  - Why: a later top-level function definition does not make an earlier top-level call valid shell execution.
- Kept the fix local to `hook_shared_13`.
  - Why: the bug was in this rule's command visibility and function resolution logic, not in the parser itself.

## Files to modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `.worklogs/2026-04-22-*.md`
