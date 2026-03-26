# Sync the Remaining Rust Routing Cleanup Slice

**Date:** 2026-03-26 13:55
**Scope:** `.plans/todo/check_review/test_hardening/README.md`, `.plans/todo/check_review/test_hardening/34-test-family-rewrite-agent-brief.md`, `.plans/todo/check_review/test_hardening/35-arch-family-rewrite-agent-brief.md`, `.plans/todo/test_architecture.md`, `apps/guardrail3/Cargo.lock`, `apps/guardrail3/crates/app/hooks/*`, `apps/guardrail3/crates/app/rs/ast/src/ast_helpers_tests.rs`, `apps/guardrail3/crates/app/rs/families/{clippy,code,garde,hexarch,hooks-rs,hooks-shared,release}/*`

## Summary
Committed the remaining dirty slice around the Rust routing refactor so the repo state is coherent and clean. The substantive pieces are the lockfile/workspace sync for the new selection and mapper crates, the hook aggregator narrowing back to shared hooks only, and a set of small test/support cleanups and plan artifacts that were already part of the active refactor stream.

## Context & Problem
Two commits already landed the real architectural work:

- `.worklogs/2026-03-26-133815-rs-selection-mapper-test-route-slice.md`
- `.worklogs/2026-03-26-134708-rs-arch-route-migration.md`

But the repo still had a large dirty tail. Some of it was mechanical formatting, some of it was lockfile/workspace fallout from the new crates, and some of it was semantically relevant:

- `guardrail3-app-hooks` had dropped the `hooks-rs` family aggregation
- the hardening-plan README referenced new rewrite briefs
- several family test/support files had small edits that needed to travel with the refactor stream

The user explicitly wanted the repository owned end-to-end and committed cleanly, rather than leaving these changes hanging around as an unexplained mixed state.

## Decisions Made

### Commit the remaining dirty slice as a separate sync checkpoint
- **Chose:** Landed the residual dirty files in one follow-up commit instead of trying to fold them into either prior architecture commit retroactively.
- **Why:** The prior two commits already had a clear architectural story. This remaining slice is best understood as repository synchronization around that work, not as part of the original route-extraction diff.
- **Alternatives considered:**
  - Amend the earlier commits — rejected because the repo already had a coherent, auditable history and the user did not ask for amended history.
  - Leave the remaining files uncommitted — rejected because the user explicitly asked for the repo to be owned and cleaned up fully.

### Keep the hook aggregator narrowed to shared hooks only
- **Chose:** Preserved the change in `apps/guardrail3/crates/app/hooks/mod.rs` and `apps/guardrail3/crates/app/hooks/Cargo.toml` that removes `guardrail3_app_rs_family_hooks_rs` from the aggregator path.
- **Why:** This state already compiled and tested cleanly, and it matches the current repo direction where the shared hook family is the active one wired through the app-level hooks entrypoint.
- **Alternatives considered:**
  - Re-add `hooks-rs` to the aggregator just to avoid a semantic diff in this cleanup commit — rejected because that would be an unrelated behavioral rollback with no validation basis.

### Treat the family/test/support edits as part of the same active stream
- **Chose:** Included the scattered family test/support changes and the lockfile refresh in this checkpoint.
- **Why:** They were already live in the working tree, they validated cleanly across the affected packages, and keeping them uncommitted would leave the architecture stream in a misleading half-finished state.
- **Alternatives considered:**
  - Split the small family/test-support edits into many tiny commits — rejected because the edits are largely mechanical and not worth separate history noise.
  - Revert everything except the hook and lockfile changes — rejected because the validation run was against the full dirty tree, not a narrower arbitrary subset.

## Architectural Notes
This commit does not change the core architecture direction. It stabilizes the repo after the routed-root refactor:

- `placement` remains the shared Rust root-scope source
- `family_selection` remains the sole family-enablement stage
- `FamilyMapper` remains a pure mapper into typed family routes
- `rs/test` and `rs/arch` remain the two families migrated off family-local live-root crawling

The lingering repo-wide differences were mostly ecosystem fallout around that migration, not competing architecture.

One notable semantic state after this sync is that `apps/guardrail3/crates/app/hooks/mod.rs` now delegates only to `guardrail3_app_rs_family_hooks_shared::check(...)`. If the project later wants `hooks-rs` back in the product path, that should be handled as an intentional hook-architecture decision, not as incidental cleanup.

## Information Sources
- `.worklogs/2026-03-26-133815-rs-selection-mapper-test-route-slice.md` — initial `family_selection` / `FamilyMapper` extraction
- `.worklogs/2026-03-26-134708-rs-arch-route-migration.md` — `rs/arch` migration onto typed routes
- `apps/guardrail3/crates/app/hooks/mod.rs` — current app-level hook aggregation state
- `apps/guardrail3/Cargo.lock` — workspace dependency graph after adding mapper/selection crates and removing the `hooks-rs` app dependency
- `.plans/todo/check_review/test_hardening/README.md` — current brief index for the test and arch family rewrites

## Open Questions / Future Considerations
- Whether `hooks-rs` should stay detached from the app-level hooks entrypoint needs an explicit product decision if it becomes relevant again.
- `hexarch` still performs substantial deep discovery internally; that is separate from top-level Rust root scoping, but it is still a remaining family architecture topic.
- The new `app/rs` plan now needs a fresh implementation review against the actual code, since the repo is finally in a clean post-refactor state.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — current source of truth for shared Rust scope, selection, and mapping
- `apps/guardrail3/crates/app/rs/runtime.rs` — shared runtime entrypoint that now performs root collection once
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — family selection boundary
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — typed family-route mapping
- `apps/guardrail3/crates/app/hooks/mod.rs` — current app-level hook aggregation path
- `apps/guardrail3/Cargo.lock` — workspace crate graph after the routing refactor
- `.worklogs/2026-03-26-133815-rs-selection-mapper-test-route-slice.md` — prior routed-root implementation checkpoint
- `.worklogs/2026-03-26-134708-rs-arch-route-migration.md` — prior arch migration checkpoint

## Next Steps / Continuation Plan
1. Re-read `apps/guardrail3/crates/app/rs/README.md` and compare it directly to the live code in `runtime.rs`, `family_selection`, `family_mapper`, `rs/arch`, and `rs/test`.
2. Identify remaining architectural mismatches, especially any place where families still decide live root scope or receive more shared state than the README now allows.
3. Decide the next migration target after `arch` and `test`, with special attention to whether `hexarch` needs a routed family entrypoint or only internal discovery cleanup.
4. If the hook aggregation narrowing was intentional, document that explicitly in the Rust/hook planning docs so it is not rediscovered later as unexplained behavior.
