# Split 3 oversized files under 500-line limit

**Date:** 2026-03-15 20:01
**Task:** Split release_crate_checks.rs (927 lines), release_repo_checks.rs (833 lines), test_checks.rs (763 lines)

## Goal
Each resulting file under 500 lines. All 137 tests pass. cargo build succeeds.

## Approach

### File 1: release_crate_checks.rs -> release_crate_checks.rs + release_crate_deps.rs
- Keep R-PUB-01 through R-PUB-08 + their tests in release_crate_checks.rs
- Move R-PUB-09 (dry run), R-PUB-10 (path deps), R-PUB-11 (version consistency) + helpers (version_satisfies, parse_version_parts, is_valid_semver) + their tests to release_crate_deps.rs
- Update check_per_crate to call into new module
- Make necessary items pub(super)

### File 2: release_repo_checks.rs -> release_repo_checks.rs + release_bin_checks.rs
- Keep R-REL-01 through R-REL-08 + their tests in release_repo_checks.rs
- Move R-BIN-01 through R-BIN-03 + their tests to release_bin_checks.rs
- Move check_binary() orchestrator and read_workflow_files helper (needed by both)

### File 3: test_checks.rs -> test_checks.rs + test_quality_checks.rs
- Keep R-TEST-01 through R-TEST-04 + their tests in test_checks.rs
- Move R-TEST-05 through R-TEST-08 + their tests to test_quality_checks.rs
- Keep check() orchestrator in test_checks.rs, call test_quality_checks

## Files to Modify
- `src/rs/validate/mod.rs` - add 3 new module declarations
- `src/rs/validate/release_crate_checks.rs` - remove dep checks
- `src/rs/validate/release_crate_deps.rs` - new file with dep checks
- `src/rs/validate/release_repo_checks.rs` - remove binary checks
- `src/rs/validate/release_bin_checks.rs` - new file with binary checks
- `src/rs/validate/release_checks.rs` - update binary check call path
- `src/rs/validate/test_checks.rs` - remove quality checks
- `src/rs/validate/test_quality_checks.rs` - new file with quality checks
