## Summary

Ran a repo-wide adversarial pass across the extracted Rust config families and
closed the biggest shared gap: most families proved ingestion in isolation but
not the real `crawl -> ingest_for_config_checks -> check` lane. This pass adds
real pipeline smoke tests across the config families and fixes three fail-open
boundaries where present-but-bad optional files were silently dropped.

## Decisions made

- Fixed present-but-bad optional files at the ingestion boundary.
  - `garde`, `release`, and `toolchain` now fail closed when optional config
    files exist but are unreadable or malformed.
  - Rejected the previous "graceful degradation to None" behavior because it
    silently disabled checks on owned config that was actually present.

- Added one real config-lane proof per family.
  - Every extracted config family ingestion package now has a
    `src/ingest_tests/pipeline.rs`.
  - These tests prove the actual lane instead of trusting ingestion-only
    greens.

- Kept the lane tests small and direct.
  - Each pipeline test uses a tiny fixture and asserts a concrete rule ID or
    title from the matching checks package.

## Key files for context

- `.plans/2026-04-09-224938-multi-family-test-attack.md`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run.rs`
- `packages/rs/*/g3rs-*-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Verification

- `cargo test --workspace -q` in:
  - `packages/rs/cargo/g3rs-cargo-ingestion`
  - `packages/rs/clippy/g3rs-clippy-ingestion`
  - `packages/rs/deny/g3rs-deny-ingestion`
  - `packages/rs/deps/g3rs-deps-ingestion`
  - `packages/rs/fmt/g3rs-fmt-ingestion`
  - `packages/rs/garde/g3rs-garde-ingestion`
  - `packages/rs/release/g3rs-release-ingestion`
  - `packages/rs/toolchain/g3rs-toolchain-ingestion`
- `git diff --check`
- grep confirmed the old silent-drop optional-file pattern is gone from
  `packages/rs/*/g3rs-*-ingestion/crates/runtime/src`

## Next steps

1. Run the same repo-wide lane-proof pass for the real AST families again if
   new AST packages land.
2. Build the missing real `garde` AST ingestion lane so its AST checks stop
   being package-only.
3. Start the same lane-proof work for future file-tree packages once those
   lanes exist.
