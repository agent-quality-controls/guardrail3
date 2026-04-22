## Summary

Fixed the shared shell-inline parsing boundary so escaped-space-before-hash no longer becomes a comment and function-tail detection no longer misreads nested shell syntax containing `}` inside command substitutions or parameter expansions. The parser runtime and hook-side inline comment helper now agree on escaped whitespace before `#`, and the parser can still find inline commands after function definitions with nested shell syntax in the body tail.

## Decisions made

- Fixed both bugs at the parser/support boundary.
  - Why: the hook rules were already consuming the shared shell parser output; the wrong-result lived in shared parsing, not in rule-specific logic.
- Added a hook-side regression for the escaped-space comment path.
  - Why: that surface is the user-facing consumer of the inline-comment helper.
- Kept the function-tail fix in the parser support scanner instead of adding a special-case tail heuristic.
  - Why: the tail scan needs to ignore nested shell syntax, not just a single delimiter pattern.

## Key files for context

- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/support.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_14_no_bypass_instructions/rule_tests/golden.rs`
- `.plans/2026-04-22-233539-hook-shell-parser-inline-boundary-fix.md`

## Next steps

- None for this fix.
