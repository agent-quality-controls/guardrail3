# Reactivate TypeScript Frontend Planning

**Date:** 2026-03-23 21:27
**Scope:** `.plans/todo/typescript/`, `.plans/todo/legacy/README.md`, moved frontend/content planning docs from `.plans/todo/checks/rs/`

## Summary
This change reactivated TypeScript/frontend planning as a first-class active area. It moved TS/frontend docs back out of `legacy/`, created a dedicated `.plans/todo/typescript/` planning root, and relocated the Rust-frontend/content attempt docs into a paused-reference area instead of pretending they are active Rust check-family plans.

## Context & Problem
The project had temporarily gone “Rust-only”, and most TS/frontend planning was archived. The user clarified that the Rust-for-frontend direction had not been adopted efficiently enough, which meant frontend work had effectively fallen into a gap: not actively driven by Rust, but also no longer organized as active TS planning. We needed a clean active planning surface for frontend/content again without losing the Rust experiment docs.

## Decisions Made

### Reactivate TS/frontend planning under its own root
- **Chose:** Create `.plans/todo/typescript/` and move the TypeScript/frontend/deploy/hook planning docs there.
- **Why:** This makes frontend/content planning active again without mixing it into the Rust backend/library check families.
- **Alternatives considered:**
  - Leave the docs in `legacy/` and just “remember” they matter again — rejected because that keeps active work buried in an archive.
  - Put them back under `checks/rs/` — rejected because they are not active Rust check-family plans right now.

### Preserve the Rust frontend/content attempt as paused reference material
- **Chose:** Move the Rust frontend/content idea docs from `.plans/todo/checks/rs/` into `.plans/todo/typescript/rust_frontend_attempt/`.
- **Why:** Those docs are still valuable, but they should not masquerade as active Rust family plans while the TypeScript/frontend direction is being revived.
- **Alternatives considered:**
  - Delete them — rejected because the exploratory architecture work may still be useful later.
  - Leave them under `checks/rs/` — rejected because it falsely signals active Rust-family implementation intent.

### Add explicit transition docs
- **Chose:** Add `typescript/README.md` and `typescript/2026-03-23-ts-frontend-shift.md`.
- **Why:** The repo needs one place that explains what is active again, what remains historical, and how the Rust frontend experiment fits into the new state.
- **Alternatives considered:**
  - Rely on implicit directory naming — rejected because the shift in direction is too important to leave undocumented.

## Architectural Notes
The planning split is now:
- Rust backend/library/app check families remain under `.plans/todo/checks/rs/`
- active TypeScript/frontend planning lives under `.plans/todo/typescript/`
- old archival material remains under `.plans/todo/legacy/`
- Rust frontend/content ideas are preserved as paused references under `.plans/todo/typescript/rust_frontend_attempt/`

This keeps the active frontend direction visible without claiming that the Rust frontend family plans are ready for implementation.

## Information Sources
- `.plans/todo/legacy/checks/deploy/ts.md`
- `.plans/todo/legacy/checks/hooks/ts.md`
- `.plans/todo/legacy/checks/hooks_deploy_audit.md`
- `.plans/todo/legacy/audit/06-ts-source-scan.md`
- `.plans/todo/legacy/audit/07-tsconfig-npmrc-jscpd.md`
- `.plans/todo/legacy/audit/13-ts-architecture.md`
- `.plans/todo/legacy/ts_guardrails_implementation.md`
- `.plans/todo/legacy/ts_additional_analysis.md`
- `.plans/todo/legacy/ts-project-types.md`
- `.plans/todo/checks/rs/frontend_arch.md`
- `.plans/todo/checks/rs/frontend_i18n.md`
- `.plans/todo/checks/rs/frontend_routes_seo.md`
- `.plans/todo/checks/rs/frontend_ui.md`
- `.plans/todo/checks/rs/content_pipeline.md`

## Open Questions / Future Considerations
- The next planning step is to separate revived TS docs into:
  - still-active frontend/content guardrails
  - outdated mechanism-specific material
- The Rust frontend/content attempt is paused, not dead. It may come back later if the project returns to Rust-native frontend enforcement.
- This commit intentionally excludes unrelated in-flight changes elsewhere in the worktree, especially active hardening edits in Rust family code and docs.

## Key Files for Context
- `.plans/todo/typescript/README.md` — new entrypoint for active TypeScript/frontend planning
- `.plans/todo/typescript/2026-03-23-ts-frontend-shift.md` — explains the shift back to TS-first frontend planning
- `.plans/todo/typescript/checks/deploy/ts.md` — reactivated deploy planning
- `.plans/todo/typescript/checks/hooks/ts.md` — reactivated TS hook planning
- `.plans/todo/typescript/checks/hooks_deploy_audit.md` — reactivated TS/shared hook/deploy audit notes
- `.plans/todo/typescript/rust_frontend_attempt/` — paused Rust frontend/content experiment docs
- `.plans/todo/legacy/README.md` — updated to reflect that some TS docs moved back out of archive

## Next Steps / Continuation Plan
1. Review the reactivated TS/frontend docs and decide which ones are still genuinely active versus merely preserved history.
2. Write a consolidated active TS/frontend checker architecture plan so the reactivated material has one modern source of truth.
3. Keep Rust backend/library guardrail work separate from frontend/content planning so the two tracks do not blur again.
