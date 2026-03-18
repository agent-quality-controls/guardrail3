# Implement ALL additional TS pre-build analysis tools

**Date:** 2026-03-17 18:24 (updated)
**Task:** Everything from ts_additional_analysis.md. Nothing skipped.

## Checks

### Package presence (T-TOOL-*)
| ID | Package | Category |
|---|---|---|
| T-TOOL-01 | cspell | core |
| T-TOOL-02 | type-coverage | core |
| T-TOOL-03 | license-checker | core |
| T-TOOL-04 | prettier | core |
| T-TOOL-05 | size-limit | content |
| T-TOOL-06 | @size-limit/preset-app | content |

### Config/script presence (T-TOOL-*)
| ID | What | Category |
|---|---|---|
| T-TOOL-07 | cspell.json exists | core |
| T-TOOL-08 | type-coverage script in package.json | core |
| T-TOOL-09 | license-check script in package.json | core |
| T-TOOL-10 | audit script in package.json | core |
| T-TOOL-11 | size-limit config in package.json | content |
| T-TOOL-12 | i18n completeness check (next-intl strict OR message file comparison) | content |

### Hook verification (H-TOOL-*)
| ID | What |
|---|---|
| H-TOOL-01 | cspell step in hook |
| H-TOOL-02 | Merge conflict marker check in hook |
| H-TOOL-03 | Lockfile integrity check in hook |
| H-TOOL-04 | Prettier format check in hook |
| H-TOOL-05 | pnpm audit step in hook |

### Hook template additions (pre_commit.rs)
All 5 steps added to the generated hook:
1. Merge conflict markers (first thing, before secrets)
2. Lockfile integrity (after migration check)
3. Prettier format check (before ESLint)
4. cspell on staged files (after ESLint)
5. pnpm audit informational (after cspell)

### i18n check (T-TOOL-12)
Detect if project uses next-intl/react-intl/i18next. If yes:
- Check that multiple locale message files exist
- Check all locale files have the same keys (or that strict mode is configured)
- Auto-skip if no i18n library detected

## Files
- package_check.rs — T-TOOL-01..06/08/09/10
- tool_config_checks.rs (new) — T-TOOL-07/11/12
- hook_script_checks.rs — H-TOOL-01..05
- pre_commit.rs — 5 hook template additions
- mod.rs — wire everything
