# Summary
Fixed `g3rs-hooks/hook-shared-18-executable-command-context-only` so executable-command classification no longer accepts lookalike commands through `command_text().contains(...)`. The rule now uses structured command context for the supported families, and the regression test covers a `cargo clippy-driver` false negative.

# Decisions made
- Kept the fix inside the rule slice and its sidecar tests.
- Replaced raw substring classification with structured command-name/argument checks.
- Reused the shared cargo subcommand helper for cargo-based families so real flag-prefixed invocations still resolve correctly.

# Key files for context
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/support.rs`

# Next steps
- None for this bug fix.
