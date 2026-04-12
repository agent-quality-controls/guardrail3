# g3rs-arch-ingestion

Builds `arch` checks inputs from a workspace crawl.

Current behavior:

- parses the pointed root `Cargo.toml`
- resolves workspace members from `[workspace].members` and `[workspace].exclude`
- builds crate-node facts from member `Cargo.toml` files
- fans out lane-pure config inputs, including dependency thresholds and feature-contract facts
- fans out lane-pure source inputs, including facade-export gating facts
- fans out lane-pure file-tree inputs, including structural split-threshold facts
- stays inside the pointed workspace crawl

Current lane support:

- `ingest_for_source_checks` is implemented
- `ingest_for_config_checks` is implemented
- `ingest_for_file_tree_checks` is implemented
