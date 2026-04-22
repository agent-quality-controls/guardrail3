## Goal

Repair the `packages/parsers/hook-shell-parser` validation failures introduced by the new command-query API so the parser package is clean under `g3rs validate`.

The repaired state is:

- `CommandQueryOptions` is a real API type, not a public-field bag
- the command-query engine is split below the file-size threshold
- the `api` sidecar tests only call the owned `api` module surface

## Approach

1. Narrow the `CommandQueryOptions` API in `command_query/api.rs`
   - make fields private
   - expose explicit constructors / builder-style setters / getters
   - update all call sites in parser tests and hook rules
2. Split `command_query/engine.rs`
   - keep orchestration in `engine.rs`
   - move wrapper-specific traversal and low-level token/state helpers into sibling modules under `command_query`
   - do not change behavior, only package ownership and file size
3. Fix the `api` sidecar boundary
   - stop importing `crate::command_query` and `crate::parse_script` directly from `api_tests/golden.rs`
   - expose the needed test-only helper through `api.rs`
   - import via the owned `api` module path from the sidecar
4. Re-run:
   - parser package tests
   - parser package fmt
   - `g3rs validate --path packages/parsers/hook-shell-parser`

## Key decisions

- Fix the parser package itself instead of suppressing the rules.
  - Reason: the validation findings are correct. The API is too raw, the file is too large, and the sidecar boundary is wrong.
- Keep the visitor behavior unchanged while splitting files.
  - Reason: the behavioral changes were already tested in the previous hooks-seam repair. This slice should only repair package quality and ownership.

## Files to modify

- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/mod.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api_tests/golden.rs`
- new sibling modules under `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/`
