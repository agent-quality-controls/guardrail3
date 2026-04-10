# g3rs-hexarch-ingestion

Builds `hexarch` checks inputs from a workspace crawl.

Current behavior:

- discovers app workspaces under `apps/`
- resolves workspace members to real crate directories
- skips fixture members
- reads member manifests and source entrypoints
- walks reachable Rust modules for source-lane crate summaries
- emits one `G3RsHexarchSourceChecksInput` per crate

Current lane support:

- `ingest_for_source_checks` is implemented
- `ingest_for_config_checks` is not implemented yet
- `ingest_for_file_tree_checks` is a stub
