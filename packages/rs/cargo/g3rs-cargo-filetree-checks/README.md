# g3rs-cargo-filetree-checks

Extracted cargo filetree checks for one pointed crawl root.

Current package boundary:

- root `Cargo.toml` missing is a hard ingestion error before filetree input exists
- one root `Cargo.toml` filetree surface
- that root may classify as:
  - workspace root
  - standalone package root
  - other Cargo manifest shape
- missing declared workspace members when the root is a workspace root
- cargo-family input failures:
  - malformed root `Cargo.toml`
  - malformed member `Cargo.toml`
  - malformed root-local `guardrail3-rs.toml`
  - malformed `[workspace].members`
  - malformed `[workspace].exclude`

Package-owned filetree rules:

- `g3rs-cargo/missing-member-cargo`
- `g3rs-cargo/input-failures`
