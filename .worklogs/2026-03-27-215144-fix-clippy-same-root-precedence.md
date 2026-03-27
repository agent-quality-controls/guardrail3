# Fix Clippy Same-Root Precedence

**Date:** 2026-03-27 21:51
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/same_root_precedence.rs`, `.plans/todo/checks/rs/clippy.md`

## Summary
Corrected `RS-CLIPPY-12` to match real Clippy behavior when both `clippy.toml` and `.clippy.toml` exist at the same policy root. The family, its tests, and the rule plan had the precedence backward; `.clippy.toml` actually wins.

## Context & Problem
The ongoing adversarial pass on `rs/clippy` was specifically trying to prove live rule semantics against upstream Clippy behavior, not to reduce repo findings. During that attack pass, same-root config precedence stood out as a likely false-green / false-positive source because the family code and plan disagreed with an older edge-case research note.

Local direct probing with a temporary crate and `cargo clippy` confirmed the upstream behavior: when both files exist in the same directory, Clippy warns that `clippy.toml` will be ignored and uses `.clippy.toml`. That meant the family implementation in `facts.rs` and the `RS-CLIPPY-12` test expectation were both inverted, and the rule plan had drifted with them.

## Decisions Made

### Make the family match real Clippy precedence
- **Chose:** Reverse the same-root precedence so `.clippy.toml` sorts ahead of `clippy.toml`.
- **Why:** This is the actual upstream Clippy resolution order and therefore the only defensible source of truth for the checker.
- **Alternatives considered:**
  - Keep current precedence because the checker only needs a stable internal rule — rejected because it would deliberately diverge from Clippy and misclassify same-root conflicts.
  - Relax `RS-CLIPPY-12` to avoid caring which one wins — rejected because the family explicitly models real policy-root behavior and same-root dual files need deterministic ownership.

### Fix the plan, not just the code
- **Chose:** Update `.plans/todo/checks/rs/clippy.md` alongside the implementation and test.
- **Why:** The drift existed in the plan itself, so leaving it untouched would guarantee the bug comes back later.
- **Alternatives considered:**
  - Fix code/tests only and leave the plan for later cleanup — rejected because the attack pass is specifically about reconciling intended behavior with implementation.

## Architectural Notes
`RS-CLIPPY-12` is a hardening rule that intentionally mirrors upstream config discovery semantics rather than inventing a project-local precedence model. This is a good example of the family boundary: the family may apply local placement policy, but where it claims to emulate Clippy’s own resolution behavior, it has to match upstream exactly.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — current precedence implementation
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/same_root_precedence.rs` — existing expectation
- `.plans/todo/checks/rs/clippy.md` — current family contract
- `.plans/by_file/tools/edge-cases/clippy.md` — earlier edge-case research note showing `.clippy.toml` first
- Local temporary-crate repro using `cargo clippy` — confirmed warning text that `.clippy.toml` is used and `clippy.toml` ignored

## Open Questions / Future Considerations
- Continue the adversarial pass on `RS-CLIPPY-12/13/18/20` for other mismatches between family assumptions and real Clippy behavior.
- Finish harvesting agent findings on plan/doc drift and family-boundary overlap with `cargo` / `code`.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — clippy config collection, precedence, and policy-context resolution
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement.rs` — rule that reports forbidden placement and same-root conflicts
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/same_root_precedence.rs` — direct regression for the precedence behavior
- `.plans/todo/checks/rs/clippy.md` — family contract and frozen policy decisions
- `.plans/by_file/tools/edge-cases/clippy.md` — upstream Clippy config-resolution research
- `.worklogs/2026-03-27-214234-tighten-clippy-policy-context-shape-validation.md` — immediately preceding clippy attack/fix checkpoint

## Next Steps / Continuation Plan
1. Keep attacking `RS-CLIPPY-12/13/18/20` and adjacent config-shape rules for concrete false positives/false negatives, using upstream Clippy behavior where possible instead of repo assumptions.
2. Harvest the active subagent reports on clippy plan/doc drift and clippy/cargo/code ownership overlap, then reconcile any concrete contradictions they find.
3. Once the deny migration stops breaking the outer workspace, rerun top-level `RS-TEST` / `RS-CLIPPY` validation on the clippy family to confirm there is no remaining hidden fallout beyond the nested workspace tests.
