# Drop redundant source scan checks + document philosophy

**Date:** 2026-03-17 11:51
**Scope:** source_scan.rs, mod.rs, help_gen.rs, adversarial_fixtures.rs, CLAUDE.md

## Summary
Removed R42 (unsafe), R43 (todo), R44 (unwrap/expect), R53 (unsafe_code=forbid) — all redundant with clippy lints verified by R26. guardrail3 enforces configuration, not violations. Updated CLAUDE.md with clear philosophy: what guardrail3 IS (config+arch enforcer) vs what it IS NOT (a linter).
