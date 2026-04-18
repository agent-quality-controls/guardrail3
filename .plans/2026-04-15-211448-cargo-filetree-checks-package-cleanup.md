Goal
- Clean `packages/rs/cargo/g3rs-cargo-filetree-checks` until full validation returns `No findings.`

Approach
- Remove the local `crates/types` wrapper and depend on `g3rs-cargo-types` directly.
- Mark the root and child crates `publish = false` so release checks stop treating this workspace as publishable.
- Add the missing workspace-root policy files:
  - `rust-toolchain.toml`
  - `rustfmt.toml`
  - `clippy.toml`
  - `deny.toml`
  - `guardrail3-rs.toml`
- Reshape runtime tests:
  - replace `#[path = "run_tests/mod.rs"]` with normal module resolution
  - turn `run_tests/mod.rs` into a facade-only dispatcher
  - move test bodies into `run_tests/cases.rs`
- Add `crates/assertions/src/run.rs` and move final result proof there.
- Re-run package tests and full validate.

Key decisions
- Delete the wrapper `types` crate instead of weakening arch and apparch rules. It adds no real boundary.
- Keep assertions simple. This package only needs one shared `run` assertions module, not a bigger helper scaffold.
- Make the whole workspace unpublished instead of adding release files, because this package is an internal guardrail workspace.

Files to modify
- `packages/rs/cargo/g3rs-cargo-filetree-checks/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/assertions/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/assertions/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/assertions/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/types/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/types/src/lib.rs`
- workspace-root policy files
