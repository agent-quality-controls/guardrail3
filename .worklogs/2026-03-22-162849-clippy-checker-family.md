# Complete Clippy Checker Family

**Date:** 2026-03-22 16:28
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/clippy/**`

## Summary
Finished the new `rs/clippy` checker family so it matches the frozen clippy contract instead of the earlier root-only prototype. The family now discovers allowed policy roots, enforces coverage and forbidden shadow configs, validates every allowed config against the hardened managed baseline, and includes adversarial sidecar tests for the main failure modes.

## Context & Problem
The initial `rs/clippy` slice compiled and had tests, but it was still modeling the old world:
- one root `clippy.toml`
- “extras” as per-member replacement files
- workspace-member inheritance warnings

That directly conflicted with the decisions made during the policy review:
- allowed locations are validation root / workspace roots / standalone package roots
- uncovered Rust units are errors
- nested shadow configs are errors
- allowed local policy roots must be self-contained
- macro bans are required, not advisory

The family needed one final pass to align its facts model and rule semantics before it could be committed as the real clippy family.

## Decisions Made

### Replace root-plus-extras facts with policy-root coverage facts
- **Chose:** Rebuild `facts.rs` around:
  - allowed configs
  - forbidden configs
  - covered Rust units
  - uncovered Rust units
- **Why:** The old extractor shape could not express the actual placement and coverage contract cleanly. Coverage is the core rule, not root existence.
- **Alternatives considered:**
  - Keep root/extras and patch rule logic around it — rejected because the underlying facts were wrong for the policy.

### Run config rules on every allowed policy root
- **Chose:** Introduce one `ConfigClippyInput` and run threshold, ban, reason, macro, and hygiene rules against every allowed config, not only the validation root.
- **Why:** Any allowed policy root replaces inherited clippy config for its subtree, so every allowed config must be treated as authoritative.
- **Alternatives considered:**
  - Keep detailed rules root-only and use one summary rule for local roots — rejected because it would hide the concrete missing pieces inside local policy roots.

### Keep g3rs-clippy/local-policy-root as the explicit self-contained local-root guard
- **Chose:** Use `g3rs-clippy/local-policy-root` as a focused summary check for non-root local policy roots while still running the detailed rule set on all allowed configs.
- **Why:** The summary rule captures the architectural invariant (“local policy roots replace inherited policy”) while the detailed rules explain exactly what is missing or wrong.
- **Alternatives considered:**
  - Drop `g3rs-clippy/local-policy-root` as redundant — rejected because it encodes a real higher-level guarantee from the plan.

### Promote macro-baseline failures to errors
- **Chose:** `RS-CLIPPY-20` now errors when a required macro ban is missing.
- **Why:** The macro baseline is part of the hardening contract, not an informational add-on.
- **Alternatives considered:**
  - Leave missing macro bans as warnings — rejected because it conflicts with the agreed managed baseline model.

### Make tests adversarial around placement and shadowing
- **Chose:** Replace the earlier prototype tests with failure-oriented coverage for:
  - uncovered standalone packages
  - forbidden nested member `clippy.toml`
  - incomplete local workspace policy roots
  - macro/hygiene failures on allowed configs
- **Why:** The user explicitly wanted tests that try to break the code rather than merely confirm the happy path.
- **Alternatives considered:**
  - Keep the earlier root-only/inheritance tests — rejected because they were exercising superseded semantics.

## Architectural Notes
This family is now the first config-family implementation that fully reflects the updated middle-layer architecture:
- `ProjectTree` provides generic file/directory queries
- `facts.rs` builds family-specific placement and coverage facts
- `inputs.rs` defines atomic rule inputs
- `mod.rs` orchestrates fan-out
- one rule file handles one rule
- sidecar tests stay collocated with the family

It also demonstrates the intended relationship to the canonical generator module:
- generator exports the hardened baseline slices
- checker imports and enforces them

That is the pattern the remaining families should converge on.

## Information Sources
- `.plans/todo/checks/rs/clippy.md` — frozen clippy family contract
- `.plans/by_file/rs/clippy-toml.md` — empirical clippy resolution behavior
- `apps/guardrail3/crates/domain/modules/clippy/**` — canonical generated clippy baseline after the refactor
- `apps/guardrail3/crates/domain/project_tree.rs` — generic tree helpers used by the extractor
- `.worklogs/2026-03-22-162712-clippy-policy-freeze.md`
- `.worklogs/2026-03-22-162754-clippy-generator-refactor.md`

## Open Questions / Future Considerations
- The checker still mostly uses the global profile name from `guardrail3.toml`. Richer per-root profile resolution may still be needed when the surrounding Rust family migration goes deeper.
- `.clippy.toml` is still outside the current implementation scope; this family enforces `clippy.toml` placement only.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/mod.rs` — clippy family orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs` — policy-root and coverage extraction
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/inputs.rs` — atomic inputs for coverage and config rules
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/clippy_support.rs` — checker expectations imported from the canonical generator module
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/clippy_tests.rs` — adversarial sidecar tests for placement, coverage, and hygiene
- `apps/guardrail3/crates/domain/modules/clippy/mod.rs` — canonical generator entrypoint used by checker expectations
- `.worklogs/2026-03-22-162754-clippy-generator-refactor.md` — generator-side refactor backstory

## Next Steps / Continuation Plan
1. Commit this checker family independently from the planning and generator commits.
2. Leave unrelated dirty files alone; they are outside the clippy line of work.
3. Continue the Rust family migration after this with the same flow:
   - finish the next family
   - run an adversarial plan-vs-code audit
   - then split generator/checker cleanup into coherent commits again.
