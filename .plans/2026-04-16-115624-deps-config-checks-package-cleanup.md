Goal
- Clean `packages/rs/deps/g3rs-deps-config-checks` until `validate` returns `No findings.`

Approach
- Normalize the workspace root first: add the missing root policy files and `guardrail3-rs.toml`, then make publish intent explicit.
- Remove the local `crates/types` wrapper if it only reexports `g3rs-deps-types`, and switch runtime and facade imports to the real shared crate.
- Reshape assertions into nested `.../rule.rs` modules, and move sidecar proof out of runtime tests into the shared assertions crate.
- Remove `#[path]` test wiring and use normal sidecar module resolution.
- Add structural waivers only if runtime and assertions remain intentionally one-rule-per-dir crates after cleanup.

Key decisions
- Follow the cleaned config-checks packages as the shape reference, not the old package layout.
- Keep fixes architectural: delete fake wrapper boundaries instead of marking them shared.
- Mark this workspace unpublished unless the manifests prove a real need to publish.

Files to modify
- `packages/rs/deps/g3rs-deps-config-checks/Cargo.toml`
- `packages/rs/deps/g3rs-deps-config-checks/guardrail3-rs.toml`
- `packages/rs/deps/g3rs-deps-config-checks/clippy.toml`
- `packages/rs/deps/g3rs-deps-config-checks/deny.toml`
- `packages/rs/deps/g3rs-deps-config-checks/rustfmt.toml`
- `packages/rs/deps/g3rs-deps-config-checks/rust-toolchain.toml`
- `packages/rs/deps/g3rs-deps-config-checks/src/lib.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/**`
- `packages/rs/deps/g3rs-deps-config-checks/crates/assertions/**`
- `packages/rs/deps/g3rs-deps-config-checks/crates/types/**`
