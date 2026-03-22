# TS-TOOL — Tool config checker (6 rules)

**Input:** Various tool config files (cspell.json, package.json scripts, size-limit)
**Current code:** `tool_config_checks.rs`, `i18n_check.rs`

## Rules

| New ID | Old ID | Tool | Description | Status |
|--------|--------|------|-------------|--------|
| TS-TOOL-07 | T-TOOL-07 | cspell | cspell.json config file exists | Implemented |
| TS-TOOL-08 | T-TOOL-08 | type-coverage | `type-coverage` script in package.json | Implemented |
| TS-TOOL-09 | T-TOOL-09 | license-checker | `license-check` script in package.json | Implemented |
| TS-TOOL-10 | T-TOOL-10 | audit | `audit` script in package.json | Implemented |
| TS-TOOL-11 | T-TOOL-11 | size-limit | `size-limit` config in package.json (content profile) | Implemented |
| TS-TOOL-12 | T-TOOL-12 | i18n | Locale file completeness (recursive key comparison) | Implemented |
