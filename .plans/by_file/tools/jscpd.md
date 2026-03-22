# jscpd

## What it does
Copy-paste detector. Finds duplicated code.

## Config file
`.jscpd.json` or `"jscpd"` in package.json

## Config discovery (verified: cosmiconfig v9 with searchStrategy: 'none')
CWD only. Does NOT walk up (cosmiconfig v9 default `searchStrategy: 'none'` disables parent search).

`--config` / `-c` bypasses cosmiconfig. When passing a scan path (`jscpd apps/`), config still resolves from CWD, not the scan target.

## Shadowing
NO. Only one config used, at CWD.

## Per-scope configs
Per-directory .jscpd.json files (like `apps/validator-rust/.jscpd.json` in steady-parent) require separate invocation with `--config`. Not auto-discovered.

## How to invoke
```bash
pnpm exec jscpd --threshold 10 .
```
From project root. For Rust: prefer cargo-dupes (AST-aware) over jscpd.

## Guardrail3's role
- **Generate/merge:** Ensure base ignore patterns. Leave threshold alone.
- **Validate:** Check config exists, ignores complete
- **Hook:** Run from root (TS) or use cargo-dupes per workspace (Rust)
