Summary
- Re-cleaned `packages/rs/clippy/g3rs-clippy-config-checks` after the sidecar rules tightened.
- Moved internal tests off `lib.rs` and off facade `mod.rs` files onto the real rule files that own the behavior.

Decisions made
- Kept the package architecture unchanged because the failure was only stale test ownership.
- Attached nested `rule_tests/mod.rs` from `rule.rs` for rules `01` through `08`.
- Attached flat `*_tests/mod.rs` directly from the matching flat rule files for rules `09` through `21`.
- Updated nested helper imports from `super::super::rule::check` to `super::super::check` after the sidecar move.

Key files for context
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/rule.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_09_missing_method_ban.rs`

Next steps
- Commit this package cleanup.
- Continue scanning packages until the next real contradictory rule appears.
