# g3rs-clippy-ingestion

Ingestion package for clippy config and filetree checks.

It takes a pointed-workspace crawl result, selects the root `clippy.toml` or
`.clippy.toml`, reads root-local `guardrail3-rs.toml` Rust policy and root-local
`.cargo/config*` override surfaces, and produces typed inputs for:

- `g3rs-clippy-config-checks`
- `g3rs-clippy-filetree-checks`

## Pipeline

```text
g3rs-workspace-crawl
  -> g3rs-clippy-ingestion
  -> g3rs-clippy-config-checks
  -> g3rs-clippy-filetree-checks
```
