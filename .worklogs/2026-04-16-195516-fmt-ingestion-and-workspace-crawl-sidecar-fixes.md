## Summary

Normalized the stale `run_tests` ownership in `packages/rs/fmt/g3rs-fmt-ingestion` by moving the sidecar from `lib.rs` onto `run.rs`. Cleaned `packages/rs/g3rs-workspace-crawl` by moving facade-owned tests onto real files, splitting owned assertions into `run` and `query`, and rewiring the sidecars so they only touch their owned module plus the matching assertions crate.

## Decisions made

- Moved `fmt-ingestion` tests from `lib.rs` to `run.rs`.
  - Why: `lib.rs` is a facade and should not own logic tests.
  - Rejected: keeping `run_tests` on `lib.rs`, because that violates the sidecar ownership rule.
- Split `g3rs-workspace-crawl` sidecars by real owner.
  - Why: crawl behavior belongs with `run.rs`; query behavior belongs with `query.rs`.
  - Rejected: leaving all tests under one facade-owned `crawl_tests/`, because that makes `lib.rs` own logic it does not implement.
- Split shared assertions into `run.rs` and `query.rs`.
  - Why: each sidecar must use only its owned assertions module, not a sibling assertions module.
  - Rejected: reusing one `crawl` assertions file, because the test rules correctly treat that as cross-module proof leakage.
- Built the `query` sidecar fixture by hand.
  - Why: `query` tests must not call sibling runtime modules like `run::crawl`.
  - Rejected: calling the crawl runtime from `query` tests, because that violates the single-owned-module test rule.

## Key files for context

- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/run.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/query.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/run_tests/ignore_state.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/query_tests/basic_queries.rs`
- `packages/rs/g3rs-workspace-crawl/crates/assertions/src/run.rs`
- `packages/rs/g3rs-workspace-crawl/crates/assertions/src/query.rs`

## Next steps

- Continue scanning `packages/rs/*/*` for the next failing package.
- Keep fixing stale sidecar ownership and proof wiring until a real rule contradiction appears.
