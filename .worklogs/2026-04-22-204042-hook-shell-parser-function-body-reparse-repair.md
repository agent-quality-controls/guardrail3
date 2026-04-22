Summary
- Repaired the remaining production-path reparsing of shell function bodies in the hook parser stack.
- `ShellFunction` now carries a parser-owned nested body snapshot, and both command traversal and hook rules consume that instead of reparsing raw `function.body` strings locally.

Decisions made
- Added `parsed_body` to `ShellFunction` in the parser types.
  - Why: the parser already owns shell structure, and nested function bodies are part of that structure.
  - Rejected: adding another helper that reparses on demand, because that would keep the same duplication under a different API.
- Kept the raw `body` string on `ShellFunction`.
  - Why: line-range logic and debugging still use the raw text.
- Rewired parser assertions to read nested executable lines from `actual.parsed_body`.
  - Why: this turns the parser test surface into proof that nested bodies are parser-owned, not reconstructed in assertions.

Key files for context
- [packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs)
- [packages/parsers/hook-shell-parser/crates/runtime/src/parser.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/runtime/src/parser.rs)
- [packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs)
- [packages/parsers/hook-shell-parser/crates/assertions/src/parser.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/assertions/src/parser.rs)
- [packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs)
- [packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_21_no_fail_open_wrappers/rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_21_no_fail_open_wrappers/rule.rs)

Next steps
- Continue the remaining Rust boundary audit from proven production-path issues only.
- The next review target is still `rs/test` file-tree and source support for any remaining check-local normalization that is not obviously rule-local analysis.
