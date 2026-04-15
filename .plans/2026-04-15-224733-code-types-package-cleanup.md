Goal
- Clean `packages/rs/code/g3rs-code-types` until `validate` returns `No findings.`

Approach
- Convert the package from old single-crate shape into the current workspace-root shape used by the clean `*-types` packages.
- Add the missing workspace root policy files and `guardrail3-rs.toml`.
- Make publish intent explicit, add the standard feature gates, and move lint policy to workspace scope.
- Split `src/lib.rs` into a small gated facade plus `src/types.rs`.
- Keep the shared transport structs public and rely on the existing shared-crate exception in `RS-CODE-SOURCE-31`.

Key decisions
- Follow `g3rs-clippy-types` and `g3rs-fmt-types` as the package shape reference.
- Keep this as a single shared crate at the workspace root, not a `crates/runtime` layout.
- Keep `module_name_repetitions = "allow"` only with an explicit waiver reason.

Files to modify
- `packages/rs/code/g3rs-code-types/Cargo.toml`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-types/src/types.rs`
- `packages/rs/code/g3rs-code-types/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-types/clippy.toml`
- `packages/rs/code/g3rs-code-types/deny.toml`
- `packages/rs/code/g3rs-code-types/rustfmt.toml`
- `packages/rs/code/g3rs-code-types/rust-toolchain.toml`
