# Fix two bugs in coverage engine

**Date:** 2026-03-19 18:25
**Task:** Fix COV-CRIT-01 (shadow detection string prefix) and COV-CRIT-02 (non-walk-up nearest ancestor)

## Goal
Both bugs produce incorrect results due to wrong comparison semantics. Fix both inline.

## Approach

### Bug 1 — COV-CRIT-01 (line 195)
`dir_i.starts_with(dir_j.as_str())` uses String prefix matching. Convert to Path::starts_with which compares components.

### Bug 2 — COV-CRIT-02 (lines 106-110)
`.find()` returns first alphabetical match, not nearest ancestor. Replace with `.filter().max_by_key(component count)`.

## Files to Modify
- `apps/guardrail3/src/commands/coverage/engine.rs` — both fixes
