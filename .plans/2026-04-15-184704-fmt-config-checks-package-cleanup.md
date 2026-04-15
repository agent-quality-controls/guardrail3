Goal

Clean `packages/rs/fmt/g3rs-fmt-config-checks` so it validates with no findings under the active rules, unless a rule contradiction appears first.

Approach

- Add the missing workspace-root policy files:
  - `guardrail3-rs.toml`
  - `rust-toolchain.toml`
  - `rustfmt.toml`
  - `clippy.toml`
  - `deny.toml`
- Make publish intent explicit for the workspace and child crates. If this package is internal-only, mark it unpublished.
- Remove old `#[path]` test wiring and move to normal module resolution with sidecar `tests/mod.rs` directories.
- Reshape runtime, assertions, and test support to match the cleaned clippy package pattern:
  - shared proof only in `crates/assertions`
  - shared input builders in `crates/test_support`
  - sidecars keep setup and execution only
- Fix `crates/assertions/src/common.rs` so `Finding` is not a public field bag.
- Fix `crates/types` feature contract and shared metadata so runtime can depend on it cleanly.
- Re-run full validation and package tests.
- If a remaining finding still looks like a rule contradiction after package cleanup, stop there and explain it.

Key decisions

- Reuse the package shape already proven on cleaned clippy workspaces instead of inventing a new fmt-only pattern.
- Keep fixes package-local unless the failure proves a broader rule bug.
- Prefer deleting thin wrapper structure only if it is truly useless. If `crates/types` carries a real package-local contract, keep it and normalize it.

Files to modify

- `packages/rs/fmt/g3rs-fmt-config-checks/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-config-checks/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/assertions/**`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/**`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/types/**`
- `packages/rs/fmt/g3rs-fmt-config-checks/guardrail3-rs.toml`
- `packages/rs/fmt/g3rs-fmt-config-checks/rust-toolchain.toml`
- `packages/rs/fmt/g3rs-fmt-config-checks/rustfmt.toml`
- `packages/rs/fmt/g3rs-fmt-config-checks/clippy.toml`
- `packages/rs/fmt/g3rs-fmt-config-checks/deny.toml`
