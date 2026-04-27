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

- `g3rs-garde/dependency-present`
- `g3rs-garde/core-method-bans`
- `g3rs-garde/extractor-type-bans`
- `g3rs-garde/reqwest-json-ban`
- `g3rs-garde/additional-method-bans`
