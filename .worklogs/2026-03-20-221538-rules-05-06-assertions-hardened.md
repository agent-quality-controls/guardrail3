# Rules 05+06 assertions hardened

**Date:** 2026-03-20 22:15

## Summary
Fixed 52 assertion FAILs across rules 05 and 06 (31 + 21). Test logic unchanged — only assertion strength improved.

## Rule 05 fixes (31→0 FAIL)
- assert_per_app/inner_hex/file_field now operate on &r5 not &errors
- is_empty() → assert_eq!(len, 0) everywhere
- Added assert_no_ts_apps + assert_no_packages on &r5 to all tests
- Added total r5.len() to GROUP H edge-case tests
- Added per-app "backend" attribution to inner-hex tests
- Added assert_file_field(&r5) to all tests

## Rule 06 fixes (21→0 FAIL)
- is_empty() → assert_eq!(len, 0) for golden/valid tests
- Proper rule6 filtering for hex-in-hex tests (tests 9-10 now correctly assert 0 rule6 errors since those are rule 02 errors)
- Added total r6 count, file field, TS/packages to all tests
- All assertions on &r6 not &errors

## Check bugs NOT fixed (intentional — tests document them)
- arch_helpers.rs: "crate subdirectories" in shared helper message (Rust-specific in generic code)
- check_06: .gitkeep-only leaf rejected (test documents gap)
- check_06: empty crates/ treated as absent (test documents gap)

246 passing, 5 intentional failures (rules 07-11).
