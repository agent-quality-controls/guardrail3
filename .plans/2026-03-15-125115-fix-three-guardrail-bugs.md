# Fix three guardrail3 bugs

**Date:** 2026-03-15 12:51
**Task:** Fix deny.toml skip parsing, phantom #![allow()] findings, and unused_crate_dependencies exemption

## Goal
All three bugs fixed, cargo build succeeds, validation output correct.

## Approach

### 1. deny_audit.rs — check_skip_entries
- Check for `crate` field first (0.19+ format: `{ crate = "name@version" }`)
- Split on `@` to get name/version
- Fall back to `name`/`version` fields
- Extract and include `reason` field

### 2. source_scan.rs — check_crate_level_allow (empty lint)
- After extracting lint, skip if empty/whitespace
- If lint contains comma, split and process each separately

### 3. source_scan.rs — check_crate_level_allow (unused_crate_dependencies)
- Always report as R31 Info regardless of file, removing is_bin_entry check for this lint

## Files to Modify
- `src/rs/validate/deny_audit.rs` — check_skip_entries function
- `src/rs/validate/source_scan.rs` — check_crate_level_allow function
