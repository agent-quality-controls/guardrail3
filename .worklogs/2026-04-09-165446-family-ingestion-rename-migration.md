# Worklog - family ingestion rename migration

## Summary

Renamed the Rust family ingestion packages so they match the agreed architecture:
one ingestion package per family, with `ingest_for_config_checks`,
`ingest_for_ast_checks`, and `ingest_for_file_tree_checks`. Folded the old
`g3rs-code-ast-ingestion` package into `g3rs-code-ingestion` and kept only the
AST lane real there for now.

## Decisions made

- Renamed every `g3rs-*-config-ingestion` package to `g3rs-*-ingestion`.
  - Why: the old package names lied about responsibility because those packages
    already exposed AST and file-tree entrypoints too.
  - Rejected: keeping the old names and only updating docs, because that would
    preserve the mismatch in the public package surface.

- Renamed the public ingestion error types to family-wide names.
  - Examples: `G3RsCargoIngestionError`, `G3RsDepsIngestionError`.
  - Why: keeping `*ConfigIngestionError` would preserve the same naming lie on
    the public API after the package rename.

- Folded `g3rs-code-ast-ingestion` into `g3rs-code-ingestion`.
  - Why: `code` was the only lane-specific ingestion package and broke the
    family-ingestion contract we already settled on.
  - Rejected: keeping `g3rs-code-ast-ingestion` as a special case, because it
    would leave the architecture inconsistent.

- Added stub `code` config/file-tree entrypoints and placeholder input types.
  - Why: the rename migration should change package shape without mixing in the
    real `code` config/file-tree implementation work.
  - Rejected: building real `code` config/file-tree ingestion in the same
    commit, because that would mix naming migration with semantic lane work.

## Key files for context

- `.plans/2026-04-09-151200-family-ingestion-rename-migration.md`
- `packages/rs/code/g3rs-code-ingestion/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/types/src/error.rs`
- `scripts/fix_paths.py`
- `scripts/extract_shared_types.py`
- `scripts/reorganize.sh`

## Verification

- `cargo test --workspace -q` in `packages/rs/cargo/g3rs-cargo-ingestion`
- `cargo test --workspace -q` in `packages/rs/clippy/g3rs-clippy-ingestion`
- `cargo test --workspace -q` in `packages/rs/deny/g3rs-deny-ingestion`
- `cargo test --workspace -q` in `packages/rs/fmt/g3rs-fmt-ingestion`
- `cargo test --workspace -q` in `packages/rs/garde/g3rs-garde-ingestion`
- `cargo test --workspace -q` in `packages/rs/release/g3rs-release-ingestion`
- `cargo test --workspace -q` in `packages/rs/toolchain/g3rs-toolchain-ingestion`
- `cargo test --workspace -q` in `packages/rs/deps/g3rs-deps-ingestion`
- `cargo test --workspace -q` in `packages/rs/code/g3rs-code-ingestion`
- mechanical audit:
  - no `packages/rs/*/g3rs-*-config-ingestion` dirs remain
  - no `packages/rs/*/g3rs-*-ast-ingestion` dirs remain
  - every `packages/rs/*/g3rs-*-ingestion/src/lib.rs` exports all three
    `ingest_for_*_checks` entrypoints

## Next steps

1. Build real `g3rs-code-ingestion::ingest_for_config_checks` for `RS-CODE-07`
   and `RS-CODE-12`.
2. Build real `g3rs-code-ingestion::ingest_for_file_tree_checks` for
   `RS-CODE-35`.
3. Decide whether `code` should get a proper `g3rs-code-types` family crate so
   the temporary placeholder config/file-tree input types can move out of the
   ingestion types crate.
