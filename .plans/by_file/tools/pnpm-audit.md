# pnpm audit

## What it does
Checks npm packages for known security vulnerabilities.

## Config file
None (uses .npmrc for registry config).

## Config discovery
N/A. Audits the entire lockfile.

## How to invoke
```bash
pnpm audit --prod
```
From project root. Checks ALL workspace packages — `--filter` is NOT supported with `pnpm audit`. Cannot scope to individual packages.

## Guardrail3's role
- **Validate:** Check audit script exists in package.json
- **Hook:** Run once from project root
