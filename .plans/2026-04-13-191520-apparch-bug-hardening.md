Goal

Fix the concrete apparch bugs found by the adversarial review and prove each one with regression tests first. The end state is a workspace-only apparch family with correct dependency edge extraction, correct dependency-direction enforcement, and source ingestion that does not miss or silently skip public trait violations.

Approach

1. Add failing config-rule tests for same-layer dependency violations:
   - `types -> types`
   - `logic -> logic`
   - `io/outbound -> io/outbound`
2. Add failing ingestion tests for config boundary bugs:
   - root manifest with `[package]` only and no `[workspace]` must fail
   - `./crates/*` workspace member glob must resolve
   - build/dev and target build/dev dependency edges must be extracted
   - renamed workspace dependency with `package = ...` plus `workspace = true` must resolve
3. Add failing source-ingestion tests for source boundary bugs:
   - shared `visited` set must not let a private-first traversal suppress a later public path
   - explicit entrypoint paths must not suppress default sibling entrypoints
   - missing declared module file must fail closed
   - file-level `#![cfg(test)]` module must not leak traits
4. Fix the architecture boundary in ingestion and checks, not the tests:
   - require a pointed workspace root
   - normalize member patterns consistently
   - collect all relevant dependency tables
   - reject same-layer forbidden edges per the original matrix
   - track source traversal reachability without losing public visibility
   - fail closed on unresolved declared modules
   - ignore file-level test-only modules
5. Add the missing direct `io/inbound` coverage to freeze those branches.
6. Run the apparch package suites and `git diff --check`.
7. Write a standalone bug-fix worklog and commit the fix separately from the family build commit.

Key decisions

- Keep the small public apparch contract unchanged unless a bug proves it insufficient.
- Fix under-enforcement in rule logic rather than inflating public types.
- Keep ingestion parser-backed; do not move parsing into checks.
- Do not treat metadata drift as part of this bug-fix commit unless a failing test or compile/runtime issue requires it.

Files to modify

- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_01_types_dependency_direction_tests/mod.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction_tests/mod.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction_tests/mod.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/rs_apparch_source_04_io_traits_in_types_tests/mod.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_01_types_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction.rs`
