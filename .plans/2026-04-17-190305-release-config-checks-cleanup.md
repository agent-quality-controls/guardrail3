Goal

Clean `packages/rs/release/g3rs-release-config-checks` to the current package shape so it passes `cargo test --workspace` and `guardrail3-rs validate --path packages/rs/release/g3rs-release-config-checks` with no findings.

Approach

- Normalize root package policy files and root metadata:
  - add `guardrail3-rs.toml`, `clippy.toml`, `deny.toml`, `rust-toolchain.toml`, and `rustfmt.toml`
  - make root publish intent explicit and align root dependencies with the current package facade pattern
- Normalize member crate metadata:
  - add explicit `publish`, `include`, docs.rs, and `guardrail3.shared` metadata where needed
  - switch runtime away from the local `crates/types` dependency to the shared `g3rs-release-types` facade if that is the correct package-local boundary
  - feature-gate the local types facade if the local types crate remains
- Convert runtime tests to owned sidecar shape:
  - split `rule.rs` and `rule_tests/` under each rule directory
  - move inline `#[cfg(test)]` bodies in `run.rs` and standalone rule files into owned sidecars
  - remove sibling-boundary escapes from sidecar helpers
- Convert assertions to owned per-rule modules:
  - move flat `crates/assertions/src/<rule>.rs` files into `crates/assertions/src/<rule>/rule.rs`
  - retarget runtime sidecars to the owned assertions `rule` modules
  - tighten `common.rs` helper visibility and remove any public field bags
- Re-run package tests and validation after each structural block to catch any real contradiction early instead of piling on speculative fixes.

Key decisions

- Treat this as package-local cleanup until a rule contradiction appears. The current findings match the same old-shape debt already cleaned in hooks and garde.
- Prefer moving to shared external facades over keeping local `crates/types` dependency edges, because that pattern has already been accepted by the cleaned packages.
- Keep sidecar ownership strict: runtime sidecars should call only their owned `rule` module and shared assertions crate, not sibling local helpers or facade exports.

Files to modify

- `packages/rs/release/g3rs-release-config-checks/Cargo.toml`
- `packages/rs/release/g3rs-release-config-checks/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-config-checks/clippy.toml`
- `packages/rs/release/g3rs-release-config-checks/deny.toml`
- `packages/rs/release/g3rs-release-config-checks/rust-toolchain.toml`
- `packages/rs/release/g3rs-release-config-checks/rustfmt.toml`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/**`
- `packages/rs/release/g3rs-release-config-checks/crates/assertions/**`
- `packages/rs/release/g3rs-release-config-checks/crates/types/**`
