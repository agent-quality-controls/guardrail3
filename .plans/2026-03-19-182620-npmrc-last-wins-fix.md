# Fix NPM-01: pnpm last-wins semantics for .npmrc duplicate keys

**Date:** 2026-03-19 18:26
**Task:** Fix two issues in npmrc_check.rs — use last-match semantics and detect duplicate keys.

## Goal
guardrail3 should read .npmrc the same way pnpm does (last value wins for duplicate keys), and flag duplicate keys as a potential attack vector.

## Approach

### Fix 1: Change `find()` to `rfind()` equivalent
In `check_expected_settings`, line 93 uses `settings.iter().find(|(k, _)| k == key)` which returns the first match. Change to `settings.iter().rfind(|(k, _)| k == key)` to get the last match, matching pnpm's last-wins behavior.

### Fix 2: Add duplicate key detection
Add a new function `check_duplicate_keys` that:
1. Iterates through settings, counting occurrences of each key
2. For any key appearing more than once, emits a `CheckResult` with id `T-NPMRC-01`, severity Error
3. Call this from `check_npmrc` after parsing

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/npmrc_check.rs` — both fixes
