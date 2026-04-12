# g3rs-fmt-ingestion

Ingestion for the `fmt` family.

- `ingest_for_config_checks(...)` selects the active root rustfmt config, parses root config files into typed or blocker states, and extracts `fmt` escape hatches from `guardrail3.toml`.
- `ingest_for_file_tree_checks(...)` discovers root and nested rustfmt config files without parsing their contents.
