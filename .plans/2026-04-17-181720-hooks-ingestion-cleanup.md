Goal

Make `packages/rs/hooks/g3rs-hooks-ingestion` validate clean under the current rules without changing any rules.

Approach

1. Normalize root and member manifests.
   - Make the root package unpublished and add root policy files.
   - Add `guardrail3-rs.toml` with the package profile, allowed deps, and structural-split waivers.
   - Add include, publish, docs.rs, and guardrail3 shared metadata to runtime, assertions, and types.

2. Clean the runtime boundary.
   - Move direct `std::fs` calls behind a local `fs` module.
   - Keep runtime code depending on the architecturally correct types and parser crates only.

3. Fix test ownership shape.
   - Replace `#[cfg(test)] mod ingest_tests;` with the owned sidecar shape on the real production file.
   - Move `ingest_tests/*` into the owned sidecar for `run.rs`.
   - Stop sidecars from calling sibling local modules through the wrong boundary.

4. Move semantic result assertions into the assertions crate.
   - Add the missing runtime dependency to assertions.
   - Add assertions modules for the runtime ingestion surface so tests call shared proof functions instead of touching `CheckResult` fields directly.

5. Clean remaining source issues.
   - Strengthen weak `expect(...)` messages in tests.
   - Feature-gate `crates/types/src/lib.rs` exports.

Files to modify

- `packages/rs/hooks/g3rs-hooks-ingestion/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/clippy.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/deny.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/rust-toolchain.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/rustfmt.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/**`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/assertions/src/**`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/types/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/types/src/lib.rs`
