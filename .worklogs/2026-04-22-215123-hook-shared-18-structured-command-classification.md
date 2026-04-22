## Summary

Fixed `RS-HOOKS-SOURCE-22` command classification so the rule uses structured `ResolvedCommand` argument shape instead of substring matching on raw shell text. Added a regression proving `cargo clippy-driver` no longer satisfies the `cargo clippy` requirement by accident.

## Decisions made

- Fixed the classifier boundary in the rule instead of adding another text-normalization layer.
  - Why: the defect was raw substring matching on executable command text, not parser coverage.
- Reused `cargo_subcommand_tail(...)` for cargo-family checks.
  - Why: that is the package-owned structured helper for command shape.
  - Rejected: more `command_text().contains(...)` variants.

## Key files for context

- `.plans/2026-04-22-214529-hook-shared-18-structured-command-classification.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/rule_tests/golden.rs`

## Next steps

- Land the `RS-HOOKS-SOURCE-15` helper-trigger fix in the same package.
- Then return to the still-pending `hook_shared_13_no_unconditional_exit_zero` loop-scope bug.
