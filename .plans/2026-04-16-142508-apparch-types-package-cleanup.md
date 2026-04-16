Goal
- Clean `packages/rs/apparch/g3rs-apparch-types` until `validate` returns `No findings.` and workspace tests pass.

Approach
- Normalize the root crate to the current `*-types` package shape with explicit workspace metadata, features, and root policy files.
- Split `src/lib.rs` into a small facade plus `src/types.rs` so lib.rs stays facade-only.
- Add `guardrail3-rs.toml` and the one waiver needed for the intentional `module_name_repetitions` allow.

Key decisions
- Keep this as a single-crate workspace because it is already the shared transport crate for the apparch family.
- Keep public field record structs because this crate is marked `shared = true` and only carries transport types.

Files to modify
- `packages/rs/apparch/g3rs-apparch-types/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`
- `packages/rs/apparch/g3rs-apparch-types/rust-toolchain.toml`
- `packages/rs/apparch/g3rs-apparch-types/rustfmt.toml`
- `packages/rs/apparch/g3rs-apparch-types/clippy.toml`
- `packages/rs/apparch/g3rs-apparch-types/deny.toml`
- `packages/rs/apparch/g3rs-apparch-types/guardrail3-rs.toml`
