Summary
- Fixed `g3rs-hooks/hook-rs-16-config-changes-trigger-validation` so a discarded trigger-like comparison inside a branch no longer counts as config-change gating.
- Helper-dispatched trigger logic still counts when the branch actually routes through a helper that performs the real guard and validation.

Decisions made
- Tightened the branch trigger check in support logic instead of weakening the rule or hardcoding one shell shape.
- Kept helper-based trigger resolution intact by distinguishing branch-header gating and helper-dispatched triggers from discarded inline comparisons.

Key files for context
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/logic.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs
- .plans/2026-04-22-232158-rs-hooks-source-15-discarded-comparison-fix.md

Next steps
- Run a fresh hooks/parser attack pass against the current tree.
- Review the remaining `rs/code` attack findings about forward alias order and top-level `#[cfg(test)]` alias leakage.
