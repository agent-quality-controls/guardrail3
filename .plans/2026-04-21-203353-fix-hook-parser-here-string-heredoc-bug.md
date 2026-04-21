## Goal

Fix the shared hook shell parser so bash here-strings (`<<<`) do not enter heredoc mode and swallow the rest of the script. The parser must keep recognizing later executable commands in real hooks like `websmasher/.githooks/pre-commit`.

## Approach

1. Add parser regression coverage in `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`.
   - Add a test proving a `while ... done <<< "$VAR"` loop does not truncate later commands.
   - Keep the proof at parser level because the bug is in shared shell parsing, not in hook-family rules.
2. Fix heredoc detection in `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`.
   - Change the heredoc marker detection so `<<` and `<<-` still count, but `<<<` does not.
   - Keep the fix local to heredoc classification rather than adding special cases elsewhere in parsing.
3. Verify at the parser package and then verify against the hooks family output on the real `websmasher` hook target.

## Key decisions

- Fix in `hook-shell-parser`, not in `g3rs-hooks-source-checks`.
  - Why: the same parser false-negative affects multiple hook rules at once.
  - Rejected: patching individual hook rules to recognize cargo in loop bodies.

- Use a minimal heredoc detection correction.
  - Why: the bug is not "loops" or "done <<<" generally. The bug is specifically classifying `<<<` as a heredoc opener.
  - Rejected: broad loop-state rewrites or downstream command-query workarounds.

## Files to modify

- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`
- `.worklogs/2026-04-21-<timestamp>-fix-hook-parser-here-string-heredoc-bug.md`
