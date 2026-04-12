# g3rs-toolchain-ingestion

Ingestion package for toolchain family packages. Takes a workspace crawl
result, selects root toolchain files, parses config files when needed, and
builds lane-specific package inputs.

## Pipeline

```
g3rs-workspace-crawl
  -> g3rs-toolchain-ingestion
  -> g3rs-toolchain-config-checks
  -> g3rs-toolchain-filetree-checks
```

## Outputs

- **Config** - parsed `rust-toolchain.toml` and optional root `Cargo.toml`
- **Filetree** - root `rust-toolchain.toml` and legacy `rust-toolchain` presence
