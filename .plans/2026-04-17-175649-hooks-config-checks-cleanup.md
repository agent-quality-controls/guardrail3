Goal

Clean `packages/rs/hooks/g3rs-hooks-config-checks` to the current package shape so it passes package tests and `guardrail3-rs validate --path ...` with no findings, without changing rules unless a real contradiction appears.

Approach

- Add the missing root policy files and package metadata so the workspace root is covered by toolchain, fmt, clippy, deny, and guardrail policy.
- Normalize publish intent and shared metadata for `crates/runtime`, `crates/assertions`, and `crates/types`, then make the root package non-publishable if its member crates stay internal.
- Replace the old flat assertions layout with per-rule assertion modules plus a shared `run` assertions module if needed, and remove the crate-level `allow(...)` blanket from the assertions crate.
- Move runtime sidecars onto the owned-sidecar shape:
  - `#[cfg(test)] #[path = "..._tests/mod.rs"] mod ..._tests; // reason: ...`
  - split each `*_tests/mod.rs` into a facade plus sibling test case files and local helpers.
- Remove direct sidecar imports of `g3rs_hooks_config_checks_types` by targeting either the owned rule module or the shared assertions crate.
- Normalize the local `crates/types` crate into a feature-gated thin re-export boundary over `g3rs-hooks-types`, and stop `crates/runtime` from depending on it if the root facade or shared package boundary is the correct dependency.
- Re-run `cargo test -q --manifest-path ... --workspace` and `guardrail3-rs validate --path ...`, then either commit the clean package or stop if a true rule contradiction remains.

Key decisions

- Keep the fix package-local first. The reported errors look like stale package shape, not rule inconsistency.
- Follow the already-cleaned garde package patterns for sidecars, assertions, and root policy files instead of inventing a hooks-specific variant.
- Do not keep the current publishable root package shape if it forces root dependencies on non-publishable member crates.

Files to modify

- `packages/rs/hooks/g3rs-hooks-config-checks/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-config-checks/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/*.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/*_tests/*`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/assertions/src/*`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/types/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/types/src/lib.rs`
- root config files under `packages/rs/hooks/g3rs-hooks-config-checks/`
