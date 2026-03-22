# CRIT-01: Add missing Rust lints to EXPECTED_RUST_LINTS

**Date:** 2026-03-19 18:26
**Task:** Add `missing_docs` (deny) and `missing_debug_implementations` (warn) to EXPECTED_RUST_LINTS

## Goal
The `EXPECTED_RUST_LINTS` constant in `cargo_lints.rs` must match the canonical `[workspace.lints.rust]` section in `canonical.rs`. Two lints are missing.

## Approach
Add two `LintExpectation` entries to the `EXPECTED_RUST_LINTS` array, following the existing pattern (name, expected_level, priority: None).

## Files to Modify
- `apps/guardrail3/src/app/rs/validate/cargo_lints.rs` — add two entries to EXPECTED_RUST_LINTS
