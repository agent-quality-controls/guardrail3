# Adversarial tests for T-STYL stylelint checks

**Date:** 2026-03-17 17:21
**Task:** Create adversarial integration tests for T-STYL-01..05 stylelint configuration checks

## Goal
New test file with 5 tests covering missing config, complete config, missing a11y plugin, missing a11y rules, and service-type exclusion.

## Approach
Single new file at `apps/guardrail3/tests/adversarial_ts_stylelint.rs`. Uses `ts validate --format json --inventory` via Command, same pattern as adversarial_categories.rs. Helper creates content-type project with optional stylelint config.

## Files to Modify
- `apps/guardrail3/tests/adversarial_ts_stylelint.rs` — NEW file with 5 tests
