Summary

- Repaired the `hook-shell-parser` package so the new command-query API is clean under `g3rs validate`.
- The parser now exposes a narrower options API, the visitor engine is split into parser-owned submodules, and the `api` sidecar tests stay inside the owned module boundary.

Decisions made

- Made `CommandQueryOptions` a real API type instead of a public-field bag.
  - Why: the validation finding was correct. Callers should opt into behaviors through explicit methods, not by mutating raw fields.
- Split `command_query/engine.rs` by parser concern, not by arbitrary line count.
  - Why: the file was too large because it mixed orchestration, state helpers, and wrapper traversal. Those belong to distinct parser-owned internals.
  - Rejected: suppressing the file-size rule or leaving the split inside hook rules.
- Fixed the `api` sidecar boundary by routing test helpers through `api.rs`.
  - Why: the sidecar should test the owned `api` module, not reach across to sibling module paths.

Key files for context

- `.plans/2026-04-22-085652-fix-hook-shell-parser-validation.md`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/state.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/wrappers.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api_tests/golden.rs`

Next steps

- Continue the package-boundary repair with `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hook_shell.rs`.
- Replace the local `rs/test` shell parser with the shared `hook-shell-parser` semantics where they match, and isolate any remaining justified differences.
