Summary
- Fixed `g3rs-hooks/hook-rs-16-config-changes-trigger-validation` so config-trigger coverage respects source order instead of treating trigger and validation as interchangeable co-presence.
- Added a regression proving validation before the trigger check in one branch stays a warning.

Decisions made
- Kept the fix in `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/logic.rs`.
  - The bug was in branch gating logic, not in the parser or assertion layer.
- Preserved the existing trigger discard filter while making the branch check order-aware.
  - That keeps the current false-positive guards while closing the path-order hole.

Key files for context
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/logic.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs`

Next steps
- None for the hook fix.
