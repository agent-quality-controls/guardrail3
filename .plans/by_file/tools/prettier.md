# Prettier

## What it does
Code formatter for JS/TS/CSS/JSON/MD.

## Config file
`.prettierrc.{json,yaml,yml,js,cjs,mjs,toml}` or `prettier.config.{js,cjs,mjs}` or `"prettier"` in package.json.
`.prettierrc.*` takes precedence over `prettier.config.*` (silent shadowing).

## Config discovery (verified: cosmiconfig walk-up)
Walk-up from each FILE being formatted. Nearest config wins. No merging.

**Goes all the way to $HOME.** Does NOT stop at .git or package.json boundaries (unlike EditorConfig which does stop). A developer's home `.prettierrc` can interfere with project formatting if the project has no config.

## Shadowing
YES. Subdirectory config completely replaces root for files in that subtree.

## NO `extends`
Prettier has NO `extends` mechanism. To share config, must use JS/MJS format and manually import+spread:
```js
import base from '../../prettier.config.mjs';
export default { ...base, semi: false };
```

## `.prettierignore`
Does NOT walk up. Only the root `.prettierignore` is used. Per-directory ignore files not supported.

## How to invoke
```bash
pnpm exec prettier --check "**/*.{ts,tsx,mjs,json,css}"
```
`--check` for CI/hooks. `--write` for actual formatting.
`--config <path>` disables cosmiconfig search.
`--no-config` uses defaults.

## Guardrail3's role
- **Validate:** Check prettier installed, config exists at root
- **Hook:** Run from root on staged files
- We do NOT generate prettier config — formatting preferences are project-specific
- **Coverage map:** Should check that no subdirectory prettierrc files exist that could shadow root (warn if found)
