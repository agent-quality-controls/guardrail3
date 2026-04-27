# Fix Clippy Same-Root Precedence

**Date:** 2026-03-27 21:52
**Scope:** `.plans/todo/checks/rs/clippy.md`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/same_root_precedence.rs`

## Summary
Corrected the clippy-family same-root precedence rule so `.clippy.toml` wins over `clippy.toml`, matching actual `cargo clippy` behavior. Kept the fix narrow to the precedence contract and updated the plan plus the existing rule regression.

## Context & Problem
The ongoing adversarial review of `RS-CLIPPY-12/13/18/20` focused on concrete semantics bugs, especially the suspected same-root precedence bug between `.clippy.toml` and `clippy.toml`. The previous checker and plan both said `clippy.toml` won. That needed validation against real tool behavior before changing the rule because this family is meant to enforce upstream Clippy semantics, not invent its own precedence.

## Decisions Made

### Match real Clippy precedence instead of preserving the old guardrail assumption
- **Chose:** Make `.clippy.toml` the higher-precedence file at the same policy root.
- **Why:** A local `cargo clippy` probe with both files present emitted: `using config file .../.clippy.toml, .../clippy.toml will be ignored`, proving the old checker behavior was reversed.
- **Alternatives considered:**
  - Keep `clippy.toml` as preferred because it was already encoded in tests — rejected because it contradicted real Clippy behavior.
  - Add a guardrail-specific precedence independent of Clippy — rejected because `RS-CLIPPY` should harden actual Clippy policy semantics, not diverge from them.

### Keep this commit scoped to the proven precedence bug
- **Chose:** Commit only the precedence correction and leave the other discovered `RS-CLIPPY-12/13` parse/fail-open issues for a follow-up.
- **Why:** The precedence fix is isolated, already covered by an existing same-root regression, and passes the nested clippy workspace tests. The other bugs affect broader fail-closed behavior and need a slightly larger contract decision.
- **Alternatives considered:**
  - Fold the malformed `Cargo.toml` and dual-failure suppression fixes into the same commit — rejected because they expand the behavioral surface beyond the proven precedence bug.
  - Leave the precedence change unstaged and report it only — rejected because the bug was proven and the fix was already cleanly represented in the working tree.

## Architectural Notes
`RS-CLIPPY-12` is a placement/override rule, but its same-root sibling conflict must mirror real Clippy lookup behavior. The family still owns local policy-root selection inside routed Rust roots; it should not define a precedence model that Clippy itself does not use.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — current same-root precedence implementation.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/same_root_precedence.rs` — existing regression updated to the corrected winner.
- `.plans/todo/checks/rs/clippy.md` — family contract text for placement/override semantics.
- Local tool probe using `cargo clippy` in a temporary crate with both `clippy.toml` and `.clippy.toml` present; Clippy explicitly reported that `.clippy.toml` was used and `clippy.toml` ignored.

## Open Questions / Future Considerations
- `collect_cargo_roots()` in clippy facts still treats malformed routed `Cargo.toml` files as “not a workspace / not a package,” which can misclassify `RS-CLIPPY-12` and suppress `g3rs-clippy/local-policy-root`.
- `g3rs-clippy/local-policy-root` still suppresses a broken local `clippy.toml` when `guardrail3.toml` policy context is also malformed.
- `RS-CLIPPY-18` and `RS-CLIPPY-20` still want extra regression coverage for mixed duplicate entry forms and short-name macro bans.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — collects policy roots and defines same-root precedence.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement.rs` — emits the same-root conflict result.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/same_root_precedence.rs` — direct regression for the precedence contract.
- `.plans/todo/checks/rs/clippy.md` — source-of-truth policy text for the clippy family.
- `.worklogs/2026-03-27-214025-harden-clippy-policy-context.md` — immediately prior clippy attack/fail-closed checkpoint.

## Next Steps / Continuation Plan
1. Revisit `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` and decide how malformed routed `Cargo.toml` files should fail closed for `RS-CLIPPY-12/13`.
2. Add a dual-failure regression for `g3rs-clippy/local-policy-root` where both `guardrail3.toml` policy context and local `clippy.toml` are malformed, then decide whether both errors should surface.
3. Add missing regression coverage for `RS-CLIPPY-18` mixed string/table duplicates and `RS-CLIPPY-20` short-name macro bans.
