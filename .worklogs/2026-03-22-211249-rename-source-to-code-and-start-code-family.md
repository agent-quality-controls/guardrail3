# Rename Source To Code And Start Code Family

**Date:** 2026-03-22 21:12
**Scope:** `.plans/todo/checks/rs/code.md`, `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`, `.plans/2026-03-21-153251-checkers-rs.md`, `.plans/todo/checks/rs/hexarch.md`, `.plans/todo/checks/rs/test.md`, `AGENTS.md`, `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/code/*`

## Summary
Renamed the Rust `source` family to `code` in the active plans and agent handoff docs, then replaced the dead `rs/code` scaffold with a real first implementation slice. The new family now orchestrates streamed Rust file parsing from `ProjectTree` and implements the first batch of real rules with one rule file and one test file per rule.

## Context & Problem
The next active Rust family after the config families was the old `rs/source` area, but the intended naming had shifted to `code`. A placeholder `rs/code` module had been mounted into the crate tree without real orchestration or rule usage, which immediately broke `cargo test --lib` under `-D dead-code`. The family needed to be renamed consistently in plans and then turned into a real checker family rather than a paper scaffold.

## Decisions Made

### Rename the family from `source` to `code`
- **Chose:** Rename the active plan and cross-references from `rs/source` / `RS-SOURCE-*` to `rs/code` / `RS-CODE-*`.
- **Why:** The user explicitly wanted the family named `code`, and the newer naming better separates general Rust code-shape checks from architecture-specific families like `hexarch`.
- **Alternatives considered:**
  - Keep `source` for compatibility — rejected because it would preserve stale terminology the user had already corrected.
  - Add `code` as an alias while keeping `source` — rejected because duplicated family names would create drift in plans and code layout.

### Start with a real first slice instead of suppressing dead code
- **Chose:** Implement the first coherent batch of rules (`RS-CODE-09..16` and `RS-CODE-19`, plus `RS-CODE-12`) and wire real orchestration.
- **Why:** The placeholder family was failing compilation. Silencing dead code would repeat the “fake scaffold” problem instead of proving the architecture with working code.
- **Alternatives considered:**
  - Add `#[allow(dead_code)]` to the scaffold — rejected because it hides the structural problem instead of fixing it.
  - Implement only discovery/facts without rules — rejected because the module would still be architecturally incomplete and low-value.

### Keep the first implementation slice focused on per-file code rules
- **Chose:** Implement discovery, parse helpers, per-file inputs, and the easiest non-profile, non-attribute-heavy rules first.
- **Why:** These rules already had legacy behavior and tests to mine, they fit the streamed parse-once model cleanly, and they made the family compile immediately.
- **Alternatives considered:**
  - Start with the attribute/reason-heavy suppression rules (`RS-CODE-01..08`) — rejected because they require more nuanced comment/attribute semantics and slower migration.
  - Start with library-profile rules (`RS-CODE-25..29`) — rejected because they first need reliable profile/root resolution.

## Architectural Notes
`rs/code` is the first deliberately streamed family in the new checker architecture. It discovers `*.rs` files from `ProjectTree`, excludes fixture paths, classifies test files, streams source content on demand through `crate::fs`, parses each file once with `syn`, and then fans a single `RustCodeFileInput` into multiple pure rule functions.

The family also establishes a second input class for workspace lint facts via `UnsafeCodeLintInput`, which is used for `RS-CODE-12`. This keeps the family aligned with the planned “two input classes” design instead of overloading file-local rules with Cargo parsing.

The first implemented rules are:
- `RS-CODE-09` file length
- `RS-CODE-10` use count error
- `RS-CODE-11` use count warning
- `RS-CODE-12` unsafe_code lint level
- `RS-CODE-13` todo/unimplemented/unreachable
- `RS-CODE-14` unwrap/expect
- `RS-CODE-15` direct std::fs usage
- `RS-CODE-16` panic! in non-test code
- `RS-CODE-19` large struct/enum inventory

## Information Sources
- `.plans/todo/checks/rs/source.md` — the prior family plan before rename
- `.plans/todo/checks/rs/code.md` — the new active plan after rename
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — target family structure and implementation order
- `apps/guardrail3/crates/app/rs/validate/source_scan.rs` — old orchestrator behavior
- `apps/guardrail3/crates/app/rs/validate/structure_checks.rs` — legacy `R38`, `R40`, `R41`, `R53`
- `apps/guardrail3/crates/app/rs/validate/code_quality_checks.rs` — legacy `R43`, `R44`, `R58`
- `apps/guardrail3/crates/app/rs/validate/ast_helpers.rs`, `ast_visitors.rs`, `extra_visitors.rs` — legacy AST logic to mine and simplify
- `apps/guardrail3/tests/unit/rs_structure_checks_test.rs`
- `apps/guardrail3/tests/unit/code_quality_checks_test.rs`
- `.worklogs/2026-03-22-203352-finish-rust-check-test-hardening.md`

## Open Questions / Future Considerations
- The rest of the `RS-CODE` family is still unimplemented, especially the suppression rules (`01..08`) and the profile-gated library rules (`25..29`).
- `RS-CODE-21` will likely require tightening `RS-CODE-15` so direct `std::fs` imports and `std::fs::*` glob bypasses are distinct checks rather than overlapping behavior.
- Profile/root resolution is still minimal in this first slice; later library-profile rules will need the same kind of explicit root-policy handling already built for `clippy` and `deny`.
- The repo still has unrelated dirty files outside this scoped commit, especially `CLAUDE.md` and prior test-file edits. They were intentionally left out.

## Key Files for Context
- `AGENTS.md` — current project handoff and family ordering, now updated to `code`
- `.plans/todo/checks/rs/code.md` — active `RS-CODE` rule inventory and target family shape
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — checker architecture contract and family layout
- `apps/guardrail3/crates/app/rs/checks/rs/mod.rs` — family registration point
- `apps/guardrail3/crates/app/rs/checks/rs/code/mod.rs` — `rs/code` orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/code/discover.rs` — Rust-file discovery and test classification
- `apps/guardrail3/crates/app/rs/checks/rs/code/facts.rs` — family facts, including workspace unsafe_code lint facts
- `apps/guardrail3/crates/app/rs/checks/rs/code/parse.rs` — family-local parse and AST helper layer
- `.worklogs/2026-03-22-203352-finish-rust-check-test-hardening.md` — prior checkpoint before starting `rs/code`

## Next Steps / Continuation Plan
1. Finish the remaining non-profile `RS-CODE` rules next: `RS-CODE-17`, `18`, `20`, `21`, `22`, `23`, `24`, using the same one-rule/one-test pattern under `apps/guardrail3/crates/app/rs/checks/rs/code/`.
2. Port the suppression/reason checks (`RS-CODE-01..08`) from the old `allow_checks.rs` logic into family-local parse helpers and rule files, keeping comment/attribute semantics explicit and tested.
3. Add profile/root resolution needed for library-only rules and then implement `RS-CODE-25..29`.
4. After the family is substantially complete, run an adversarial audit against `.plans/todo/checks/rs/code.md` and mine the old source-scan/adversarial fixtures rule by rule.
