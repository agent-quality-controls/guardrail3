# ESLint

## What it does
JavaScript/TypeScript linter. Plugin-based rules.

## Config file
`eslint.config.{js,mjs,cjs,ts,mts,cts}` (flat config). JS variants take precedence over TS in same dir.

## Config discovery (verified: ESLint v10 released Feb 2026)

**v10 (CURRENT — Feb 2026+):** Walk-up from each LINTED FILE. Nearest `eslint.config.*` wins. No cascade, no merging. This is the DEFAULT behavior — the `unstable_config_lookup_from_file` flag was removed because it's now always on.

**v9 (legacy):** CWD-based. Only root config used. Per-app configs ignored unless running from that directory or using `--flag v10_config_lookup_from_file`.

**`--config <path>`:** Overrides discovery completely. One config for all files.

## Shadowing
YES (v10). `apps/landing/eslint.config.mjs` automatically applies to all files in `apps/landing/` even when running `eslint apps/` from root. Root config is never reached for those files. ALL root guardrails silently lost.

This is NOT theoretical — it's the default behavior since v10.

## Per-app config interaction
- Per-app config REPLACES root entirely for files in that subtree
- To inherit root rules, per-app must explicitly `import` root or the engine
- Framework configs (eslint-config-next) work in the flat config array — later entries override earlier
- Plugin resolution in pnpm strict mode can fail if plugins aren't hoisted

## Other changes in v10
- `.eslintignore` is DEAD. Use `ignores` in config or `includeIgnoreFile` from `@eslint/compat`
- `eslint.config.ts` stable since v9.18.0 (needs jiti >= 2.0)

## How to invoke
```bash
pnpm exec eslint --max-warnings 0 apps/ packages/
```
From project root. v10 walks up per file automatically.

## Guardrail3's role
- **Generate:** Shadow-import — engine to `.guardrail3/generated/eslint-engine.mjs`, user's root + per-app configs import it
- **Validate:** Check root config imports engine. Check per-app configs import engine. WARN on per-app configs that DON'T import engine (enforcement gap since v10 walk-up means they WILL be used).
- **Hook:** Run from root. v10 handles per-file config resolution automatically.
- **Coverage map:** Walk-up simulation per TS app/package showing which eslint config covers each. Critical now that v10 makes this the default.
