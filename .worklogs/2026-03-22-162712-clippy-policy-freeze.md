# Freeze Clippy Policy Contract

**Date:** 2026-03-22 16:27
**Scope:** `.plans/todo/checks/rs/clippy.md`, `.plans/by_file/rs/clippy-toml.md`

## Summary
Updated the clippy planning docs to reflect the final policy decisions from the architecture review and the now-implemented checker direction. The docs now describe policy-root placement, coverage semantics, hardened managed keys, exact threshold values, and the canonical source-of-truth order without the earlier contradictory notes.

## Context & Problem
The clippy family had accumulated drift across several places:
- the todo family inventory
- the by-file `clippy.toml` design doc
- the old generator module and validator behavior
- in-session decisions about hardening, extractability, and placement rules

Before committing the code changes, the planning documents needed to become explicit about the real contract. Without that freeze, the checker and generator refactor would still be anchored to stale assumptions like “workspace root only”, warning-level macro enforcement, or `RS-CLIPPY-19` flagging known user-owned keys like `msrv`.

## Decisions Made

### Freeze policy-root placement and coverage
- **Chose:** Define allowed `clippy.toml` locations as validation root, workspace roots, and standalone package roots that are not workspace members.
- **Why:** This keeps the checker coupled to Rust project structure instead of hex-arch or repo folder naming while still preserving extractability and anti-shadowing guarantees.
- **Alternatives considered:**
  - One global root only — rejected because it weakens extractability and local policy ownership.
  - Allow arbitrary per-member configs — rejected because clippy resolution is shadow-based and too easy to weaken.

### Freeze the managed key set and exact values
- **Chose:** Document the full managed key set: 7 thresholds, 3 booleans, and 3 ban arrays, with exact threshold values and required macro baseline.
- **Why:** The checker is supposed to harden upstream enforcement knobs, not only scan source after the fact. Exact values prevent quietly relaxed configs.
- **Alternatives considered:**
  - Keep only the older partial threshold set — rejected because it leaves universal clippy hardening on the table.
  - Describe only categories, not exact values — rejected because the generator and checker need a precise contract.

### Clarify RS-CLIPPY-19 and rejected findings
- **Chose:** State that known user-owned keys like `msrv` must not trigger `RS-CLIPPY-19`, and update the rejected-findings table accordingly.
- **Why:** The earlier doc still contradicted the agreed “warn only on truly unknown / typo-looking keys” behavior.
- **Alternatives considered:**
  - Leave the stale note in place and rely on code comments — rejected because the plan is supposed to be the policy source.

## Architectural Notes
This doc pass is intentionally policy-first. It does not attempt to encode implementation details like individual helper functions. Instead it freezes the contract that both the canonical generator module and the new checker family must follow:
- placement and coverage
- managed scalar settings
- method/type/macro baselines
- reason requirements
- source-of-truth order

That gives the next session a stable rule contract even if the rest of the Rust families are still mid-refactor.

## Information Sources
- In-session policy decisions from the clippy architecture review
- `.plans/todo/checks/rs/clippy.md` — main family rule inventory
- `.plans/by_file/rs/clippy-toml.md` — empirical clippy resolution behavior and generation/merge notes
- `.worklogs/2026-03-22-150728-project-tree-query-helpers.md`
- `.worklogs/2026-03-22-145320-cargo-fmt-audit-hardening.md`

## Open Questions / Future Considerations
- The docs still mention override/removal flows for generation that have not been fully reworked into the new generator architecture; that cleanup remains future work.
- Per-root profile resolution is still mostly global in the checker implementation. The contract is hardened now, but richer per-root profile derivation may still need a later pass.

## Key Files for Context
- `.plans/todo/checks/rs/clippy.md` — frozen clippy family rule contract
- `.plans/by_file/rs/clippy-toml.md` — clippy resolution model and generation/merge rules
- `apps/guardrail3/crates/domain/modules/clippy/mod.rs` — canonical generated clippy baseline entrypoint
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/mod.rs` — checker family entrypoint that should match the plan
- `.worklogs/2026-03-22-150728-project-tree-query-helpers.md` — prior architectural checkpoint

## Next Steps / Continuation Plan
1. Commit these planning docs first so the policy contract is explicit in history before the code refactor lands.
2. Commit the generator-side clippy refactor next:
   - `apps/guardrail3/crates/domain/modules/clippy/**`
   - `apps/guardrail3/crates/domain/modules/mod.rs`
3. Commit the checker-side family completion last:
   - `apps/guardrail3/crates/app/rs/checks/rs/clippy/**`
   - `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`
4. Leave unrelated dirty files in the worktree untouched; they are not part of the clippy line of work.
