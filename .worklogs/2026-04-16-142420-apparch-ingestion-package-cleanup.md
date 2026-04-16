Summary
- Cleaned `packages/rs/apparch/g3rs-apparch-ingestion` until `validate` returned `No findings.` and the package workspace tests passed.
- Removed the fake local `types` crate, moved the real ingestion error into the runtime crate, split `run` into focused modules, and moved final proof into a new shared assertions crate.

Decisions made
- Deleted `crates/types` because it only wrapped `g3rs-apparch-types` and a local error enum, which created fake arch and apparch coupling.
- Kept the real ingestion error local to the runtime crate instead of carrying a dedicated wrapper crate for it.
- Split the old mixed `run_tests` harness into `config_tests` and `source_tests` so each sidecar belongs to a real production file instead of a facade-only `mod.rs`.
- Moved shared proof into nested assertions files under `crates/assertions/src/run/` so the sidecars and assertions tree match the owned sidecar contract.
- Replaced the direct production `std::fs::read_to_string` call with the local `fs` boundary in `view.rs`.

Key files for context
- `packages/rs/apparch/g3rs-apparch-ingestion/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/guardrail3-rs.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/view.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/mod.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/workspace.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/assertions/src/run/config.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/assertions/src/run/source.rs`

Next steps
- Continue to the next remaining apparch package and stop only on the next real rule bug or contradiction.
- Reuse this package as the reference shape for apparch ingestion workspaces with split runtime modules, per-file sidecar tests, and nested shared assertions files.
