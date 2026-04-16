Summary
- Cleaned `packages/rs/toolchain/g3rs-toolchain-config-checks` until both workspace tests and full `validate` returned clean.
- The package now uses the shared `g3rs-toolchain-types` crate directly, explicit unpublished manifests, root policy files, nested assertions modules, and the exact owned `rule_tests` sidecar shape.

Decisions made
- Removed the local `crates/types` crate.
  - Why: it was a pure wrapper around `g3rs-toolchain-types`, not a real boundary.
  - Rejected: keeping a fake local types crate just to preserve old imports.
- Kept the per-rule module directories in runtime.
  - Why: the production shape was already right; only the test and assertions surfaces were stale.
- Moved the assertions crate to nested `.../rule.rs` modules.
  - Why: the test rules require the shared proof file to match the owned rule directory.
  - Rejected: leaving flat assertion files and teaching the rules about a second shape.
- Kept the sidecar helpers calling `rule::check(...)` explicitly.
  - Why: this keeps the owned production module name visible in the call path and avoids reaching through unrelated local modules.

Key files for context
- `packages/rs/toolchain/g3rs-toolchain-config-checks/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/guardrail3-rs.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/mod.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule_tests/helpers.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/src/common.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/src/rs_toolchain_config_01_channel_and_components/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/assertions/src/rs_toolchain_config_02_msrv_consistency/rule.rs`

Next steps
- Continue with `packages/rs/toolchain/g3rs-toolchain-filetree-checks`.
- Stop only if the next blocker is a real rule contradiction instead of normal package debt.
