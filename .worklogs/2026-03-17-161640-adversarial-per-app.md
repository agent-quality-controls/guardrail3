# Adversarial tests for per-app types + case sensitivity fix

**Date:** 2026-03-17 16:16
**Scope:** adversarial_categories.rs, domain/report.rs

## Summary
Added 12 adversarial tests targeting per-app type profiles. Fixed case-insensitive type parsing discovered by the tests.

## Tests Added
- Config edge cases: typo in type, name mismatch, ghost app, no apps dir, case sensitivity, global override precedence
- Import boundary bypass: T-ARCH-02 violations correctly gated by content/library/service type
- Mixed monorepo with 3 app types verifying only service gets arch checks

## Bug Found and Fixed
`from_str_or_default` was case-sensitive — `type = "Content"` silently defaulted to Service. Fixed by using `to_ascii_lowercase()` before matching.
