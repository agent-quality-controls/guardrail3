## Summary

Fixed `g3rs-hooks/hook-shared-13-no-unconditional-exit-zero` so same-line scoped control-flow forms like `if true; then exit 0; fi` are no longer misclassified as unconditional `exit 0` bypasses. The rule now skips pure same-line `if` and `case` scope lines before top-level exit scanning, while keeping tail forms like `fi && exit 0` and `esac && exit 0` visible.

## Decisions made

- Fixed the scope check in the rule rather than changing the shell parser.
  - Why: the parser already exposes the needed line text; the bug was in the rule's top-level scan guard.
- Added red-first coverage for the concrete failing `if` shape and kept pure same-line `case` coverage in the same sidecar.
  - Why: the `if` line was the actual false positive, and the `case` shape stays covered as a sibling control-flow form.
- Kept the change minimal and local to the hook source-check package.
  - Why: the regression is at rule classification time, not in package wiring.

## Key files for context

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `.plans/2026-04-22-224228-rs-hooks-shared-13-same-line-scope-fix.md`

## Next steps

- None for this fix.
