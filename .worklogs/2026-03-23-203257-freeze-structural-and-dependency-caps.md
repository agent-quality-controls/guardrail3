# Freeze Structural And Dependency Caps

**Date:** 2026-03-23 20:32
**Scope:** `.plans/todo/check_review/06-new-rust-rule-candidates.md`, `.plans/todo/checks/rs/code.md`

## Summary
This pass promoted structural organization caps into the set of frozen universal Rust rule candidates and aligned the candidate-review doc with the already-frozen generic/dependency-cap direction. The result is a clearer always-on hard-rule set for anti-sprawl enforcement.

## Context & Problem
We were reviewing remaining Rust rule ideas using a strict standard: if a rule cannot be enforced cleanly with no bypasses, it should not exist. The user clarified that the project’s role is to enforce hard quality policy directly, not to soften rules because code is internal. That made very-high-threshold anti-sprawl rules viable, provided the thresholds are set high enough that crossing them is almost certainly bad code rather than stylistic variation.

## Decisions Made

### Add structural organization caps as an always-on `rs/code` rule
- **Chose:** Plan `RS-CODE-35` as an `Error` for per-crate source trees that exceed any of:
  - module depth `> 6`
  - sibling subdirectories `> 12`
  - sibling `.rs` files `> 20`
- **Why:** These thresholds are high enough to catch real sprawl and agentic code accretion rather than normal crate structure.
- **Alternatives considered:**
  - Lower thresholds — rejected because they drift toward taste/style enforcement.
  - Per-rule configurability — rejected because the project intentionally avoids local bypass mechanisms.

### Keep measurement anchored at the crate root
- **Chose:** Measure structural depth from each crate’s `Cargo.toml`, not from repo root or workspace root.
- **Why:** That makes the rule universal across app crates, leaf crates, and nested hexarch crates without coupling it to repo layout.
- **Alternatives considered:**
  - Measure from repo root — rejected because nested workspace structure would distort the signal.
  - Measure from `src/` only — rejected because the crate root is the more stable universal anchor.

### Reflect the frozen anti-sprawl rules in the candidate review doc
- **Chose:** Update the candidate classification so generic-count, direct-dependency-count, and structural caps all appear in the “implement” bucket.
- **Why:** The candidate file should match the actual product decisions already made in discussion, not lag behind them.
- **Alternatives considered:**
  - Leave them in a mixed “maybe” state — rejected because the scope had already been concretely decided.

## Architectural Notes
The structural cap belongs in `rs/code`, not `rs/hexarch`, because:
- it is per-crate, not per-architecture-style
- it should work for any Rust crate, including non-hexarch code

The dependency direct-count cap remains conceptually paired with this work, but it had already been frozen in the previous planning pass; this batch only kept the candidate overview in sync and added the structural cap to `rs/code`.

## Information Sources
- `.plans/todo/check_review/06-new-rust-rule-candidates.md` — candidate inventory and classification
- `.plans/todo/checks/rs/code.md` — live `rs/code` family contract where the new structural rule is now planned
- user discussion in this session clarifying:
  - no internal-vs-publishable quality split
  - no per-rule configurability
  - high thresholds should catch clearly bad code rather than style disagreements
- `.worklogs/2026-03-23-201014-freeze-additional-rust-rule-candidates.md` — prior freeze of generic-count and direct-dependency-count direction

## Open Questions / Future Considerations
- We still have not frozen exact semantics for what counts as a Rust “source/module directory” in generated-code-heavy crates; implementation should keep the measurement on source/module trees only.
- `pub(crate)` discipline and several other candidates remain out because they still lack a clean no-bypass enforcement shape.

## Key Files for Context
- `.plans/todo/check_review/06-new-rust-rule-candidates.md` — current classification of candidate Rust rules
- `.plans/todo/checks/rs/code.md` — planned `RS-CODE-31`, `33`, `34`, and `35`
- `.worklogs/2026-03-23-201014-freeze-additional-rust-rule-candidates.md` — previous candidate-freeze context

## Next Steps / Continuation Plan
1. Continue reviewing remaining candidate rules only through the “clean no-bypass enforcement” lens.
2. When implementation starts, add `RS-CODE-35` as a per-crate structural rule with full family-local tests.
3. Keep planning commits scoped tightly so they do not mix with the parallel hardening-lane code changes already in progress elsewhere.
