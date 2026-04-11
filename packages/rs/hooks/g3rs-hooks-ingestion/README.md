# g3rs-hooks-ingestion

Builds `hooks` checks inputs from a workspace crawl.

Current behavior:

- selects the effective pre-commit hook
- prefers `.githooks/pre-commit` over `hooks/pre-commit`
- ingests direct `.githooks/pre-commit.d/*` scripts for source and file-tree lanes
- records whether modular mode exists
- reads hook content for config and source checks
- reads hook file stats and executable bits for file-tree checks
- inventories local override scripts
- resolves `git config core.hooksPath`
- detects trust risks like Husky, Lefthook, and shadow `.git/hooks/pre-commit`
- detects installed hook tools from PATH for config checks
- parses root `Cargo.toml` only to compute workspace-project context for Rust hook checks
- emits:
  - one `G3RsHooksConfigChecksInput`
  - zero or more `G3RsHooksSourceChecksInput`
  - one `G3RsHooksFileTreeChecksInput`

Current lane support:

- `ingest_for_source_checks` is implemented
- `ingest_for_config_checks` is implemented
- `ingest_for_file_tree_checks` is implemented
