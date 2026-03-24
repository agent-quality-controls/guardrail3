# Add Rust Validation Cutover Agent Brief

**Date:** 2026-03-24 15:04
**Scope:** `.plans/todo/check_review/test_hardening/28-rust-validation-cutover-agent-brief.md`

## Summary
Added a single droppable handoff file for the Rust validation runtime cutover. The brief packages the already-frozen cutover contract and names the concrete runtime, config, CLI, help, and test surfaces an implementation agent must touch.

## Context & Problem
After freezing the Rust validation cutover spec, the user asked for a single entry file that could be handed to a new agent directly. The cutover spec itself was necessary but not sufficient as a handoff because it did not enumerate the live code entrypoints, legacy runtime surfaces, or the concrete acceptance criteria in the same concise task format used by the other family briefs.

## Decisions Made

### Create one dedicated cutover handoff brief
- **Chose:** Added `28-rust-validation-cutover-agent-brief.md` under the same handoff area as the other Rust execution briefs.
- **Why:** The user wanted a droppable file, not a scattered reading list assembled manually each time.
- **Alternatives considered:**
  - Reuse only the cutover spec file — rejected because it is a contract doc, not a task brief.
  - Fold the cutover into an existing family brief — rejected because this is cross-family runtime work, not a single family hardening lane.

### Make the brief task-oriented, not explanatory
- **Chose:** The brief names the exact files to read, exact runtime surfaces to replace, key semantic traps, and concrete done criteria.
- **Why:** A new implementation agent needs to move quickly from context-building to code changes without rediscovering the same runtime pitfalls.
- **Alternatives considered:**
  - Copy large chunks of the cutover spec verbatim — rejected because it would be longer and lower-signal than a purpose-built brief.

## Architectural Notes
This brief sits on top of the cutover spec rather than replacing it. The intended stack is:
- `AGENTS.md`
- checker architecture
- Rust validation cutover spec
- this brief

That keeps the brief small while still pointing the agent at the authoritative contract.

## Information Sources
- `.plans/todo/checks/2026-03-24-rust-validation-cutover.md` — the actual cutover spec
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — structural checker contract
- `apps/guardrail3/crates/main.rs` — public `rs validate` path
- `apps/guardrail3/crates/adapters/inbound/cli/{cli.rs,validate.rs,help_gen.rs,init.rs}` — CLI/help/init surfaces
- `apps/guardrail3/crates/domain/{config/types.rs,report/mod.rs}`
- `apps/guardrail3/crates/app/rs/checks/**` — new runtime family entrypoints

## Open Questions / Future Considerations
- This brief assumes the cutover spec remains the source of truth. If the spec changes materially, this brief must be kept in sync.
- The brief does not attempt to resolve implementation decomposition across multiple agents; it is a single-agent packet.

## Key Files for Context
- `.plans/todo/check_review/test_hardening/28-rust-validation-cutover-agent-brief.md` — droppable handoff for the cutover work
- `.plans/todo/checks/2026-03-24-rust-validation-cutover.md` — exact runtime/config/report cutover contract
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — checker architecture invariants
- `.worklogs/2026-03-24-145614-freeze-rust-validation-cutover-spec.md` — reasoning behind the cutover contract

## Next Steps / Continuation Plan
1. Hand the new brief to an implementation agent for the Rust validation runtime cutover.
2. Have that agent start with `main.rs`, `cli.rs`, `validate.rs`, `types.rs`, and `report/mod.rs` as the first edit cluster.
3. After the runtime is switched, validate against this repo itself and then update help/init/tests/snapshots in the same branch before deleting legacy Rust validate paths.
