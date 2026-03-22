# gitleaks

## What it does
Scans for secrets (API keys, passwords, tokens) in staged files.

## Config file
`.gitleaks.toml` (dot prefix required for auto-discovery). `gitleaks.toml` without dot requires `--config`.

## Config discovery (verified from research)
Looks for `.gitleaks.toml` in the TARGET PATH (source root), NOT CWD. No walk-up.

Resolution order: `--config` flag > `GITLEAKS_CONFIG` env var > `<source>/.gitleaks.toml` > built-in defaults.

No per-directory configs. No merging. Single config for the entire scan.

## How to invoke
```bash
gitleaks protect --staged --no-banner
```
From project root. Checks all staged files regardless of language or location. One run covers everything.

## Guardrail3's role
- **Validate:** Check gitleaks installed
- **Hook:** Run once from project root
