Goal:
Fix escaped hash handling in the hook shell parser boundary so `\#` outside quotes stays executable text instead of starting an inline comment.

Approach:
- Add red-first regressions in the parser golden tests for an executable line containing `\#` and for a hook-side comment scan that should ignore escaped hash text.
- Patch the shared inline-comment stripping logic in the parser runtime and the matching hook-support helper so comment detection respects escaped `#` outside quotes.
- If needed, adjust the command-query lexer path in the parser runtime to use the same escape-aware behavior.
- Run the parser package tests and the hooks package tests, then validate the touched package paths with `g3rs validate`.

Key decisions:
- Fix at the parser/support boundary, not in individual hook rules.
- Keep the behavior shell-valid: escaped `#` should remain part of the executable command text.

Files to modify:
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/lex.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/support.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_14_no_bypass_instructions/rule_tests/golden.rs`
- `.worklogs/<dated>-hook-shell-escaped-hash-parser-fix.md`
