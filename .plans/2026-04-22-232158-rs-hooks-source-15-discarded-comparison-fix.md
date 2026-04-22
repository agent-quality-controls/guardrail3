## Goal
Fix RS-HOOKS-SOURCE-15 so a discarded trigger-like comparison line inside a branch does not count as guarded config-change coverage.

## Approach
1. Add a red regression in `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs` for a branch that runs a trigger-like comparison and validation on the same line with the comparison result discarded.
2. Tighten the shared hook support logic in `support/logic.rs` so config-trigger coverage only counts when the trigger actually gates validation, not when it is merely present in the same branch.
3. Run the hook package tests and `g3rs validate --path packages/rs/hooks/g3rs-hooks-source-checks`.
4. Write a worklog and commit the fix as a standalone bug fix.

## Key decisions
- Fix the support logic, not the rule text.
  - Reason: the bug is a coverage boundary problem in trigger/validation interpretation.
- Keep the regression as a valid shell shape.
  - Reason: the current parser should already accept the script, so this is a real production-path check miss.

## Files to modify
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/logic.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/text.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs`
