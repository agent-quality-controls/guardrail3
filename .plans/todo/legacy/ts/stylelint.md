# TS-STYL — Stylelint checker (6 rules)

**Input:** stylelint config file (.stylelintrc.mjs, .json, .yml, etc.)
**Current code:** `stylelint_check.rs`
**Profile:** Content profile only

## Rules

| New ID | Old ID | What | Description | Status |
|--------|--------|------|-------------|--------|
| TS-STYL-01 | T-STYL-01 | config exists | Stylelint config file found | Implemented |
| TS-STYL-02 | T-STYL-02 | extends standard | `stylelint-config-standard` in extends | Implemented |
| TS-STYL-03 | T-STYL-03 | extends tailwind | `stylelint-config-tailwindcss` in extends | Implemented |
| TS-STYL-04 | T-STYL-04 | a11y plugin | `@double-great/stylelint-a11y` in plugins | Implemented |
| TS-STYL-05 | T-STYL-05 | a11y rules | All 11 required `a11y/*` CSS accessibility rules enabled | Implemented |
| TS-STYL-06 | T-STYL-06 | exceptions | Architecture exceptions (intentionally disabled rules) | Implemented |
