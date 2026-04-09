# g3rs-cargo-ingestion

Ingestion package for Cargo config checks. Takes a workspace crawl result,
selects the root `Cargo.toml`, parses it, and produces the input type for
`g3rs-cargo-config-checks`.

## Pipeline

```
g3rs-workspace-crawl → g3rs-cargo-ingestion → g3rs-cargo-config-checks
```
