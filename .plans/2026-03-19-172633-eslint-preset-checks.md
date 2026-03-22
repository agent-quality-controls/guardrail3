# Add ESLint preset presence checks T-ESLP-13 and T-ESLP-14

**Date:** 2026-03-19 17:26
**Task:** Add two new ESLint config presence checks for strictTypeChecked and stylisticTypeChecked presets.

## Goal
Two new checks in eslint_check.rs that verify ESLint config contains the tseslint preset strings.

## Approach
1. Add a new function `check_eslint_presets` following the T6 boundary check pattern (content.contains with if/else)
2. Wire it into `check_eslint_config` orchestrator
3. T-ESLP-13: checks for "strictTypeChecked", Error severity
4. T-ESLP-14: checks for "stylisticTypeChecked", Error severity

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/eslint_check.rs` — add new function + wire into orchestrator
