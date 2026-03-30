# Reason Policy Workspace Artifact Cleanup

**Date:** 2026-03-30 21:01
**Scope:** `packages/reason-policy/.gitignore`, `packages/reason-policy/target/`, `.worklogs/2026-03-30-210129-reason-policy-artifact-cleanup.md`

## Summary
Cleaned up accidentally committed build artifacts from the new `packages/reason-policy` workspace and added a workspace-local ignore rule so future local builds do not stage `target/` output there.

## Context & Problem
The first commit that introduced `packages/reason-policy` also included the new workspace’s `target/` directory. The repo root already ignores `/target/` and `/apps/*/target/`, but it did not cover `packages/reason-policy/target/`, so a normal local test run produced tracked artifacts under the new workspace.

## Decisions Made

### Add a local ignore at the new workspace root
- **Chose:** create `packages/reason-policy/.gitignore` with `/target/`
- **Why:** the new workspace should carry its own obvious build-output ignore instead of depending on a repo-root glob that currently does not cover it
- **Alternatives considered:**
  - broaden the root `.gitignore` immediately — rejected for now because the smallest safe fix is local to the newly introduced workspace
  - leave the tracked artifacts and rely on future cleanup — rejected because generated build output should not stay versioned

### Remove generated artifacts in a separate follow-up commit
- **Chose:** clean the tracked `target/` output without amending the prior commit
- **Why:** repository instructions explicitly say not to amend commits unless requested
- **Alternatives considered:**
  - amend the previous commit — rejected by repo policy

## Architectural Notes
This cleanup does not change runtime behavior. It only restores the intended repository hygiene for the new shared workspace.

## Information Sources
- `.gitignore` at the repo root, which already ignored `/target/` and `/apps/*/target/` but not `packages/reason-policy/target/`
- the prior commit contents showing the accidental inclusion of generated files under `packages/reason-policy/target/`

## Open Questions / Future Considerations
- If more top-level workspaces are introduced under `packages/`, the repo root may eventually want a broader ignore rule for `/packages/*/target/`. For now the local workspace ignore is enough.

## Key Files for Context
- `packages/reason-policy/.gitignore` — local ignore for the new shared workspace
- `.worklogs/2026-03-30-210029-reason-policy-code-bypass.md` — original implementation worklog
- `.worklogs/2026-03-30-210129-reason-policy-artifact-cleanup.md` — this cleanup record

## Next Steps / Continuation Plan
1. Continue the planned audit for escape-hatch and ignore surfaces in other families.
2. Reuse `guardrail3-reason-policy` for any family that currently validates justification text locally.
