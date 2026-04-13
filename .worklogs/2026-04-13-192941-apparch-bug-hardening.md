Summary

Hardened the new apparch package family by proving and fixing the concrete bugs found by adversarial review. The fixes closed same-layer dependency under-enforcement, workspace/member fail-open paths, incomplete dependency-edge extraction, and source-trait discovery holes in private modules and mixed entrypoint layouts.

Decisions made

- Fixed the root logic in ingestion and the three config rules instead of adding rule-local exceptions.
  - Rejected: patching tests or adding ad hoc post-filters.
- Kept the small public apparch contract unchanged.
  - The bugs were in normalization and traversal, not in the public input types.
- Treated apparch source intent as "io crates must not define `pub trait` anywhere in reachable crate code," not merely "must not expose a public trait through a public module chain."
  - This matches the original rule-family plan wording and closes the private-module leak.
- Made workspace member resolution fail closed for missing and invalid member patterns.
  - Rejected: silently dropping unresolved members and hoping later checks catch the absence.

Key files for context

- `.plans/2026-04-11-144026-apparch-rule-family.md`
- `.plans/2026-04-13-185135-apparch-family-implementation.md`
- `.plans/2026-04-13-191520-apparch-bug-hardening.md`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_01_types_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/rs_apparch_source_04_io_traits_in_types_tests/mod.rs`

Next steps

- If apparch stays active, add package metadata parity with the older families only as a separate cleanup.
- If apparch and hexarch are meant to be mutually exclusive at runtime selection, encode that family-selection rule outside this package surface rather than inside apparch ingestion.
