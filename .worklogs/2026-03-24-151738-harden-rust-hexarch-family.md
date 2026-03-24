# Harden Rust Hexarch Family

**Date:** 2026-03-24 15:17
**Scope:** `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/core/project_walker_tests.rs`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/**`, `.plans/todo/check_review/test_hardening/01-hexarch.md`, `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md`, `.plans/todo/check_review/test_hardening/16-hexarch-execution-plan.md`

## Summary
Committed the `rs/hexarch` hardening batch together with the `ProjectTree` walker changes it depends on. The packet deepens ownership, workspace-member, dependency-edge, and fail-closed coverage across the family and strengthens walker behavior around ignored directories so structural rules do not silently miss owned children.

## Context & Problem
The remaining dirty tree after the other family commits was a single coherent hexarch/discovery packet:
- `ProjectTree` walker behavior for ignored directories and recovered children
- large fact/input changes in the hexarch family
- many new attack-vector files across workspace, dependency, and boundary rules
- updated hexarch hardening docs and execution notes

Keeping the walker changes separate would have made the hexarch test hardening look arbitrary. Several strengthened attacks depend directly on the walker preserving or recovering the right child directories instead of letting them disappear before ownership rules can see them.

## Decisions Made

### Commit walker changes with hexarch
- **Chose:** Include `project_walker.rs` and its tests in the hexarch batch.
- **Why:** The walker behavior is structural input to hexarch ownership/discovery rules, not a generic unrelated refactor.
- **Alternatives considered:**
  - Put the walker in a standalone substrate commit — rejected because the motivating semantics are hexarch-specific in this dirty-tree slice.
  - Leave the walker for a later cleanup — rejected because the current hexarch tests would then be ahead of the actual tree semantics.

### Treat the batch as convergence on real workspace semantics
- **Chose:** Frame the batch around semantic resolution of workspace members, app-boundary ownership, and dependency direction rather than around raw test count growth.
- **Why:** The changes are about catching fake hexarch folder layouts and real Cargo/workspace drift, not just widening the suite.
- **Alternatives considered:**
  - Describe it as sidecar expansion only — rejected because the facts/input layers changed significantly.

### Keep the hardening docs with the family batch
- **Chose:** Include the updated lane status, brief, and execution plan.
- **Why:** Those docs now record the real ownership/discovery fixes that landed in the code, especially around `RS-HEXARCH-06/07/10/11/13`.
- **Alternatives considered:**
  - Move docs later — rejected because they would lag behind the family behavior immediately.

## Architectural Notes
- `ProjectTree` is still the shared discovery substrate, but hexarch is the family currently most sensitive to whether ignored and recovered directories remain visible.
- This batch strengthens the “hexarch must be real Cargo/workspace structure, not just folders” direction:
  - workspace-member matching is semantic
  - app-boundary membership is explicit
  - cross-app and out-of-boundary edges are treated distinctly

## Information Sources
- `apps/guardrail3/crates/app/core/project_walker.rs`
- `apps/guardrail3/crates/app/core/project_walker_tests.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/**`
- `.plans/todo/check_review/test_hardening/01-hexarch.md`
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md`
- `.plans/todo/check_review/test_hardening/16-hexarch-execution-plan.md`

## Open Questions / Future Considerations
- The Rust validation cutover still needs to make this family the live public path so the tool catches fake hexarch package layout in its own runtime.
- Long-term test performance still points toward real crate/test-target splitting, but that is separate from this family hardening packet.

## Key Files for Context
- `apps/guardrail3/crates/app/core/project_walker.rs` — tree discovery behavior that feeds family orchestrators
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/facts.rs` — normalized hexarch facts
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/dependency_facts.rs` — dependency-edge fact layer
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/source_facts.rs` — source-surface fact layer
- `.plans/todo/check_review/test_hardening/01-hexarch.md` — hexarch lane status after these fixes

## Next Steps / Continuation Plan
1. Commit the remaining small generator-helper test that was stranded after the clippy/deny packet.
2. Verify the worktree is clean.
3. Hand the cleaned branch back for runtime cutover and remaining implementation work.
