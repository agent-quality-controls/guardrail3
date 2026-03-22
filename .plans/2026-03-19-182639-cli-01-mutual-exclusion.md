# CLI-01: Add mutual exclusion for scope flags

**Date:** 2026-03-19 18:26
**Task:** Make --staged, --dirty, --commits, --files mutually exclusive in ValidateArgs

## Goal
Only one scope flag can be used at a time. Clap should reject conflicting combinations with a clear error.

## Approach
Add `#[arg(group = "scope")]` to each of the four scope flags in `ValidateArgs`. This uses clap's ArgGroup derive support where multiple args sharing the same group name are automatically mutually exclusive.

## Files to Modify
- `apps/guardrail3/src/cli.rs` — add group attribute to staged, dirty, commits, files args
