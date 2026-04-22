Summary
- Fixed the hook-shell-parser function-tail split so braces in trailing comments or quoted strings do not move the function closer.
- Added parser regressions for `finish() { echo ok; }; exit 0 # }` and `finish() { echo ok; }; exit 0; echo "}"`, then verified the runtime package and repo validator pass for `packages/parsers/hook-shell-parser`.

Decisions made
- Fixed the bug at the parser support boundary instead of patching hook rules.
- Replaced the reverse brace split with a quote/comment-aware forward scan that finds the matching function closer.
- Kept the change narrow to the existing parser support helpers and parser golden tests.

Key files for context
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser.rs`

Next steps
- None for this fix. If similar tail-splitting bugs appear again, check other parser helpers that still use reverse delimiter scans.
