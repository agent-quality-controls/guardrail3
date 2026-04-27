# RS-CARGO Family Slice In New Checks Architecture

**Date:** 2026-03-22 14:19
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/cargo/**`

## Summary
Implemented the first substantial `rs/cargo` family under the new checks architecture. The new family now owns workspace lint completeness and level checks, allow-inventory reporting, per-member lint inheritance, weakened override detection, resolver enforcement, workspace metadata reporting, and member edition drift detection, all driven from family-local facts and typed inputs.

## Context & Problem
After proving the new architecture with `rs/fmt`, the next important family was `rs/cargo` because it is the first one that truly needs parent/child orchestration. The old validator logic in `cargo_lints.rs` and `workspace_metadata.rs` was workspace-centric and flat: it mixed discovery, parsing, iteration, and assertion in a way that would make the new library drift back toward oversized rule inputs. The goal of this slice was to prove that `rs/cargo` can be expressed as:

- one workspace-level input
- one workspace/member pair input
- one membership-set input

with orchestrator-owned fan-out and pure rule files.

## Decisions Made

### Make `rs/cargo` a real family, not a thin wrapper over old code
- **Chose:** Build a new `apps/guardrail3/crates/app/rs/checks/rs/cargo/` family with its own `discover.rs`, `facts.rs`, `inputs.rs`, and rule files.
- **Why:** The point of this migration is architectural separation, not just moving code around. Reusing the old flat validator directly would preserve the same input-shape problems.
- **Alternatives considered:**
  - Call the old `cargo_lints.rs` from the new family module — rejected because it keeps traversal and rule logic coupled.
  - Start with only one or two cargo rules — rejected because `rs/cargo` is the best place to prove both workspace-level and pair-level rules in one family.

### Let the orchestrator own declared-member expansion and pairing
- **Chose:** `discover.rs` resolves workspace member patterns against discovered member `Cargo.toml` locations, then binds one `WorkspaceMemberInput` per declared-and-discovered member.
- **Why:** The rule should judge one relationship at a time. It should not receive “workspace plus all children” and decide what belongs together.
- **Alternatives considered:**
  - Pass the whole discovered member list into every member-oriented rule — rejected because it violates the atomic-input model.
  - Pair every discovered `Cargo.toml`, even if not a declared workspace member — rejected because non-member crates should not be treated as workspace children.

### Keep a membership-set input even before a set-diff rule exists
- **Chose:** Define `WorkspaceMembersSetInput` and bind it in the orchestrator/tests even though no dedicated set-comparison rule is implemented yet.
- **Why:** The user specifically called out that the architecture must support “one pair at a time” instead of bag inputs, and the set-level input is part of that proof. Binding it now validates the family shape before a later set-diff rule is added.
- **Alternatives considered:**
  - Omit the set input until a rule needs it — rejected because it would postpone validating the full orchestrator design.

### Remove unused speculative fields immediately
- **Chose:** Drop an unused `MemberCargoFacts.parse_error` variant when the compiler flagged it, then later reintroduce parse-error handling only after making it real and exercised by rules.
- **Why:** This family is meant to prove minimal inputs. Carrying dead fields forward would defeat the point.
- **Alternatives considered:**
  - Keep the unused field for “future completeness” — rejected because the compiler was correctly signaling input over-design.

### Fix silent parse-error loss before adding more rules
- **Chose:** Refactor the collector so invalid workspace or member `Cargo.toml` files produce facts with parse-error state instead of being dropped by `ok()?`.
- **Why:** The first version accidentally made workspace parse errors impossible to report. That would have baked a silent-skip path into the new family.
- **Alternatives considered:**
  - Leave parse-error handling for a later cleanup — rejected because later rules depend on parsed Cargo facts and need the failure mode to be explicit now.

## Architectural Notes
The `rs/cargo` family now demonstrates the core architecture that the rest of the Rust families should follow:

- `discover.rs` owns tree traversal, workspace-member expansion, and fact construction
- `facts.rs` stores normalized workspace and member state
- `inputs.rs` defines:
  - `WorkspaceCargoInput`
  - `WorkspaceMemberInput`
  - `WorkspaceMembersSetInput`
- each rule file performs one local assertion over one typed input

The first slice covers these rules:
- `g3rs-cargo/workspace-lints` workspace lint completeness
- `g3rs-cargo/lint-levels` workspace lint levels and group priorities
- `RS-CARGO-03` approved allow inventory
- `RS-CARGO-04` member `[lints] workspace = true`
- `g3rs-cargo/priority-order` workspace edition / rust-version metadata
- `RS-CARGO-06` weakened member overrides
- `g3rs-cargo/resolver` specific-lint negative priority
- `g3rs-cargo/disallowed-macros-deny` resolver enforcement
- `RS-CARGO-09` member edition drift

This is enough to prove that `rs/cargo` can host both workspace rules and pair rules without collapsing back into a bag-of-files checker.

## Information Sources
- `.plans/todo/checks/rs/cargo.md` — canonical cargo rule inventory
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — family/facts/inputs/orchestrator pattern
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/*` — first family specimen for module/test shape
- `apps/guardrail3/crates/app/rs/validate/cargo_lints.rs` — old cargo lint semantics and expected lint lists
- `apps/guardrail3/crates/app/rs/validate/workspace_metadata.rs` — old workspace metadata semantics
- `cargo test --lib checks::rs::cargo`
- `cargo test --lib checks::rs::fmt`

## Open Questions / Future Considerations
- `g3rs-cargo/workspace-lints` still does not implement profile-aware extra Rust lints for library profile.
- The family currently resolves `workspace.members` via direct paths and glob patterns, but does not yet model `exclude`.
- There is still no dedicated set-diff rule using `WorkspaceMembersSetInput`; the input is present and tested, but the comparison rule itself remains future work.
- This family is still side-by-side with the old validator and is not wired into the top-level validate pipeline yet.

## Key Files for Context
- `AGENTS.md` — current source of truth for architecture and scope
- `.plans/todo/checks/rs/cargo.md` — cargo rule contract
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — architecture that this family is implementing
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/mod.rs` — family orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs` — discovery and binding logic
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/facts.rs` — normalized workspace/member facts
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/inputs.rs` — atomic cargo inputs
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/lint_support.rs` — shared cargo lint expectations/helpers
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/cargo_tests.rs` — sidecar tests proving fan-out and rule behavior
- `.worklogs/2026-03-22-140612-docs-handoff-rust-architecture.md` — prior checkpoint that established the architecture and `rs/fmt` specimen

## Next Steps / Continuation Plan
1. Decide whether the next migration target should be `rs/toolchain` or `rs/clippy`; both are simpler than `source`/`hexarch` and can reuse the proven family shape.
2. Keep `rs/cargo` side-by-side for now rather than wiring it into the top-level validator until 2-3 families have confirmed the architecture.
3. Add a real set-diff rule for `WorkspaceMembersSetInput` once the desired cargo membership invariant is finalized.
4. When returning to `rs/cargo`, add profile-aware behavior and member exclusion handling instead of broadening rule inputs.
