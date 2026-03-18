# Fix deny.toml resolution description

**Date:** 2026-03-18 20:30
**Scope:** apps/guardrail3/src/commands/coverage/deny.rs

## Summary
Corrected deny.toml config resolution from "CWD only" to "manifest directory." Verified by testing: `cargo deny --manifest-path apps/validator-rust/Cargo.toml check` finds `apps/validator-rust/deny.toml` regardless of CWD.

## Decisions Made
- **kept `covered_by` on crates:** Coverage is derived from the workspace's deny_toml (not per-crate walk-up), but the field is still useful — tells you WHICH deny.toml covers each crate.
