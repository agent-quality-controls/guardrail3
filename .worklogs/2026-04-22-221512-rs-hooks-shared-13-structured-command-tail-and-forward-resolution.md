## Summary

Fixed the remaining RS-HOOKS-SOURCE-18 misses in `hook_shared_13_no_unconditional_exit_zero`: same-line function-definition tails, same-line loop-terminator tails, forward function calls, and shell wrapper execution paths. Added red-first regressions in the rule-specific golden tests and kept the change inside the hook source-check slice plus plan/worklog artifacts.

## Decisions made

- Kept the fix in `rule.rs` instead of changing the parser.
  - Why: the parser already exposes structured executable lines and command traversal; the bug was in how the rule consumed those facts.
- Used structured command traversal for raw-line wrapper detection and tail inspection.
  - Why: `sh -c 'exit 0'` / `bash -c 'exit 0'` should be resolved from command context, not substring matching.
- Relaxed function lookup to allow later definitions.
  - Why: the forward-call regression is a real production miss and the rule needed to resolve the body even when the definition appears later in the file.
- Preserved existing line attribution for normal function calls by checking function resolution before the raw-line fallback.
  - Why: it keeps the prior body-line reporting for called functions while still catching wrapper and tail cases.

## Key files for context

- `.plans/2026-04-22-221236-rs-hooks-shared-13-structured-command-tail-and-forward-resolution.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`

## Next steps

- None for this fix.
