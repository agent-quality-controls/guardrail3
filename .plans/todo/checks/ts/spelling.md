# TS-SPELLING — Spelling checker

**Input:** spelling config, root/package `package.json`, spelling scripts
**Parser:** JSON + structured config inspection
**Current code:** pieces currently mixed into `package_deps.rs` and `tool_config_checks.rs`
**Owned root:** nearest TS package/app root with spelling config or spelling scripts

## Owns

- `cspell` package presence
- spelling config existence and parseability
- spelling script presence where required
- spelling baseline wiring in the TS toolchain

## Does not own

- generic package policy
  - that belongs to `ts/package`
- source-code semantic rules
  - that belongs to `ts/code`

## Contract direction

This family owns codebase spelling quality through the dedicated spelling toolchain.
