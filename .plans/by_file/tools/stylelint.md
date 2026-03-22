# Stylelint

## What it does
CSS linter. Code quality + accessibility rules.

## Config file
`.stylelintrc.{json,yaml,yml,js,cjs,mjs}` or `stylelint.config.{js,cjs,mjs}` or `"stylelint"` in package.json.
`.stylelintrc.*` takes precedence over `stylelint.config.*` if both exist (silent shadowing).

## Config discovery (verified: cosmiconfig walk-up)
Walk-up from each LINTED FILE. Nearest config wins. No cascade.

Cosmiconfig has `stopDir` internally but stylelint does NOT expose it as CLI flag. Only way to stop walk-up: `--config <path>` (disables cosmiconfig entirely).

## Shadowing
YES. `apps/landing/.stylelintrc.mjs` shadows root for all CSS in `apps/landing/`. All root guardrails (a11y rules) lost for that app.

## `extends` behavior
Unlike most tools: `extends` MERGES rules, doesn't replace.
- Single-value properties (`customSyntax`): replaced by child
- Arrays/objects (`rules`, `plugins`): merged. Last item in extends array has highest precedence.
- The direct config's rules always win over extended configs.

## How to invoke
```bash
pnpm exec stylelint --max-warnings 0 "**/*.css"
```
From project root. Walk-up finds nearest config per file.

## Guardrail3's role
- **Generate:** Shadow-import — engine with a11y rules, user's config extends it
- **Validate:** Check a11y plugin + rules present. WARN on per-app stylelint configs that don't extend the engine.
- **Hook:** Run from root on staged CSS files
- **Coverage map:** Walk-up simulation per TS app showing which stylelint config covers each
