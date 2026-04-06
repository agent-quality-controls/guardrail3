# Add G3RS Workspace Crawl And Ingestion Plan

**Date:** 2026-04-06 12:54
**Scope:** `.plans/2026-04-06-g3rs-workspace-crawl-and-ingestion.md`

## Summary
Added a detailed architecture plan for the new `g3rs` runtime stack built around a shared per-workspace crawl package, per-family ingestion packages, and the already extracted checks packages. The plan records the agreed boundaries so the next implementation phase does not drift back into old `guardrail3` legality/topology behavior.

## Context & Problem
The extracted `g3rs-*-(config|ast)-checks` packages now exist, but the new Rust-only runtime that should exercise them has not been built yet. The open design problem was how to validate real workspaces without rebuilding the old repo-wide discovery, legality, and family-mapper machinery.

The user explicitly pushed the design toward:

- one explicit workspace at a time
- a shared workspace crawl so ignore/recovery/path semantics do not drift
- per-family ingestion packages that decide which files they need, parse them, and build checks input
- pure checks packages that stay ignorant of crawl and ingestion

The plan file captures that architecture before implementation starts.

## Decisions Made

### Introduce a shared per-workspace crawl package
- **Chose:** define `g3rs-workspace-crawl` as the first stage in the new pipeline.
- **Why:** it centralizes filesystem traversal, ignore handling, and recoverability semantics once, which avoids each ingestion package becoming its own mini crawler.
- **Alternatives considered:**
  - Let every ingestion package crawl for itself — rejected because it would duplicate `.gitignore` and file-visibility logic and quickly drift.
  - Reuse the old full-repo topology/legality stack — rejected because the new app is supposed to validate one explicit workspace, not solve ownership across the whole repository.

### Name the middle layer `ingestion`, not `mapping`
- **Chose:** describe the per-family middle stage as ingestion.
- **Why:** this stage is doing more than slicing existing parsed data; it selects files, reads them, parses them, and assembles checks input.
- **Alternatives considered:**
  - Keep calling it a mapper — rejected because that term implies a narrower already-parsed transformation step.
  - Split ingestion and mapping immediately — rejected because there is no demonstrated need for a second post-parse layer yet.

### Keep stage dependencies types-only between packages
- **Chose:** document that crawl, ingestion, and checks facades should expose `types` and `logic` separately, with packages depending on each other only through `types`.
- **Why:** it keeps stages independently testable and prevents hidden runtime coupling where ingestion starts calling checks or crawl starts embedding family semantics.
- **Alternatives considered:**
  - Let packages call each other directly — rejected because it blurs stage boundaries and recreates the old tightly coupled flow.
  - Collapse ingestion into the checks packages — rejected because that would make checks impure again and mix file reading/parsing into rule crates.

## Architectural Notes
- The agreed pipeline is:

  ```text
  workspace root
    -> g3rs-workspace-crawl
    -> g3rs-<family>-<surface>-ingestion
    -> g3rs-<family>-<surface>-checks
  ```

- The simplification relative to the old app is not that crawl/slice disappears. The simplification is that the new system validates one explicit workspace and never tries to resolve cross-workspace legality or ownership.
- `g3rs-workspace-crawl` should remain filesystem-neutral. Family-specific file selection belongs only in ingestion packages.
- This plan intentionally stops before building the new minimal app. The next implementation stage is package-level first, then the orchestrator last.

## Information Sources
- `AGENTS.md` — repo instructions and Rust-only direction
- `.worklogs/2026-04-06-112700-rename-g3rs-config-ast-packages.md` — latest extracted-package rename state
- `packages/g3rs-garde-ast-checks/` — current example of an extracted AST checks package
- Conversation decisions from this session about:
  - one explicit workspace root
  - shared crawl semantics
  - ingestion owning selection/parsing
  - checks staying pure

## Open Questions / Future Considerations
- The plan still needs a concrete package-structure spec for the new crawl and ingestion packages, based on the actual current `g3rs-*` package layout.
- The exact crawl output types are still placeholders; they should be finalized only after inspecting how the existing checks packages structure their facades and internal crates.
- Some ingestion packages may need structured ingestion-failure outputs in addition to checks-input outputs; the plan intentionally leaves that open for the first implementation pass.

## Key Files for Context
- `.plans/2026-04-06-g3rs-workspace-crawl-and-ingestion.md` — the new architecture plan for the `crawl -> ingestion -> checks` stack
- `.worklogs/2026-04-06-112700-rename-g3rs-config-ast-packages.md` — package rename context and current extracted-package baseline
- `packages/g3rs-garde-ast-checks/Cargo.toml` — representative extracted AST package manifest
- `packages/g3rs-garde-ast-checks/crates/runtime/src/run.rs` — representative extracted AST runtime entrypoint
- `packages/g3rs-fmt-config-checks/crates/types/src/lib.rs` — representative config-check input facade

## Next Steps / Continuation Plan
1. Commit the plan file so it becomes part of the tracked architecture record.
2. Inspect the current `g3rs-*` package layout, especially `g3rs-garde-ast-checks` and a representative config package, and extract the package-structure conventions that should be preserved.
3. Write a second detailed plan describing the exact package structure for:
   - `g3rs-workspace-crawl`
   - `g3rs-<family>-<surface>-ingestion`
   - the relationship to existing `g3rs-<family>-<surface>-checks`
4. Only after that structure plan is stable, start implementation with `g3rs-workspace-crawl`, then one ingestion package, then the new minimal orchestrator.
