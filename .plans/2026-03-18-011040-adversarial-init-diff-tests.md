# Adversarial integration tests for init roundtrip and diff detection

**Date:** 2026-03-18 01:10
**Task:** Write two adversarial integration test files for guardrail3

## Goal
Create `adversarial_init_roundtrip.rs` (5 tests) and `adversarial_diff_detection.rs` (5 tests) following the exact pattern from `adversarial_generate.rs`.

## Approach
- Copy the suppress lines, helper pattern, and Command usage from existing tests
- Each test uses tempfile::tempdir() for isolation
- Tests document known bugs (AV-8.5: rs generate --dry-run shows ALL files) via assertions that expect correct behavior

## Key decisions
- `rs generate --dry-run` routes through `diff::run` which calls `generate_expected` (generates ALL files). Test 5 in roundtrip documents this bug.
- `ts init --force` uses `replace_typescript_section` which skips lines starting with `[typescript` but stops at other `[section]` headers, so `[rust]` should be preserved.

## Files to Modify
- `apps/guardrail3/tests/adversarial_init_roundtrip.rs` -- new file, 5 tests
- `apps/guardrail3/tests/adversarial_diff_detection.rs` -- new file, 5 tests
