Goal
- Clean `packages/rs/deps/g3rs-deps-filetree-checks` until `validate` returns `No findings.`

Approach
- Normalize the workspace root first: add the missing root policy files and `guardrail3-rs.toml`, then make publish intent explicit.
- Remove the local `crates/types` wrapper if it only reexports `g3rs-deps-types`, and switch runtime and facade imports to the real shared crate.
- Match the cleaned cargo filetree package shape for file-module tests: owned `*_tests/mod.rs` sidecars with facade-only `mod.rs` and sibling `cases.rs`.
- Add shared assertions files for each owned sidecar and move final result proof into the assertions crate.
- Keep structural waivers only for intentional runtime/assertions one-rule-per-file layouts.

Key decisions
- Follow the cleaned filetree package pattern, not the old mixed inline/sidecar layout.
- Keep fixes package-local unless a real rule contradiction appears.
- Mark this workspace unpublished unless the manifests prove a real need to publish.

Files to modify
- `packages/rs/deps/g3rs-deps-filetree-checks/Cargo.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/guardrail3-rs.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/clippy.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/deny.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/rustfmt.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/rust-toolchain.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/src/lib.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/runtime/**`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/assertions/**`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/types/**`
