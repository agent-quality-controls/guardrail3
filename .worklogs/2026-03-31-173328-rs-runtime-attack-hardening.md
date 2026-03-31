# RS Runtime Attack Hardening

**Date:** 2026-03-31 17:33
**Scope:** `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs`, `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_16_workspace_local_file_placement_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_16_workspace_local_file_placement_tests/outside_workspace_guardrail.rs`

## Summary
Ran adversarial post-refactor attack passes against the Rust runtime, family mapper, and arch placement path after the strict `ProjectTree` separation. The attack pass found two real regressions: the `code` runner dropped repo-root `guardrail3.toml` from routed surfaces, and arch filtered misplaced `guardrail3.toml` out before `RS-ARCH-16` could report it. Both were fixed and pinned with tests.

## Context & Problem
The current Rust refactor changed the input boundary from raw `ProjectTree` access to routed `RsProjectSurface` slices. That kind of change is high risk because the families can still compile while silently losing files they previously relied on, or because routed workspace-local runs can start leaking into sibling workspaces or stop running per legal workspace.

The user asked specifically for attack-style verification, not a happy-path rerun:
- make the new surface break if it is wrong
- verify workspace-local families actually run per legal workspace
- verify scoped runs do not leak into sibling workspaces
- verify refactoring the surface did not make config files invisible

## Decisions Made

### Fix dropped repo-root policy input for `code`
- **Chose:** add `guardrail3.toml` back into the routed surface built by `run_code()` via `code_extra_files(...)`.
- **Why:** the runtime attack immediately exposed that `RS-CODE-30` disappeared on malformed repo-root policy input because the routed `code` surface no longer carried `guardrail3.toml`.
- **Alternatives considered:**
  - Leave the surface root-only and treat missing repo policy as acceptable in scoped `code` runs — rejected because it weakens fail-closed behavior and contradicts the family’s existing dependency on repo policy.
  - Rework `code` into a fully different routing shape during this pass — rejected because the concrete regression was a dropped file, not a proven need for a new routing model.

### Fix arch placement reporting for misplaced `guardrail3.toml`
- **Chose:** stop filtering `RustFamilyFileKind::GuardrailToml` out of `is_arch_tracked_family_file(...)`.
- **Why:** legality already classified misplaced workspace-local family files, and arch already had `RS-ARCH-16` to report them, but the mapper dropped `guardrail3.toml` before arch ever saw it.
- **Alternatives considered:**
  - Keep `guardrail3.toml` excluded to avoid duplicate reports across `cargo` / `deps` / `garde` ownership — rejected because complete invisibility is worse than duplication, and the current architecture says arch owns illegal family-file placement.
  - Move misplaced `guardrail3.toml` reporting into the consuming families — rejected because that would push placement legality back into family logic, which is the opposite of the current architecture split.

### Strengthen runtime tests around workspace-local routing
- **Chose:** add runtime helpers/tests for scoped `clippy` and scoped `cargo`, then tighten the assertions so they prove one owning workspace surface rather than only proving expected files are present.
- **Why:** the fanout logic in `runners.rs` is now central. The attack pass needed coverage that a scoped run stays inside the owning legal workspace and does not spill into siblings.
- **Alternatives considered:**
  - Rely only on mapper tests — rejected because routing bugs can still happen between mapper output and runner surface construction.
  - Assert a single live finding for `cargo` — rejected after attack rerun showed `cargo` legitimately emits multiple live rule results for one workspace; the correct invariant is one owning workspace, not one result.

## Architectural Notes
- The runtime currently has three distinct layers that matter for these failures:
  - `structure` and `legality` discover and classify roots/files once.
  - `FamilyMapper` slices legal family surfaces plus arch-visible illegal file placement.
  - runtime runners build the concrete `RsProjectSurface` each family sees.
- The `code` regression happened in the runner layer, not the mapper.
- The `guardrail3.toml` arch leak happened in the mapper layer, not in legality or the arch family itself.
- The attack pass also surfaced a still-open contract mismatch:
  - docs/spec still describe `code` and probably `test` as repo-global families
  - current runtime/tests still narrow `code` by validation scope
  - this worklog records that mismatch but does not resolve it

## Information Sources
- Local plans and family docs:
  - `.plans/todo/checks/rs/arch.md`
  - `.plans/by_family/rs/arch.md`
  - `apps/guardrail3/crates/app/rs/README.md`
  - family READMEs for `deny`, `cargo`, `clippy`, `toolchain`
- Runtime and mapper code:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs_tests/mod.rs`
- Arch family runtime/tests:
  - `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_16_workspace_local_file_placement.rs`
- Prior context:
  - `.worklogs/2026-03-31-171258-close-project-tree-family-leak.md`

## Open Questions / Future Considerations
- `deny` and other arch-owned placement families still have a product/contract question for family-only runs: if a caller requests only one workspace-local family, should illegal placement still surface automatically through arch, or is that intentionally outside the selected run?
- `code` and `test` still need a contract decision or follow-up implementation because docs currently describe them as global while runtime/tests still narrow at least `code`.
- `fmt` and `test` still lack the same style of dedicated runtime routing attack coverage that now exists for `toolchain`, `clippy`, `cargo`, `deps`, and `release`.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — concrete family surface construction; this is where the dropped `guardrail3.toml` bug lived.
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` — runtime routing/scope regression tests; now includes stronger workspace-local scope assertions.
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — arch illegal-file routing filter and family slicing logic.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_16_workspace_local_file_placement.rs` — rule that reports illegal family-file placement once arch sees the file.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_16_workspace_local_file_placement_tests/outside_workspace_guardrail.rs` — regression test proving misplaced `guardrail3.toml` is no longer invisible.
- `.worklogs/2026-03-31-171258-close-project-tree-family-leak.md` — prior strict separation worklog that this attack pass was validating.

## Next Steps / Continuation Plan
1. Resolve the contract for family-only runs versus arch-owned illegal placement. If arch must always report illegal placements, add an explicit runtime mechanism instead of leaving it implicit in `--family` selection.
2. Decide and implement the real global-vs-local behavior for `code` and `test`, then update both runtime tests and docs so they stop disagreeing.
3. Add parity attack coverage for the remaining risky families:
   `deny`, `garde`, `fmt`, and `test`, with explicit sibling-isolation or global-surface assertions depending on family scope.
