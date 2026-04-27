Summary
- Cleaned `packages/rs/cargo/g3rs-cargo-ingestion` to `No findings.` The package now has workspace-root policy files, explicit unpublished manifests, `run_tests` sidecars that own the `run` module boundary, and shared assertions in `crates/assertions/src/run.rs`.

Decisions made
- Kept the real `types` crate because it owns the public ingestion error. Rejected deleting it because unlike the old wrapper crates, this one is a real boundary.
- Marked the whole workspace unpublished with explicit `publish = false` instead of building release scaffolding for an internal package.
- Renamed `ingest_tests` to `run_tests` and changed local calls to `crate::run::...`. Rejected keeping `ingest_tests` because the tests are about the runtime entry points in `run.rs`, not a missing `ingest.rs` module.
- Moved final config and filetree proof into the shared assertions crate. Rejected local result-check helpers in `basic.rs` because `g3rs-test/real-proof-site` was correctly surfacing them as split proof.
- Tightened weak state and error assertions so they prove specific payloads instead of wildcard matches.

Key files for context
- `.plans/2026-04-15-213544-cargo-ingestion-package-cleanup.md`
- `packages/rs/cargo/g3rs-cargo-ingestion/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/guardrail3-rs.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/assertions/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/pipeline.rs`

Next steps
- Check the last cargo package.
- Stop only on the next real outdated or contradictory rule.
