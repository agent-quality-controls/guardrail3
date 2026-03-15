# guardrail3 — Final round fixes

**Date:** 2026-03-15 15:56
**Scope:** All source files, guardrail3.toml, generated configs

## Summary
Fixed all remaining audit findings: removed minimal profile, fixed duplication tool per-language, added missing ESLint/tsconfig/package checks, switched self to service profile.

## Decisions Made

### Removed minimal profile
- **Why:** "Weaker guardrails" defeats the purpose. Use local overrides for exceptions instead.

### Duplication tool per-language
- **Chose:** cargo-dupes for Rust, jscpd for TypeScript, both for mixed
- **Why:** cargo-dupes is AST-aware and doesn't require Node.js. jscpd is the right tool for TS.

### Self-profile changed to service
- **Chose:** service profile with full bans
- **Why:** guardrail3 should enforce the same rules it validates others against. FS/process access justified via #[allow] with reasons.
