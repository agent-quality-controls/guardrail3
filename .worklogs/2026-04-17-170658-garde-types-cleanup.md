Summary

Cleaned `packages/rs/garde/g3rs-garde-types` by normalizing its package baseline to match the already-clean shared family type packages. The package now has the expected root policy files, unpublished/shared metadata, and full workspace lint baseline.

Decisions made

- Kept `g3rs-garde-types` as a single root crate. It is a passive shared types package and does not need the runtime/assertions split used by behavior-heavy packages.
- Reused the established shared-types baseline instead of inventing garde-specific variants for root policy files or lint configuration.
- Kept only the parser dependencies the package actually uses and added a narrow `g3rs-cargo/approved-allow-inventory` waiver reason for the intentional family-prefix type names.

Key files for context

- `.plans/2026-04-17-170504-garde-types-cleanup.md`
- `packages/rs/garde/g3rs-garde-types/Cargo.toml`
- `packages/rs/garde/g3rs-garde-types/clippy.toml`
- `packages/rs/garde/g3rs-garde-types/deny.toml`
- `packages/rs/garde/g3rs-garde-types/rust-toolchain.toml`
- `packages/rs/garde/g3rs-garde-types/rustfmt.toml`
- `packages/rs/garde/g3rs-garde-types/guardrail3-rs.toml`

Next steps

- Commit this slice by itself.
- Return to `packages/rs/garde/g3rs-garde-config-checks`, which still carries the older fake local-types crate and stale sidecar/assertions layout.
