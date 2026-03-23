# Archive Audit Plans And Extract Rust Follow-ups

**Date:** 2026-03-23 13:53
**Scope:** `.plans/todo/legacy/`, `.plans/todo/checks/`, active Rust family plans under `.plans/todo/checks/rs/`

## Summary
Archived outdated top-level planning and audit material under `.plans/todo/legacy/` and extracted the remaining live Rust/shared hardening work into a single active follow-up backlog. Also updated active Rust plans where legacy notes still contained requirements that should remain visible in the current Rust-only roadmap.

## Context & Problem
The top-level `.plans/todo/` area had accumulated a large mix of historical audit notes, mixed Rust/TS planning, and old validator-era backlog. That made it hard to tell which documents were still active sources of truth for the current Rust-only direction. The goal was to review those old notes adversarially against the current code and active plans, preserve only the still-live Rust/shared requirements, and move the rest into an archive so future work does not keep re-reading obsolete mixed-stack planning.

## Decisions Made

### Archive historical audits instead of deleting them
- **Chose:** Move old top-level notes and the `audit/` tree into `.plans/todo/legacy/`.
- **Why:** The material still has value as historical/adversarial reference, but it should not compete with current Rust family plans.
- **Alternatives considered:**
  - Delete the files entirely — rejected because they still contain useful historical context and attack ideas.
  - Leave them in `.plans/todo/` — rejected because that keeps obsolete mixed-stack material looking active.

### Consolidate live cross-cutting Rust debt into one follow-up backlog
- **Chose:** Add `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` for the still-live items that were not already clearly owned by a family plan.
- **Why:** The surviving items are cross-cutting and do not fit cleanly into a single family file. Putting them into one improvement backlog prevents them from being lost while still allowing the old audit docs to be archived.
- **Alternatives considered:**
  - Scatter each item into many family plans immediately — rejected because several items are shared infrastructure debts, not single-family rule work.
  - Keep them only in legacy README notes — rejected because that would make live debt too easy to ignore.

### Keep family-specific carry-forward in the active Rust plans
- **Chose:** Update active family plans where archived docs still carried specific live requirements, especially in `clippy.md`, `code.md`, `garde.md`, `release.md`, and `test.md`.
- **Why:** Family-local requirements should live with the family, not in a generic archive or audit note.
- **Alternatives considered:**
  - Put everything into the new follow-up backlog — rejected because that would weaken traceability for family-specific requirements.

## Architectural Notes
The repo’s planning structure is now more explicit:
- active Rust family work stays under `.plans/todo/checks/rs/`
- shared cross-cutting Rust hardening debt lives in `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`
- historical mixed-stack and validator-era material lives under `.plans/todo/legacy/`

This mirrors the code architecture direction: active source-of-truth plans should align with the new Rust family checker layout, while legacy notes are retained only as historical context.

## Information Sources
- Active Rust plans under `.plans/todo/checks/rs/`
- Active hook plans under `.plans/todo/checks/hooks/`
- Implemented Rust families under `apps/guardrail3/crates/app/rs/checks/rs/`
- Archived review inputs moved in this change:
  - `.plans/todo/tests_guardrails.md`
  - `.plans/todo/audit/`
  - `.plans/todo/release_setup_validator.md`
  - `.plans/todo/migrate_to_ast_parsing.md`
  - `.plans/todo/GARDE_GUARDRAILS.md`
  - `.plans/todo/2026-03-15-183125-guardrail3-domains.md`
  - `.plans/todo/remaining-fixes.md`
  - `.plans/todo/semver_releases.md`
- Agent-backed adversarial review results from this session, especially for:
  - tests/self-validation and mutation-hook notes
  - config/deny audits
  - source/deps/arch audits
  - coverage/generator and CLI/crawler audits
  - TS-era audit sweeps for shared carry-forward only

## Open Questions / Future Considerations
- The new hardening follow-up backlog still needs to be worked down into concrete implementation phases.
- Remaining active debt includes mutation-hook rigor, CLI/domain routing cleanup, generator/checker drift handling, whole-type `#[garde(skip)]` ownership, legacy raw-filesystem cleanup, and hook-generation inconsistencies.
- The archived audit material should stay available for adversarial back-reference, but future sessions should not treat it as primary planning input unless an active plan points back to it deliberately.

## Key Files for Context
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` — consolidated cross-cutting Rust/shared hardening backlog extracted from archived audits
- `.plans/todo/checks/rs/clippy.md` — active clippy contract with legacy carry-forward notes
- `.plans/todo/checks/rs/code.md` — active code-family contract including parsing hardening carry-forward
- `.plans/todo/checks/rs/garde.md` — active garde-family contract including unmet wrapper/extractor follow-ups
- `.plans/todo/checks/rs/release.md` — active release-family contract including semver-release template carry-forward
- `.plans/todo/checks/rs/test.md` — active test-family contract including parser/test-hardening carry-forward
- `.plans/todo/legacy/README.md` — archive index explaining what moved and where live requirements were carried forward
- `.worklogs/2026-03-23-130823-complete-release-family.md` — immediate prior milestone before this planning cleanup

## Next Steps / Continuation Plan
1. Use `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` as the live queue for cross-cutting Rust cleanup that is not owned by a single family.
2. Continue reviewing active plans under `.plans/todo/checks/` in small adversarial batches, recording concrete plan-vs-code gaps after each round instead of relying on the archived audit tree.
3. When a follow-up item clearly belongs to one family or subsystem, move it out of the generic follow-up backlog and into the owning active plan before implementation.
