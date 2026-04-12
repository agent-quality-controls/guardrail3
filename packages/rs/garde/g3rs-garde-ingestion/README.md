# g3rs-garde-ingestion

Ingestion package for garde checks. It converts one workspace crawl into the
typed inputs for:

- `g3rs-garde-config-checks`
- `g3rs-garde-source-checks`

Config ingestion:
- requires root `Cargo.toml`
- preserves covering clippy state as:
  - parsed
  - missing
  - invalid

Source ingestion:
- requires root `Cargo.toml`
- requires root `guardrail3.toml`
- selects governed non-test Rust source files
- leaves source and guardrail read/parse failures to `RS-GARDE-SOURCE-10`
