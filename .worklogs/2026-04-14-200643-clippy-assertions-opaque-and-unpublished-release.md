# Summary
Fixed the remaining decided clippy package issues by making the assertions helper `Finding` opaque and marking the config-checks workspace unpublished. The package no longer trips `RS-CODE-SOURCE-31`, and the release family no longer reports publish dry-run failures for this workspace.

# Decisions Made
- Made `crates/assertions/src/common.rs::Finding` fields private instead of weakening `RS-CODE-SOURCE-31`. Rejected a rule relaxation because this crate already had constructor and assertion helpers, so an opaque DTO strengthens the API without adding complexity.
- Added a focused assertions-crate regression that uses only the helper API. Moved that proof inline into `common.rs` after the first sidecar attempt created a new `mod.rs` facade finding.
- Set `publish = false` on the root package and all three subcrates. Rejected any release-family special case because `publish = false` is already the intended unpublished contract in the release rules.

# Key Files For Context
- `.plans/2026-04-14-200643-clippy-assertions-opaque-and-unpublished-release.md`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/common.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/types/Cargo.toml`

# Next Steps
- Fix `RS-ARCH-SOURCE-04` by making rule and test sidecar `mod.rs` files pure facades.
- Continue ignoring the `test` family slice until its policy is brought back into scope.
- Decide whether the duplicated `RS-CODE-CONFIG-07` inventory warnings on `deny.toml` exception comments are acceptable or should be deduplicated.
