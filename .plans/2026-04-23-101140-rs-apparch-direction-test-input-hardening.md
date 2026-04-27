Goal
- Stop the apparch direction-rule sidecars from rebuilding bound dependencies out of raw edge bags, and tighten their assertions so they prove the forbidden target that each rule reports.

Approach
- Replace the helper inputs for `g3rs-apparch/types-dependency-direction`, `02`, `03`, and `07` with direct `G3RsApparchCrateDependencyChecksInput` builders.
- Add or adjust test cases so the rules are exercised through the same input shape they receive in production.
- Strengthen the assertions crates to require the exact forbidden target crate in error output, not only a generic title fragment.
- Verify the touched apparch config-checks package with `cargo test` and `g3rs validate`.

Key decisions
- Keep the change test-only. The production rules already consume the correct input type.
- Do not rebuild repo-wide config bags in rule sidecars when the rules take one crate-local dependency input.

Files to modify
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_01_types_dependency_direction_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_01_types_dependency_direction_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_07_dev_dependency_direction_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_07_dev_dependency_direction_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_01_types_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_02_logic_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_03_io_outbound_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_07_dev_dependency_direction.rs`
