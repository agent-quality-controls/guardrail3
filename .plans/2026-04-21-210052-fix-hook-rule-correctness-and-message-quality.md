## Goal

Make the currently firing Rust hook findings on `websmasher` both correct and actionable. Fix the false-positive/false-negative rules first, then rewrite the remaining hook messages so they identify the real hook file, the concrete command shape to add or change, where it belongs, and why.

## Approach

1. Fix `RS-HOOKS-SOURCE-10` in `hook_rs_09_clippy_denies_warnings`.
   - Add a reproducing test for the real wrapped clippy command shape from `websmasher`.
   - Fix the rule at the evaluator boundary so it uses the parsed executable command surface correctly instead of missing the already-extracted cargo command.
2. Fix `RS-HOOKS-SOURCE-18` in `hook_shared_13_no_unconditional_exit_zero`.
   - Add a reproducing test for the `if [ -z "$STAGED_FILES" ]; then ... exit 0 ... fi` no-op branch.
   - Narrow the rule so it does not flag that standard no-staged-files early-exit pattern as a failure-masking bypass.
3. Rewrite the user-facing messages for the hook rules that currently fire on `websmasher`.
   - Rules:
     - `RS-HOOKS-SOURCE-08`
     - `RS-HOOKS-SOURCE-09`
     - `RS-HOOKS-SOURCE-10`
     - `RS-HOOKS-SOURCE-13`
     - `RS-HOOKS-SOURCE-14`
     - `RS-HOOKS-SOURCE-15`
     - `RS-HOOKS-SOURCE-16`
     - `RS-HOOKS-SOURCE-18`
     - `RS-HOOKS-SOURCE-20`
     - `RS-HOOKS-SOURCE-23`
     - `RS-HOOKS-SOURCE-25`
   - Make the messages concrete:
     - name `.githooks/pre-commit`
     - name the exact command or config to add/change
     - point to the actual hook block when the rule is about the Rust per-root checks block
     - explain why the command exists
4. Pin the rewritten messages in assertion helpers or direct rule tests so they do not drift back into abstract phrasing.
5. Re-run the hook source checks package tests, formatting, `g3rs validate` on the package, and a live `g3rs validate --family hooks` against `websmasher` from current source.

## Key decisions

- Fix correctness before rewriting text.
  - Why: a clearer false positive is still a false positive.
  - Rejected: message-only edits that leave `RS-HOOKS-SOURCE-10` and `RS-HOOKS-SOURCE-18` wrong.

- Keep the message fixes local to the hook rules and hook assertion helpers.
  - Why: the problem is in rule phrasing, not in the shared result formatter.
  - Rejected: formatting-layer hacks that cannot express rule-specific commands and reasons.

- Pin messages in tests.
  - Why: the whole bug here is user-facing output quality and specificity.
  - Rejected: relying on manual review of output text.

## Files to modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- The currently firing hook rule files and matching assertion helpers under `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/**/rule.rs` and `crates/assertions/src/**/rule.rs`
- `.worklogs/2026-04-21-<timestamp>-fix-hook-rule-correctness-and-message-quality.md`
