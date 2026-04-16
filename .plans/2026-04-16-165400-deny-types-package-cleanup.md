Goal

Normalize `packages/rs/deny/g3rs-deny-types` to the current shared `*-types` workspace shape and make it validate clean.

Approach

- Convert the root crate into a one-crate workspace with explicit `publish = false`, shared workspace lints, and feature-gated facade exports.
- Split `src/lib.rs` into a small facade and `src/types.rs` for the actual shared data types.
- Add workspace-root policy files and `guardrail3-rs.toml`.
- Add the existing naming waiver for the intentional family-prefixed shared type names.
- Re-run tests and validation, then continue until the next real rule contradiction appears.

Key decisions

- Keep public fields on the plain shared transport structs; this package is the intended exception path already covered by the shared-types rule shape.
- Do not invent child crates here; this is a single shared types crate, so the normalized one-crate workspace shape is enough.

Files to modify

- `packages/rs/deny/g3rs-deny-types/Cargo.toml`
- `packages/rs/deny/g3rs-deny-types/src/lib.rs`
- `packages/rs/deny/g3rs-deny-types/src/types.rs`
- workspace-root policy files for this package
- `packages/rs/deny/g3rs-deny-types/guardrail3-rs.toml`
