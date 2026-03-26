# RS-TEST Discovery Orchestrator Refactor

**Date:** 2026-03-26 11:57
**Scope:** `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`

## Summary
Moved root discovery, component discovery, manifest/config parsing, and file classification out of `facts.rs` into `discover.rs` for the `rs/test` family. Left behavior unchanged so the family still passes its own test suite and self-validation, but the ownership bug around nested hex components now sits in the correct orchestration layer.

## Context & Problem
The active checker architecture requires family orchestrators to own discovery from `ProjectTree`, while `facts.rs` should carry normalized shared family facts. `rs/test` had drifted away from that contract: `facts.rs` contained the real orchestrator logic, including cargo-root collection, workspace membership expansion, root selection, component detection, and file classification.

That made the code harder to reason about and directly contributed to the current confusion around what counts as an `RS-TEST` root versus what counts as a component root. Before changing nested component ownership semantics, the family needed to be put back into the intended structural shape.

## Decisions Made

### Move discovery responsibilities into `discover.rs`
- **Chose:** Put the family-wide collection entrypoint and discovery helpers in `discover.rs`.
- **Why:** This matches the architecture contract: the family layer performs discovery and normalization before rules run.
- **Alternatives considered:**
  - Keep the existing `facts::collect(...)` shape and only rename comments — rejected because it preserves the architectural confusion.
  - Move everything directly into `lib.rs` — rejected because it would turn the orchestrator into a monolith and make the discovery code harder to test and maintain.

### Reduce `facts.rs` to data structures only
- **Chose:** Leave `facts.rs` with the shared structs used by the family and the rules.
- **Why:** This makes the boundary explicit: `discover.rs` builds facts, rules consume facts, and `facts.rs` stops being an implicit second orchestrator.
- **Alternatives considered:**
  - Leave helper parsing/reading functions in `facts.rs` — rejected because they are part of the discovery pipeline, not normalized facts.

### Preserve current behavior before changing ownership semantics
- **Chose:** Refactor without changing how roots/components are discovered yet.
- **Why:** The next step is a semantic change for nested hex components. Keeping this commit behavior-preserving isolates the architectural cleanup and makes the next diff easier to review.
- **Alternatives considered:**
  - Combine the refactor with the nested discovery fix — rejected because it would blur structural cleanup with behavior change and make regressions harder to localize.

## Architectural Notes
`rs/test` now follows the intended family layering more closely:

- `discover.rs` performs family-specific discovery from `ProjectTree`
- `facts.rs` defines normalized family data
- `lib.rs` orchestrates rule execution over those facts

The current nested-hex false-positive bug is still present, but it now lives where it belongs: in discovery/orchestration rather than in a facts module pretending not to do discovery.

## Information Sources
- `AGENTS.md` — worklog and session rules
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — architecture contract for orchestrators vs facts vs rules
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — current family orchestrator
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs` — previous discovery-heavy implementation
- `.worklogs/2026-03-26-112409-rs-test-direct-component-shape-only.md` — prior direct-shape-only discovery decision

## Open Questions / Future Considerations
- Nested hex/component ownership is still wrong. The family currently treats `component-root/crates/runtime` as valid only at one direct location and still fails on nested component roots inside larger owned roots.
- The current `RS-TEST` root model is still inconsistent with the user’s requirement that every `Cargo.toml` root should participate in discovery.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — now contains root discovery, component discovery, and file classification
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs` — normalized shared structs for the family
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — family orchestrator entrypoint and per-root analysis
- `apps/guardrail3/crates/app/rs/families/test/README.md` — current contract, still needing alignment with the next discovery change
- `.worklogs/2026-03-26-112409-rs-test-direct-component-shape-only.md` — previous decision that narrowed discovery too far

## Next Steps / Continuation Plan
1. Rework `rs/test` root discovery so every relevant `Cargo.toml` participates, while component roots are inferred structurally as `<component-root>/crates/{runtime,assertions}` instead of only at one direct location.
2. Update file classification so files bind to the deepest matching discovered component root rather than falling back to non-component harness errors in nested hex layouts.
3. Add targeted regression tests for nested component roots, including a path like `crates/adapters/inbound/mcp/crates/domain/...`, and verify the family still self-hosts cleanly after the semantic change.
