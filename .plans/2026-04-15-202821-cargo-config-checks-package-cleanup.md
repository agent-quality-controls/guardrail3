# Goal
Make `packages/rs/cargo/g3rs-cargo-config-checks` validate clean unless a real rule contradiction appears.

# Approach
- Remove the local `crates/types` wrapper and use `g3rs-cargo-types` directly from the facade and runtime crates.
- Add missing workspace-root policy files and `guardrail3-rs.toml`.
- Make publish intent explicit and keep the package unpublished unless the package proves it must publish.
- Re-run validation, then fix the remaining old sidecar layout if the rules are consistent.

# Key decisions
- Treat the local `crates/types` crate as leftover scaffolding because it only re-exports `g3rs-cargo-types`.
- Follow the same unpublished package shape used in the already-clean fmt/clippy packages.

# Files to modify
- packages/rs/cargo/g3rs-cargo-config-checks/Cargo.toml
- packages/rs/cargo/g3rs-cargo-config-checks/src/lib.rs
- packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/Cargo.toml
- packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/run.rs
- packages/rs/cargo/g3rs-cargo-config-checks/crates/assertions/Cargo.toml
- packages/rs/cargo/g3rs-cargo-config-checks/crates/types/Cargo.toml
- packages/rs/cargo/g3rs-cargo-config-checks/crates/types/src/lib.rs
- packages/rs/cargo/g3rs-cargo-config-checks/guardrail3-rs.toml
- packages/rs/cargo/g3rs-cargo-config-checks/clippy.toml
- packages/rs/cargo/g3rs-cargo-config-checks/deny.toml
- packages/rs/cargo/g3rs-cargo-config-checks/rustfmt.toml
- packages/rs/cargo/g3rs-cargo-config-checks/rust-toolchain.toml
