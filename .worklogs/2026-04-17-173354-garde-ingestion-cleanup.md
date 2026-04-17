Summary

Cleaned `packages/rs/garde/g3rs-garde-ingestion` to the current ingestion package shape. The package now passes its workspace tests and validates with no findings.

Decisions made

- Moved runtime tests off `lib.rs` and onto `run.rs` as `run_tests`, because the public ingestion entry points are owned by `run.rs`.
- Kept the local `crates/types` crate for the ingestion error type, but marked the whole package non-publishable instead of forcing a publishable facade over internal member crates.
- Added a local `crates/assertions/src/run.rs` shared proof surface and moved sidecar result-shape assertions there, so runtime sidecars no longer inspect `CheckResult` fields directly.
- Replaced the old `unreachable!()` branch in Rust policy parsing with a direct file-read and parse path, so the ingestion lane only models the states it can actually see.
- Moved runtime test fixture helpers into `run_tests/helpers.rs` and left `run_tests/source/mod.rs` facade-only with re-exports.

Key files for context

- `packages/rs/garde/g3rs-garde-ingestion/Cargo.toml`
- `packages/rs/garde/g3rs-garde-ingestion/guardrail3-rs.toml`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/assertions/src/run.rs`

Next steps

- Continue with the next non-clean garde package from the full validate sweep.
- Do not change rules unless the next package reaches a real contradiction after its stale package debt is removed.
