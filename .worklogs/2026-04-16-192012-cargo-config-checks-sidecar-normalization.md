Summary
- Re-cleaned `packages/rs/cargo/g3rs-cargo-config-checks` after the test sidecar rules tightened.
- Moved internal tests off facades and off `lib.rs`, and attached them to the real rule files that own the behavior.

Decisions made
- Kept the existing rule layout and only changed test ownership because this was package debt, not a rule bug.
- For `rule.rs` files under nested rule modules, attached `rule_tests/mod.rs` directly from `rule.rs`.
- For flat rule files `rs_cargo_config_07` through `13`, attached each `*_tests/mod.rs` sidecar directly from its matching rule file.

Key files for context
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/mod.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_07_approved_allow_inventory.rs`

Next steps
- Commit this package cleanup.
- Continue scanning packages until the next real contradictory rule appears.
