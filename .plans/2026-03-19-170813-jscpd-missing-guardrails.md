# Add missing jscpd guardrails (T-JSCPD-01, T-JSCPD-02, T-JSCPD-03)

**Date:** 2026-03-19 17:08
**Task:** Add three new checks to jscpd_check.rs

## Goal
Add checks for missing `minTokens`, missing `absolute: true`, and required ignore patterns.

## Approach
Insert three new check blocks after T22, following the existing pattern of `json.get()` + match/if-let on `serde_json::Value` variants. Each emits `CheckResult` with appropriate severity and check ID.

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/jscpd_check.rs` — add T-JSCPD-01, T-JSCPD-02, T-JSCPD-03 after existing T22 block
