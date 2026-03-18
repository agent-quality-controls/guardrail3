# Step 31: Verify Adversarial Fixtures After Syn Migration

**Date:** 2026-03-16 08:52
**Task:** Check which GREP_BUG tests now pass correctly and update markers

## Goal
Document the improvement from syn migration by updating GREP_BUG markers to FIXED where bugs are eliminated.

## Input Information
- All 40 adversarial tests pass
- 1 test already marked FIXED (string_unwrap.rs - R44 false positive eliminated)
- 2 tests still marked GREP_BUG and still exhibit the bug:
  - `multiline_string.rs` (R32 false positive) - multiline string fools line-by-line scanner
  - `cfg_gated_use.rs` (R58 false positive) - `#[cfg(test)] use std::fs` still flagged
- Both GREP_BUG tests assert the buggy behavior and pass, meaning those bugs are NOT fixed

## Approach
No code changes needed. The two remaining GREP_BUG tests still exhibit their bugs. Report status.

## Summary
- 1 FIXED (string_unwrap.rs)
- 2 GREP_BUG remaining (multiline_string.rs, cfg_gated_use.rs)
- Both remaining bugs are in checks that weren't fully migrated to syn AST
