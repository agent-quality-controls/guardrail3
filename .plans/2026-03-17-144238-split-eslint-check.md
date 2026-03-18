# Split eslint_check.rs to get under 500 lines

**Date:** 2026-03-17 14:42
**Task:** Split eslint_check.rs (688 lines) into two files, each under 500 lines.

## Goal
Both eslint_check.rs and the new eslint_rule_descriptions.rs should be under 500 lines.

## Approach

Extract from eslint_check.rs into new `eslint_rule_descriptions.rs`:
- `eslint_rule_explanation` function (lines 340-457, ~118 lines)
- `RuleValueResult` enum (lines 569-574)
- `check_rule_value` function (lines 580-611)
- `extract_number_from_line` function (lines 616-650)
- `check_eslint_rule` function (lines 460-531)
- `check_eslint_rule_presence` function (lines 652-688)

These are the "rule checking infrastructure" functions used by the config-checking functions.

After extraction:
- eslint_check.rs keeps: check_eslint_config + all T-check functions (~338 lines) + imports from new module
- eslint_rule_descriptions.rs gets: rule explanation + rule checking infra (~350 lines)

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/eslint_check.rs` — remove extracted functions, add `use super::eslint_rule_infra::*`
- `apps/guardrail3/src/app/ts/validate/eslint_rule_infra.rs` — new file with extracted functions
- `apps/guardrail3/src/app/ts/validate/mod.rs` — add `mod eslint_rule_infra;`
