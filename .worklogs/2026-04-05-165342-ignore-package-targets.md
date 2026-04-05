# Ignore Package Targets

**Date:** 2026-04-05 16:53
**Scope:** `.gitignore`

## Summary
Added `packages/*/target/` to `.gitignore` so local builds of extracted package workspaces do not keep polluting the worktree.

## Context & Problem
The repo now contains multiple standalone package workspaces under `packages/`, and their test/build flows create local `target/` directories under each package root. Those artifacts are not source and should not show up as dirty files during package-local work.

## Decisions Made

### Ignore per-package Cargo build artifacts
- **Chose:** add `/packages/*/target/` to `.gitignore`.
- **Why:** This matches how the extracted package workspaces are being built and keeps the worktree focused on actual source changes.
- **Alternatives considered:**
  - Leave package `target/` directories visible — rejected because they are pure build output and add noise.
  - Ignore all `target/` directories recursively — rejected because the narrower package-scoped rule is sufficient and less blunt.

## Architectural Notes
This is a repo hygiene change that supports the extracted-package workflow. It does not affect runtime behavior.

## Information Sources
- `.gitignore` — existing ignore policy
- Current package-local test flows under `packages/g3-*`

## Open Questions / Future Considerations
- None.

## Key Files for Context
- `.gitignore` — repo-wide ignore rules
- `.worklogs/2026-04-05-164952-cargo-content-checks-extraction.md` — immediate prior extracted-package work
- `.worklogs/2026-04-05-165235-deny-content-tests-refactor.md` — immediate prior package-local test refactor

## Next Steps / Continuation Plan
1. Leave the remaining handoff/plan note files uncommitted unless they are intentionally curated into a real documentation pass.
2. If those notes are meant to become durable docs, normalize their content first instead of committing them as raw session artifacts.
