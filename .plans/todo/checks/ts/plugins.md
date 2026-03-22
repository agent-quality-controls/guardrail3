# TS-PLUG — Plugin/tool dependency checker (25 rules)

**Input:** package.json devDependencies + dependencies (JSON parsed)
**Current code:** `package_deps.rs`

## Plugin dependencies (19 rules)

| New ID | Old ID | Package | Description | Status |
|--------|--------|---------|-------------|--------|
| TS-PLUG-01 | T-PLUG-01 | eslint-plugin-unicorn | Must be in devDeps | Implemented |
| TS-PLUG-02 | T-PLUG-02 | eslint-plugin-regexp | Must be in devDeps | Implemented |
| TS-PLUG-03 | T-PLUG-03 | eslint-plugin-sonarjs | Must be in devDeps | Implemented |
| TS-PLUG-04 | T-PLUG-04 | eslint-plugin-jsx-a11y | Must be in devDeps (content profile) | Implemented |
| TS-PLUG-05 | T-PLUG-05 | stylelint | Must be in devDeps (content profile) | Implemented |
| TS-PLUG-06 | T-PLUG-06 | @double-great/stylelint-a11y | Must be in devDeps (content profile) | Implemented |
| TS-PLUG-07 | T-PLUG-07 | stylelint-config-standard | Must be in devDeps (content profile) | Implemented |
| TS-PLUG-08 | T-PLUG-08 | stylelint-config-tailwindcss | Must be in devDeps (content profile) | Implemented |
| TS-PLUG-09 | T-PLUG-09 | eslint-plugin-tailwind-ban | Must be in devDeps (content profile) | Implemented |
| TS-PLUG-10 | T-PLUG-10 | knip | Must be in devDeps | Implemented |
| TS-PLUG-11 | T-PLUG-11 | scripts.knip | knip script must exist | Implemented |
| TS-PLUG-12 | T-PLUG-12 | eslint | Must be in devDeps | Implemented |
| TS-PLUG-13 | T-PLUG-13 | typescript | Must be in devDeps | Implemented |
| TS-PLUG-14 | T-PLUG-14 | typescript-eslint | Must be in devDeps | Implemented |
| TS-PLUG-15 | T-PLUG-15 | eslint-plugin-import-x | Must be in devDeps | Implemented |
| TS-PLUG-16 | T-PLUG-16 | eslint-import-resolver-typescript | Must be in devDeps | Implemented |
| TS-PLUG-17 | T-PLUG-17 | eslint-plugin-boundaries | Must be in devDeps | Implemented |
| TS-PLUG-18 | T-PLUG-18 | only-allow | Must be in devDeps | Implemented |
| TS-PLUG-19 | T-PLUG-19 | jscpd | Must be in devDeps | Implemented |

## Tool dependencies (6 rules)

| New ID | Old ID | Package | Description | Status |
|--------|--------|---------|-------------|--------|
| TS-TOOL-01 | T-TOOL-01 | cspell | Must be in devDeps | Implemented |
| TS-TOOL-02 | T-TOOL-02 | type-coverage | Must be in devDeps | Implemented |
| TS-TOOL-03 | T-TOOL-03 | license-checker | Must be in devDeps | Implemented |
| TS-TOOL-04 | T-TOOL-04 | prettier | Must be in devDeps | Implemented |
| TS-TOOL-05 | T-TOOL-05 | size-limit | Must be in devDeps (content profile) | Implemented |
| TS-TOOL-06 | T-TOOL-06 | @size-limit/preset-app | Must be in devDeps (content profile) | Implemented |
