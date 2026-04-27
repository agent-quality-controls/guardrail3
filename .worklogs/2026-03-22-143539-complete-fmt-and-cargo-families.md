# Complete FMT And Cargo Families

**Date:** 2026-03-22 14:35
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/fmt/**`, `apps/guardrail3/crates/app/rs/checks/rs/cargo/**`

## Summary
Finished the remaining planned `rs/fmt` rules and closed the last practical `rs/cargo` completion gaps. `fmt` now covers all 8 planned rules, and `cargo` now includes the previously deferred profile-aware expectations and declared-member-missing warning behavior.

## Context & Problem
Earlier in the session, `rs/fmt` and `rs/cargo` were intentionally used as architectural proofs. Once `cargo` validated the parent/child and set-fanout model, the user explicitly redirected the work from “prove architecture” to “finish actual rule groups completely before spreading wider.”

That meant:
- `rs/fmt` still needed its remaining audit-derived rules
- `rs/cargo` still had a few planned-but-deferred completion items despite already covering its main 9-rule set

## Decisions Made

### Finish `rs/fmt` before moving to more families
- **Chose:** Complete `g3rs-fmt/nightly-keys-on-stable`, `06`, and `07` rather than leaving `fmt` as a half-finished specimen.
- **Why:** The architecture question had already been answered by `cargo`, so there was no longer a reason to keep `fmt` intentionally partial.
- **Alternatives considered:**
  - Move directly to `rs/clippy` — rejected because it would leave an easy family incomplete and create unnecessary breadth.

### Treat `ignore` as a dedicated escape hatch, not generic extra inventory
- **Chose:** Add `RS-FMT-07` as the explicit warning for `ignore`, and exclude `ignore` from `g3rs-fmt/extra-settings` generic extra-setting inventory.
- **Why:** The plan explicitly promoted `ignore` from passive inventory to explicit warning. Reporting it twice would be noisy and would blur the purpose of the dedicated escape-hatch rule.
- **Alternatives considered:**
  - Keep both signals — rejected because the duplicate reporting adds noise without improving clarity.

### Use root toolchain facts for nightly-only rustfmt key validation
- **Chose:** Extend `fmt` family facts with the root toolchain channel and use that in `g3rs-fmt/nightly-keys-on-stable`.
- **Why:** The nightly-key rule is inherently cross-file (`rustfmt.toml` + `rust-toolchain.toml`), but the rule itself should still receive pre-bound facts rather than parsing both files.
- **Alternatives considered:**
  - Parse `rust-toolchain.toml` inside the rule — rejected because that breaks the orchestrator/facts split.

### Finish `rs/cargo` with profile-aware behavior inside the existing family
- **Chose:** Add `guardrail3.toml` profile extraction to `cargo` facts and wire the deferred library-profile behavior into `g3rs-cargo/workspace-lints` and `g3rs-cargo/priority-order`.
- **Why:** The plan already said those enhancements belonged to the cargo family. They are small, durable, and do not require widening rule inputs.
- **Alternatives considered:**
  - Leave profile-awareness for later — rejected because it was the main remaining gap between “implemented” and “actually complete” for the family.

### Use the set-level cargo input for missing declared members
- **Chose:** Reuse `WorkspaceMembersSetInput` to warn when a member is declared in `[workspace].members` but no `Cargo.toml` is discovered there.
- **Why:** This closes a known migration bug and also validates that the set-level input is not just a theoretical architecture artifact.
- **Alternatives considered:**
  - Keep silent skip behavior until a dedicated new rule exists — rejected because the plan explicitly called out the silent skip as a migration fix.

## Architectural Notes
With this checkpoint:

`rs/fmt` now covers:
- `RS-FMT-01` existence
- `g3rs-fmt/settings` baseline settings
- `g3rs-fmt/extra-settings` extra-setting inventory
- `g3rs-fmt/nightly-keys-on-stable` nightly-only settings on stable
- `RS-FMT-05` per-crate override warning
- `g3rs-fmt/edition-mismatch` edition mismatch
- `RS-FMT-07` `ignore` escape hatch
- `RS-FMT-08` dual-file conflict

`rs/cargo` now additionally handles:
- library-profile extra rust lint expectations
- library-profile `rust-version` warning when missing
- declared member without `Cargo.toml` warning

This means the two initial anchor families are no longer just architecture proofs. They are now viable reference implementations for future family completion work.

## Information Sources
- `.plans/todo/checks/rs/fmt.md`
- `.plans/todo/checks/rs/cargo.md`
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/*`
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/*`
- `apps/guardrail3/crates/app/core/project_walker.rs`
- `cargo test --lib checks::rs::fmt`
- `cargo test --lib checks::rs::cargo`

## Open Questions / Future Considerations
- `rs/cargo` still does not include the non-runtime canonical drift test that compares lint expectations to generated canonical config.
- The next family decision remains `rs/clippy` vs `rs/deny`, but either can now build on fully completed `fmt` and `cargo` references instead of partial ones.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/mod.rs` — completed fmt orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/facts.rs` — fmt cross-file facts including toolchain channel
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/fmt_tests.rs` — fmt completion regression tests
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/mod.rs` — completed cargo orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs` — cargo workspace/profile/member fact collection
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/cargo_tests.rs` — cargo fan-out and completion regression tests
- `.plans/todo/checks/rs/fmt.md` — fmt rule inventory
- `.plans/todo/checks/rs/cargo.md` — cargo rule inventory
- `.worklogs/2026-03-22-141917-rs-cargo-family-slice.md`
- `.worklogs/2026-03-22-142254-rs-toolchain-family-slice.md`

## Next Steps / Continuation Plan
1. Start `rs/clippy` next as the next config family to finish completely.
2. Follow the same pattern: family-local facts, family-local inputs, one file per rule, sidecar tests.
3. Keep `fmt`, `cargo`, and `toolchain` as the stable reference families while moving into `clippy` and `deny`.
