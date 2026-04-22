## Goal

Fix `hook_shared_13` so same-line scoped control-flow forms like `if true; then exit 0; fi` and `case "$x" in a) exit 0 ;; esac` are treated as scoped, not unconditional bypasses.

## Approach

- Add red regressions in the hook sidecar tests for same-line `if` and `case` forms that contain `exit 0` inside the scoped body.
- Adjust the rule's scope-open classification so same-line open/close forms do not get scanned as top-level executable lines.
- Keep the fix in the existing hook rule and its tests.
- Verify with the runtime package tests and `g3rs validate` for the hooks source-checks package.

## Key decisions

- Fix the rule's scope classifier instead of adding a parser pass.
  - Why: the parser already exposes the line text and executable commands; the bug is in how the rule classifies same-line scope boundaries.
- Treat same-line `if` and `case` as scoped control flow even when the close keyword appears on the same line.
  - Why: the unconditional-exit rule should not inspect exits that are syntactically guarded by the branch body on that same line.

## Files to modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `.worklogs/2026-04-22-224228-rs-hooks-shared-13-same-line-scope-fix.md`
