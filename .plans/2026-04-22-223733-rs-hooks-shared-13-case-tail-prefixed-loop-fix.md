Goal:
Fix the remaining shell-valid unconditional-exit detection gaps in `hook_shared_13_no_unconditional_exit_zero` without widening scope.

Approach:
- Add red-first regression tests for:
  - a prefixed loop opener such as `time while false; do`
  - a same-line `case ... esac` tail such as `case "$x" in a) : ;; esac && exit 0`
- Keep the fix in the rule unless parser structure is missing; the parser currently does not provide dedicated `case` tail structure, so the rule should handle it directly.
- Use the existing scope-tracking logic with minimal additions rather than replacing the whole scan.
- Run the runtime package tests and the package validate command after the fix.

Key decisions:
- `time while ...` is shell-valid and must stay scoped as a loop; the rule should recognize the prefix instead of treating it as a top-level command.
- Same-line `case` tails are also shell-valid and should be evaluated after the `esac` terminator, but only on the rule side because the parser does not expose a case-tail model.

Files to modify:
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `.worklogs/<dated>-rs-hooks-shared-13-case-tail-prefixed-loop-fix.md`
