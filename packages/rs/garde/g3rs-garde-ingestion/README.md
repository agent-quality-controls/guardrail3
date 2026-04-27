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
- optionally parses root `guardrail3-rs.toml`
- selects governed non-test Rust source files
- leaves source and Rust-policy read/parse failures to `g3rs-garde/input-failures`
