# Summary

Applied the Serde-first fixture-output rule across the remaining Rust family type crates that can derive serialization cleanly. The migration adds `serde::Serialize` to owned family and parser types, keeps output serialization on the real structs, and does not add adapters, exporters, replay suites, record maps, or fixture-only output models.

# Decisions Made

- Added `serde = { version = "1", features = ["derive"] }` to each touched type crate that did not already own it.
- Added `serde` to each touched `guardrail3-rs.toml` allowlist where the package uses dependency allowlisting.
- Extended parser type crates used by these family facts: cargo TOML document, clippy TOML document, release-plz TOML, cliff TOML, and hook shell parser.
- Stopped the code family instead of forcing serialization because `G3RsCodeParsedSourceState::Parsed(syn::File)` contains `syn::File`, and `syn` 2.0.117 has no Serde feature.
- Fixed two existing `g3rs-arch-ingestion` clippy failures found by adversarial review: `ExportCounters::record` is now `const fn`, and a manual `iter().any()` equality check now uses `contains`.

# Key Files

- `.plans/2026-05-15-154946-rust-family-types-serde-first.md`
- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `packages/rs/cargo/g3rs-cargo-types/src/types.rs`
- `packages/rs/clippy/g3rs-clippy-types/src/types.rs`
- `packages/rs/deny/g3rs-deny-types/src/types.rs`
- `packages/rs/deps/g3rs-deps-types/src/types.rs`
- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-contract-types/src/types.rs`
- `packages/rs/hooks/g3rs-hooks-types/src/types.rs`
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/toolchain/g3rs-toolchain-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-types/src/types.rs`

# Verification

- `cargo check`, `cargo test`, and `g3rs validate` passed on every changed parser/type workspace and its corresponding ingestion workspace.
- `cargo clippy --manifest-path packages/rs/arch/g3rs-arch-ingestion/Cargo.toml --workspace --all-targets --all-features -- -D warnings` passed after the adversarial clippy finding was fixed.
- `python3 scripts/behavior/verify-fixture-contract-language.py` passed.
- `scripts/behavior/verify-all.sh` passed.
- Three adversarial reviews ran. The first found missing release source-check derives, which were fixed. The second found the arch ingestion clippy failures, which were fixed. The final review reported no findings.

# Next Steps

- Redesign the code-family source replay boundary before adding fixture serialization there. The current public source input stores `syn::File`, which must not be hidden with `#[serde(skip)]` or converted through a fixture-only adapter.
