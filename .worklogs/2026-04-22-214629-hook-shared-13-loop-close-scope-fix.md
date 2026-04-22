Summary
- Fixed the production-path loop-scope defect in `hook_shared_13_no_unconditional_exit_zero`.
- Loop closure now keys off parser-produced executable lines, which handles real shell forms like `done <<< ...` that the old raw suffix checks missed.

Decisions made
- Kept the change inside the rule slice and its owned tests.
- Added a regression test that proves the bug on the live `run -> check` path.
- Reused parser output for loop closure instead of widening the raw shell text heuristics.
- Kept the existing loop-opening heuristic in place because the defect was in loop close recognition.

Key files for context
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`

Next steps
- Package tests passed for `hook_shared_13_no_unconditional_exit_zero`.
- `cargo run -p guardrail3-rs -- validate --path /Users/tartakovsky/Projects/websmasher/guardrail3 --family hooks` failed on a pre-existing unrelated import error in `hook_rs_16_config_changes_trigger_validation/support/text.rs`.
