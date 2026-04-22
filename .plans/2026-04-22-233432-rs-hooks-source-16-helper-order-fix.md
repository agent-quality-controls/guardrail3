Goal
- Fix helper resolution in RS-HOOKS-SOURCE-15 so config-trigger helpers are resolved by line order, not by name alone.

Approach
- Add red tests in `rule_tests/golden.rs` that prove a forward call before helper definition does not count and that a later redefinition overrides an earlier noop helper.
- Update `support/text.rs` to resolve helper bodies by the latest definition at or before the call line.
- Keep the rule criteria unchanged; only fix helper lookup order.

Key decisions
- Use line-order-aware resolution rather than loosening the trigger heuristics.
- Preserve recursive helper traversal and cycle protection.
- Cover both false negative and false positive shapes in tests.

Files to modify
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/text.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs`
