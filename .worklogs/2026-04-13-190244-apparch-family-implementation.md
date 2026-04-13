Summary

Built the `apparch` family as real packages with a small public contract, real ingestion, config dependency-direction checks, and the source trait-ownership check. The family now exposes only the lanes justified by the current rule inventory: `config` and `source`.

Decisions made

- Dropped the old plan's custom `dep` lane.
  - Kept dependency-direction rules in `config` because they are driven by parsed `Cargo.toml` dependency data.
- Did not add a `filetree` lane.
  - The current apparch rule inventory does not justify one, and fake public lanes were explicitly avoided.
- Kept the public family types minimal.
  - Exported only `layer`, `crate`, `dependency edge`, `public trait`, and the two lane inputs.
  - Rejected richer per-edge and per-crate fact types at the package boundary.
- Kept parsing and traversal in ingestion.
  - Config checks receive normalized workspace-internal dependency edges.
  - Source checks receive extracted public trait facts, not raw source strings.
- Included the hybrid root package when `[package]` exists at the pointed workspace root.
  - This keeps dependency normalization honest for workspaces that also ship a root crate.

Key files for context

- `.plans/2026-04-13-185135-apparch-family-implementation.md`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_01_types_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_02_logic_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_03_io_outbound_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/rs_apparch_source_04_io_traits_in_types.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

Next steps

- Run an adversarial family review against the apparch implementation before wiring it into any old app inventory or selector.
- Decide separately whether apparch needs opt-in family selection metadata before any app-level integration.
- If future apparch rules introduce real file placement constraints, add a `filetree` lane then, not before.
