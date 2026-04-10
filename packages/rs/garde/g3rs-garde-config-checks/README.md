# g3rs-garde-config-checks

Extracted garde root-policy config checks.

This package owns only the parsed-file garde checks that operate on:

- one root `Cargo.toml`
- one covering `clippy.toml` / `.clippy.toml`

The app family still owns:

- routed root discovery
- garde applicability gating from policy and source adoption
- missing / unparseable clippy handling for the garde ban rules
- source garde rules
- `RS-GARDE-10` malformed-input reporting

Current package rules:

- `RS-GARDE-CONFIG-01`
- `RS-GARDE-CONFIG-02`
- `RS-GARDE-CONFIG-03`
- `RS-GARDE-CONFIG-04`
- `RS-GARDE-CONFIG-05`
