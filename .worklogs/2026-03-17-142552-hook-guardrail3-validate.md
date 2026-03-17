# Replace grep-based hook checks with guardrail3 validate

**Date:** 2026-03-17 14:25
**Scope:** domain/modules/pre_commit.rs, .githooks/pre-commit, CLAUDE.md

## Summary
Replaced grep-based "guardrail tamper detection" and "structural health checks" in the pre-commit hook with `guardrail3 rs validate --staged` and `guardrail3 ts validate --staged`. Removed ~100 lines of fragile grep patterns.

## Context & Problem
The pre-commit hook had grep-based checks for `#[allow(` without reason, `eslint-disable` without reason, config relaxation, file length, use count, and crate-wide allows. These were a broken duplicate of what guardrail3's AST scanner already does correctly. The grep patterns caused false positives on source files containing these patterns in string literals (like guardrail3's own allow_checks.rs).

## Decisions Made

### Replace, don't patch
- **Chose:** Remove all grep-based tamper detection and structural health, replace with guardrail3 validate
- **Why:** guardrail3 already does all of this with AST parsing (zero false positives). Running it in the hook is the correct approach — the hook should use the tool, not reimplement it with grep.
- **Alternatives considered:**
  - Fix grep patterns with string concatenation — rejected because it's a bandaid on a fundamentally broken approach
  - Keep grep as fast fallback, add guardrail3 as optional — rejected because grep false positives are worse than slightly slower hooks

### Graceful degradation
- **Chose:** `if command -v guardrail3` check with WARNING if not installed
- **Why:** The hook should work even if guardrail3 isn't installed yet (new developer setup)
