# Adversarial tests for TS per-app type profile system

**Date:** 2026-03-17 16:11
**Task:** Write 9 adversarial integration tests targeting edge cases in the per-app type profile system

## Goal
Append 9 tests to `adversarial_categories.rs` that probe edge cases: typos in type field, name mismatches, ghost configs, library type, case sensitivity, check overrides, mixed monorepos, missing apps dir, and global vs type default precedence.

## Approach
Use existing helpers (`setup_ts_monorepo`, `run_ts_validate`, `collect_check_ids`, `assert_*`). Each test creates a temp TS monorepo with specific config edge cases and asserts expected behavior.

## Key observations from code review
- `from_str_or_default` does exact match: "content" and "library" only, everything else -> Service
- This means typos, wrong case ("Content"), unknown strings all become Service
- `resolve_app_contexts` looks up apps by directory name in the config map
- If no config entry matches the dir name, `app_cfg` is None -> defaults to Service
- Ghost configs (no matching dir) are simply never iterated (discovery is disk-based)
- `categories.architecture` at line 45 of mod.rs gates the entire arch section globally
- If global architecture=false, `resolve_app_contexts` is never called, so per-app types are irrelevant

## Files to Modify
- `apps/guardrail3/tests/adversarial_categories.rs` -- append 9 tests
