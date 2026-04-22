Summary
- Fixed RS-HOOKS-SOURCE-15 helper resolution so helper bodies are resolved by definition order, not name only.
- Added regression tests for a forward call before helper definition and for later redefinition overriding an earlier noop.

Decisions made
- Kept the fix at the helper-resolution boundary in `support/text.rs` instead of loosening the rule.
- Chose the latest matching function at or before the call line so shell definition order is respected.
- Added red tests first to pin the two surviving failure modes before changing production logic.

Key files for context
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/text.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs`
- `.plans/2026-04-22-233432-rs-hooks-source-16-helper-order-fix.md`

Next steps
- Keep the worktree isolated to these files if more follow-up is needed.
- Re-run the package test suite and `g3rs validate` after any later edits to the same helper-resolution path.
