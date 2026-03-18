# Adversarial integration tests for path resolution and TS generate

**Date:** 2026-03-18 01:11
**Task:** Write 2 integration test files testing RS path resolution and TS generate correctness

## Goal
Two new test files that exercise edge cases in app name -> path resolution (RS) and TS config generation correctness.

## Approach
1. Read existing test pattern from `adversarial_generate.rs`
2. Write `adversarial_path_resolution.rs` (6 tests) for RS generate path logic
3. Write `adversarial_ts_generate.rs` (7 tests) for TS generate config correctness
4. Run `cargo test` to verify compilation and passing tests

## Files to Modify
- `apps/guardrail3/tests/adversarial_path_resolution.rs` — new file
- `apps/guardrail3/tests/adversarial_ts_generate.rs` — new file
