# TS-JSCPD — .jscpd.json checker (10 rules)

**Input:** .jscpd.json (JSON parsed) + source scan for content imports
**Current code:** `jscpd_check.rs`

## Rules

| New ID | Old ID | Setting | Description | Status |
|--------|--------|---------|-------------|--------|
| TS-JSCPD-01 | T19 | file existence | .jscpd.json exists + valid JSON | Implemented |
| TS-JSCPD-02 | T20 | threshold | threshold = 0 (zero tolerance) | Implemented |
| TS-JSCPD-03 | T21 | minTokens | minTokens inventory (non-default value) | Implemented |
| TS-JSCPD-04 | T22 | ignore | Ignore patterns inventory | Implemented |
| TS-JSCPD-05 | T-JSCPD-01 | minTokens field | minTokens field present in config | Implemented |
| TS-JSCPD-06 | T-JSCPD-02 | absolute | `absolute: true` for meaningful monorepo paths | Implemented |
| TS-JSCPD-07 | T-JSCPD-03 | required ignores | Required ignore patterns (node_modules, .next, dist, etc.) | Implemented |
| TS-JSCPD-08 | T-JSCPD-04 | format | `format` field listing scanned languages | Implemented |

## Content-specific rules (checked via jscpd_check.rs but content-profile only)

| New ID | Old ID | What | Description | Status |
|--------|--------|------|-------------|--------|
| TS-CONTENT-01 | T60 | content imports | Content directory import restriction in ESLint | Implemented |
| TS-CONTENT-02 | T61 | velite config | Velite content config existence | Implemented |
