# g3-garde-content-checks

Extracted garde root-policy content checks.

This package owns only the parsed-file garde checks that operate on:

- one root `Cargo.toml`
- one covering `clippy.toml` / `.clippy.toml`

The app family still owns:

- routed root discovery
- garde applicability gating from policy and source adoption
- missing / unparseable clippy handling for the garde ban rules
- AST/source garde rules
- `RS-GARDE-10` malformed-input reporting

Current package rules:

- `RS-GARDE-01`
- `RS-GARDE-02`
- `RS-GARDE-03`
- `RS-GARDE-04`
- `RS-GARDE-06`
