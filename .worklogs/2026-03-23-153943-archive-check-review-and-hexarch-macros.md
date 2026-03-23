# Archive Review Backlog And Allow Optional Hexarch Macros

**Date:** 2026-03-23 15:39
**Scope:** `.plans/todo/check_review/`, `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`, `.plans/todo/legacy/README.md`, `.plans/todo/legacy/NEW_CHECKS.md`, `.plans/todo/legacy/checks/`, `.plans/todo/checks/rs/hexarch.md`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_01_crates_exists.rs`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents.rs`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents_tests.rs`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_12_src_banned.rs`

## Summary
This batch finalized the review-backlog reorganization by moving stale TS/deploy planning into legacy, grouping the active Rust hardening backlog into semantic `check_review` documents, and archiving `NEW_CHECKS.md` after mining its Rust-side candidates. It also updated the hexarch structural rule so `crates/macros/` is allowed as an optional sibling while the four core hex directories remain required.

## Context & Problem
The active todo root had accumulated mixed planning material: current Rust hardening followups, old TS/deploy audit docs, and a broad `NEW_CHECKS.md` idea dump. That made it hard to see what still mattered for Rust. In parallel, the user clarified a structural change for hexarch: `crates/` should still require `adapters`, `app`, `domain`, and `ports`, but `macros/` must be allowed as an optional top-level crate folder. The existing `RS-HEXARCH-02` rule still treated any fifth directory as an error.

## Decisions Made

### Group review backlog by theme instead of keeping one monolith
- **Chose:** Create `.plans/todo/check_review/` with grouped files for hooks/CLI, fail-closed behavior, parity, migration closure, release/policy, new rule candidates, and self-validation.
- **Why:** The single followup file was becoming a dumping ground. Grouped docs make the next implementation passes easier to sequence and audit.
- **Alternatives considered:**
  - Keep everything in `2026-03-23-rust-hardening-followups.md` — rejected because it obscures ownership and turns the file into another legacy blob.
  - Split by family only — rejected because several followups are cross-cutting rather than family-specific.

### Archive stale TS/deploy and idea-dump docs instead of deleting them
- **Chose:** Move TS/deploy planning docs and `NEW_CHECKS.md` into `.plans/todo/legacy/`.
- **Why:** The Rust-only direction is explicit, but the historical docs still have value as background and audit trail.
- **Alternatives considered:**
  - Delete them outright — rejected because it would erase context the user explicitly wanted preserved.
  - Leave them in active todo root — rejected because it keeps dead scope mixed with active Rust work.

### Treat optional `crates/macros/` as a rule-level allowance, not special discovery logic
- **Chose:** Update `RS-HEXARCH-02` to keep the same required four directories and permit `macros` as an allowed optional sibling for every discovered hex root.
- **Why:** Hex-root discovery already happens in the orchestrator. The rule should remain local: check one hex root’s directory set without learning about nested-vs-top-level specifics.
- **Alternatives considered:**
  - Add nested-specific handling to the rule — rejected because it violates the existing architecture split.
  - Add macros handling in the collector only — rejected because the actual policy is about allowed directory names at a single root, which belongs in the rule.

## Architectural Notes
The hexarch change preserves the intended layering:
- `facts.rs` discovers all hex roots, including nested ones
- `HexRootInput` remains the local rule input
- `RS-HEXARCH-02` only validates one root’s top-level directory set

That means optional `macros/` now applies uniformly to top-level and nested hex roots without adding root-type branching to the rule.

The planning reorganization also establishes a new distinction:
- `checks/` contains active architecture/family plans and the grouped review backlog
- `legacy/` contains mined-but-inactive TS/deploy and superseded planning context

## Information Sources
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` — consolidated active gap list extracted from prior adversarial sweeps
- `.plans/todo/NEW_CHECKS.md` — source of additional Rust rule candidates before archiving
- `.plans/todo/checks/rs/hexarch.md` — active rule contract for hexarch
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/facts.rs` — confirmed nested hex-root discovery lives in the orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents.rs` — local rule updated for optional `macros/`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents_tests.rs` — direct rule regression coverage for `crates/macros/`
- `.worklogs/2026-03-23-135318-archive-audit-plans-and-followups.md` — earlier archival/followup extraction pass
- `.worklogs/2026-03-23-135409-finalize-legacy-plan-moves.md` — prior legacy move cleanup

## Open Questions / Future Considerations
- The current hexarch change only covers optional folder allowance. It does not yet add policy for macro crate naming, `proc-macro` crate type, or whether macro crates may live outside parent hex workspaces.
- `check_review/` now reflects migrated backlog items, but it still needs prioritization into an execution order.
- The consolidated followup file still exists alongside `check_review/`; a later pass may want to shrink it into a pointer doc once the grouped files become the primary backlog.

## Key Files for Context
- `.plans/todo/check_review/README.md` — grouped review backlog entry point
- `.plans/todo/check_review/01-hooks-and-cli.md` — hook and CLI migration gaps
- `.plans/todo/check_review/02-fail-closed-and-input-integrity.md` — parser/input failure and scope gaps
- `.plans/todo/check_review/03-generator-checker-parity.md` — generator/checker drift backlog
- `.plans/todo/check_review/04-plan-hygiene-and-migration-closure.md` — plan cleanup and legacy-retirement gaps
- `.plans/todo/check_review/05-release-and-policy-decisions.md` — release and scope decisions still unresolved
- `.plans/todo/check_review/06-new-rust-rule-candidates.md` — new Rust guardrail candidates mined from legacy notes
- `.plans/todo/check_review/07-self-validation-and-test-hardening.md` — self-validation and mutation-hardening backlog
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` — consolidated source backlog from the review sweeps
- `.plans/todo/checks/rs/hexarch.md` — current hexarch rule contract including optional `macros/`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/facts.rs` — hex-root discovery and nested-root recursion
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents.rs` — exact-contents rule with optional `macros/`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents_tests.rs` — regression tests for allowed/forbidden top-level crates dirs

## Next Steps / Continuation Plan
1. Review the grouped `check_review/` files and mark which items are already closed versus still active; the user explicitly wants that next.
2. Use `check_review/` as the execution backlog for the hardening phase, starting with the biggest active gaps: hooks, fail-closed behavior, and generator/checker parity.
3. If macro-crate policy needs to go beyond “optional folder allowed”, extend the hexarch plan and add dedicated rules/tests for naming, workspace membership, and crate type rather than overloading `RS-HEXARCH-02`.
