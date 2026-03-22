# ProjectTree Query Helpers

**Date:** 2026-03-22 15:07
**Scope:** `apps/guardrail3/crates/domain/project_tree.rs`, `apps/guardrail3/crates/domain/project_tree_tests.rs`, `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs`, `apps/guardrail3/crates/app/rs/checks/rs/toolchain/discover.rs`

## Summary
Moved generic directory and file query logic into `ProjectTree` so family discoverers stop reimplementing basic traversal. Then updated the cargo and toolchain family discoverers to use the new tree helpers instead of open-coded structure walking.

## Context & Problem
While reviewing the new family architecture, the user called out that `cargo/discover.rs` looked too much like a reimplementation of `ProjectTree`. That concern was valid in a narrow way: the family discoverer contained generic “all dirs”, “dirs containing Cargo.toml”, and directory glob matching logic that should be available directly from the tree abstraction.

The architectural goal is:
- `ProjectTree` owns generic repo query helpers
- family discoverers own family-specific interpretation and fact construction

Before this change, `ProjectTree` only exposed very primitive operations (`dir_exists`, `dir_contents`, `file_content`, `join_rel`), which pushed each family toward ad hoc tree traversal.

## Decisions Made

### Add generic tree query helpers to `ProjectTree`
- **Chose:** Add `file_exists`, `all_dir_rels`, `dirs_with_file`, and `matching_dir_rels` to `ProjectTree`.
- **Why:** These are generic repository queries, not cargo-specific semantics. Families should be able to ask the tree for these facts without rewalking `structure`.
- **Alternatives considered:**
  - Leave traversal in each family discoverer — rejected because it duplicates generic logic and makes the tree abstraction too weak.
  - Add higher-level cargo-specific helpers to `ProjectTree` — rejected because workspace-member semantics belong in the cargo family, not in the generic tree object.

### Keep family `discover.rs`, but slim it down
- **Chose:** Retain `cargo/discover.rs` and `toolchain/discover.rs`, but rewrite them to use the new `ProjectTree` helpers.
- **Why:** We still need a family orchestrator/extractor layer that turns generic tree data into typed family facts. Eliminating `discover.rs` entirely would push family semantics into `ProjectTree`, which would be the wrong abstraction.
- **Alternatives considered:**
  - Remove `discover.rs` and build facts directly in `mod.rs` — rejected because it would blur orchestration and fact-building.
  - Move cargo member resolution into `ProjectTree` — rejected because that is family semantics, not generic repository shape.

### Add direct tests for the new tree API
- **Chose:** Add sidecar tests in `project_tree_tests.rs`.
- **Why:** The new helper methods are domain infrastructure and should have direct tests, especially glob-matching behavior and root-vs-nested file existence.
- **Alternatives considered:**
  - Rely only on cargo/toolchain family tests — rejected because the helpers are now reusable infrastructure and deserve their own contract tests.

## Architectural Notes
This change clarifies the intended layering:
- `ProjectTree`: generic repository snapshot + generic query API
- `discover.rs`: family-specific extraction over `ProjectTree`
- `facts.rs`: normalized family facts
- `inputs.rs`: minimal rule inputs
- rule files: pure checks over atomic inputs

This is the right kind of reuse. The tree became more capable, but it still does not know anything about Cargo workspaces, rustfmt policy, or toolchain semantics.

## Information Sources
- `AGENTS.md` — current architectural direction for the checker library
- `apps/guardrail3/crates/domain/project_tree.rs` — existing tree API and its limitations
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs` — family-local traversal that exposed the missing generic helpers
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/discover.rs` — another family that benefited from `file_exists`
- `.worklogs/2026-03-22-145320-cargo-fmt-audit-hardening.md` — prior checkpoint that hardened cargo semantics before this refactor

## Open Questions / Future Considerations
- `fmt/facts.rs` still does some direct iteration over `tree.structure`; some of that may later collapse onto `ProjectTree` helpers if the same patterns appear in more than one family.
- As more families are added, we should resist turning `ProjectTree` into a semantic grab bag. Only generic query helpers belong there.

## Key Files for Context
- `apps/guardrail3/crates/domain/project_tree.rs` — generic tree API now includes reusable directory/file query helpers
- `apps/guardrail3/crates/domain/project_tree_tests.rs` — contract tests for the new tree helper methods
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs` — cargo extractor now consuming the generic tree helpers
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/discover.rs` — toolchain extractor using `file_exists`
- `.worklogs/2026-03-22-145320-cargo-fmt-audit-hardening.md` — previous cargo hardening checkpoint

## Next Steps / Continuation Plan
1. Commit this refactor as infrastructure so later family work can build on the stronger `ProjectTree` API.
2. Start `rs/clippy` next using the same family shape:
   - `facts.rs`
   - `inputs.rs`
   - `mod.rs`
   - sidecar tests
3. As `rs/clippy` is built, use the old adversarial config fixtures and plan doc to define failure-oriented tests rather than only happy-path checks.
