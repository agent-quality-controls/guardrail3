# Create By-Family Rust Plan Surface

**Date:** 2026-03-30 11:16
**Scope:** `.plans/by_family/**`, `.plans/todo/checks/rs/*.md`

## Summary
Created a new `.plans/by_family` planning surface for Rust families and marked the older `.plans/todo/checks/rs/*.md` files as superseded primary plans rather than deleting them. The new tree is meant to centralize family-level status and source-of-truth routing while preserving the older ledgers as detailed rule history.

## Context & Problem
The planning surface had drifted across multiple directories:

- `.plans/todo/checks/rs/*.md`
- `.plans/by_file/**`
- `.plans/per-app-config-design/**`
- family-local READMEs
- tactical handoffs and hardening briefs

That made it hard to answer basic questions like which family docs are current, which files are tactical execution notes, and which old ledgers are still worth reading only for history. The user explicitly asked to start reconciling this family-by-family, beginning with Rust, and to retire the older spread of primary plan docs in favor of a single current family planning surface.

## Decisions Made

### Create a family-indexed planning tree instead of rewriting every old ledger in place
- **Chose:** Add `.plans/by_family/README.md`, `.plans/by_family/rs/README.md`, and one Rust file per family.
- **Why:** This creates a stable index layer quickly without destroying historical material or forcing a risky one-pass rewrite of all detailed ledgers.
- **Alternatives considered:**
  - Rewrite every `.plans/todo/checks/rs/*.md` file into one final current version — rejected because the old ledgers still contain useful rule-by-rule detail and migration context.
  - Delete the older plan files immediately — rejected because they are still valuable as detailed rule ledgers and execution history.

### Keep family/shared READMEs above plan indexes for behavior
- **Chose:** Treat live code first, then shared/family READMEs, then `.plans/by_family/**` for planning/status reconciliation.
- **Why:** The new by-family tree should centralize planning status, but it should not become another shadow behavioral spec that can drift away from the live family READMEs.
- **Alternatives considered:**
  - Make `.plans/by_family/**` outrank family READMEs — rejected because that would recreate the same authority confusion the cleanup is meant to remove.

### Demote the old Rust family plan files with explicit superseded banners
- **Chose:** Add short superseded notices to the older `.plans/todo/checks/rs/*.md` files.
- **Why:** Readers landing in those files still need an immediate signal that the primary family planning surface moved.
- **Alternatives considered:**
  - Leave the old files unchanged and rely on the new index only — rejected because cold-start readers will continue treating them as primary.
  - Move the old files into a new archive directory — rejected for now because it would create noisy churn and break a lot of existing references.

### Add only a TypeScript placeholder in this pass
- **Chose:** Create `.plans/by_family/ts/README.md` as a reserved future cutover point, without trying to consolidate TS yet.
- **Why:** The user said TypeScript is important next, but this pass was specifically to start family-by-family with Rust.
- **Alternatives considered:**
  - Ignore TypeScript entirely in the new tree — rejected because the directory structure should already make room for the next phase.
  - Fully consolidate TypeScript in the same change — rejected because it would mix two broad reconciliations and make the Rust cutover less trustworthy.

## Architectural Notes
The resulting authority split is:

1. live code
2. shared Rust architecture and family-local READMEs
3. `.plans/by_family/**` for family-level planning/status
4. tactical handoffs, attack briefs, and migration ledgers
5. historical/research material

For Rust specifically:
- `.plans/by_family/rs/README.md` is the new family index
- each `.plans/by_family/rs/<family>.md` points at the implementation root, family README, and remaining history docs
- `.plans/todo/checks/rs/*.md` remain detailed rule ledgers rather than the primary planning surface

This keeps the family plan cutover lightweight while still making the source-of-truth hierarchy explicit.

## Information Sources
- `AGENTS.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `.plans/todo/checks/rs/*.md`
- `apps/guardrail3/crates/app/rs/families/*/README.md`
- `.plans/todo/check_review/README.md`
- `.plans/todo/check_review/test_hardening/README.md`
- `.plans/by_file/**`
- `.plans/per-app-config-design/**`
- `.worklogs/2026-03-30-093006-commit-code-and-clippy-planning-docs.md`
- `.worklogs/2026-03-30-092910-clean-residual-rust-family-tail.md`

## Open Questions / Future Considerations
- `RS-RELEASE` still needs a family README, so its old ledger remains unusually important.
- Hooks families are still documented outside this first Rust-family cutover.
- TypeScript still needs the same consolidation, but should be done in a separate pass so the Rust authority tree stays clear.
- Some older Rust ledgers still contain stale implementation-path notes; those can be cleaned incrementally now that the primary planning surface is separate.

## Key Files for Context
- `AGENTS.md` — repo-level source of truth and current direction.
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust architecture and family routing model.
- `.plans/by_family/README.md` — top-level family planning authority and directory purpose.
- `.plans/by_family/rs/README.md` — current Rust family index and authority ordering.
- `.plans/by_family/rs/arch.md` — example of the new per-family status file format for an audited family.
- `.plans/by_family/rs/code.md` — example showing how family supplements like `FIXES.md` and `EXPANSION.md` are demoted from primary authority.
- `.plans/todo/checks/rs/arch.md` — example of the old ledger now marked superseded rather than deleted.
- `.worklogs/2026-03-30-093006-commit-code-and-clippy-planning-docs.md` — recent planning/doc cleanup context in the same area.

## Next Steps / Continuation Plan
1. Add `apps/guardrail3/crates/app/rs/families/release/README.md` so `RS-RELEASE` can follow the same authority split as the other Rust families.
2. Do a second Rust docs pass to clean stale implementation-path references inside the older `.plans/todo/checks/rs/*.md` ledgers now that they are clearly secondary.
3. Start the TypeScript cutover under `.plans/by_family/ts/`, mapping current TS family docs to the same authority hierarchy used here for Rust.
4. Once the TS surface exists, explicitly relabel `.plans/by_file/**` and `.plans/per-app-config-design/**` files that are research-only or historical.
