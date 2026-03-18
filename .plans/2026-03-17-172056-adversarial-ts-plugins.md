# Adversarial integration tests for T-PLUG package checks

**Date:** 2026-03-17 17:20
**Task:** Write adversarial integration tests for T-PLUG checks

## Goal
Create a new test file that exercises the T-PLUG-01..10 check IDs from `package_check::check_lint_plugins`.

## Approach
Create `/apps/guardrail3/tests/adversarial_ts_plugins.rs` with 6 test cases covering:
1. Missing core plugin fires error
2. Present core plugin does not fire error
3. All 4 core plugins present = no T-PLUG errors for 01/02/03/10
4. Content plugins not checked without content type config
5. Content plugins checked with content type config
6. All content plugins present = no T-PLUG-04..09 errors

Uses same pattern as `adversarial_categories.rs` but with TS project setup helpers.

## Files to Modify
- `apps/guardrail3/tests/adversarial_ts_plugins.rs` -- new file
