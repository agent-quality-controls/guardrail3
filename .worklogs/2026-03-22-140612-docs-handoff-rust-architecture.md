# Rust-Only Handoff And First Checks Family

**Date:** 2026-03-22 14:06
**Scope:** `AGENTS.md`, `CLAUDE.md`, `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`, `.plans/todo/checks/hooks/shared.md`, `.plans/todo/checks/hooks/rs.md`, `apps/guardrail3/crates/app/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/**`

## Summary
Replaced the stale `CLAUDE.md` architecture with a current `AGENTS.md` handoff focused on Rust guardrails and the new checker pipeline. Also added the first real family in the new code layout, `rs/fmt`, to prove the orchestrator-plus-typed-input architecture and the correct collocated sidecar test pattern.

## Context & Problem
The repo documentation had drifted away from the actual codebase and planning direction. `CLAUDE.md` still described a much older, broader system that included TypeScript as active scope and an outdated `src/...` layout, while current planning had already moved toward Rust-only work, `ProjectTree`, family orchestrators, and minimal rule inputs. At the same time, the new checker architecture needed one real family in code to prove the intended layout before migrating harder families like `rs/cargo`.

## Decisions Made

### Make `AGENTS.md` the canonical handoff document
- **Chose:** Rewrite `AGENTS.md` as the primary project handoff and reduce `CLAUDE.md` to a historical pointer.
- **Why:** The old `CLAUDE.md` was too stale to serve as the main instruction file. The repo needed one compact source of truth that reflected current scope, actual file layout, and the new architecture.
- **Alternatives considered:**
  - Keep `CLAUDE.md` as the primary document and patch it in place — rejected because it mixed too much obsolete material with current direction.
  - Keep both documents equally authoritative — rejected because it guarantees future drift and ambiguity.

### Record Rust-only scope explicitly
- **Chose:** State that Rust guardrails are the active direction and TypeScript/deploy work is legacy background unless explicitly requested.
- **Why:** Recent planning and user direction made clear that Rust is the only active implementation scope. The docs needed to stop implying TS expansion is still in flight.
- **Alternatives considered:**
  - Leave TS in the active roadmap with a “lower priority” note — rejected because it still invites unwanted work.
  - Delete all TS references entirely — rejected because existing code and docs still exist and remain useful as historical context.

### Freeze the new checker architecture in docs before broad migration
- **Chose:** Document the pipeline as `ProjectTree -> family orchestrator -> typed input -> pure rule`.
- **Why:** The missing middle layer had been the main architecture gap. Without writing this down clearly, families would keep drifting toward oversized inputs and tree-crawling rules.
- **Alternatives considered:**
  - Keep the architecture implicit in plan notes — rejected because it is too easy to misapply during implementation.
  - Let each family invent its own orchestration pattern — rejected because it would fragment the library immediately.

### Use `rs/fmt` as the first architecture specimen
- **Chose:** Add a new `apps/guardrail3/crates/app/rs/checks` tree and implement `rs/fmt` with family-local `facts.rs`, `inputs.rs`, `mod.rs`, one-file-per-rule, and a sidecar test file.
- **Why:** `fmt` is simple enough to prove the shape without the complexity of workspace/member pairing. It validates the module layout, fan-out model, and test pattern before tackling `rs/cargo`.
- **Alternatives considered:**
  - Start with `rs/cargo` — rejected because it is the more important but more failure-prone family for first implementation.
  - Fully migrate `fmt` into the old validate pipeline immediately — rejected because the goal here was architectural proof, not full replacement yet.

### Use sidecar collocated test files, not inline or external integration tests
- **Chose:** Keep tests beside the family module via `#[cfg(test)] #[path = "fmt_tests.rs"] mod tests;`.
- **Why:** This matches the repo’s established pattern and avoids widening visibility just so integration-style tests can reach internal helpers.
- **Alternatives considered:**
  - Inline `mod tests { ... }` inside production files — rejected because the repo does not want inline test bodies as the default.
  - Put tests under `tests/unit.rs` — rejected because that pushed visibility in the wrong direction and did not match the preferred local sidecar pattern.

