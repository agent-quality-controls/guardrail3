Goal
- Clean `packages/rs/apparch/g3rs-apparch-ingestion` until `validate` returns `No findings.` and workspace tests pass.

Approach
- Remove the fake local `crates/types` wrapper and keep the ingestion error in the root crate.
- Normalize the workspace root to the current package shape: root policy files, `guardrail3-rs.toml`, explicit `publish = false`, full workspace lints, feature-gated facades.
- Add a shared `crates/assertions` crate, move final proof out of runtime sidecars, and reshape tests from `ingest_tests` into owned `run_tests` sidecars.
- Fix package-local code findings in runtime tests and filesystem access without changing family rules.

Key decisions
- Keep the local ingestion error type, but move it to the root crate because a dedicated `types` crate is not justified here.
- Use the existing `x_tests/mod.rs` sidecar contract with `#[path = "x_tests/mod.rs"] mod x_tests;`.
- Keep runtime focused on ingestion work and shared proof in the assertions crate.

Files to modify
- `packages/rs/apparch/g3rs-apparch-ingestion/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/src/error.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/guardrail3-rs.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/clippy.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/deny.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/rust-toolchain.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/rustfmt.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/view.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run_tests/*`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/assertions/*`
