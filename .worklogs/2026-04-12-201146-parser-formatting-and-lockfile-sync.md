# Summary

Committed the remaining parser formatting cleanup and the corresponding app lockfile sync. The parser changes are formatting-only; the lockfile change records the already-added `guardrail3-reason-policy` dependency in the app workspace.

## Decisions made

- Kept parser formatting and lockfile sync together.
  - Why: both are small residual maintenance changes and neither introduces new rule behavior.
- Did not expand scope beyond the existing modified files.
  - Why: this was cleanup of leftover worktree state, not a refactor pass.

## Key files for context

- `apps/guardrail3/Cargo.lock`
- `packages/parsers/rust-toolchain-toml-parser/crates/parser/assertions/src/parser.rs`
- `packages/parsers/rustfmt-toml-parser/crates/parser/assertions/src/parser.rs`
- `packages/parsers/rustfmt-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/rustfmt-toml-parser/src/lib.rs`

## Next steps

- After these commits, the worktree should be clean and ready for the next family audit.
