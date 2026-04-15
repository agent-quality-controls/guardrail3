Goal

- Clean `packages/rs/fmt/g3rs-fmt-types` under the active rules without weakening any good rule.
- Stop only if the remaining signal is no longer clearly a package bug.

Approach

- Convert the old standalone crate into an explicit one-crate workspace.
- Add the missing workspace-root policy files.
- Make publish intent explicit.
- Split `src/lib.rs` into a facade plus `src/types.rs`.
- Re-run validation and stop only if the remaining signal is a real rule question.

Key decisions

- Treat the active package shape as a workspace even when it has one crate, because current families normalize around workspace-local roots.
- Reuse the same root shape already proven on `g3rs-clippy-types` instead of inventing another exception path.

Files to modify

- `packages/rs/fmt/g3rs-fmt-types/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-types/clippy.toml`
- `packages/rs/fmt/g3rs-fmt-types/deny.toml`
- `packages/rs/fmt/g3rs-fmt-types/guardrail3-rs.toml`
- `packages/rs/fmt/g3rs-fmt-types/rust-toolchain.toml`
- `packages/rs/fmt/g3rs-fmt-types/rustfmt.toml`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
