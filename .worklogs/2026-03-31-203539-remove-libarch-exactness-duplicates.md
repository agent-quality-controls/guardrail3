# Remove Libarch Exactness Duplicates

**Date:** 2026-03-31 20:35
**Scope:** `apps/guardrail3/crates/app/rs/families/libarch`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_12_declared_workspace_members_only_tests/mod.rs`, `libarch` docs/plans

## Summary
Removed the last local duplicate workspace-membership exactness rules from `libarch`, deleted their supporting facts/assertions/tests, and added package-side `RS-ARCH-12` proof coverage so exactness is now owned only by `arch` across both apps and packages. I then attacked the separation directly with repo-wide grep and lean family runs to prove `hexarch` and `libarch` no longer emit their old exactness IDs.

## Context & Problem
The previous commit removed duplicate exactness ownership from `hexarch`, but the split was still not actually clean because `libarch` still enforced the same concept through:

- `RS-LIBARCH-05`
- `RS-LIBARCH-06`

That meant the repo still had three different owner stories for one topology concept:
- `arch` for global exactness
- `hexarch` no longer
- `libarch` still yes

The user explicitly wanted the split to be complete:
- `arch` enforces workspace-membership exactness
- `hexarch` does not
- `libarch` does not

So this pass had to finish the separation instead of stopping at the app-side half.

## Decisions Made

### Delete `RS-LIBARCH-05/06` instead of renaming or soft-deprecating them
- **Chose:** remove the production rules, their assertion modules, and their rule-sidecar tests entirely.
- **Why:** these rules were duplicate ownership, not a distinct library concept. Keeping them under a new name would preserve the architectural mistake.
- **Alternatives considered:**
  - Keep them as warnings or inventory-only checks — rejected because they would still mean `libarch` owns workspace-membership exactness.
  - Keep the files but stop calling them — rejected because dead rule artifacts and tests would continue to mislead future agents.

### Move the proof to `arch`, not to new `libarch` tests
- **Chose:** add package-root exactness cases to `RS-ARCH-12` tests rather than migrating `libarch` exactness fixtures.
- **Why:** the proof should live with the real owner. If package-side exactness matters, `arch` should prove it directly.
- **Alternatives considered:**
  - Port the `libarch` fixtures into generic shared legality tests only — rejected because the family-level owner is `RS-ARCH-12`, so family tests should prove package-side ownership too.
  - Keep package-only exactness tests under `libarch` “for convenience” — rejected because it preserves the wrong mental model.

### Remove dead `libarch` facts/helpers instead of leaving hidden support behind
- **Chose:** delete `workspace_members*`, `WorkspaceMemberFacts`, `expected_layer_dir_rels()`, and the member-parsing helpers from `libarch` facts/package-support.
- **Why:** after deleting `05/06`, those facts no longer powered any live rule. Leaving them around would keep the duplicate concept latent inside the family.
- **Alternatives considered:**
  - Leave the facts in place in case a future rule wants them — rejected because dormant ownership is still ownership drift.
  - Move them to shared support now — rejected because `arch` already has the real exactness implementation in shared legality.

## Architectural Notes
- `RS-ARCH-12` is now the only live owner of workspace-membership exactness across governed workspaces.
- `libarch` now owns:
  - escalation
  - layered crate-set shape
  - layered dependency direction
  - root facade export policy
- `libarch` no longer owns:
  - missing workspace members
  - extra workspace members
  - workspace-membership parse failures as a separate library-local enforcement concept
- The proof is now layered:
  - direct `arch` tests cover app and package workspaces
  - lean `hexarch` and `libarch` runs still include `arch` results, but no longer emit their own duplicate rule IDs

## Information Sources
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/facts/package_support.rs`
- `apps/guardrail3/crates/app/rs/families/libarch/README.md`
- `.plans/by_family/rs/libarch.md`
- `.plans/todo/checks/rs/libarch.md`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_12_declared_workspace_members_only_tests/mod.rs`
- prior worklog: `.worklogs/2026-03-31-200335-arch-workspace-membership-exactness.md`

## Open Questions / Future Considerations
- The runtime expectation suite is still stale in a few areas unrelated to this specific owner split, especially around routed global-family behavior.
- `hexarch`’s broad unit corpus still needs its own cleanup pass, but the exactness ownership proof no longer depends on that suite because the duplicate rule IDs are gone from code and outputs.
- `RS-ARCH-12` is now clearly responsible for package-side exactness. If package-topology policy expands further, keep adding those tests under `arch` rather than drifting back into `libarch`.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/lib.rs` — live libarch rule inventory after removing `05/06`.
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/facts.rs` — simplified libarch fact model with dead exactness state removed.
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/facts/package_support.rs` — dead workspace-member parsing removed.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_12_declared_workspace_members_only_tests/mod.rs` — now includes package-side exactness proof cases.
- `apps/guardrail3/crates/app/rs/families/libarch/README.md` — updated family ownership statement.
- `.plans/by_family/rs/libarch.md` — updated family-level handoff contract.
- `.plans/todo/checks/rs/libarch.md` — detailed ledger showing `libarch` no longer owns workspace-membership exactness.
- `.worklogs/2026-03-31-200335-arch-workspace-membership-exactness.md` — prior app-side half of the ownership cleanup.

## Next Steps / Continuation Plan
1. Repair the remaining stale runtime expectation tests so the runtime suite explicitly proves the current global-vs-local routed-family contracts.
2. Continue the `hexarch` suite cleanup, but treat it as a separate job: classify failures into stale assumptions, stale routed-surface expectations, and real production bugs.
3. Keep using lean family runs plus repo-wide grep as the attack pattern for future ownership moves: first delete duplicate rule IDs, then prove the surviving owner still shows up in unrelated family runs.
