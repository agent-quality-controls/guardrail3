# g3rs-fmt-ingestion

Ingestion for the `fmt` family.

- `ingest_for_config_checks(...)` selects the active root rustfmt config, parses root config files into typed or blocker states, and extracts `fmt` waivers from `guardrail3-rs.toml`.
- `ingest_for_file_tree_checks(...)` discovers root and nested rustfmt config files without parsing their contents.
