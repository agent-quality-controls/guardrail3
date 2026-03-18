# Add ValidateDomains struct and CLI flags

**Date:** 2026-03-15 19:26
**Task:** Add ValidateDomains struct to rs/validate/mod.rs and domain/thorough CLI flags to ValidateArgs

## Goal
Infrastructure for domain-filtered validation: the struct that controls which domains run, and the CLI flags that users will pass.

## Approach
1. Add `ValidateDomains` struct before `detect_profile` in `src/rs/validate/mod.rs`
2. Add 5 new fields (code, architecture, release, tests, thorough) to `ValidateArgs` in `src/cli.rs`
3. `cargo build` to verify compilation

## Files to Modify
- `src/rs/validate/mod.rs` — add ValidateDomains struct
- `src/cli.rs` — add CLI flag fields to ValidateArgs
