Goal
- Remove production-path reparsing of shell function bodies from hook rules and hook-shell-parser command traversal.
- Make parsed function bodies part of the parser-owned contract so downstream code consumes parser structure instead of reparsing raw strings.

Approach
- Extend `hook-shell-parser` function types to carry a parsed body snapshot alongside the raw body text.
- Add a parser test that proves a parsed function body exposes nested executable lines without reparsing in assertions.
- Rewire parser assertions and command-query traversal to use the stored parsed body.
- Rewire `RS-HOOKS-SOURCE-18` and `RS-HOOKS-SOURCE-24` to recurse through parser-owned nested bodies instead of calling `parse_script(&function.body)` locally.
- Verify parser and hooks packages, then write worklog and commit as a stand-alone bug fix.

Key decisions
- Keep the raw `body` text on `ShellFunction`.
  - Why: existing line-offset logic and debugging output still use it.
  - Alternative rejected: replace raw body with parsed-only state, which would force unrelated callers to reconstruct text context.
- Store nested parsed bodies on `ShellFunction` rather than adding a second helper API.
  - Why: this fixes the root duplication once, and both assertions and command traversal can consume the same owned structure.

Files to modify
- packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs
- packages/parsers/hook-shell-parser/crates/runtime/src/parser.rs
- packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs
- packages/parsers/hook-shell-parser/crates/assertions/src/parser.rs
- packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_21_no_fail_open_wrappers/rule.rs
