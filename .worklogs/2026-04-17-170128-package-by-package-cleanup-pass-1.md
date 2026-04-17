Summary

Cleaned the next two dirty package roots from the full validate sweep: `g3rs-deny-config-checks` and `g3rs-toolchain-config-checks`. The fixes were small package-debt cleanups only: one import-boundary cleanup in deny support code and two stale sidecar helper imports in toolchain tests.

Decisions made

- Reduced the top-level import count in `deny-config-checks` by aliasing parser types through `deny_toml_parser::types as deny`. Rejected splitting the support file because the reported issue was only the import surface, not file cohesion.
- Fixed `toolchain-config-checks` by making sidecar helpers call their owned production module via `super::super::check`. Rejected touching the rules because the current package shape already matched the intended owned-sidecar contract; only the helper imports were stale.
- Kept this as a small standalone cleanup slice so the next pass can start from the next dirty package root without mixing in larger family rewrites.

Key files for context

- `.plans/2026-04-17-165913-package-by-package-cleanup-pass-1.md`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/support/unknown_keys.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule_tests/helpers.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule_tests/helpers.rs`

Next steps

- Commit this slice by itself.
- Continue package by package from the next dirty root in the sweep, which is likely the `garde` cluster unless another smaller leftover is found first.
- Stop and report back only if the next package hits a real rule contradiction rather than more package debt.
