# g3rs-deps-ingestion

Turns a workspace crawl into package lane inputs for the deps family.

Current ingestion supports:

- config checks
  - workspace `Cargo.toml`
  - workspace `guardrail3-rs.toml`
  - member `Cargo.toml` files selected from `[workspace].members`
  - normalized external dependency facts
- filetree checks
  - root `Cargo.lock`
  - root `.gitignore` masking for `Cargo.lock`
  - optional root profile from `guardrail3-rs.toml`

Source ingestion is still a stub.
