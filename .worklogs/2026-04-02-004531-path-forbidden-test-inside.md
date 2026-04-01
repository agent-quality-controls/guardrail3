# #[path] forbidden in arch, RS-CODE-24 removed, tests inside modules

**Date:** 2026-04-02 00:45

## Summary

Three coordinated changes across arch, code, and test families:
1. New RS-ARCH-09: #[path] attribute unconditionally forbidden
2. RS-CODE-24 removed from code family (arch now owns this)
3. RS-TEST-02 changed from sidecar (_tests/) to inside-module (tests/) pattern

## Changes

### RS-ARCH-09: #[path] forbidden (new rule)
- Scans all .rs files for #[path] on mod declarations
- Unconditional error, zero exceptions (no test sidecar carveout, no reason allowance)
- Finds 1365 violations across the project

### RS-CODE-24: removed
- Removed rule wiring from code family lib.rs
- Removed assertions module
- Removed dead parse helpers (PathAttrInfo, find_path_attrs, PathAttrVisitor,
  collect_path_attrs, collect_cfg_attr_path_attrs, path_string_has_parent_segment)
- Kept path_string_has_parent_segment as private in analysis_helpers (still used by RS-CODE-23 include bypass)

### RS-TEST-02: inside-module tests
- Changed from `<module>_tests/` sidecar pattern to `<module>/tests/` inside pattern
- Owner detection: `tests/` directory inside module → owner is `module/mod.rs`
- `_tests/` suffix directories no longer recognized as valid test locations
- cfg(test) module declarations must be `mod tests;` resolving to `tests/mod.rs`
- #[path] on test modules forbidden (aligns with RS-ARCH-09)
- No backward compatibility with old sidecar pattern
- Finds 309 violations for old-style cfg(test) declarations

## Key Files
- `arch/.../facade/rs_arch_09_no_path_attr.rs` — new rule
- `code/.../lib.rs` — RS-CODE-24 unwired
- `test/.../rs_test_02_owned_sidecar_shape.rs` — rewritten for inside pattern
