Goal
- Clean `packages/rs/code/g3rs-code-config-checks` to `No findings.` Remove the local wrapper `types` crate, make publish intent explicit, add workspace-root policy files, and move shared test proof into nested assertions modules that match the runtime sidecars.

Approach
- Add the missing workspace-root policy files and `guardrail3-rs.toml`.
- Mark the whole workspace unpublished with explicit `publish = false`.
- Delete the local wrapper `crates/types` crate and depend on `g3rs-code-types` directly from the root and runtime crates.
- Make `crates/assertions` depend on `crates/runtime`, keep `common` private, and move rule assertions into:
  - `crates/assertions/src/rs_code_config_07_exception_comment_inventory/rule.rs`
  - `crates/assertions/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- Rewire runtime sidecars to call the owned `rule` assertions modules and stop calling `run` or the local wrapper crate.

Key decisions
- Do not keep the wrapper `types` crate. It only reexports `g3rs-code-types` and adds fake structure.
- Do not change rules unless a new contradiction shows up. The current failures are package shape issues.
- Keep the sidecar pattern:
  - `rule_tests/mod.rs`
  - local `helpers.rs`
  - shared final proof in `crates/assertions/src/.../rule.rs`

Files to modify
- `packages/rs/code/g3rs-code-config-checks/Cargo.toml`
- `packages/rs/code/g3rs-code-config-checks/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-config-checks/clippy.toml`
- `packages/rs/code/g3rs-code-config-checks/deny.toml`
- `packages/rs/code/g3rs-code-config-checks/rust-toolchain.toml`
- `packages/rs/code/g3rs-code-config-checks/rustfmt.toml`
- `packages/rs/code/g3rs-code-config-checks/src/lib.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_07_exception_comment_inventory/rule_tests/helpers.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule_tests/helpers.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/common.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_07_exception_comment_inventory.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_12_unsafe_code_lint.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_07_exception_comment_inventory/mod.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_07_exception_comment_inventory/rule.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_12_unsafe_code_lint/mod.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/types/Cargo.toml`
- `packages/rs/code/g3rs-code-config-checks/crates/types/src/lib.rs`
