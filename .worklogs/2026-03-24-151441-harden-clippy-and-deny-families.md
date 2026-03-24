# Harden Clippy And Deny Families

**Date:** 2026-03-24 15:14
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/clippy/**`, `apps/guardrail3/crates/app/rs/checks/rs/deny/**`, `apps/guardrail3/crates/domain/modules/clippy/{methods.rs,types.rs}`, `apps/guardrail3/crates/domain/modules/deny.rs`, `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`, `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md`, `.plans/todo/check_review/test_hardening/14-clippy-deny-agent-brief.md`, `.plans/todo/check_review/test_hardening/14-clippy-deny-coverage-matrix.md`

## Summary
Committed the shared clippy/deny hardening packet. The batch expands the canonical ban surfaces, fixes generator/profile drift that affected local-root deny generation, adds broader parity and ownership coverage across both families, and updates the combined hardening docs so they describe the corrected root/profile model.

## Context & Problem
The dirty tree showed one coherent policy packet touching:
- clippy family test coverage and facts/support layers
- deny family test coverage and selected rule behavior
- canonical clippy ban sets
- canonical deny ban generation
- generator behavior for root-local deny profile selection
- the shared hardening docs and coverage matrix for clippy+deny

This is one system, not two isolated families. The checker families validate generated canonical policy, and the generator bug on deny profile resolution would have left the new parity tests describing the wrong product behavior if committed separately.

## Decisions Made

### Commit clippy and deny together
- **Chose:** Keep both families, their canonical domain modules, and the generator profile fix in one commit.
- **Why:** The main value of the batch is checker/generator parity across the shared policy surface.
- **Alternatives considered:**
  - Split clippy and deny into separate commits — rejected because the canonical modules and docs are shared, and the deny generator fix would become orphaned.
  - Commit domain-module changes separately — rejected because the new parity tests are the proof that the expanded canonical surfaces matter.

### Expand canonical garde-related clippy bans rather than patching tests around missing policy
- **Chose:** Add the broader extractor and deserialization surface to the canonical clippy modules.
- **Why:** The hardening pass had already concluded these bans belong to the managed baseline; the code and tests should reflect that instead of treating them as local exceptions.
- **Alternatives considered:**
  - Leave the canonical baseline smaller and weaken tests — rejected because that preserves drift between plan, generator, and checker.

### Make deny generation profile-aware at the real effective root profile
- **Chose:** Switch `build_deny_for_profile` call sites to the effective root profile.
- **Why:** Mixed-root/profile repos can otherwise generate the wrong deny baseline for local roots, which makes family parity tests lie.
- **Alternatives considered:**
  - Keep using the outer/default profile — rejected because the hardening docs and facts layer now explicitly model root-local ownership.

## Architectural Notes
- `apps/guardrail3/crates/domain/modules/clippy/*` and `.../deny.rs` are shared canonical policy sources used by generation and indirectly by checker parity.
- The combined hardening docs now reflect a more precise root/profile model:
  - `packages/shared-types` in the golden scaffold is a workspace member, not a standalone root
  - local allowed configs replace ancestor coverage only for their owned root
- Several deny tests now explicitly model local-root replacement instead of assuming repo-global ownership.

## Information Sources
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/**`
- `apps/guardrail3/crates/app/rs/checks/rs/deny/**`
- `apps/guardrail3/crates/domain/modules/clippy/methods.rs`
- `apps/guardrail3/crates/domain/modules/clippy/types.rs`
- `apps/guardrail3/crates/domain/modules/deny.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`
- `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md`
- `.plans/todo/check_review/test_hardening/14-clippy-deny-agent-brief.md`
- `.plans/todo/check_review/test_hardening/14-clippy-deny-coverage-matrix.md`

## Open Questions / Future Considerations
- The Rust runtime cutover still needs to wire these families into the actual `rs validate` path.
- If future deny policy allows more additive user surfaces, that should be reflected in the generator docs rather than improvised in these families.

## Key Files for Context
- `apps/guardrail3/crates/domain/modules/clippy/methods.rs` — canonical managed deserialization-ban surface
- `apps/guardrail3/crates/domain/modules/clippy/types.rs` — canonical managed extractor/type-ban surface
- `apps/guardrail3/crates/domain/modules/deny.rs` — canonical deny generation baseline
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs` — effective-profile deny generation fix
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/` — clippy family hardening packet
- `apps/guardrail3/crates/app/rs/checks/rs/deny/` — deny family hardening packet
- `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md` — updated lane status

## Next Steps / Continuation Plan
1. Commit the code family packet with its expanded direct/inventory/false-positive coverage and related fixture updates.
2. Commit the garde family packet, including the new rules 11–13 and the remaining brief/plan updates.
3. Finish with deps and hexarch/shared-discovery so the Rust worktree is clean.
