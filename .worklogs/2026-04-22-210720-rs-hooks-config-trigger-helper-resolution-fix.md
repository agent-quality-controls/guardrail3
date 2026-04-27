## Summary

Fixed a real `g3rs-hooks/hook-rs-16-config-changes-trigger-validation` false positive in Rust hook trigger coverage. The rule was reparsing matching branch text and direct trigger lines in isolation, which dropped helper-function resolution from the full shell script and incorrectly reported missing Rust config triggers.

## Decisions made

- Added red-first tests for both failing shapes:
  - direct trigger line calls helper defined elsewhere
  - matching conditional branch calls helper defined outside the branch
- Fixed the bug at the architecturally correct place:
  - `hook_rs_16_config_changes_trigger_validation` now resolves guardrail validation against the full parsed script while preserving original line numbers
  - it no longer depends on isolated branch reparsing for helper dispatch
- Reused the parser-owned command query surface instead of adding another local shell matcher:
  - added line-aware helpers in `hook_rs_08_guardrail_validate_staged_present`
- Split `support` into `support/mod.rs`, `support/logic.rs`, and `support/text.rs` to satisfy the repo's facade-only `mod.rs` rule and keep the file under the code-line cap
- Removed an obsolete path-qualified script-level helper export that became dead after the line-aware rewrite

## Key files for context

- [rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule.rs)
- [golden.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs)
- [mod.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/mod.rs)
- [logic.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/logic.rs)
- [text.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/text.rs)
- [rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/rule.rs)

## Next steps

- Continue the remaining Rust boundary audit from the next most suspicious production-path check package.
- Keep preferring full parsed-script resolution over local shell-text reparsing in hook rules.
