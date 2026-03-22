# Cargo/Fmt Audit Hardening

**Date:** 2026-03-22 14:53
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/cargo/*`, `apps/guardrail3/crates/domain/modules/canonical.rs`

## Summary
Ran an adversarial completeness audit against the new `RS-CARGO` and `RS-FMT` family implementations using the current plan docs as the contract. `RS-FMT` matched the plan; `RS-CARGO` had three real mismatches, which were fixed along with stronger sidecar tests designed to break the family on resolver semantics, globbed member discovery, and duplicate/noisy weakened-override behavior.

## Context & Problem
The new family architecture had already been proven on `fmt`, `cargo`, and `toolchain`, but the user explicitly wanted these families finished completely rather than treated as partial specimens. The next risk was not architecture shape, but contract drift: the implementation might look complete while silently missing behavior the plan still requires.

The audit target files were:
- `.plans/todo/checks/rs/cargo.md`
- `.plans/todo/checks/rs/fmt.md`
- the implemented family code under `apps/guardrail3/crates/app/rs/checks/rs/cargo/` and `.../fmt/`

The user also clarified that test migration should happen when the new checks exist, and that the goal of tests is adversarial: break the code, not merely confirm the happy path.

## Decisions Made

### Fix cargo at the facts/extractor layer, not only in rule files
- **Chose:** Update `discover.rs` so workspace-member glob expansion matches all discovered directories, not only directories already containing `Cargo.toml`.
- **Why:** The missing `Cargo.toml` case for globbed members was invisible to the checker because the extractor filtered too early. A rule-level fix would not help if the broken member never entered the fact graph.
- **Alternatives considered:**
  - Patch only `RS-CARGO-04` — rejected because the broken case never reached the rule.
  - Add a separate filesystem crawl in the rule — rejected because it would violate the orchestrator/facts architecture.

### Gate weakened-override checks on actual lint inheritance
- **Chose:** Make `RS-CARGO-06` return early unless the member already has `[lints] workspace = true`.
- **Why:** The plan is explicit that override weakening is only meaningful after inheritance is confirmed. Running both rules on a non-inheriting member creates duplicate or misframed failures.
- **Alternatives considered:**
  - Keep the current behavior and accept overlap — rejected because it diverges from the written contract.
  - Move the gate into the orchestrator only — rejected because the rule should also protect itself against misuse.

### Tighten missing-resolver semantics to match the plan
- **Chose:** Make missing `resolver` only inventory-level `Info` for non-virtual workspaces with package edition `2021+`; emit `Error` otherwise.
- **Why:** The prior implementation treated any non-virtual workspace as safe, which was looser than the plan and overstated Cargo’s implicit modern-resolver behavior.
- **Alternatives considered:**
  - Leave all non-virtual workspaces as `Info` — rejected because it contradicts the plan.
  - Make all missing resolvers `Error` — rejected because the plan intentionally allows the modern non-virtual case as informational only.

### Add drift tests against the canonical generated cargo-lints module
- **Chose:** Add a sidecar test that parses `canonical::CARGO_LINTS` and asserts the expected lint sets are all present.
- **Why:** The plan explicitly called out canonical drift as a migration risk. The new test immediately exposed a real mismatch: `missing_docs_in_private_items = "allow"` existed in the checker expectations but not in the canonical module.
- **Alternatives considered:**
  - Trust comments/counts in the plan only — rejected because the point is to catch code/doc drift automatically.
  - Compare against hard-coded counts only — rejected because presence/absence is more useful than a bare number.

### Fix canonical cargo-lints to match the validator contract
- **Chose:** Add `missing_docs_in_private_items = "allow"` to `canonical::CARGO_LINTS`.
- **Why:** The validator and generator must agree on the allowed baseline. The new drift test showed they did not.
- **Alternatives considered:**
  - Remove the lint from checker expectations — rejected because both old validator logic and the new cargo family already treat it as part of the approved allow inventory.

## Architectural Notes
This work reinforced the intended architecture:
- `discover.rs` owns workspace-member expansion and pairing.
- rules stay local and pure over typed inputs.
- adversarial tests should target extractor blind spots and semantic boundary conditions, not only “does this rule ever fire”.

The changes also validated the sidecar test pattern for family-local testing. The new cargo tests are still collocated with the family, but they now assert more failure-oriented semantics derived from the older adversarial config coverage.

## Information Sources
- `.plans/todo/checks/rs/cargo.md` — current `RS-CARGO` contract
- `.plans/todo/checks/rs/fmt.md` — current `RS-FMT` contract
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/*` — new cargo family implementation
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/*` — new fmt family implementation
- `apps/guardrail3/tests/adversarial_config_tests.rs` — old adversarial config coverage for `R21`, `R26`, `R27`, `R29`
- `apps/guardrail3/crates/domain/modules/canonical.rs` — generated baseline modules, used for drift checking

## Open Questions / Future Considerations
- The `RS-CARGO` plan still states “30 clippy deny lints”, while the canonical and checker expectations include 31 (`verbose_file_reads`). This is now safe in code because canonical and validator agree, but the plan doc should be updated later to avoid a stale count.
- The new sidecar cargo tests still cover only a small subset of the old adversarial-config fixture surface. More of the old fixture cases should be harvested as each family is finalized.
- `RS-FMT` matched the plan in this audit, but it still only has a few sidecar tests. The old coverage for fmt was thin, so future additions will likely need fresh adversarial cases rather than direct migration.

## Key Files for Context
- `AGENTS.md` — current project scope and architecture reference; Rust-only direction and family-local test pattern
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — checker architecture with orchestrator/facts/inputs split
- `.plans/todo/checks/rs/cargo.md` — `RS-CARGO` rule contract and migration notes
- `.plans/todo/checks/rs/fmt.md` — `RS-FMT` rule contract
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs` — cargo extractor; important because glob expansion bug was fixed here
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/cargo_tests.rs` — adversarial sidecar tests for the new cargo family
- `apps/guardrail3/crates/domain/modules/canonical.rs` — generated canonical baselines; now includes the missing cargo allow lint
- `.worklogs/2026-03-22-143539-complete-fmt-and-cargo-families.md` — prior checkpoint where the family implementations were completed before this adversarial hardening pass

## Next Steps / Continuation Plan
1. Update `.plans/todo/checks/rs/cargo.md` to remove the stale “30 clippy deny lints” count and reflect the now-implemented migration note about canonical drift tests.
2. Harvest a few more old adversarial-config cases into `cargo_tests.rs`, especially ones that stress completeness vs wrong-level semantics and inventory behavior.
3. Move to the next Rust family (`rs/clippy` or `rs/deny`) using the same audit-first approach: implement the family, then attack it against the plan with sidecar tests designed to break extractor and rule semantics.
