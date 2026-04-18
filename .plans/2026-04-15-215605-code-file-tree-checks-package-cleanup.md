Goal
- Clean `packages/rs/code/g3rs-code-file-tree-checks` to `No findings.` Remove the wrapper `types` crate, make the package explicitly unpublished, add workspace-root policy files, and strip the unused public assertions helper shape.

Approach
- Add the missing workspace-root policy files and `guardrail3-rs.toml`.
- Mark the root and child crates `publish = false`.
- Delete the local wrapper `crates/types` crate and use `g3rs-code-types` directly from the root and runtime crates.
- Keep the assertions crate only as a minimal scaffold:
  - add runtime dependency
  - remove the unused exported `common` helper and its public field bag
- Update the root facade to reexport `g3rs-code-types` directly.

Key decisions
- Do not invent test scaffolding. This package has no runtime tests yet, so it does not need a fake assertions API.
- Do not keep the wrapper `types` crate. It only reexports `g3rs-code-types` and adds fake structure.

Files to modify
- `packages/rs/code/g3rs-code-file-tree-checks/Cargo.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/clippy.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/deny.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/rust-toolchain.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/rustfmt.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/src/lib.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/assertions/Cargo.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/assertions/src/lib.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/assertions/src/common.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/types/Cargo.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/types/src/lib.rs`
