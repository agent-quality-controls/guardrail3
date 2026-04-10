# Summary

Extracted the non-file-tree `hexarch` source lane into `packages/rs/hexarch` as a working slice. The new package set covers `RS-HEXARCH-22` and `RS-HEXARCH-23` end to end through `crawl -> ingest_for_source_checks -> g3rs-hexarch-source-checks::check`.

# Decisions made

- Extracted only the source slice for now.
  - Rejected extracting the much larger config/dependency slice in the same commit.
- Fixed the source input boundary to be one crate per input.
  - Rejected bundling all crates into one `G3RsHexarchSourceChecksInput`, because both migrated rules are local per-crate assertions.
- Added rule-local tests and ingestion pipeline tests before freezing the slice.
  - Rejected keeping only a single smoke pipeline test, because it would miss module reachability and fail-closed cases.

# Key files for context

- `packages/rs/hexarch/g3rs-hexarch-types/src/types.rs`
- `packages/rs/hexarch/g3rs-hexarch-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hexarch/g3rs-hexarch-source-checks/crates/runtime/src/rs_hexarch_22_ports_trait_dominance.rs`
- `packages/rs/hexarch/g3rs-hexarch-source-checks/crates/runtime/src/rs_hexarch_23_adapter_pub_trait.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/source_layout.rs`

# Next steps

- Build `g3rs-hexarch-config-checks`.
- Implement `g3rs-hexarch-ingestion::ingest_for_config_checks(...)` for `RS-HEXARCH-08`, `10`, `11`, `13`-`21`, `24`-`27`.
- Leave structural/file-tree rules out until the file-tree lane is tackled explicitly.
