Goal
- Let apparch ignore package-internal test-crate edges inside one app component package.
- Keep real app-layer coupling checks intact.
- Prove the bug first with rule tests and then verify the app validates further.

Approach
- Add failing tests for g3rs-apparch/logic-dependency-direction, 03, and 07 covering `component/crates/assertions -> component/crates/runtime` and the matching runtime dev-dep back to assertions.
- Fix the overlap in `g3rs-apparch-config-checks` at the shared helper level so the same package-owner check is reused by all three rules.
- Verify apparch rule tests, then rerun `apps/guardrail3-rs` validation to expose the next real package issue.

Key decisions
- Use the existing `rel_dir` to derive package ownership from the prefix before `/crates/` instead of adding a new ingestion field.
- Limit the exception to package-internal `runtime` and `assertions` sibling crates in the same app component package. Do not weaken general same-layer coupling checks.
- Fix this in apparch, not in the app package, because the app is already in the intended package-style shape.

Files to modify
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_07_dev_dependency_direction.rs
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction_tests/*
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction_tests/*
- packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_07_dev_dependency_direction_tests/*
