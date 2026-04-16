Summary
- Fixed the apparch overlap that treated package-internal `assertions -> runtime` and `runtime --dev-dependencies-> assertions` edges as forbidden layer coupling.
- Added regression tests for logic and io/outbound package-style component crates.

Decisions made
- Derived package ownership from the existing `rel_dir` by splitting on `/crates/` instead of adding a new ingestion field.
- Limited the exception to one exact shape: same package root, `assertions -> runtime` for runtime deps and `runtime -> assertions` for dev-deps.
- Kept general same-layer apparch checks intact for all other same-layer edges.

Key files for context
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_07_dev_dependency_direction.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction_tests/cases.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction_tests/cases.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_07_dev_dependency_direction_tests/cases.rs

Next steps
- Keep apparch focused on app-layer boundaries, not package-internal test-crate mechanics.
- If more internal package-style component crates appear under app layers, they should already fit this exception without new rule branches.
