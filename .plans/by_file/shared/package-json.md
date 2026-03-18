# package.json (root + per-app)

## Category: Validate-only

guardrail3 NEVER writes to package.json. Too much project content (deps, scripts, metadata).

## What validate checks

### Root package.json
- `packageManager` field present (T18)
- `engines` field present (T57)
- `preinstall` script contains `only-allow pnpm` (T55)
- `prepare` script present (T56)
- Banned packages not in dependencies or devDependencies (T17): axios, lodash, moment, uuid, nanoid, pg, express, classnames, winston, pino, request, got, superagent, node-fetch, isomorphic-fetch, underscore
- Required devDependencies present (T-PLUG checks): eslint-plugin-unicorn, eslint-plugin-regexp, eslint-plugin-sonarjs, knip, cspell, type-coverage, license-checker, prettier
- For content apps: eslint-plugin-jsx-a11y, stylelint, @double-great/stylelint-a11y, stylelint-config-standard, stylelint-config-tailwindcss, eslint-plugin-tailwind-ban, size-limit, @size-limit/preset-app
- `pnpm.overrides` has required overrides: zod, @eslint/js (T15-T16)

### Per-app package.json
- Same banned packages check
- Required devDependencies for app type
- Scripts: type-coverage, license-check, audit (T-TOOL-08..10)
- For content: size-limit config (T-TOOL-11)

## No merge, no override mechanism

Validation only. User fixes issues manually based on validate output.
