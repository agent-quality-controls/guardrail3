# --inventory flag, agent-actionable messages, allow cleanup, README

**Date:** 2026-03-16 20:06
**Scope:** Major quality pass across entire codebase

## Summary
Four parallel improvements:
1. --inventory flag: hides 142 non-actionable confirmations from output. Problems always visible.
2. All error messages rewritten to be imperative + specific for agent consumers.
3. Eliminated 23+ lazy #[allow] suppressions (case_sensitive, dead_code, format_push, type_complexity, or_fun_call).
4. README.md written with project vision, philosophy, and architecture.

Self-validation: 0 errors, 2 warnings, 338 info (142 hidden). 415 tests, 0 failures.
