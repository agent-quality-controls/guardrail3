# TS-TYPECOV — Type coverage checker

**Input:** type-coverage config, root/package `package.json`, type-coverage scripts
**Parser:** JSON + structured config inspection
**Current code:** pieces currently mixed into `package_deps.rs` and `tool_config_checks.rs`
**Owned root:** nearest TS package/app root with type-coverage config or type-coverage scripts

## Owns

- `type-coverage` package presence
- type-coverage config existence and parseability
- type-coverage script presence where required
- enforced type-coverage threshold policy

## Does not own

- TypeScript compiler strictness
  - that belongs to `ts/tsconfig`
- generic package policy
  - that belongs to `ts/package`

## Contract direction

This family owns the explicit type-coverage tool surface, not compiler strictness in general.