## Architectural Notes
The new checks layout is intentionally family-local:
- `mod.rs` owns orchestration
- `facts.rs` owns normalized family data
- `inputs.rs` owns atomic rule inputs
- each rule file owns one pure assertion

The crucial constraint is that rules do not receive `ProjectTree` and do not perform discovery. The orchestrator is responsible for traversal, parsing, normalization, and fan-out. This is especially important for future families like `rs/cargo`, where rules should receive either one workspace-level input, one workspace/member pair, or one membership-set comparison, never “workspace plus all children”.

Hook planning was also updated earlier in this workstream to reflect durable Rust/shared hook invariants, but hooks are not the first implementation target. The next code family should be `rs/cargo`, because it validates the harder parent/child and set-fanout side of the architecture.

## Information Sources
- `CLAUDE.md` — the previous broad architectural statement that had drifted from reality
- `AGENTS.md` — rewritten as the new source of truth
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — active checker architecture plan
- `.plans/todo/checks/rs/*` — Rust family rule inventory
- `.plans/todo/checks/hooks/shared.md` — shared hook rule inventory
- `.plans/todo/checks/hooks/rs.md` — Rust hook rule inventory
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks_tests.rs`
- `apps/guardrail3/crates/app/rs/validate/test_checks.rs`
- `apps/guardrail3/crates/domain/project_tree.rs`
- `apps/guardrail3/crates/app/core/project_walker.rs`
- `cargo test --lib checks::rs::fmt`

## Open Questions / Future Considerations
- `rs/fmt` is an architecture specimen, not a full migration yet. It still needs eventual production wiring and completion of the remaining planned `RS-FMT-*` rules.
- The next meaningful proof point is `rs/cargo`, because that family will confirm whether the orchestrator truly handles parent/child and set-based fan-out cleanly.
- The GitNexus instructions were preserved in `AGENTS.md`, but the current environment for this work did not expose GitNexus tooling directly.

## Key Files for Context
- `AGENTS.md` — current source of truth for scope, architecture, and handoff
- `CLAUDE.md` — historical stub pointing back to `AGENTS.md`
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — canonical checker pipeline and family implementation order
- `.plans/todo/checks/hooks/shared.md` — durable shared hook rule inventory
- `.plans/todo/checks/hooks/rs.md` — durable Rust hook rule inventory
- `apps/guardrail3/crates/domain/project_tree.rs` — shared repository snapshot abstraction
- `apps/guardrail3/crates/app/core/project_walker.rs` — tree-building entry point
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/mod.rs` — first real family orchestrator specimen
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/facts.rs` — example of family-local normalized facts
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/inputs.rs` — example of minimal typed inputs
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/fmt_tests.rs` — preferred sidecar family test pattern
- `.worklogs/2026-03-21-214628-project-tree-walker.md` — project tree implementation background
- `.worklogs/2026-03-21-215438-dump-tree-cli.md` — project tree debug/inspection background

## Next Steps / Continuation Plan
1. Read `AGENTS.md`, the checker architecture plan, and the `rs/cargo` rule inventory before making the next family changes.
2. Implement `apps/guardrail3/crates/app/rs/checks/rs/cargo/` with the same pattern as `rs/fmt`: `mod.rs`, `facts.rs`, `inputs.rs`, one rule file per rule, and a collocated sidecar test file.
3. Start `rs/cargo` with only three input classes: one workspace-level input, one workspace/member pair input, and one membership-set comparison input.
4. Add a small first slice of `rs/cargo` rules that proves all three fan-out modes before migrating the rest of the family.
5. Only after `rs/cargo` feels clean, decide whether `rs/fmt` needs any architectural adjustments before broader family migration.
