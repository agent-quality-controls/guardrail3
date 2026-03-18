# Add 5 hook template steps and 5 verification checks

**Date:** 2026-03-17 18:29
**Task:** Add merge conflict markers, lockfile integrity, prettier, cspell, pnpm audit to pre-commit template and corresponding H-TOOL-01 through H-TOOL-05 checks

## Goal
Pre-commit template gains 5 new steps; hook_script_checks gains 5 new check functions; hook_checks wires them in.

## Approach

### Part 1: pre_commit.rs
1. Merge conflict markers — after "No staged files" exit (line 23), before secret scanning (line 25)
2. Lockfile integrity — after migration consistency (line 71), before stack detection (line 73)
3. Prettier — in TS section, before ESLint (line 115)
4. cspell — in TS section, after ESLint (line 119)
5. pnpm audit — after cspell, still in TS section

### Part 2: hook_script_checks.rs
Add 5 functions after check_stylelint_hook (line 297)

### Part 3: hook_checks.rs
Wire the 5 new checks into check_hook_structure, after the stylelint call.
H-TOOL-02 (conflict) and H-TOOL-03 (lockfile) are universal; rest gated on has_typescript.

## Files to Modify
- `apps/guardrail3/src/domain/modules/pre_commit.rs`
- `apps/guardrail3/src/app/hooks/hook_script_checks.rs`
- `apps/guardrail3/src/app/hooks/hook_checks.rs`
