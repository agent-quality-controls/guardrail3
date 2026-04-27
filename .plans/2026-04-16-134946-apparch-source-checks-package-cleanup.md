Goal
- Clean `packages/rs/apparch/g3rs-apparch-source-checks` until `validate` returns `No findings.` without changing good rules.

Approach
- Normalize the workspace root first: add the missing root policy files and `guardrail3-rs.toml`, make publish intent explicit, and gate the root facade behind features.
- Delete the fake local `crates/types` wrapper and depend on `g3rs-apparch-types` directly from the root and runtime crates.
- Add a sibling `crates/assertions` crate and move final result proof there.
- Keep the chosen file-module sidecar contract: `#[path = "x_tests/mod.rs"] mod x_tests;`, then split every sidecar `mod.rs` into facade-only `mod.rs` plus `cases.rs` and `helpers.rs`.
- Re-run tests and validate, then stop only if another rule is genuinely contradictory or outdated.

Key decisions
- Do not change the rule set unless rereading the rule proves a contradiction. The current `g3rs-test/owned-sidecar-shape` complaint is package debt because the source files still declare `mod tests` instead of `mod x_tests`.
- Treat this as the same cleanup shape as `g3rs-apparch-config-checks`, unless a package-specific rule exposes a real bug.

Files to modify
- `packages/rs/apparch/g3rs-apparch-source-checks/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/guardrail3-rs.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/clippy.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/deny.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/rustfmt.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/rust-toolchain.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/**`
