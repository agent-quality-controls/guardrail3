# TypeScript (tsc)

## What it does
Type checks TypeScript code. Does NOT lint — only checks types.

## Config file
`tsconfig.json`

## Config discovery (verified from TS docs + research)

**With `-p <path>`:** Uses the specified tsconfig. No discovery.

**Without `-p` and without file arguments:** Walks UP from CWD looking for tsconfig.json. First found wins.

**With file arguments but no `-p`:** Does NOT search for tsconfig — uses command-line compiler options only.

## `extends` semantics
- `compilerOptions`: deep-merged. Child wins on conflicts.
- `include`, `exclude`, `files`: inherited if ABSENT in child. Completely REPLACED if child defines them. Paths resolve relative to the config file that declares them.
- `references`: NOT inherited via extends. Must be in the config that uses them.
- `paths` in `compilerOptions`: completely REPLACED, not merged. Per-app configs must duplicate base paths.
- Array extends (since TS 5.0): `"extends": ["@tsconfig/node20", "./local.json"]`. Later entries win.
- Extends from node_modules: supported since TS 3.2. Bare specifiers use Node resolution.

## Composite projects / references
- Referenced projects must have `composite: true`
- Plain `tsc` does NOT build dependencies — need `tsc --build`
- `tsc --build` detects base config changes via `.tsbuildinfo`

## How to invoke
```bash
pnpm exec tsc -p apps/landing/tsconfig.json --noEmit
pnpm exec tsc -p apps/admin/tsconfig.json --noEmit
```
Per app/package. Each needs its own tsconfig.json pointed to explicitly.

Hook should discover all tsconfig.json files from crawler, NOT hardcode `apps/*/tsconfig.json`.

## Guardrail3's role
- **Generate/merge:** Ensure strict flags in `tsconfig.base.json` (root). DON'T generate per-app tsconfigs.
- **Validate:** Check per-app tsconfigs extend base OR have all strict flags inline. Check each TS app has tsconfig.
- **Hook:** Run `tsc -p <tsconfig> --noEmit` for each discovered tsconfig.json
- **Coverage map:** Show which tsconfigs extend base, which are standalone, which apps have no tsconfig
