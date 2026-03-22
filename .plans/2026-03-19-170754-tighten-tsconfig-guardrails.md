# Tighten tsconfig guardrails

**Date:** 2026-03-19 17:07
**Task:** Edit tsconfig_check.rs to tighten guardrails: move isolatedModules to Error, add esModuleInterop as required bool, add string checks for target/module/moduleResolution.

## Goal
Stricter tsconfig validation: fewer warnings, more errors, new string-value checks.

## Approach

1. Move `isolatedModules` from `warn_bools` to `additional_required_bools` (T54 ID preserved)
2. Add `esModuleInterop` to `additional_required_bools` with new check behavior (needs ID — will use existing pattern, likely T54 area — actually needs a unique ID. Looking at existing IDs: T60-T64 are taken. Will use T65? No, T65-T67 are reserved for string checks. esModuleInterop needs an ID too. The user didn't specify one — I'll check what's available. T54 was isolatedModules. esModuleInterop could be T68, but let me re-read: the user says "Add it alongside the other required bools" — I'll assign it a new ID. Looking at the pattern, T60-T64 are used. T65-T67 are for string checks. So esModuleInterop gets T68? Actually, let me just pick the next available after T67. T68 it is.)
3. Add string-value checks for target (T65), module (T66), moduleResolution (T67)
4. Add explanation entries for esModuleInterop, target, module, moduleResolution
5. Empty out warn_bools (isolatedModules was the only entry)

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/tsconfig_check.rs`
