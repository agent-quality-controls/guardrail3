# g3rs-garde-config-checks

Extracted garde config checks.

This package owns the garde rules that operate on:

- one root `Cargo.toml`
- one covering clippy input state:
  - parsed `clippy.toml` / `.clippy.toml`
  - missing clippy config
  - invalid or unreadable clippy config

This package is responsible for:

- garde dependency presence
- quiet gating of ban rules when `garde` is absent
- warn-level "cannot verify" results when covering clippy config is missing or invalid

Current package rules:

- `RS-GARDE-CONFIG-01`
- `RS-GARDE-CONFIG-02`
- `RS-GARDE-CONFIG-03`
- `RS-GARDE-CONFIG-04`
- `RS-GARDE-CONFIG-05`
