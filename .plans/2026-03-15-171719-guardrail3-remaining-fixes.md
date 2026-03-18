# Fix all remaining guardrail3 issues

**Date:** 2026-03-15 17:17
**Task:** Fix 8 priorities: R58 std::fs check, string literal bug, split source_scan.rs, migrate check.rs/diff.rs to crate::fs, exempt test files from R38/R40, exempt .mjs from T30, fix CLAUDE.md, add 3 ESLint rules.

## Goal
All validation checks passing, source_scan.rs under 500 lines, check.rs and diff.rs use centralized fs module.

## Approach

### P1: Add R58 check for direct std::fs usage
- Add `check_direct_fs_usage` to source_scan.rs
- Wire into the per-file check loop
- Catches `use std::fs` imports and inline `std::fs::` calls
- Skip fs.rs itself and modules/ directory (contains string literals with "std::fs::")

### P2: Fix string literal bug in filter_non_comment_lines
- Add `strip_string_literals` function
- Use it before `/*` `*/` detection in filter_non_comment_lines
- Also fix strip_inline_block_comments

### P3: Split source_scan.rs
- source_scan.rs is 860 lines with duplication against allow_checks.rs
- The allow_checks.rs already has copies of functions from source_scan.rs
- Plan: Remove duplicated functions from source_scan.rs (keep them in allow_checks.rs)
- Create structure_checks.rs (R38-R42 file length, use count, unsafe)
- Create code_quality_checks.rs (R43-R44, R58 todo/unwrap/expect, direct fs)
- Keep orchestrator + filter_non_comment_lines + helpers in source_scan.rs

### P4: Migrate check.rs and diff.rs
- Replace `use std::fs; fs::read_to_string` with `crate::fs::read_file_err`

### P5: Exempt test files from R38/R40
- Add is_test check to check_file_length and check_use_count

### P6: Exempt .mjs from T30
- In check_process_env, skip .mjs files

### P7: Fix CLAUDE.md
- Change "minimal" to "service" in guardrails profile line

### P8: Add 3 ESLint rules
- Add restrict-template-expressions, no-throw-literal, no-empty to eslint_check.rs

## Files to Modify
- `src/rs/validate/source_scan.rs` — P1, P2, P3 (major refactor)
- `src/rs/validate/allow_checks.rs` — P3 (already exists, keep as-is)
- `src/rs/validate/structure_checks.rs` — P3 (new file)
- `src/rs/validate/code_quality_checks.rs` — P3 (new file)
- `src/rs/validate/mod.rs` — P3 (register new modules)
- `src/commands/check.rs` — P4
- `src/commands/diff.rs` — P4
- `src/ts/validate/source_scan.rs` — P6
- `src/ts/validate/eslint_check.rs` — P8
- `CLAUDE.md` — P7
