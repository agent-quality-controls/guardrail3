# Fix 3 bugs in collect_toml_entries parser

**Date:** 2026-03-17 23:44
**Task:** Fix multiline entry handling, no-space syntax, and cross-section dedup in diff.rs

## Goal
`collect_toml_entries` correctly parses multiline entries, tolerates `{path=` (no spaces), and prefixes entries with their TOML section name for section-aware comparison.

## Approach
Replace `collect_toml_entries` with the version provided by the user that handles all three bugs. Single edit in diff.rs lines 177-192.

## Files to Modify
- `apps/guardrail3/src/commands/diff.rs` — replace `collect_toml_entries` function
