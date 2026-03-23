# TS-NPMRC — .npmrc checker (5 rules)

**Input:** .npmrc (line-based key=value)
**Current code:** `npmrc_check.rs`

## Rules

| New ID | Old ID | Setting | Description | Status |
|--------|--------|---------|-------------|--------|
| TS-NPMRC-01 | T11 | file existence | `.npmrc` config file exists | Implemented |
| TS-NPMRC-02 | T12 | missing setting | Expected setting missing entirely | Implemented |
| TS-NPMRC-03 | T13 | wrong value | Expected setting present but wrong value | Implemented |
| TS-NPMRC-04 | T14 | extra settings | Inventory of non-baseline settings | Implemented |
| TS-NPMRC-05 | T-NPMRC-01 | duplicate key | Duplicate key detection (pnpm last-wins) | Implemented |
