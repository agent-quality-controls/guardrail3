# Move ValidateDomains to report/types.rs

**Date:** 2026-03-16 12:29
**Task:** Move ValidateDomains struct from rs/validate/mod.rs to report/types.rs and update all imports.

## Goal
ValidateDomains lives alongside CheckResult, Section, Report in report/types.rs.

## Approach
1. Remove struct from src/rs/validate/mod.rs
2. Add struct to src/report/types.rs
3. Update 5 import sites
4. Add re-import in rs/validate/mod.rs

## Files to Modify
- `src/report/types.rs` — add ValidateDomains struct
- `src/rs/validate/mod.rs` — remove struct, add import
- `src/main.rs` — update import path
- `src/commands/validate.rs` — update import path
- `src/ts/validate/mod.rs` — update import path
- `src/hooks/validate.rs` — update import path
