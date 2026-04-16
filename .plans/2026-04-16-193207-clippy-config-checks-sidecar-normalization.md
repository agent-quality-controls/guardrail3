Goal
- Re-clean `packages/rs/clippy/g3rs-clippy-config-checks` after the test sidecar rules tightened.

Approach
- Remove test sidecar declarations from `lib.rs` and from facade `mod.rs` files.
- Attach nested `rule_tests` to the real `rule.rs` files for rules `01` through `08`.
- Attach flat `*_tests` sidecars directly to the real rule files for rules `09` through `21`.
- Fix helper imports that relied on the old facade-owned test layout.

Key decisions
- Treat this as package debt, not a rule bug.
- Keep the existing rule/assertions shape and only change test ownership.

Files to modify
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/rule.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/rule_tests/helpers.rs`
- same pattern for the other affected rule files and helpers
