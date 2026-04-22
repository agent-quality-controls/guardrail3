Goal
- Fix the two surviving `hook_shared_13_no_unconditional_exit_zero` shell-valid bypasses after the latest fixes.

Approach
- Add red tests first for:
  - same-line opener+closer+suffix scope leakage, covering a later top-level `exit 0`
  - function-definition tail extraction when a later `}` appears in a comment or string
- Update `rule.rs` at the rule boundary with minimal complexity:
  - extract function tails using a quote/comment-aware closer scan instead of `rsplit_once('}')`
  - treat same-line control-flow opener+closer+suffix forms as closed on the same line and scan only the suffix
- Run the hooks package tests and `g3rs validate` after the patch.

Key decisions
- Keep the fix local to `hook_shared_13_no_unconditional_exit_zero` instead of broadening parser behavior.
- Prefer a small tail-scanning helper over parsing more of shell syntax than this rule needs.
- Preserve the existing behavior for same-line control-flow bodies that do not leak to later top-level lines.

Files to modify
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
