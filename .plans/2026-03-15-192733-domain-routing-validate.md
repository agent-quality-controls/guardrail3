# Wire ValidateDomains into validate command

**Date:** 2026-03-15 19:27
**Task:** Build ValidateDomains struct from CLI args in src/commands/validate.rs

## Goal
The validate command should construct a ValidateDomains struct from the new CLI flags (code, architecture, release, tests). When no flags are set, all domains run. The struct is built but not yet passed downstream (rs/ts/hooks signatures unchanged).

## Approach
1. Add `use crate::rs::validate::ValidateDomains;` import
2. After `let args`, build the domains struct with the run_all logic
3. Compile and verify

## Files to Modify
- `src/commands/validate.rs` — add import + build ValidateDomains after args
