# g3rs-hooks-ingestion

Builds `hooks` checks inputs from a workspace crawl.

Current behavior:

- selects the effective pre-commit hook
- prefers `.githooks/pre-commit` over `hooks/pre-commit`
- ingests direct `.githooks/pre-commit.d/*` scripts
- classifies pre-commit versus modular scripts
- records whether modular mode exists
- parses root `Cargo.toml` only to compute workspace-project context for Rust hook checks
- emits one `G3RsHooksSourceChecksInput` per selected hook script

Current lane support:

- `ingest_for_source_checks` is implemented
- `ingest_for_config_checks` is a stub
- `ingest_for_file_tree_checks` is a stub
