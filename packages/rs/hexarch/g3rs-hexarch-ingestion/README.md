# g3rs-hexarch-ingestion

Builds `code` checks inputs from a workspace crawl.

Current behavior:

- scans owned config files for `EXCEPTION:` comments
- parses workspace `Cargo.toml` files for `workspace.lints.rust.unsafe_code`
- selects `.rs` files from the crawl
- skips fixture paths
- classifies `is_test`
- resolves `profile_name` as `library` or `binary` when Cargo target ownership is clear
- marks the exact library root file with `is_library_root`
- reads source content
- emits one `G3RsCodeSourceChecksInput` per file

Current lane support:

- `ingest_for_source_checks` is implemented
- `ingest_for_config_checks` is implemented
- `ingest_for_file_tree_checks` is a stub
