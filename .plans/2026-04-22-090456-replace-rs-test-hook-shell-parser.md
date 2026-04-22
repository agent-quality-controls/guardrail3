## Goal

Remove the duplicate shell parser from `packages/rs/test/g3rs-test-ingestion` and replace its mutation-hook detection with the shared `hook-shell-parser` command-query API.

The repaired state is:

- `rs/test` no longer owns `hook_shell.rs`
- mutation-hook detection is delegated to `hook-shell-parser`
- behavior is covered by tests, including function-dispatched hook commands

## Approach

1. Add tests around `hooks.rs` that prove mutation-hook detection should work through shell shapes the local parser misses.
   - minimum case: function-dispatched `cargo mutants`
   - keep the existing direct-command cases green
2. Add `hook-shell-parser` as a dependency of `g3rs-test-ingestion-runtime`.
3. Replace `hook_shell.rs` usage in `hooks.rs` with shared parser queries:
   - parse hook script with `hook_shell_parser::parse_script`
   - detect `cargo mutants` through resolved commands
4. Delete `hook_shell.rs` and remove its module wiring.
5. Re-run:
   - `g3rs-test-ingestion` tests
   - `g3rs validate --path packages/rs/test/g3rs-test-ingestion`

## Key decisions

- Treat this as a real bug, not just boundary cleanup.
  - Reason: function-dispatched hook commands are a valid shell shape, and the current local parser can miss them.
- Replace the local parser entirely instead of wrapping it.
  - Reason: the shared parser is already richer and is now the authoritative shell semantics owner.

## Files to modify

- `packages/rs/test/g3rs-test-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hooks.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hook_shell.rs` (delete)
- new sidecar tests under `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hooks_tests/`
