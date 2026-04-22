## Summary

Fixed the `hook_rs_16_config_changes_trigger_validation` config-trigger check so it resolves helper-defined trigger coverage from parsed shell structure instead of rebuilding it from raw shell text. Added a regression proving a helper branch that triggers config validation now counts as covered.

## Decisions made

- Moved trigger reachability into the structured shell parse path.
  - Why: the defect was caused by ad hoc text expansion missing helper-defined coverage.
  - Rejected: adding more raw-text cases for helper calls or branch syntax.
- Kept the fix inside the rule slice and its tests.
  - Why: the bug was local to this hook runtime and its golden cases.

## Key files for context

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/logic.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/text.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs`

## Next steps

- None for this fix.
