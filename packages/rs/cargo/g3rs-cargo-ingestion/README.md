# g3rs-cargo-ingestion

Ingestion package for cargo config and filetree checks.

## Pipeline

```
g3rs-workspace-crawl
  -> g3rs-cargo-ingestion
  -> g3rs-cargo-config-checks
  -> g3rs-cargo-filetree-checks
```
