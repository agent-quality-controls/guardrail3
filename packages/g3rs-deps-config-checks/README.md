# g3rs-deps-config-checks

Extracted dependency-policy config checks for guardrail3 Rust workspaces.

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

- `RS-DEPS-CONFIG-01`
- `RS-DEPS-CONFIG-02`
- `RS-DEPS-CONFIG-03`
- `RS-DEPS-CONFIG-04`
- `RS-DEPS-CONFIG-05`
