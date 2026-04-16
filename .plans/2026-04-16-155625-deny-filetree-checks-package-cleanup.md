Goal

Clean `packages/rs/deny/g3rs-deny-filetree-checks` until workspace tests and `guardrail3-rs validate` return clean without changing rules.

Approach

- Remove the fake local `crates/types` crate and wire the workspace directly to `g3rs-deny-types`.
- Add the missing workspace-root policy files and `guardrail3-rs.toml`.
- Make publish intent explicit and mark the workspace and child crates unpublished.
- Replace runtime-local `test_support.rs` with a sibling `crates/test_support` crate for generic test input builders.
- Rewrite runtime test wiring to the approved owned sidecar pattern with `#[path = "..._tests/mod.rs"]` and facade-only sidecar `mod.rs` files.
- Move final proof into the shared assertions crate, including a shared assertions file for `run`.
- Re-run package tests and validator, then stop only if a real rule contradiction appears.

Key decisions

- Follow the established deny package cleanup pattern from `g3rs-deny-config-checks` instead of inventing new local exceptions.
- Keep `#[path = ...]` bridges for file-module sidecars because plain `mod x_tests;` resolves to the wrong place for file modules.
- Keep shared `test_support` generic only; package-specific fixtures stay in owned sidecars if needed.

Files to modify

- `packages/rs/deny/g3rs-deny-filetree-checks/Cargo.toml`
- `packages/rs/deny/g3rs-deny-filetree-checks/src/lib.rs`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/runtime/**`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/assertions/**`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/test_support/**`
- workspace root policy/config files for this package
