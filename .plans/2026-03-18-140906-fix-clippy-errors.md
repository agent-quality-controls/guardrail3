# Fix all remaining clippy errors

**Date:** 2026-03-18 14:09
**Task:** Fix all clippy errors across 4 test files

## Goal
Zero clippy errors when running `cargo clippy --all-targets`.

## Approach
Fix errors in 4 files:
1. `adversarial_ws_discovery.rs:42` — doc comment missing backticks
2. `adversarial_init_roundtrip.rs:369` — eprintln in test
3. `adversarial_diff_detection.rs` — string indexing (x3), eprintln (x2), len comparison
4. `adversarial_nightmare_monorepo.rs` — doc comments (x3), shadow_unrelated (x6), used_underscore_binding (x3)

## Files to Modify
- `apps/guardrail3/tests/adversarial_ws_discovery.rs` — backtick doc comment
- `apps/guardrail3/tests/adversarial_init_roundtrip.rs` — allow eprintln on test fn
- `apps/guardrail3/tests/adversarial_diff_detection.rs` — allow string_slice, print_stderr, fix len_zero
- `apps/guardrail3/tests/adversarial_nightmare_monorepo.rs` — backtick doc comments, allow shadow/underscore on test fns
