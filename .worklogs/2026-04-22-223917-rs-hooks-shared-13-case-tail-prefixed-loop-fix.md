Summary:
Fixed the remaining shell-valid unconditional-exit detection gaps in `hook_shared_13`: prefixed loop openers like `time while false; do` now stay scoped as loops, and same-line `case ... esac` tails now evaluate the trailing executable path instead of being skipped.

Decisions made:
- Kept the prefixed-loop fix in the rule with a small lexical helper instead of changing parser structure.
- Kept the same-line `case` fix in the rule because the parser does not expose dedicated case-tail structure.
- Added red-first regressions before changing behavior to prove both failures on current code.

Key files for context:
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
- `packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs`
- `.plans/2026-04-22-223733-rs-hooks-shared-13-case-tail-prefixed-loop-fix.md`

Next steps:
- If more shell-structure regressions appear, attack them with the same red-first flow and keep the fix in the narrowest boundary that owns the behavior.
