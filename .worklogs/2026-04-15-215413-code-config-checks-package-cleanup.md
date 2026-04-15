Summary
- Cleaned `packages/rs/code/g3rs-code-config-checks` to `No findings.` The package now uses `g3rs-code-types` directly, has explicit unpublished manifests and workspace-root policy files, and its shared proof lives in nested assertions modules that match the runtime sidecars.

Decisions made
- Deleted the local wrapper `crates/types` crate and depended on `g3rs-code-types` directly. Rejected keeping the wrapper because it only reexported shared code-family types and added fake structure.
- Marked the whole workspace unpublished with explicit `publish = false` instead of carrying release burden for an internal package.
- Moved rule assertions to:
  - `crates/assertions/src/rs_code_config_07_exception_comment_inventory/rule.rs`
  - `crates/assertions/src/rs_code_config_12_unsafe_code_lint/rule.rs`
  Rejected the old flat assertions modules because runtime sidecars were importing sibling assertions modules instead of their owned proof.
- Removed the shared `common` assertion helper after it became dead and encouraged the wrong import shape. Tests now call only the owned rule assertions module.

Key files for context
- `.plans/2026-04-15-215137-code-config-checks-package-cleanup.md`
- `packages/rs/code/g3rs-code-config-checks/Cargo.toml`
- `packages/rs/code/g3rs-code-config-checks/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-config-checks/src/lib.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_07_exception_comment_inventory/rule_tests/helpers.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule_tests/helpers.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_07_exception_comment_inventory/rule.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_12_unsafe_code_lint/rule.rs`

Next steps
- Move to `packages/rs/code/g3rs-code-file-tree-checks`.
- Stop only on the next real outdated or contradictory rule.
