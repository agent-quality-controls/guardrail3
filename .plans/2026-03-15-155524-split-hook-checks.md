# Split hook_checks.rs to under 500 effective lines

**Date:** 2026-03-15 15:55
**Task:** Split hook_checks.rs (592 lines) so it passes the 500-line guardrail

## Goal
Move tool-related check functions into a new `tool_checks.rs` module, reducing `hook_checks.rs` to under 500 effective lines.

## Approach
Extract these functions (~155 lines total) into `src/hooks/tool_checks.rs`:
- `check_required_tools` (lines 513-548) — H8 tool installation checks
- `check_duplication_tools` (lines 192-267) — H12 duplication tool checks

Update `hook_checks.rs` to import from `tool_checks` via `super::tool_checks::`.
Update `mod.rs` to declare `mod tool_checks;`.

## Files to Modify
- `src/hooks/tool_checks.rs` — NEW, extracted functions
- `src/hooks/hook_checks.rs` — remove extracted functions, add imports
- `src/hooks/mod.rs` — add `mod tool_checks;`
