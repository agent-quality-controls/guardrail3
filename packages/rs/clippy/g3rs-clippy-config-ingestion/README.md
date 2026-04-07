# g3rs-clippy-config-ingestion

Ingestion package for clippy config checks. Takes a workspace crawl result,
selects the root `clippy.toml` or `.clippy.toml`, parses it, and produces the
input type for `g3rs-clippy-config-checks`.

## Pipeline

```
g3rs-workspace-crawl → g3rs-clippy-config-ingestion → g3rs-clippy-config-checks
```
