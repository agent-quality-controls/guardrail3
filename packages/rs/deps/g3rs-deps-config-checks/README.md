# g3rs-deps-config-checks

Extracted dependency-policy config checks for guardrail3 Rust workspaces.

## Boundary

This package validates content only. Other deps package lanes own:

- `g3rs-deps-filetree-checks`
  - root `Cargo.lock` presence
  - root `.gitignore` masking of `Cargo.lock`
- `g3rs-deps-ingestion`
  - file discovery and authoritative file selection
  - fail-closed ingestion errors for unreadable, malformed, or untrustworthy deps inputs

The package receives full parsed files only:

- workspace `Cargo.toml`
- crate `Cargo.toml`
- workspace `guardrail3-rs.toml`
- process PATH tool-discovery facts from ingestion

It does not receive derived helper structs, resolved allowlists, or ad hoc
subset policy types.

## Initial rule target

This package owns:

- `g3rs-deps/dependencies-allowlisted`
- `g3rs-deps/build-dependencies-allowlisted`
- `g3rs-deps/dev-dependencies-allowlisted`
- `g3rs-deps/library-allowlist-present`
- `g3rs-deps/direct-dependency-cap`
- `g3rs-deps/cargo-deny-installed`
- `g3rs-deps/cargo-machete-installed`
- `g3rs-deps/cargo-dupes-installed`
- `g3rs-deps/gitleaks-installed`
