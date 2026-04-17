Goal

Make `packages/rs/hooks/g3rs-hooks-file-tree-checks` validate clean under the current rules without changing any rules. Keep the package on the same normalized shape as `g3rs-hooks-config-checks`.

Approach

1. Normalize the package root and member manifests.
   - Make the root package unpublished and add the missing root policy files.
   - Add `guardrail3-rs.toml` with the package profile, required allowed deps, and structural-split waivers.
   - Align runtime, assertions, and types manifests with the cleaned hooks config package: include, publish, docs.rs, guardrail3 shared metadata.

2. Remove the local package-types dependency from runtime.
   - Switch runtime code to depend directly on `g3rs-hooks-types`.
   - Feature-gate the local `crates/types` facade so it remains a clean shared surface instead of a required internal dependency.

3. Normalize runtime rule layout.
   - Convert each flat rule file into a directory module with `mod.rs` and `rule.rs`.
   - Replace `#[path = "..._tests/mod.rs"] mod tests;` with the owned sidecar `rule_tests/mod.rs` shape and reason comment.
   - Remove `test_support.rs` and move helpers into per-rule sidecars.

4. Normalize assertions layout.
   - Replace the public field bag assertion helper with the cleaned `Finding`-based surface used in hooks config checks.
   - Convert each flat assertion module into a directory with `mod.rs` and `rule.rs`.
   - Add the runtime dependency required by the current test rule shape.

5. Verify and stop only if a real contradiction appears.
   - Run package tests.
   - Run `guardrail3-rs validate --path packages/rs/hooks/g3rs-hooks-file-tree-checks`.

Key decisions

- Reuse the cleaned hooks config package shape instead of inventing a second hooks package pattern.
- Keep the local `crates/types` package as the public facade crate because the package root still exposes it, but stop using it as an internal runtime dependency.
- Move helper construction into rule-local sidecars so tests no longer reach through `crate::test_support`.

Files to modify

- `packages/rs/hooks/g3rs-hooks-file-tree-checks/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/clippy.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/deny.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/rust-toolchain.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/rustfmt.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/runtime/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/runtime/src/**`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/assertions/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/assertions/src/**`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/types/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/types/src/lib.rs`
