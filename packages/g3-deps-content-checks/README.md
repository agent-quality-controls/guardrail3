# g3-deps-content-checks

Extracted dependency-policy content checks for guardrail3 Rust workspaces.

## Boundary

This package validates content only. The app family still owns:

- tool presence on `PATH`
- `Cargo.lock` presence and `.gitignore` masking
- file discovery and authoritative file selection
- malformed-input fail-closed reporting

The package receives full parsed files only:

- workspace `Cargo.toml`
- crate `Cargo.toml`
- workspace `guardrail3.toml`

It does not receive derived helper structs, resolved allowlists, or ad hoc
subset policy types.

## Initial rule target

This package owns:

- `RS-DEPS-05`
- `RS-DEPS-06`
- `RS-DEPS-07`
- `RS-DEPS-08`
- `RS-DEPS-12`
