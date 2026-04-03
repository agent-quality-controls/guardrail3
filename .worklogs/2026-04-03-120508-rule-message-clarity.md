# Rule message clarity audit — ARCH + TOPOLOGY families

Audited all RS-ARCH (9 rules) and RS-TOPOLOGY (15 rules) error messages for clarity, actionability, and correctness. Applied copy fixes to make messages agent-friendly.

## Changes made

### RS-ARCH family (6 rules modified)
- **ARCH-01**: Added fix action (create src/lib.rs or src/main.rs)
- **ARCH-02**: Replaced vague allowance list with concrete action (move to submodule)
- **ARCH-03**: Removed speculative "#[path]" language, added concrete fix (create mod.rs with mod declarations)
- **ARCH-04**: Matched ARCH-02 style, added missing example on broad re-export
- **ARCH-05**: Added WHY sentence about coupling
- **ARCH-07**: Replaced "sibling" jargon with "top-level under src/", made fix concrete (extract into sub-crates)

### RS-TOPOLOGY family (11 rules modified)
- **TOPOLOGY-02**: Added fix action (move under apps/packages or declare auxiliary)
- **TOPOLOGY-03**: Replaced hexarch/libarch jargon, added fix action
- **TOPOLOGY-04**: Added fix action (restructure so zones don't nest)
- **TOPOLOGY-07**: Simplified inventory message jargon
- **TOPOLOGY-09**: Added fix action (add [workspace] section)
- **TOPOLOGY-10**: Added fix action (add [workspace] or move under existing)
- **TOPOLOGY-11**: Added WHY (Cargo doesn't support nested workspaces) and fix action
- **TOPOLOGY-12**: Added fix actions to both variants (add to members / remove stale entry)
- **TOPOLOGY-13**: Replaced "escapes" jargon with concrete explanation (uses ..)
- **TOPOLOGY-14**: Clarified what auxiliary means, added fix action

### Upstream (family_mapper)
- Rewrote all 6 illegal file placement reason messages to remove jargon (legal, attached, etc.) and add fix actions. Fixed debug formatting ({:?}) on workspace root list.

## Decisions
- Left ARCH-06, ARCH-08, ARCH-09 unchanged (already good)
- Left TOPOLOGY-01, TOPOLOGY-05, TOPOLOGY-08 unchanged
- Did not touch TOPOLOGY-06 (candidate for removal — references dead libarch family, logic is questionable)

## Rule splitting doc
Created `.plans/2026-04-03-110256-rule-splitting.md` noting rules that check multiple things or are candidates for consolidation.

## Key files
- `.plans/2026-04-03-110256-rule-splitting.md` — splitting/consolidation candidates
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs/mod.rs` — upstream placement reason messages
- All rule.rs files under arch/crates/runtime/src/ and topology/crates/runtime/src/

## Next steps
- Audit remaining families (clippy, code, deps, fmt, garde, hexarch, test, toolchain, etc.)
- Execute rule splitting from the splitting doc
- Remove or rework TOPOLOGY-06
