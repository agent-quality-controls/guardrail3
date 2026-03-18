# Fix all guardrail3 bugs and issues

**Date:** 2026-03-15 12:20
**Task:** Fix 17 categories of bugs across guardrail3 codebase

## Goal
All check IDs match the specification, dead code is fixed, severity levels are correct, score formula works properly, and cargo build succeeds.

## Approach

### Critical fixes
1. Move check_dependency_direction out of .rs file loop into separate Cargo.toml iteration
2. Fix ALL check ID mappings across every file
3. Remove lib.rs from is_bin_crate_entry
4. Fix severity mismatches (R24=Error, R45-R48=Error, R22=Warn, R16=Error)
5. Fix score formula (only errors affect score)

### Medium fixes
6. Fix unsafe detection in strings/comments
7. Fix multi-line #[allow] handling
8. Fix cfg_attr(allow) requiring justification
9. Add missing deny.toml checks
10. Add per-crate clippy.toml content validation
11. Fix unreachable! severity

### Low priority fixes
12. Add R23 check (rustfmt extra settings)
13. Add R52 check (dependency graph inventory)
14. Add R55-R57 checks
15. Fix garde(skip) matching in comments
16. Fix workspace exclude patterns
17. Add block comment tracking

## Files to Modify
- src/rs/validate/source_scan.rs — fixes 1, 3, 6, 7, 8, 11, 15, 17, check IDs
- src/rs/validate/config_files.rs — fix check IDs, add R23, R55-R57
- src/rs/validate/deny_audit.rs — fix check IDs, add missing checks
- src/rs/validate/clippy_coverage.rs — fix check IDs, add per-crate validation
- src/rs/validate/dependency_scan.rs — fix severity R45-R48
- src/rs/validate/cargo_lints.rs — fix check IDs
- src/report/types.rs — fix score formula
- src/discover.rs — fix workspace exclude
- src/rs/validate/mod.rs — wire up new dependency direction + R52
