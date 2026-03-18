# Fix 59 clippy errors in test code

**Date:** 2026-03-15 20:33
**Task:** Fix all clippy errors blocking commit

## Goal
Zero clippy errors with `cargo clippy --tests -- -D warnings`. All 139 tests still pass.

## Approach

### Errors by category:
1. **expect_used** (47 total): Add `#[allow(clippy::expect_used)]` to individual test functions
2. **disallowed_methods** (10 total): Add `#[allow(clippy::disallowed_methods)]` to test functions using stdfs::
3. **needless_raw_string_hashes** (1): garde_checks.rs line 566 - change `r#"..."#` to `r"..."`
4. **doc_markdown** (1): garde_checks.rs line 203 - backtick `reqwest::Response::json`

### Files to modify:
- `src/rs/validate/garde_checks.rs` - doc_markdown + needless_raw_string_hashes
- `src/rs/validate/release_crate_checks.rs` - expect_used on 14 test fns
- `src/rs/validate/release_crate_deps.rs` - expect_used on 11 test fns
- `src/rs/validate/release_bin_checks.rs` - expect_used on 2 test fns
- `src/rs/validate/test_checks.rs` - expect_used + disallowed_methods on test fns
- `src/rs/validate/test_quality_checks.rs` - expect_used + disallowed_methods on test fns
- `src/ts/validate/test_checks.rs` - already has annotations, verify
