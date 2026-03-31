# New Arch Family Cutover

**Date:** 2026-03-31 22:39
**Scope:** `apps/guardrail3/crates/domain/validation-model/src/families.rs`, `apps/guardrail3/crates/domain/config/types.rs`, `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/{lib.rs,rs.rs,views.rs}`, `apps/guardrail3/crates/app/rs/runtime/{Cargo.toml,src/lib.rs,src/runners.rs}`, `apps/guardrail3/crates/bin/guardrail3/Cargo.toml`, `apps/guardrail3/crates/adapters/inbound/cli/cli.rs`, `apps/guardrail3/crates/domain/report/mod.rs`, `apps/guardrail3/crates/app/rs/families/arch/**`, `apps/guardrail3/crates/app/rs/families/libarch/**`, `apps/guardrail3/crates/app/commands/src/messages.rs`, `apps/guardrail3/crates/domain/modules/guide.rs`, `apps/guardrail3/crates/app/rs/README.md`, `.plans/by_family/rs/{README.md,arch.md,libarch.md}`, `AGENTS.md`

## Summary
Introduced a new live `RS-ARCH` family for generic split-library crate architecture, wired it through the runtime/config/CLI/model stack, and moved the generic split rules out of `RS-LIBARCH`. Then removed the dead `libarch` rule/assertion/test files for `RS-LIBARCH-01/02/03` so runtime behavior and source ownership match.

## Context & Problem
After renaming the old repo-governance `arch` family to `topology`, the repo still had no real cross-crate architecture family. The user wanted the next phase to be explicit: create a new `arch` family only after the old `arch` name was gone, then dismantle `libarch` into it gradually.

The immediate migration target was the part of `libarch` that was never truly layered-shape-specific:
- escalation from flat library to internal split architecture
- split-root facade/workspace requirements
- prohibition on direct external dependencies into internal crates

Those rules did not belong in retiring `libarch`, and they did not belong in `topology` or app-only `hexarch`.

## Decisions Made

### Create `RS-ARCH` as a real first-class family now
- **Chose:** Add a new active `RustValidateFamily::Arch` with runtime/config/report/CLI support.
- **Why:** The repo needed an actual owner for generic crate architecture before more `libarch` removal could happen.
- **Alternatives considered:**
  - Keep those rules in `libarch` until the whole replacement family is finished — rejected because it prolongs mixed ownership and keeps the wrong semantics attached to `libarch`.
  - Move them into `topology` — rejected because they are not root/workspace topology rules.

### Route `arch` over enabled roots, not family-file legality
- **Chose:** `map_rs_arch()` uses `map_global_roots_for_family(...)` and an empty local-family-file surface.
- **Why:** `arch` is not a “family owns a config file placement” family. When routed through family-file legality, flat package libraries disappeared because `Cargo.toml` legality is workspace/member-oriented.
- **Alternatives considered:**
  - Reuse manifest-file legality like `cargo`/`toolchain`/`libarch` — rejected because it made flat standalone library packages invisible.
  - Invent new `arch`-specific family-file ownership rules for `Cargo.toml` — rejected because that would be fake file ownership for a semantic architecture family.

### Keep the first `arch` candidate set intentionally narrow
- **Chose:** Current `arch` facts only materialize package-scoped split-library candidates, then allow findings about external consumers outside the package subtree.
- **Why:** This gets the `libarch` migration landed without immediately colliding with app-level `hexarch` semantics or accusing app workspaces of being facade packages.
- **Alternatives considered:**
  - Make `arch` immediately repo-wide for every root — rejected because the candidate heuristics would falsely classify app workspaces and other roots before the family contract is mature.
  - Keep `arch` package-only forever — rejected because the long-term goal is broader than the old `libarch`, but not necessary for this cutover.

### Remove migrated `libarch` source files completely
- **Chose:** Delete the old runtime/assertion/test files for `RS-LIBARCH-01/02/03` and strip their now-unused facts.
- **Why:** Leaving dead files behind would keep the wrong ownership visible in the repo and confuse future agents about what `libarch` still owns.
- **Alternatives considered:**
  - Keep dead files around but stop calling them from `lib.rs` — rejected because that is only runtime separation, not source-of-truth separation.
  - Add `#[allow(dead_code)]` to the old facts and modules — rejected because it papers over the ownership change instead of completing it.

## Architectural Notes
- `RS-ARCH` is now the generic split-library crate-architecture family.
- `RS-LIBARCH` is now explicitly retirement-only and only retains the legacy layered-shape contract (`api/core/infra` era).
- `RS-ARCH` does not currently rely on local family-file legality. It is routed from enabled roots and does its own package-candidate narrowing in `facts.rs`.
- `RS-TOPOLOGY` still appears in lean `arch` and `libarch` runs because topology remains the shared legality/reporting surface for root legality and routing prerequisites.
- The first `arch` rules are:
  - `RS-ARCH-01` flat library exceeds thresholds and must split
  - `RS-ARCH-02` split root must remain workspace facade package
  - `RS-ARCH-03` split root must actually own internal member crates
  - `RS-ARCH-04` external roots must not depend directly on internal member crates

## Information Sources
- Existing repo architecture docs:
  - `AGENTS.md`
  - `apps/guardrail3/crates/app/rs/README.md`
- Current family plans:
  - `.plans/by_family/rs/README.md`
  - `.plans/by_family/rs/libarch.md`
- Prior migration planning:
  - `.plans/todo/arch-topology-libarch-migration-handoff.md`
- Existing `libarch` implementation and rule surface:
  - `apps/guardrail3/crates/app/rs/families/libarch/README.md`
  - `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/facts.rs`
- Shared runtime/mapper/config/model code:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
  - `apps/guardrail3/crates/domain/config/types.rs`
  - `apps/guardrail3/crates/domain/validation-model/src/families.rs`

## Open Questions / Future Considerations
- `RS-ARCH` is currently package-scoped for candidate discovery. The intended end state is broader, but broadening should happen deliberately so it does not overlap incorrectly with `hexarch`.
- `libarch` still exists because the layered-shape rules have not been generalized or deleted yet.
- Lean `arch` and `libarch` CLI runs still include a `topology` section by design because topology remains a shared prerequisite/reporting family.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — new family contract and intended ownership.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs` — live `arch` runtime entrypoint and rule list.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs` — candidate discovery and external dependency hit modeling.
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — routing choice for `map_rs_arch()`.
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — live runtime dispatch and lean family execution wiring.
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/lib.rs` — shows what `libarch` still runs after the cutover.
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/facts.rs` — remaining `libarch` fact surface after removing generic split logic.
- `.plans/by_family/rs/arch.md` — family-agent handoff for the new `arch`.
- `.plans/by_family/rs/libarch.md` — updated retirement status and remaining `libarch` ownership.
- `.worklogs/2026-03-31-223941-new-arch-family-cutover.md` — this worklog.

## Next Steps / Continuation Plan
1. Expand `RS-ARCH` from package-scoped split-library candidates toward the intended repo-wide crate-architecture surface, but do it with explicit candidate heuristics and tests instead of reusing `hexarch` concepts.
2. Identify which remaining `libarch` rules are truly obsolete layered-shape baggage versus candidates for generalized migration into `arch`.
3. Add stronger runtime/regression tests that prove `arch` catches direct external dependencies on internal member crates across more real repo shapes, not just the small unit fixtures.
4. Update the remaining user-facing Rust family docs/help that still present `libarch` as a first-class long-term family rather than a retiring one, if that distinction should become public.
