Goal
- Clean `packages/rs/clippy/g3rs-clippy-filetree-checks` under the active rules without weakening good rules.
- Stop only if a real rule contradiction appears.

Approach
- Add the missing workspace-root policy files so validation is checking the package shape instead of missing-root noise.
- Make release intent explicit and align it with the other internal clippy family packages.
- Remove the dead `types` wrapper crate if it only reexports external types.
- Replace the runtime-local test support pattern with the package shape already used by the clean clippy config package: shared assertions crate plus sibling `test_support` crate.
- Reshape `mod.rs` and rule sidecars to the facade-only pattern and remove `#[path]` test wiring.
- Re-run package tests, package validation, then do a test-attack pass on the touched test rules.

Key decisions
- Prefer deleting the dead wrapper crate over adding more waivers or shared flags.
- Copy the clean shape from sibling clippy packages where it fits, instead of inventing a new layout.
- Mark the package unpublished unless the code clearly proves it is intended to publish.

Files to modify
- packages/rs/clippy/g3rs-clippy-filetree-checks/Cargo.toml
- packages/rs/clippy/g3rs-clippy-filetree-checks/clippy.toml
- packages/rs/clippy/g3rs-clippy-filetree-checks/deny.toml
- packages/rs/clippy/g3rs-clippy-filetree-checks/guardrail3-rs.toml
- packages/rs/clippy/g3rs-clippy-filetree-checks/rust-toolchain.toml
- packages/rs/clippy/g3rs-clippy-filetree-checks/rustfmt.toml
- packages/rs/clippy/g3rs-clippy-filetree-checks/src/lib.rs
- packages/rs/clippy/g3rs-clippy-filetree-checks/crates/runtime/**
- packages/rs/clippy/g3rs-clippy-filetree-checks/crates/assertions/**
- packages/rs/clippy/g3rs-clippy-filetree-checks/crates/test_support/**
- packages/rs/clippy/g3rs-clippy-filetree-checks/crates/types/**
