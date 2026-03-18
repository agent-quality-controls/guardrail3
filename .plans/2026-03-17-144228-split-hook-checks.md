# Split hook_checks.rs to get under 500 lines

**Date:** 2026-03-17 14:42
**Task:** Split hook_checks.rs (627 lines) into two files, each under 500 lines.

## Goal
Both hook_checks.rs and the new hook_script_checks.rs should be under 500 lines.

## Approach

Extract these functions to `hook_script_checks.rs`:
- `PatternCheck` struct + `HOOK_PATTERN_CHECKS` const (lines 360-439, ~80 lines)
- `check_monolithic_patterns` (lines 441-479, ~39 lines)
- `check_modular_scripts` (lines 481-505, ~25 lines)
- `check_dispatcher_pattern` (lines 155-205, ~51 lines)
- `emit_script_stats` (lines 253-289, ~37 lines)
- `inventory_scripts` (lines 571-627, ~57 lines)
- `check_local_scripts` (lines 291-309, ~19 lines)

That's ~308 lines for the new file (with imports). Remaining in hook_checks.rs: ~319 lines.

## Files to Modify
- `apps/guardrail3/src/app/hooks/hook_script_checks.rs` — new file with extracted functions
- `apps/guardrail3/src/app/hooks/hook_checks.rs` — remove extracted functions, add `use super::hook_script_checks::*`
- `apps/guardrail3/src/app/hooks/mod.rs` — add `mod hook_script_checks;`
