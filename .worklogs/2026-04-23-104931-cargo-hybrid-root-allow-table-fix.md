Summary
- Fixed cargo config allow inventory so hybrid workspace roots also inspect root-package `[lints.*]` tables.
- Added regressions proving hybrid roots no longer hide approved or unapproved allow entries in the root package table.

Decisions made
- Added a root-package-specific lint-table helper in `support.rs` instead of changing the existing workspace-policy helper.
- Updated the approved and unapproved allow inventory rules to inspect both the workspace policy table and the root-package table when both exist.

Key files for context
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/support.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_07_approved_allow_inventory.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_07_approved_allow_inventory_tests/cases.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_11_unapproved_allow_entries.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_11_unapproved_allow_entries_tests/cases.rs`

Next steps
- Commit the cargo fix, then report the two commit SHAs and the verification commands that passed.
