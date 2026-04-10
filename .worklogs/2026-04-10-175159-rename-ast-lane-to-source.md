# Rename AST Lane To Source

## Summary

Renamed the Rust `ast` lane to `source` across the extracted checks packages and family ingestion APIs. The concrete package roots are now `g3rs-code-source-checks`, `g3rs-garde-source-checks`, and `g3rs-test-source-checks`, and ingestion now exports `ingest_for_source_checks`.

## Decisions made

### Rename the public lane API too
- Chose: rename `ingest_for_ast_checks` to `ingest_for_source_checks`, and rename `*AstChecksInput` to `*SourceChecksInput`.
- Why: package-directory-only renames would leave the old boundary mistake in the public API.
- Rejected: keeping old function and type names as compatibility aliases.

### Rename placeholder source-lane types in stub families
- Chose: rename placeholder lane types and stub errors in all current family ingestion packages.
- Why: the lane name must be coherent even where source checks are not implemented yet.
- Rejected: updating only `code`, `garde`, and `test`.

### Keep rule IDs and internal rule module names
- Chose: keep rule IDs such as `RS-GARDE-AST-*` and internal folders such as `rs_code_ast_*`.
- Why: the lane rename is about package/API boundaries, not rule inventory or parser implementation names.
- Rejected: renaming rule IDs or all internal `*_ast_*` module paths.

## Key files for context

- `.plans/2026-04-10-174543-rename-ast-lane-to-source.md`
- `packages/rs/code/g3rs-code-source-checks/Cargo.toml`
- `packages/rs/garde/g3rs-garde-source-checks/Cargo.toml`
- `packages/rs/test/g3rs-test-source-checks/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/src/lib.rs`
- `packages/rs/test/g3rs-test-ingestion/src/lib.rs`
- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/source/mod.rs`
- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md`

## Next steps

1. Decide whether the top-level lane names in the remaining active plan files should also be renamed from `ast` to `source` in their filenames, not just in their contents.
2. Build the next source-lane family only under the `source` naming.
3. Keep file-tree work separate; this rename did not change file-tree boundaries.
