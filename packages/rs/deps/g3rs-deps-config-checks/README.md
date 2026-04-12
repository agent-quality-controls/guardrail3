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

- `RS-DEPS-CONFIG-01`
- `RS-DEPS-CONFIG-02`
- `RS-DEPS-CONFIG-03`
- `RS-DEPS-CONFIG-04`
- `RS-DEPS-CONFIG-05`
- `RS-DEPS-CONFIG-06`
- `RS-DEPS-CONFIG-07`
- `RS-DEPS-CONFIG-08`
- `RS-DEPS-CONFIG-09`
