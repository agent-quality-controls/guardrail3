Goal
- Fix the hook-shell-parser function-tail split so braces in comments or quoted strings after a real function close do not move the tail boundary.

Approach
- Add parser-level red tests in `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs` that prove the current tail extraction breaks on `}` inside a comment and inside a quoted string.
- Fix the split at the parser support boundary in `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs` by scanning the function body tail with quote/comment awareness instead of using `rsplit_once('}')`.
- Keep the change minimal and avoid touching hook rules.

Key decisions
- Use parser tests rather than hook tests so the bug is proven at the source of truth.
- Treat the real function close as the first top-level `}` after the opening brace, ignoring `}` inside quotes and after `#` comments.
- Preserve existing behavior for normal single-line function definitions and inline tails.

Files to modify
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`
