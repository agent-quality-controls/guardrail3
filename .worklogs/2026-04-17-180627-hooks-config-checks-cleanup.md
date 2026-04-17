Summary

Cleaned `packages/rs/hooks/g3rs-hooks-config-checks` to the current config-checks package shape. The package now passes its workspace tests and validates with no findings.

Decisions made

- Made the whole package non-publishable and normalized member-crate metadata instead of keeping a publishable root facade that depended on non-publishable internal member crates.
- Removed the runtime dependency on the local `crates/types` crate and pointed runtime directly at `g3rs-hooks-types`, leaving `crates/types` as a thin feature-gated public re-export boundary.
- Reshaped the three flat runtime rule files into directory modules with `mod.rs`, `rule.rs`, and owned `rule_tests/` sidecars so the package satisfies both the owned-sidecar and facade-only contracts without changing rules.
- Reshaped the flat assertions files into owned assertion modules and switched the package to `define_result_assertions!`, because the test source checks only recognize `assert_contains` and `assert_no_findings` as real proof steps when they come from that macro surface.
- Added the standard root policy/config files and structural-split waivers used by the cleaned config-checks packages so the runtime/assertions split is explicit instead of ad hoc.

Key files for context

- `.plans/2026-04-17-175649-hooks-config-checks-cleanup.md`
- `packages/rs/hooks/g3rs-hooks-config-checks/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-config-checks/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/hook_rs_06_required_tools_installed/mod.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/hook_rs_06_required_tools_installed/rule.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/hook_rs_06_required_tools_installed/rule_tests/cases.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/assertions/src/common.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/assertions/src/hook_rs_06_required_tools_installed/rule.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/types/src/lib.rs`

Next steps

- Commit this slice by itself.
- Continue with the next dirty hooks package root.
- Do not change rules unless the next hooks package reaches a real contradiction after package-local cleanup.
