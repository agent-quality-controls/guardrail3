# Goal

Make the fmt family ingestion output types serializable without adding adapters or fixture-only output models.

# Approach

- Update `packages/rs/fmt/g3rs-fmt-types` because `g3rs-fmt-ingestion` returns the check input structs from that crate.
- Add a direct `serde` dependency with the `derive` feature to `g3rs-fmt-types`.
- Derive `serde::Serialize` on every owned fmt-family type that can appear in ingestion output.
- Do not add `serde_json` here because this crate owns types, not fixture command output.
- Run G3RS on the changed `g3rs-fmt-types` workspace and the `g3rs-fmt-ingestion` workspace.

# Files to Modify

- `packages/rs/fmt/g3rs-fmt-types/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
- `packages/rs/fmt/g3rs-fmt-types/Cargo.lock`

# Verification

- `cargo check --manifest-path packages/rs/fmt/g3rs-fmt-types/Cargo.toml`
- `cargo check --manifest-path packages/rs/fmt/g3rs-fmt-ingestion/Cargo.toml`
- `g3rs validate --path packages/rs/fmt/g3rs-fmt-types`
- `g3rs validate --path packages/rs/fmt/g3rs-fmt-ingestion`
