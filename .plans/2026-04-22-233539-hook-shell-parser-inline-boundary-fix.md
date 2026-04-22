## Goal
Fix the shared shell inline parsing boundary so escaped-space-before-hash does not become a comment and function-tail detection ignores `}` inside command substitutions and parameter expansions.

## Approach
1. Add red regressions in `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs` for escaped-space-before-hash and for a function definition whose tail contains nested shell syntax with `}`.
2. Add the hook-side regression in `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_14_no_bypass_instructions/rule_tests/golden.rs` for escaped-space-before-hash bypass text.
3. Fix the parser boundary helpers in `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs` and mirror the escaped-comment behavior in `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/support.rs`.
4. Run the parser package tests, hooks package tests, both `g3rs validate` commands, then write a worklog and commit the fix as a standalone bug fix.

## Key decisions
- Fix the parser/support boundary rather than patching individual hook rules.
  - Reason: both bugs come from the same shell surface interpretation.
- Keep the function-tail fix shell-aware instead of using delimiter substring heuristics.
  - Reason: the current closer scan needs to ignore nested shell syntax, not just a single brace case.

## Files to modify
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/support.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_14_no_bypass_instructions/rule_tests/golden.rs`
