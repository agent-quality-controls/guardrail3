# Summary
g3rs-arch/dependency-count-split was counting dev-dependencies as if they were production coupling. I split arch dependency counts into production and dev counts, then made the hard split rule use only production dependencies.

# Decisions made
- Count `[dependencies]` and `[build-dependencies]` as production coupling because they affect the real crate boundary.
- Count `[dev-dependencies]` separately because they are test-only and should not force a production crate split.
- Kept this as a bug fix only. No new soft rule for excessive dev-dependencies was added.

# Key files for context
- packages/rs/arch/g3rs-arch-types/src/types.rs
- packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs
- packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs
- packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_07b_dependency_count_split.rs
- packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_07b_dependency_count_split_tests/mod.rs

# Next steps
- Clean `packages/rs/fmt/g3rs-fmt-ingestion` package-local issues.
- Stop if another rule shows a real contradiction instead of a package bug.
