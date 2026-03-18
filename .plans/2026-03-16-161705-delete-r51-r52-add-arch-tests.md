# Delete R51/R52 and add adversarial R-ARCH tests

**Date:** 2026-03-16 16:17
**Task:** Remove legacy dependency direction checks (R51/R52) and add exhaustive tests for R-ARCH-01/02/03

## Goal
1. R51/R52 are gone — the file, the mod declaration, all calls, and help text references
2. hex_arch_checks.rs has exhaustive tests covering every dependency flow combination for R-ARCH-02 and adversarial tests for R-ARCH-03, plus edge cases
3. Golden test files still compile (R52 entries will naturally disappear when re-run)

## Approach

### Task 1: Delete R51/R52
- Delete `src/app/rs/validate/dependency_direction.rs`
- In `mod.rs`: remove `mod dependency_direction;` line and the block calling `dependency_direction::check_all_dependency_directions` and `dependency_direction::check_dependency_graph`
- In `help_gen.rs`: remove lines 344-345 (R51/R52 entries) and update line 136 reference from R51 to R-ARCH-02

### Task 2: Add adversarial tests to hex_arch_checks.rs
Add to the existing `#[cfg(test)] mod tests` block:
- R-ARCH-02: domain->ports fails, domain->app fails, ports->domain ok, ports->app fails, ports->adapters fails, app->ports ok, app->adapters fails, adapters->domain+ports+app ok, multiple violations reported
- R-ARCH-03: library->other library ok (verify existing), service internal->package ok
- Edge cases: crate with no layer skipped, external dep not checked

## Files to Modify
- `src/app/rs/validate/dependency_direction.rs` — DELETE
- `src/app/rs/validate/mod.rs` — remove mod + calls
- `src/help_gen.rs` — remove R51/R52 lines, update R51 reference
- `src/app/rs/validate/hex_arch_checks.rs` — add tests
