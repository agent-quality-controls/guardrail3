# Add Garde clippy ban modules

**Date:** 2026-03-15 19:46
**Task:** Add Garde-specific clippy ban modules (method + type) to guardrail3's module system

## Goal
Two new module constants (GARDE_METHOD_BANS, GARDE_TYPE_BANS) registered in profiles and all_modules(), with clippy_coverage.rs updated to expect these bans.

## Approach

### Step-by-step plan
1. Add GARDE_METHOD_BANS and GARDE_TYPE_BANS constants to src/modules/clippy.rs following existing pattern
2. Add them to service_profile_methods/types and library_profile_methods/types
3. Register in src/modules/mod.rs all_modules()
4. Add the 8+4 ban paths to EXPECTED_METHOD_BANS and EXPECTED_TYPE_BANS in clippy_coverage.rs
5. cargo build + cargo test

## Files to Modify
- `src/modules/clippy.rs` -- add 2 module constants, update profile functions
- `src/modules/mod.rs` -- register in all_modules()
- `src/rs/validate/clippy_coverage.rs` -- add expected bans
