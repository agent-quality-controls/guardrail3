# Create src/rs/validate/test_checks.rs with 8 test quality checks

**Date:** 2026-03-15 19:26
**Task:** Implement R-TEST-01 through R-TEST-08 in a new test_checks.rs module

## Goal
New file with all 8 checks + public orchestrator + ~17 tests covering positive and negative cases for each check.

## Approach

### Step-by-step plan
1. Create `src/rs/validate/test_checks.rs` with:
   - `check(workspace_root)` public orchestrator
   - 8 check functions following existing patterns from dependency_scan.rs and code_quality_checks.rs
   - Helper functions for content-based checks (R-TEST-04, R-TEST-05, R-TEST-07) that accept `&str` for testability
   - Tests module with ~17 tests using tempdir for filesystem checks, direct content for scan checks

### Key decisions
- **Use `std::process::Command` for R-TEST-01:** Same pattern as dependency_scan::check_tool_installed with `#[allow(clippy::disallowed_methods)]` + reason
- **Helper functions take `&str`:** For R-TEST-04/05/07, extract content-scanning logic into helpers that tests can call directly
- **walkdir for directory traversal:** Already a dependency, used in source_scan.rs
- **crate::fs for file reads:** Required by codebase rules

## Files to Modify
- `src/rs/validate/test_checks.rs` — NEW file with all 8 checks and tests
