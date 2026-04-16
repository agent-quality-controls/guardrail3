Goal
- Make `packages/rs/toolchain/g3rs-toolchain-config-checks` pass full `validate`.

Approach
- Remove the local `crates/types` wrapper and use `g3rs-toolchain-types` directly from the root facade and runtime crate.
- Mark the workspace and child crates unpublished and add the missing root policy files plus `guardrail3-rs.toml`.
- Reshape the assertions crate into nested `.../rule.rs` modules so sidecars can use the owned shared proof path the test rules require.
- Change `rule.rs` files from `mod tests;` to the exact owned sidecar shape:
  - `#[cfg(test)]`
  - `#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module`
  - `mod rule_tests;`
- Rewire sidecar helpers to call the owned production `rule` module and the shared assertions crate only.

Key decisions
- Remove `crates/types`.
  - Why: it is a pure wrapper around `g3rs-toolchain-types`, not a real boundary.
- Keep the runtime rule directories.
  - Why: they already match the one-rule-per-directory shape and only need test-sidecar cleanup.
- Keep `#[path]` only for the exact `rule_tests` sidecar bridge with a reason comment.
  - Why: that now matches the narrowed rule contract.

Files to modify
- `packages/rs/toolchain/g3rs-toolchain-config-checks/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule_tests/*`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule_tests/*`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/src/common.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/src/rs_toolchain_config_01_channel_and_components/*`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/src/rs_toolchain_config_02_msrv_consistency/*`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/guardrail3-rs.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/rust-toolchain.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/rustfmt.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/clippy.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/deny.toml`
