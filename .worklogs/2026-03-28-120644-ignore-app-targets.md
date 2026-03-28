# Ignore App Workspace Targets

**Date:** 2026-03-28 12:06
**Scope:** `.gitignore`, `.worklogs/2026-03-28-120644-ignore-app-targets.md`

## Summary
Added an ignore rule for app-local Cargo target directories so `apps/guardrail3/target/` does not reappear as untracked after builds. This closes the repo-hygiene hole exposed by the cleanup commit that removed accidentally tracked build output.

## Context & Problem
After removing the tracked `apps/guardrail3/target/` subtree, the worktree still showed `apps/guardrail3/target/` as untracked. The repository ignore file only covered the repository-root `/target/`, not nested workspace targets under `apps/*/target/`. That meant future builds of `apps/guardrail3` would immediately dirty the tree again.

## Decisions Made

### Ignore nested app workspace targets explicitly
- **Chose:** Add `/apps/*/target/` to the repo root `.gitignore`.
- **Why:** `apps/guardrail3` is an app-root Cargo workspace that produces its own `target/` directory. The ignore policy should cover that generated output directly.
- **Alternatives considered:**
  - Keep deleting `apps/guardrail3/target/` manually — rejected because the tree would become dirty after every build.
  - Ignore every nested `target/` directory recursively — rejected because a narrower app-root rule matches the current repo structure and avoids broader-than-needed ignore scope.

## Architectural Notes
This is repository hygiene, not a source-code behavior change. It aligns the ignore rules with the one-app-one-workspace layout currently used under `apps/`.

## Information Sources
- `.gitignore`
- `git check-ignore -v apps/guardrail3/target`
- `.worklogs/2026-03-28-120454-remove-target-artifacts.md`

## Open Questions / Future Considerations
- If more app roots are added under `apps/`, the new ignore rule already covers them.
- If the repository later standardizes workspace output elsewhere, the ignore grammar may need another pass.

## Key Files for Context
- `.gitignore` — repository-wide ignore policy
- `.worklogs/2026-03-28-120454-remove-target-artifacts.md` — prior cleanup that exposed the missing ignore rule
- `.worklogs/2026-03-28-120301-bundle-rs-family-progress.md` — broader source checkpoint this hygiene work follows

## Next Steps / Continuation Plan
1. Stage the `.gitignore` update and this worklog.
2. Commit the ignore fix as a small follow-up hygiene change.
3. Verify `git status` is clean after the current `apps/guardrail3/target/` directory remains ignored.
