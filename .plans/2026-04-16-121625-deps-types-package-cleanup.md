Goal
- Clean `packages/rs/deps/g3rs-deps-types` until `validate` returns `No findings.`

Approach
- Convert the package to the current shared `*-types` workspace shape with explicit `publish = false`, features, workspace lints, and root policy files.
- Add `guardrail3-rs.toml` with the one real parser dependency and the documented waiver for `module_name_repetitions`.
- Split `src/lib.rs` into a small gated facade plus `src/types.rs`, and keep the existing plain shared transport structs there.

Key decisions
- Treat this as ordinary old root-shape debt, not a rule problem, because it matches the same migration pattern already used for `cargo`, `fmt`, and `code` types packages.
- Keep the types plain and public because this crate is the shared transport boundary for the deps family.

Files to modify
- `packages/rs/deps/g3rs-deps-types/Cargo.toml`
- `packages/rs/deps/g3rs-deps-types/guardrail3-rs.toml`
- `packages/rs/deps/g3rs-deps-types/clippy.toml`
- `packages/rs/deps/g3rs-deps-types/deny.toml`
- `packages/rs/deps/g3rs-deps-types/rustfmt.toml`
- `packages/rs/deps/g3rs-deps-types/rust-toolchain.toml`
- `packages/rs/deps/g3rs-deps-types/README.md`
- `packages/rs/deps/g3rs-deps-types/src/lib.rs`
- `packages/rs/deps/g3rs-deps-types/src/types.rs`
