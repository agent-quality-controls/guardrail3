# Fix jscpd coverage engine + audit all TS guardrails

**Date:** 2026-03-19 17:00
**Task:** Fix jscpd non-walk-up coverage logic, then audit every TS guardrail check against steady-parent's actual config to tighten everything to maximum strictness.

## Goal
1. Fix jscpd engine bug — non-walk-up tools should cover all dirs under config, not just at config level
2. Audit every TS guardrail check against steady-parent to find gaps and tighten severity

## Approach

### Step 1: Fix jscpd engine (engine.rs)
Change non-walk-up resolution from `cf.parent() == Some(dir)` to `dir.starts_with(config_dir)`.
A config at directory X covers all source dirs under X.

### Step 2: Audit plan (after jscpd fix)
Compare guardrail3's required baselines against steady-parent's actual configs:
- ESLint: 33 rules at Warn → should be Error; steady-parent has them all at Error
- tsconfig: verify all 13 flags match canonical
- npmrc: verify all 13 settings match canonical
- jscpd: verify threshold=0, minTokens=50
- package.json: verify required tool packages present
- cspell: verify config exists
- prettier: verify config exists (or decide it's not required)

## Files to Modify
- `src/commands/coverage/engine.rs` — fix non-walk-up logic
