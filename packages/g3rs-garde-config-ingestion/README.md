# g3rs-garde-config-ingestion

Ingestion package for garde config checks. Takes a workspace crawl result,
selects `Cargo.toml` and `clippy.toml`/`.clippy.toml`, parses both, and
produces the input types for `g3rs-garde-config-checks`.

Returns a result with:
- `dependency`: always present (from Cargo.toml)
- `clippy_bans`: present only when a clippy config exists
