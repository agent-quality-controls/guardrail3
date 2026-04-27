# Goal
Make g3rs-arch/dependency-count-split judge only production coupling. Dev-dependencies should not trip the hard crate-split error.

# Approach
- Add tests that prove a crate with 12 production deps and extra dev-deps stays clean, while 13 production deps still errors.
- Split the mixed dependency count in arch types and ingestion into production and dev counts.
- Keep the hard cap rule on production dependencies only.
- Verify on the real fmt-ingestion package.

# Key decisions
- Count `[dependencies]` and `[build-dependencies]` as production coupling.
- Count `[dev-dependencies]` separately.
- Do not add a new dev-dependency rule in this change.

# Files to modify
- packages/rs/arch/g3rs-arch-types/src/types.rs
- packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs
- packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs
- packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_07b_dependency_count_split.rs
- packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_07b_dependency_count_split_tests/mod.rs
- packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/test_support.rs
