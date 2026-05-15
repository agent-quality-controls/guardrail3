# Summary

Made the fmt family owned ingestion output types serializable. This validates the Serde-first plan on one workspace without adding adapters, exporters, or fixture-only output models.

# Decisions Made

- Added `serde` directly to `g3rs-fmt-types` because that crate owns the check input and parse-state structs returned by `g3rs-fmt-ingestion`.
- Added `serde` to `g3rs-fmt-types/guardrail3-rs.toml` because G3RS correctly rejected the new direct dependency until it was allowlisted.
- Did not add `serde_json` to the type crate because JSON emission belongs to verifier or fixture command code.

# Key Files

- `.plans/2026-05-15-152441-fmt-types-serde-first-output.md`
- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
- `packages/rs/fmt/g3rs-fmt-types/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-types/guardrail3-rs.toml`

# Verification

- `cargo check --manifest-path packages/rs/fmt/g3rs-fmt-types/Cargo.toml`
- `cargo check --manifest-path packages/rs/fmt/g3rs-fmt-ingestion/Cargo.toml`
- `cargo test --manifest-path packages/rs/fmt/g3rs-fmt-types/Cargo.toml`
- `cargo test --manifest-path packages/rs/fmt/g3rs-fmt-ingestion/Cargo.toml`
- `g3rs validate --path packages/rs/fmt/g3rs-fmt-types`
- `g3rs validate --path packages/rs/fmt/g3rs-fmt-ingestion`

# Next Steps

- Apply the same Serde-first type derive pattern to the next ingestion family.
- Run G3RS on each changed type workspace and its ingestion workspace immediately after each family.
