Summary
- Hardened the `RS-APPARCH-CONFIG-01/02/03/07` direction-rule sidecars so they now feed the rules their real crate-local dependency input instead of rebuilding full config bags in test helpers.
- Tightened the assertions to prove the forbidden target crate and, for the dev-direction rule, the dependency kind label.

Decisions made
- Kept the change test-only.
  - The production rules already consume `G3RsApparchCrateDependencyChecksInput`; the defect was in the sidecar shape and weak assertions.
- Switched the helper fixtures to explicit crate display names.
  - This made exact target attribution assertions readable and stable.

Key files for context
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_01_types_dependency_direction_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_07_dev_dependency_direction_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_01_types_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_02_logic_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_03_io_outbound_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_07_dev_dependency_direction.rs`
- `.plans/2026-04-23-101140-rs-apparch-direction-test-input-hardening.md`

Next steps
- Fold the fresh adversarial findings from the wider Rust pass into the next batch, if any remain after the current commits.
