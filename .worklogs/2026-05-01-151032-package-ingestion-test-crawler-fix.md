## Summary

Fixed stale TS package ingestion tests that were using the Rust-workspace-only crawler entry point for temporary TypeScript fixtures.

## Decisions Made

- Switched package ingestion tests from `crawl` to `crawl_any_root`.
- Kept production ingestion unchanged because the bug was in the test harness boundary, not in package-family behavior.
- Rejected adding fake `Cargo.toml` files to fixtures because that would make TypeScript package tests depend on Rust workspace shape.

## Key Files

- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test --manifest-path packages/ts/package/g3ts-package-ingestion/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/ts/package/g3ts-package-ingestion --inventory`

## Next Steps

- Continue the package-family cleanup by committing the generic `validate` script contract separately.
