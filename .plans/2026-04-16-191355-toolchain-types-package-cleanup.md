Goal
- Normalize `packages/rs/toolchain/g3rs-toolchain-types` to the current shared `*-types` package shape and make package validation pass cleanly.

Approach
- Replace the old single-crate root `Cargo.toml` shape with the current workspace-root `*-types` shape.
- Add the missing workspace policy files and `guardrail3-rs.toml`.
- Split `src/lib.rs` into a small gated facade plus `src/types.rs`.
- Keep the current shared transport structs and parsers, but make publish intent and lint policy explicit.

Key decisions
- Keep public fields on these shared input structs because this crate is the shared transport contract.
- Keep the `module_name_repetitions` waiver because the family prefix is intentional in shared types.

Files to modify
- `packages/rs/toolchain/g3rs-toolchain-types/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-types/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-types/src/types.rs`
- `packages/rs/toolchain/g3rs-toolchain-types/guardrail3-rs.toml`
- `packages/rs/toolchain/g3rs-toolchain-types/clippy.toml`
- `packages/rs/toolchain/g3rs-toolchain-types/deny.toml`
- `packages/rs/toolchain/g3rs-toolchain-types/rust-toolchain.toml`
- `packages/rs/toolchain/g3rs-toolchain-types/rustfmt.toml`
