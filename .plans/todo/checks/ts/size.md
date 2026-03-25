# TS-SIZE — Bundle/size budget checker

**Input:** size-limit config, root/package `package.json`, size-budget scripts
**Parser:** JSON + structured config inspection
**Current code:** pieces currently mixed into `package_deps.rs` and `tool_config_checks.rs`
**Owned root:** TS package/app roots where size budgets are part of the product contract

## Owns

- `size-limit` package presence
- size-budget preset package presence
- size config existence and parseability
- size-budget script presence where required
- enforced size-budget policy
- profile gating for roots where size budgets are required at all

## Does not own

- generic package policy
  - that belongs to `ts/package`
- content pipeline/model
  - that belongs to `ts/content`

## Contract direction

This family owns explicit bundle or artifact size budgets.
It should be enabled only where size budgets are part of the intended product surface.
