# TS-FMT — Formatting checker

**Input:** prettier config, root/package `package.json`, formatting scripts
**Parser:** JSON + structured config inspection
**Current code:** pieces currently mixed into `package_deps.rs` and `tool_config_checks.rs`
**Owned root:** nearest TS package/app root with formatting config or formatting scripts

## Owns

- prettier package presence
- prettier config existence and parseability
- formatting script presence where required
- formatter policy wiring in the TS toolchain

## Does not own

- CSS lint/accessibility policy
  - that belongs to `ts/css`
- generic package policy
  - that belongs to `ts/package`

## Contract direction

This family exists because formatting is a real separate tool surface.
It should not stay buried in a generic tooling bucket.
