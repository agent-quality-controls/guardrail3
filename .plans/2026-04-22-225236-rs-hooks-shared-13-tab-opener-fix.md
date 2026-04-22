## Goal

Fix `hook_shared_13` so tab-delimited `if` and `case` openers are treated as scoped control flow, not unconditional `exit 0` bypasses.

## Approach

- Add red regressions in the hook sidecar tests for `if\ttrue; then exit 0; fi` and `case\t"$x" in a) exit 0 ;; esac`.
- Update the rule's opener classification to recognize shell keywords followed by tabs as valid scope openers.
- Keep the change in the existing hook rule and its tests.
- Verify with the hooks runtime package tests and `g3rs validate` for the hooks source-checks package.

## Key decisions

- Fix the opener predicate in the rule instead of changing parser structure.
  - Why: the parser already gives the line text and the bug is in the rule's lexical classification.
- Use the existing keyword helper rather than adding a new shell parser path.
  - Why: tab handling is the same keyword-boundary problem as the existing space-delimited openers.

## Files to modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `.worklogs/2026-04-22-225236-rs-hooks-shared-13-tab-opener-fix.md`
