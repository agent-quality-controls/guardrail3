Goal
- Bring `packages/rs/release/g3rs-release-source-checks` to the current internal package shape so it validates with `No findings.`

Approach
- Normalize the package shell:
  - add root policy files and `guardrail3-rs.toml`
  - make member crate `publish` intent explicit
  - add missing package metadata, docs.rs metadata, and include lists
- Clean the crate boundaries:
  - switch runtime off the local `crates/types` facade onto `g3rs-release-types`
  - feature-gate the root and local types facade exports
  - remove the impure dependency from the local types crate
- Reshape tests to the current owned-sidecar pattern:
  - delete `crates/runtime/src/test_support.rs`
  - move `run.rs` and the two rule files onto owned sidecar directories
  - mirror that proof surface in `crates/assertions`
- Replace stub docs where needed so release checks pass.

Key decisions
- Keep the cleanup package-local. Do not touch release rules unless a real contradiction appears.
- Follow the cleaned release family boundary used in config, filetree, ingestion, and repo-root packages: runtime depends on `g3rs-release-types`, not the local types crate.
- Use typed local helpers inside each owned test sidecar instead of rebuilding another shared `test_support` crate or file.

Files to modify
- `packages/rs/release/g3rs-release-source-checks/Cargo.toml`
- `packages/rs/release/g3rs-release-source-checks/README.md`
- `packages/rs/release/g3rs-release-source-checks/src/lib.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/Cargo.toml`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/rs_release_source_01_readme_quality.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/rs_release_source_02_input_failures.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/types/Cargo.toml`
- `packages/rs/release/g3rs-release-source-checks/crates/types/src/lib.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/assertions/Cargo.toml`
- `packages/rs/release/g3rs-release-source-checks/crates/assertions/src/lib.rs`
- new root policy files and new owned sidecar/assertion files under `crates/runtime/src/**` and `crates/assertions/src/**`
