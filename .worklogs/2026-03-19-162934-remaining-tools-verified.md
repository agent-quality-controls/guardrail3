# Verify remaining 5 tools: stylelint, prettier, cspell, npmrc, jscpd

**Date:** 2026-03-19 16:29
**Scope:** `src/commands/coverage/{stylelint,prettier,cspell,npmrc,jscpd}.rs`

## Summary
Verified config resolution for the remaining 5 TypeScript tools. Prettier and jscpd verified empirically on local machine. Stylelint, cspell, npmrc verified via source code analysis and official docs.

## Verification Results

| Tool | Method | Walk-up? | Per-file? | Extends/Merge? | Shadow? |
|---|---|---|---|---|---|
| stylelint 17.4 | source (cosmiconfig `global`) | YES to $HOME | YES per-file | extends deep-merges | YES |
| prettier 3.8.1 | empirical | YES | YES per-file | NO extends | YES |
| cspell | docs (cspell.org) | YES | YES from CWD/file | `import` (explicit) | YES |
| npmrc | npm source code | YES (to package.json) | NO (project root) | NO (cascade: project > user > global) | YES |
| jscpd | empirical + source | NO | N/A (CWD only) | N/A | N/A |

## Key Findings

### jscpd does NOT use cosmiconfig
Previous comment claimed "cosmiconfig v9 default `searchStrategy: 'none'`" — this is wrong. jscpd has NO cosmiconfig dependency at all. It uses a hand-rolled `path.resolve(".jscpd.json")` from CWD. Also checks `package.json` `"jscpd"` key in CWD.

### stylelint explicitly uses `searchStrategy: 'global'`
Despite cosmiconfig v9 defaulting to `'none'` (no walk-up), stylelint 17.4.0 explicitly sets `searchStrategy: 'global'` for backward compatibility. Walk-up to $HOME is preserved.

### prettier empirical confirmation
`npx prettier --find-config-path` from subdir finds parent `.prettierrc`. Intermediate config at subdir shadows parent completely.

### npmrc resolution is NOT walk-up per se
npm walks up to find `localPrefix` (first dir with `package.json` or `node_modules`), then loads `.npmrc` from that project root. It's walk-up to find the project, not walk-up per config file.

## Open Questions
- cspell: does it search per-file or per-CWD? Docs say both "current directory" and "files being checked" — unclear without empirical test
