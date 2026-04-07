# g3rs-deps-config-ingestion

Turns a workspace crawl into `g3rs-deps-config-checks` inputs.

Current config ingestion reads:

- workspace `Cargo.toml`
- workspace `guardrail3-rs.toml`
- member `Cargo.toml` files selected from `[workspace].members`

It normalizes external dependency facts for config checks and leaves AST/file-tree
ingestion as explicit stubs.
