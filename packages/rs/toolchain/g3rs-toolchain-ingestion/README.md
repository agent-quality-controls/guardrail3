# g3rs-toolchain-ingestion

Ingestion package for toolchain config checks. Takes a workspace crawl result,
selects the root `rust-toolchain.toml` (and optionally `Cargo.toml`), parses
them, and produces two input types for `g3rs-toolchain-config-checks`.

## Pipeline

```
g3rs-workspace-crawl → g3rs-toolchain-ingestion → g3rs-toolchain-config-checks
```

## Outputs

- **Channel & Components** — always produced when `rust-toolchain.toml` exists
- **MSRV Consistency** — only produced when both `rust-toolchain.toml` and `Cargo.toml` are present
