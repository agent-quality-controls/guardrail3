Summary
- Fixed the shared shell wrapper parser so short flag clusters containing `c` now consume the next token as the shell command string.
- Added a regression proving `bash -ceu 'g3rs validate --path .'` resolves `g3rs` instead of dropping the script.

Decisions made
- Kept the fix in `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/wrappers.rs`.
  - The bug was in wrapper token handling, not in downstream command resolution.
- Removed the old inline-suffix path for shell clusters.
  - It misparsed valid clustered forms like `-ceu` by treating trailing letters as the command string.

Key files for context
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/wrappers.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api_tests/golden.rs`

Next steps
- None for the parser fix.
