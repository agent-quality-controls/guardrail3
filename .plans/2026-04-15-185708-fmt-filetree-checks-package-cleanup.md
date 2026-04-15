Goal

Clean `packages/rs/fmt/g3rs-fmt-filetree-checks` so it validates with no findings under the active rules, unless a rule contradiction appears first.

Approach

- Add the missing workspace-root policy files:
  - `guardrail3-rs.toml`
  - `rust-toolchain.toml`
  - `rustfmt.toml`
  - `clippy.toml`
  - `deny.toml`
- Mark the workspace and child crates unpublished with explicit `publish = false`.
- Delete the useless local `crates/types` wrapper and make the facade and runtime use `g3rs-fmt-types` directly.
- Add a sibling `crates/test_support` crate and move the old runtime-local `test_support.rs` helper there.
- Reshape runtime and assertions to the cleaned filetree package pattern:
  - nested `mod.rs` plus `rule.rs`
  - sidecar `rule_tests/mod.rs`
  - shared proof only in `crates/assertions`
- Re-run package tests and full validation.
- If a remaining failure still looks like a rule contradiction after cleanup, stop there and explain it.

Key decisions

- Reuse the already-clean `g3rs-clippy-filetree-checks` shape instead of inventing a fmt-only filetree pattern.
- Delete the local `types` crate if it is only a thin re-export wrapper, because the cleaned clippy filetree package already proved that wrapper is unnecessary here.
- Keep fixes package-local unless the failure proves a broader rule bug.

Files to modify

- `packages/rs/fmt/g3rs-fmt-filetree-checks/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/assertions/**`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/**`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/test_support/**`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/guardrail3-rs.toml`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/rust-toolchain.toml`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/rustfmt.toml`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/clippy.toml`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/deny.toml`
