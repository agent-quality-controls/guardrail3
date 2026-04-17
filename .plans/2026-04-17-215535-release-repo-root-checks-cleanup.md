Goal
- Make `packages/rs/release/g3rs-release-repo-root-checks` validate cleanly under current rules without changing the rules.

Approach
- Normalize the package shell: root policy files, `guardrail3-rs.toml`, publish metadata, feature gates, and package allowlist.
- Remove the runtime dependency on the local `crates/types` facade and depend on `g3rs-release-types` directly where appropriate.
- Replace the old `#[cfg(test)] mod tests;` and flat assertions layout with owned `rule.rs` plus `rule_tests/` sidecars and matching `crates/assertions/src/<rule>/rule.rs` proof modules.
- Delete runtime `test_support` and move test builders into owned sidecar helper modules so sidecars stop reaching across siblings.
- Re-run package tests and package validation until the root reports `No findings.`

Key decisions
- Keep fixes package-local. If any rule contradicts the cleaned shape, stop and handle that as a separate bug instead of widening this cleanup.
- Treat the local `crates/types` crate as a facade-only package crate. Runtime should not route through it.
- Mirror the owned production rule shape in the assertions crate (`.../<rule>/rule.rs`) so internal and external tests share the same proof surface.

Files to modify
- `packages/rs/release/g3rs-release-repo-root-checks/Cargo.toml`
- `packages/rs/release/g3rs-release-repo-root-checks/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/*`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/assertions/src/*`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/types/*`
- package README/policy files as needed
