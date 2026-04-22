## Goal

Fix the remaining RS-HOOKS-SOURCE-18 production-path misses in `hook_shared_13_no_unconditional_exit_zero` without widening the slice. The rule should catch:
- same-line function definition tails that end in `exit 0`
- same-line loop terminator tails that end in `exit 0`
- forward function calls that resolve to a later definition
- shell wrapper forms like `sh -c 'exit 0'` and `bash -c 'exit 0'`

## Approach

1. Add red-first regressions in the rule-specific golden tests for the four concrete shapes.
2. Tighten the rule so it stops treating every function-definition line as opaque and instead evaluates only the executable tail after the closing brace.
3. Reuse structured command traversal for raw executable lines so shell wrappers are resolved instead of substring-matched.
4. Relax function lookup so a call can resolve to a later definition when that is the only matching body in the parsed script.

## Key Decisions

- Keep the fix in the rule slice rather than the parser.
  - The parser already exposes structured executable lines and command traversal; the bug is in how the rule composes those facts.
- Prefer the parser's structured traversal over new ad hoc substring checks.
  - That preserves the recent boundary work and avoids another local shell interpreter.
- Keep the write set limited to the rule file, its tests, and the dated plan/worklog artifacts.
  - No unrelated package or parser edits.

## Files to Modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `.worklogs/2026-04-22-221236-rs-hooks-shared-13-structured-command-tail-and-forward-resolution.md`
