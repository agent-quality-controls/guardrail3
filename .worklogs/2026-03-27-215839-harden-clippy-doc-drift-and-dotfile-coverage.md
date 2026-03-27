# Harden Clippy Doc Drift And Dotfile Coverage

**Date:** 2026-03-27 21:58
**Scope:** `.plans/todo/checks/rs/clippy.md`, `.plans/by_file/rs/clippy-toml.md`, `.plans/per-app-config-design/01-rust-config-scoping.md`, `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md`, `.plans/todo/check_review/test_hardening/14-clippy-deny-agent-brief.md`, `.plans/todo/check_review/test_hardening/14-clippy-deny-coverage-matrix.md`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_01_coverage_tests/*`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/*`

## Summary
Tightened the clippy family’s current documentation so it points at the live nested workspace instead of stale paths, and marked older planning notes as historical where they now contradict the family contract. Added explicit attack coverage proving `.clippy.toml` is both an allowed policy-root file and a valid coverage root, rather than relying on incidental implementation behavior.

## Context & Problem
The adversarial pass on `rs/clippy` turned up two classes of drift after the same-root precedence fix:

1. The current source-of-truth plan still referenced the old `app/rs/checks/rs/clippy` path, while several older design/hardening docs contained now-wrong statements about Clippy walk-up behavior, test-relaxation ownership, and current rule counts.
2. The live family accepted `.clippy.toml` at allowed roots, but tests only pinned that indirectly through forbidden nested cases and same-root conflict handling. There was no direct negative coverage proving `.clippy.toml` alone is an allowed covering config.

The user explicitly asked for continued attack work, including searching past clippy plans and verifying conflicts against the live implementation, so both the documentation drift and the missing dotfile coverage were in scope.

## Decisions Made

### Fix the authoritative clippy plan instead of preserving old path references
- **Chose:** Update `clippy.md` so its “Current code” and “source of truth” sections point at the nested family workspace, and clarify that allowed coverage/placement applies to either `clippy.toml` or `.clippy.toml`.
- **Why:** This file is the live family contract. Leaving stale implementation paths there would keep reintroducing confusion during future attack or migration passes.
- **Alternatives considered:**
  - Leave the old paths and rely on README/worklogs for correction — rejected because the plan is meant to be the current contract, not historical archaeology.

### Mark older clippy hardening/design docs as historical rather than rewriting them wholesale
- **Chose:** Add explicit “historical / superseded” notes to older clippy design and hardening docs that are still useful as backstory but no longer safe as source of truth.
- **Why:** Several of those files are long, mixed-purpose design artifacts. Marking them clearly prevents misuse without pretending they are still current.
- **Alternatives considered:**
  - Fully rewrite every older doc to current semantics — rejected for now because it is high-effort and low-signal compared to making the authoritative docs unambiguous.
  - Leave them untouched — rejected because the adversarial review already proved they contain live contradictions.

### Add direct `.clippy.toml` attack coverage
- **Chose:** Add focused tests for `.clippy.toml` as:
  - an allowed config file at validation/workspace/standalone package roots (`RS-CLIPPY-12`)
  - a valid validation-root covering config for descendant workspaces without root Cargo (`RS-CLIPPY-01`)
- **Why:** The family accepted this today, but only implicitly. Attack-grade coverage should pin both placement and coverage behavior explicitly.
- **Alternatives considered:**
  - Rely on same-root precedence plus implementation details — rejected because that would not catch future regressions where dotfile support survives in one path but breaks in another.

## Architectural Notes
The clippy family intentionally models real Clippy config semantics where those semantics matter to guardrail policy, but it treats older exploratory docs as migration history, not live contract. This checkpoint makes that boundary clearer:
- `clippy.md` is the authoritative family contract
- older by-file and hardening docs are historical
- dotfile handling is now explicitly tested at the family level, not just implied by config collection internals

One notable open architectural gap remains: the family still does not model or forbid `CLIPPY_CONF_DIR` overrides from `.cargo/config.toml`. No repo-local instance exists today, so this remains an open completeness question rather than a proven active bug.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — live config collection behavior
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_01_coverage_tests/*` — coverage tests
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/*` — placement tests
- `.plans/todo/checks/rs/clippy.md` — current family contract
- `.plans/by_file/tools/edge-cases/clippy.md` — upstream Clippy discovery behavior reference
- `.plans/by_file/rs/clippy-toml.md` — older design note now partially superseded
- `.plans/per-app-config-design/01-rust-config-scoping.md` — older per-app design note with stale clippy lookup assumptions
- `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md`
- `.plans/todo/check_review/test_hardening/14-clippy-deny-agent-brief.md`
- `.plans/todo/check_review/test_hardening/14-clippy-deny-coverage-matrix.md`
- subagent review findings from `019d3142-df94-7383-b550-457918493071`

## Open Questions / Future Considerations
- Decide whether `RS-CLIPPY` should explicitly forbid repo-local `CLIPPY_CONF_DIR` overrides in `.cargo/config.toml`, or model them as part of the coverage/placement contract.
- Continue harvesting the remaining clippy attack agents for rule-boundary findings against `cargo` / `code`.
- Once the outer workspace is healthy again, rerun top-level `RS-TEST` / `RS-CLIPPY` validation on the clippy family instead of relying only on the nested workspace.

## Key Files for Context
- `.plans/todo/checks/rs/clippy.md` — live clippy family contract
- `.plans/by_file/tools/edge-cases/clippy.md` — upstream Clippy config-discovery behavior
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — family-local architecture and ownership boundaries
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — coverage, placement, precedence, and policy-context resolution
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_01_coverage.rs` — coverage rule orchestration boundary
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement.rs` — placement/shadowing rule
- `.worklogs/2026-03-27-215144-fix-clippy-same-root-precedence.md` — preceding clippy semantics fix
- `.worklogs/2026-03-27-214234-tighten-clippy-policy-context-shape-validation.md` — prior clippy fail-closed hardening

## Next Steps / Continuation Plan
1. Keep the adversarial clippy pass focused on concrete semantics and ownership boundaries, especially any repo-local config surfaces that can bypass family assumptions (`CLIPPY_CONF_DIR`, `.cargo/config.toml`, cargo/clippy overlap).
2. Harvest remaining subagent findings and fold any real code-path bugs into the family before shifting effort to the next family.
3. Once the deny migration stops breaking the outer workspace, rerun top-level family validation to confirm the nested clippy green state matches full-pipeline behavior.
