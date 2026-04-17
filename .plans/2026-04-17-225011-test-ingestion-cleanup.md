Goal
- Bring `packages/rs/test/g3rs-test-ingestion` to the current internal package shape and eliminate all findings.

Approach
- Normalize the package shell:
  - add root policy files and `guardrail3-rs.toml`
  - mark the facade and member crates non-publishable
  - add missing include/docs/shared metadata
  - split root features into `types`, `runtime`, and `api`
- Clean the crate boundaries:
  - keep the local ingestion types crate for the typed error surface
  - feature-gate the root and local types facade exports
  - add the required allowed deps in `guardrail3-rs.toml`
- Reshape runtime ownership:
  - replace `run.rs` with an owned `ingest` module so `ingest_tests/` is no longer orphaned
  - move the semantic test proofs into `crates/assertions/src/ingest.rs`
  - update sidecars to call the shared assertions module instead of inspecting `CheckResult` directly
- Reduce runtime structural complexity:
  - split `components.rs` into smaller lane-focused modules under `components/`
  - tighten weak `expect(...)` messages in the remaining sidecars

Key decisions
- Keep the cleanup package-local. No rule changes unless the new grouped module shape exposes a real contradiction.
- Keep the shared assertions surface small but proof-bearing: one `ingest` assertions module that owns the result-shape checks used by the sidecars.
- Split `components.rs` by ingestion lane instead of waiving the file-size rule.

Files to modify
- `packages/rs/test/g3rs-test-ingestion/Cargo.toml`
- `packages/rs/test/g3rs-test-ingestion/README.md`
- `packages/rs/test/g3rs-test-ingestion/src/lib.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest.rs` or `crates/runtime/src/ingest/mod.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components.rs` split into `components/`
- `packages/rs/test/g3rs-test-ingestion/crates/types/Cargo.toml`
- `packages/rs/test/g3rs-test-ingestion/crates/types/src/lib.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/test/g3rs-test-ingestion/crates/assertions/src/lib.rs`
- new `crates/assertions/src/ingest.rs`
- sidecar files under `crates/runtime/src/ingest_tests/`
- new root policy files and `guardrail3-rs.toml`
