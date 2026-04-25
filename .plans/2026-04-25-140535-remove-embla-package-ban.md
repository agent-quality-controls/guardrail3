# Goal

Remove `embla-carousel*` from the active TypeScript package-family Syncpack forbidden dependency policy.

# Approach

1. Remove `embla-carousel*` from `FORBIDDEN_SYNCPACK_DEPS` in package ingestion.
2. Update ingestion tests that expected missing Embla Syncpack bans.
3. Update package config-check fixtures and expected messages so the active policy only reports the remaining forbidden dependencies.
4. Run the package ingestion and package config-check test suites.

# Files To Modify

- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/run_tests/cases.rs`
