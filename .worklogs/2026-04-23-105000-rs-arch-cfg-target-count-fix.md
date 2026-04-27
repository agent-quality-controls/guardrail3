Summary
- Fixed arch dependency counting so `target.'cfg(...)'.dependencies` and `target.'cfg(...)'.build-dependencies` are included in the production dependency total.
- Added a regression proving a crate with 11 root dependencies plus 2 cfg-targeted production dependencies now trips `g3rs-arch/dependency-count-split`.

Decisions made
- Kept the fix in `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/run.rs`.
- Counted target-specific production dependency tables during ingestion instead of teaching the check rule about cfg sections.
- Left dev-dependency counting unchanged.

Key files for context
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/config_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_07b_dependency_count_split.rs`

Next steps
- Commit the arch fix as a standalone change, then move to the release workflow and garde duplicate-name regressions.
