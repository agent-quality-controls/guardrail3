# Hexarch Naming Correction

**Date:** 2026-03-31 21:57
**Scope:** `apps/guardrail3/crates/app/hexarch-helpers`, `apps/guardrail3/crates/app/ts/validate/topology`, `apps/guardrail3/crates/app/rs/validate`, `apps/guardrail3/crates/adapters/inbound/cli/init.rs`, `apps/guardrail3/crates/domain/{modules,report}`, `README.md`, `GUARDRAIL3_GUIDE.md`, `apps/guardrail3/crates/app/commands/src/messages.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/**`

## Summary
Corrected the over-rename introduced by the strict `arch` to `topology` cleanup. The repo had started describing hex-structure checks as `hex topology` and had renamed the shared helper crate to `topology-helpers`, which conflated the global `topology` family with the separate `hexarch` concept. This batch moves those surfaces to `hexarch` naming instead.

## Context & Problem
After the strict topology rename commit (`107698b7`), a test-attack pass found a semantic bug rather than a stale old-name bug: several surfaces that are not part of the global `topology` family had been renamed too far. In particular:

- the shared helper crate was renamed to `topology-helpers`
- TS topology checks and legacy Rust validate surfaces started saying `hex topology`
- active help/docs started teaching `hex topology` in places where the project really means `hexarch`

The user clarified the desired naming rule: do not use standalone `hex`; use family names like `hexarch-*`.

## Decisions Made

### Rename the shared helper crate to `hexarch-helpers`
- **Chose:** Move `apps/guardrail3/crates/app/topology-helpers` to `apps/guardrail3/crates/app/hexarch-helpers` and rename the package/import surface to `guardrail3-app-hexarch-helpers`.
- **Why:** The crate is not helping the global `topology` family. It is only used by hex-structure checking code in TS and legacy Rust validate surfaces, so `topology-helpers` was semantically wrong.
- **Alternatives considered:**
  - Keep `topology-helpers` because the current TS family name is `topology` — rejected because the helper logic is about hexarch structure, not repo-global topology.
  - Rename it to generic `hex-helpers` — rejected because the user explicitly wanted family-based naming, not standalone `hex`.

### Rename `hex topology` wording to `hexarch`
- **Chose:** Replace `hex topology`, `hex topology template`, and `hex topology layout` with `hexarch`, `hexarch template`, and `hexarch layout`.
- **Why:** The previous wording implied that the global `topology` family owned or defined the app-local hex structure concept. It does not. `hexarch` is the right family/contract name.
- **Alternatives considered:**
  - Use `hexagonal topology` — rejected because the user explicitly asked to use family names.
  - Use generic `architecture` everywhere — rejected because the repo already has concrete family names and the user wants those names used directly.

### Keep `T-TOPOLOGY-*` and `RS-TOPOLOGY-01` IDs where they are the actual rule IDs
- **Chose:** Leave the topology family/rule IDs unchanged while renaming helper/function prose around hex structure to `hexarch`.
- **Why:** This batch is a naming-correction pass, not a rule-ownership migration. The IDs and category routing are still current product surfaces and changing them would be a much larger semantic move.
- **Alternatives considered:**
  - Rename TS topology IDs to `T-HEXARCH-*` — rejected because that would change the active family surface, not just fix wording overshoot.

## Architectural Notes
This restores the distinction between:

- `topology`: the global legality/placement family
- `hexarch`: the app-local structural pattern and its helper utilities

The shared helper crate now reflects what it actually helps. The TS topology family still owns the TS rule IDs/categories, but the underlying structure it checks is described as `hexarch`, which matches the project’s family vocabulary.

## Information Sources
- Test-attack results from this session flagging rename overshoot.
- `apps/guardrail3/crates/app/hexarch-helpers/src/lib.rs`
- `apps/guardrail3/crates/app/ts/validate/topology/ts_topology_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/mod.rs`
- `apps/guardrail3/crates/app/rs/validate/topology/rs_topology_01/mod.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs`
- Prior rename worklog: `.worklogs/2026-03-31-214504-strict-topology-rename-finalization.md`

## Open Questions / Future Considerations
- The broader test-attack pass also found stale user-facing guide/help contradictions around `topology` and `libarch` in some command/guide surfaces. Those are separate from this `hexarch` naming correction and should be cleaned in a follow-up pass.
- Legacy `app/rs/validate` remains outside the workspace. Active surfaces compile through the main workspace, but the legacy crate still cannot be checked directly without either re-adding it to the workspace or adding local workspace isolation in its manifest.

## Key Files for Context
- `apps/guardrail3/crates/app/hexarch-helpers/src/lib.rs` — shared helper crate after the correction.
- `apps/guardrail3/crates/app/ts/validate/topology/ts_topology_checks.rs` — TS topology family now uses `hexarch` wording and helper import names.
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy Rust validate module paths now point at `hexarch_*` modules.
- `apps/guardrail3/crates/app/rs/validate/topology/hexarch_structure.rs` — renamed legacy hexarch structure module.
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs` — generated TS config/help wording now says `hexarch`.
- `.worklogs/2026-03-31-214504-strict-topology-rename-finalization.md` — prior strict topology rename checkpoint that this batch corrects.

## Next Steps / Continuation Plan
1. Clean the remaining test-attack findings unrelated to this naming overshoot:
   - stale `architecture checks` wording in family READMEs/tests
   - contradictory `topology` / `libarch` guide and help text in `messages.rs`, `guide.rs`, and `GUARDRAIL3_GUIDE.md`
2. Re-run the same adversarial greps used here after that cleanup to make sure no new wording drift was introduced.
3. If legacy `app/rs/validate` needs ongoing care, decide whether to make it a self-contained non-workspace crate or bring it back under workspace control for direct checks.
