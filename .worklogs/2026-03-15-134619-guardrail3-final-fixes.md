# guardrail3 — Final round fixes (11 issues)

**Date:** 2026-03-15 13:46
**Scope:** discover.rs, config_files.rs, clippy_coverage.rs, deny_audit.rs, source_scan.rs (rs+ts), pre_commit.rs, generate.rs, init.rs, canonical.rs

## Summary
Fixed all 11 issues from the final adversarial audit round. Template validation dropped from 17 errors to 2 (both real).

## Decisions Made

### Monorepo workspace root discovery
- **Chose:** Detect root Cargo.toml with empty workspace members as a marker, fall through to apps/backend/
- **Why:** ts-rust-railway root Cargo.toml exists for rust-analyzer but has no members. The real workspace is at apps/backend/.

### Profile-aware validation
- **Chose:** Read guardrail3.toml if present, adjust expectations per profile. Default to service if no config.
- **Why:** Minimal profile intentionally has fewer bans. Flagging them as missing is a false positive.

### Pre-commit hook parameterization
- **Chose:** GUARDRAIL3_RUST_WORKSPACE env var with default from guardrail3.toml
- **Why:** Hardcoded apps/backend breaks non-template projects.
