Goal
- Clean `packages/rs/deps/g3rs-deps-ingestion` until `validate` returns `No findings.`

Approach
- Normalize the workspace root first: add the missing root policy files and `guardrail3-rs.toml`, then make the root and child crates explicitly unpublished.
- Keep the real local `crates/types` crate because it owns the public ingestion error boundary, but normalize its manifest and facade shape.
- Rename `ingest_tests` to `run_tests`, make `mod.rs` facade-only, move shared test helpers into `helpers.rs`, and move all final proof into `crates/assertions/src/run.rs`.
- Replace the direct `std::fs::metadata` call in `run.rs` with the existing fs boundary.

Key decisions
- Treat the local `types` crate as real and keep it, because it owns the ingestion error type instead of reexporting another package.
- Test the public `run` entrypoint from sidecars, not internal helper modules, because the package already exports ingestion through `run.rs`.

Files to modify
- `packages/rs/deps/g3rs-deps-ingestion/Cargo.toml`
- `packages/rs/deps/g3rs-deps-ingestion/guardrail3-rs.toml`
- `packages/rs/deps/g3rs-deps-ingestion/clippy.toml`
- `packages/rs/deps/g3rs-deps-ingestion/deny.toml`
- `packages/rs/deps/g3rs-deps-ingestion/rustfmt.toml`
- `packages/rs/deps/g3rs-deps-ingestion/rust-toolchain.toml`
- `packages/rs/deps/g3rs-deps-ingestion/src/lib.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/assertions/**`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/**`
- `packages/rs/deps/g3rs-deps-ingestion/crates/types/**`
