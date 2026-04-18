Goal
- Clean `packages/rs/apparch/g3rs-apparch-config-checks` until `validate` returns `No findings.` without changing good rules.

Approach
- Normalize the workspace root first: add the missing root policy files and `guardrail3-rs.toml`, then make publish intent explicit across the root and child crates.
- Remove the fake local `types` wrapper if it is only a passthrough; otherwise mark it shared and gate facades correctly.
- Replace every old `#[path] mod tests;` file-module sidecar with standard `x_tests/mod.rs` resolution via `mod x_tests;`.
- Reshape every sidecar `mod.rs` into a facade-only file, move helpers into sibling files, and move final proof into a sibling `crates/assertions` crate.
- Re-run `validate` and stop only if another rule is clearly wrong or contradictory.

Key decisions
- Do not patch the test-family rules here unless a reread proves they are wrong. The current evidence says the package still uses the old sidecar contract.
- Treat `run.rs` and the `rs_apparch_config_*` rule modules the same way: `x.rs` owns `x_tests/mod.rs`.

Files to modify
- `packages/rs/apparch/g3rs-apparch-config-checks/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/guardrail3-rs.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/clippy.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/deny.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/rustfmt.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/rust-toolchain.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/**`
