# g3rs-deny-config-checks

Extracted config checks for `deny.toml`.

This package receives an already parsed `deny_toml_parser::types::DenyToml` and
validates only `deny.toml` config semantics. Ingestion owns file discovery,
authoritative config selection, parse-failure routing, and profile resolution.

Current package status:

- public input contract is defined
- runtime crate exists and compiles
- rule area modules are split into advisories, bans, licenses, and sources
- migrated config rules:
  - `RS-DENY-CONFIG-01..27` except filetree-owned `01` and `03`
- no deny source lane currently exists
