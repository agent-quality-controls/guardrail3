# g3rs-cargo-ingestion

Ingestion package for cargo config and filetree checks.

Current ingestion contract:

- reads the root `Cargo.toml` from the crawl root
- treats that root manifest as the cargo policy root
- supports these root kinds:
  - workspace root
  - standalone package root
  - other Cargo manifest shape
- collects workspace member manifests only when the root manifest is a workspace root
  - expands `[workspace].members` literals and globs
  - applies `[workspace].exclude`
  - normalizes `./` and slash noise
  - deduplicates matched members
  - degrades invalid member or exclude patterns into ingestion failures
- reads root-local `guardrail3-rs.toml` after the root `Cargo.toml` boundary passes
- source ingestion is not implemented
- missing root `Cargo.toml` is a hard ingestion error

## Pipeline

```
g3rs-workspace-crawl
  -> g3rs-cargo-ingestion
  -> g3rs-cargo-config-checks
  -> g3rs-cargo-filetree-checks
```

Current outputs:

- config checks input
- filetree checks input

Current non-output:

- source checks
  - `ingest_for_source_checks` returns `SourceIngestionNotImplemented`
