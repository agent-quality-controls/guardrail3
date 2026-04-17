Goal

Clean `packages/rs/garde/g3rs-garde-types` against the current package baseline.

Approach

- Normalize `Cargo.toml` to the same root-package shape used by the already-clean shared family type packages.
- Add the missing root policy files: `clippy.toml`, `deny.toml`, `rust-toolchain.toml`, `rustfmt.toml`, and `guardrail3-rs.toml`.
- Keep the package as a shared unpublished family types crate with only the parser dependencies it actually uses.
- Re-run package validation and tests after the baseline files are in place.

Key decisions

- Treat this as ordinary package baseline drift, not a family-specific redesign.
- Reuse the proven shared-types package baseline instead of inventing a garde-specific variant.
- Keep the single root crate shape. This package is passive shared types, not a multi-crate runtime/assertions package.

Files to modify

- `packages/rs/garde/g3rs-garde-types/Cargo.toml`
- `packages/rs/garde/g3rs-garde-types/clippy.toml`
- `packages/rs/garde/g3rs-garde-types/deny.toml`
- `packages/rs/garde/g3rs-garde-types/rust-toolchain.toml`
- `packages/rs/garde/g3rs-garde-types/rustfmt.toml`
- `packages/rs/garde/g3rs-garde-types/guardrail3-rs.toml`
