# Add regression tests for guardrail3 bugs fixed in rounds 1-4

**Date:** 2026-03-15 17:27
**Task:** Write unit tests inside source files for 8 bug categories previously fixed in audit rounds 1-4.

## Goal
Every bug fixed in rounds 1-4 has a regression test so it cannot recur.

## Approach

Since guardrail3 is a binary crate (no lib.rs), integration tests in `tests/` cannot access internal modules. All tests go as `#[cfg(test)] mod tests` at the bottom of each source file.

### Files to modify
1. `src/rs/validate/source_scan.rs` — Bug 1: filter_non_comment_lines string literal handling, strip_string_literals
2. `src/rs/validate/allow_checks.rs` — Bug 2: Check ID mappings (R30-R35), Bug 7: unused_crate_dependencies universal exemption
3. `src/report/types.rs` — Bug 3: Score formula only counts errors
4. `src/rs/validate/structure_checks.rs` — Bug 4: Test file exemptions for R38
5. `src/rs/validate/code_quality_checks.rs` — Bug 5: R58 direct std::fs detection
6. `src/rs/validate/deny_inventory.rs` — Bug 6: deny.toml skip entry parsing (crate@version format)

Bug 8 (monorepo workspace detection) requires filesystem access to the template repo, which is fragile. Will add a unit test in discover.rs that tests the logic with a temp dir.

### Key decisions
- Unit tests only (no integration tests) because binary crate
- Tests verify specific IDs and severities to catch swap bugs
- No logic changes, only test additions
