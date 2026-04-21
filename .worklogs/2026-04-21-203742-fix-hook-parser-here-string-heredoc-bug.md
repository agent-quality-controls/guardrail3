## Summary

Fixed the shared hook shell parser so bash here-strings (`<<<`) no longer enter heredoc mode and truncate the rest of the script. Added a regression proving that commands after a `done <<< "$VAR"` loop stay visible to downstream hook rules.

## Decisions made

- Fixed the bug in `hook-shell-parser` heredoc detection.
  - Why: the same parser failure was causing false negatives across multiple hook rules at once.
  - Rejected: patching `g3rs-hooks-source-checks` rules individually.

- Added parser-level proof for the real failure shape.
  - The regression uses a live loop body plus `done <<< "$STAGED_FILES"` and then asserts that later commands are still parsed.
  - Why: this is the smallest architectural place that proves the broad bug.

- Narrowed the fix to heredoc marker classification only.
  - `<<` and `<<-` still count as heredoc starts.
  - `<<<` is now explicitly excluded.
  - Rejected: broader parser control-flow rewrites, because the bug was not in loop handling.

## Key files for context

- `.plans/2026-04-21-203353-fix-hook-parser-here-string-heredoc-bug.md`
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`

## Verification

- `cargo test -q --manifest-path packages/parsers/hook-shell-parser/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/parsers/hook-shell-parser/Cargo.toml`
- `g3rs validate --path packages/parsers/hook-shell-parser`

## Broader scope checked

- Reproduced the failure on the real `websmasher/.githooks/pre-commit` before the fix.
  - The parser stopped at the first `done <<< "$STAGED_FILES"` and exposed no later cargo commands.
- Re-ran the parser directly on the same file after the fix.
  - Later cargo commands became visible again, including `cargo fmt`, `cargo clippy`, `cargo deny`, `cargo machete`, and `cargo test`.

## Next steps

- The next hook-family bug is downstream rule coverage and output cleanup, not here-string parsing.
- If another parser issue appears in hooks, start with `hook-shell-parser` before touching family rules.
