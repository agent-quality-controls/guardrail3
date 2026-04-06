# Build g3rs-cargo-config-ingestion

## Goal
First ingestion package. Takes crawl output, selects root Cargo.toml, parses it, returns checks input type.

## Package structure
```
packages/g3rs-cargo-config-ingestion/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── TODO.md
├── src/lib.rs
├── crates/
│   ├── types/        → G3RsCargoConfigIngestionError
│   ├── runtime/      → ingest(), select, parse, assemble
│   └── assertions/   → test helpers
```

## Dependencies
- runtime → g3rs-workspace-crawl-types, g3rs-cargo-config-checks-types, cargo-toml-parser-runtime
- types → nothing
- assertions → types

## Logic
1. select: crawl.root_file("Cargo.toml") → Option<&Entry>
2. parse: read abs_path, cargo_toml_parser::parse(&content)
3. assemble: G3RsCargoConfigChecksInput { cargo_rel_path, cargo }
