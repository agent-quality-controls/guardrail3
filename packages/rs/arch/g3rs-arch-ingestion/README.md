# g3rs-arch-ingestion

Builds `arch` checks inputs from a workspace crawl.

Current behavior:

- parses the pointed root `Cargo.toml`
- resolves workspace members from `[workspace].members` and `[workspace].exclude`
- builds crate-node facts from member `Cargo.toml` files
- derives dependency-edge facts for config checks
- derives facade surfaces and owned source files for source checks
- derives module-directory structural facts for file-tree checks
- stays inside the pointed workspace crawl

Current lane support:

- `ingest_for_source_checks` is implemented
- `ingest_for_config_checks` is implemented
- `ingest_for_file_tree_checks` is implemented
