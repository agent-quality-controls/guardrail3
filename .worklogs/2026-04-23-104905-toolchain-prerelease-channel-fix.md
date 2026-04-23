Summary
- Fixed prerelease Rust toolchain channels so `1.85.0-rc.1` and `1.85.0-dev` are no longer classified as pinned stable.
- Added regressions for the channel classifier and the MSRV consistency check.

Decisions made
- Kept the fix in the two rule implementations that classify pinned stable toolchains.
- Rejected the old suffix-stripping behavior entirely instead of trying to whitelist prerelease suffixes.

Key files for context
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule_tests/prerelease.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule_tests/prerelease.rs`

Next steps
- Commit the cargo hybrid-root allow-table fix as a separate bug-fix commit, then rerun or reuse the package verification results for the final report.
