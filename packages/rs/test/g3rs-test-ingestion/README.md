# g3rs-test-ingestion

Builds `test` checks inputs from a workspace crawl.

Current behavior:

- discovers owned test roots from workspace Cargo manifests
- parses root `Cargo.toml`
- parses optional `nextest.toml` and `.cargo/mutants.toml`
- detects test and tokio-test activity from owned Rust files
- detects active mutation hooks
- classifies owned Rust files for the root-scoped source lane
- emits one `G3RsTestSourceChecksInput` per owned test root
- emits one `G3RsTestConfigChecksInput` per owned test root

Current lane support:

- `ingest_for_config_checks` is implemented
- `ingest_for_source_checks` is implemented
- `ingest_for_file_tree_checks` is a stub
