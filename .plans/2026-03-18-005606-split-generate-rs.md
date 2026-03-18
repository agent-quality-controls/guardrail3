# Split generate.rs into generate.rs + generate_helpers.rs

**Date:** 2026-03-18 00:56
**Task:** File exceeds 500-line limit; extract helper functions into a new submodule.

## Goal
Split `generate.rs` (591 lines) into two files under 500 lines each.

## Approach

### What moves to `generate_helpers.rs`
- `LocalOverrides` struct
- `load_local_overrides()`
- `validate_override_content()`
- `deduplicated_override()` (pub(crate))
- `resolve_rust_root()`
- `resolve_app_paths()`
- `detect_ts_app_types()`
- `build_deny_for_profile()`

### What stays in `generate.rs`
- All `pub fn run*` functions
- `load_config()`
- `GeneratedFile` struct
- `GeneratedPair` type
- `warn_if_overwriting()`
- `generate_and_install_hooks()`
- `generate_all_files()`
- `generate_ts_files()`
- `generate_rust_files()`
- `generate_expected_ts()`
- `generate_expected()`

### Wiring
- Add `mod generate_helpers;` and `pub(crate) use generate_helpers::*;` in generate.rs
- The external callers (deny.rs, clippy.rs) use `crate::commands::generate::deduplicated_override` -- this path works since generate_helpers re-exports via `pub(crate) use generate_helpers::*`

## Files to Modify
- `apps/guardrail3/src/commands/generate.rs` -- remove helpers, add mod + use
- `apps/guardrail3/src/commands/generate_helpers.rs` -- new file with extracted functions
