# Self-validation status after guardrail tightening

**Date:** 2026-03-19 19:36
**Scope:** Cargo.toml

## Summary
Ran guardrail3 on itself after all tightening. 2 known errors, 2 known warnings.

## Known violations (tech debt, not bugs)

### R26 missing_docs (ERROR)
Canonical module requires `missing_docs = "deny"`. Cannot enable because 470 public items lack doc comments. With `warnings = "deny"` even `warn` level blocks the build. Commented out in Cargo.toml with TODO.

### R26 missing_debug_implementations (ERROR)
Canonical module requires `missing_debug_implementations = "warn"`. Cannot enable because 34 types lack Debug derives. Same `warnings = "deny"` interaction.

### R41 many use statements (WARN)
One file has >20 use statements. Could split.

### R-TEST-08 no mutation hook (WARN)
Mutation testing (cargo-mutants) not configured in hooks.

## Decision
Keep these lints in EXPECTED_RUST_LINTS (the guardrail is correct for target projects). guardrail3 self-validation correctly flags its own non-compliance. Fix as separate work items.
