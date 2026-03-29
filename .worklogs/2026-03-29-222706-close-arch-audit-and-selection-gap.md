# Close Arch Audit And Selection Gap

**Date:** 2026-03-29 22:27
**Scope:** `.plans/todo/checks/rs/arch.md`, `apps/guardrail3/crates/app/rs/families/arch/README.md`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs`, `apps/guardrail3/crates/app/rs/placement/src/roots.rs`, `apps/guardrail3/crates/app/rs/family_selection/src/lib.rs`, `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs`, `apps/guardrail3/crates/app/rs/family_selection/src/selection_tests.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`

## Summary
Closed the remaining `RS-ARCH` audit gaps that were still live after the earlier overlap and fail-closed work. The substantive fix is in shared family selection: explicit `--family arch` requests now survive config disablement and surface `RS-ARCH-02` inactivity instead of disappearing. I also tightened the last `RS-ARCH-07` metadata coverage and cleaned the live `arch` docs so they stop pointing at deleted paths and stale rule counts.

## Context & Problem
The earlier adversarial review of `RS-ARCH` had already driven the big rule fixes:
- `RS-ARCH-04` became a real layout-level overlap rule
- `RS-ARCH-07` started failing closed on malformed governed manifests
- `RS-ARCH-06` got app-scoped override coverage
- governed-root `arch_role` misuse became a fail-closed input error
- inactive `RS-ARCH-02` reporting became explicit inventory instead of silent suppression

But there was still one real product-surface gap: explicit `--family arch` requests were still being filtered out by `family_selection` when `[rust.checks] arch = false`. That contradicted the stronger runtime expectation: explicit user requests should run the family and report that misplaced-root enforcement is inactive, not silently omit the family.

The follow-up test attacks also found one remaining lock-in gap and some doc drift:
- governed package roots were covered for `[package.metadata.guardrail3].arch_role`, but not the `[workspace.metadata.guardrail3]` variant
- the live `arch` plan still pointed at the deleted `checks/rs/arch` tree and still spoke as if the family only had five rules
- the docs understated that malformed eligible out-of-zone roots fail closed generally, not only when auxiliary metadata resolution happens

## Decisions Made

### Make explicit family requests bypass config-enabled filtering
- **Chose:** Change `family_selection::resolve` so a non-empty `requested_families` list is taken verbatim rather than intersected with config-enabled families.
- **Why:** Explicit CLI requests are different from ambient auto-selection. For `arch`, this is required to surface inactive `RS-ARCH-02` inventory and other fail-closed/reporting behavior even when global config disables the family.
- **Alternatives considered:**
  - Keep filtering requested families through config enablement — rejected because it silently hides explicitly requested families and makes product behavior contradict the runtime/family contract.
  - Special-case only `arch` in runtime — rejected because requested-family handling belongs in shared family selection, not ad hoc per-family runtime branching.

### Add selector-level coverage instead of relying only on runtime regression
- **Chose:** Add unit tests in `family_selection` for explicit-request bypass and empty-request filtering, while also keeping the runtime regression for explicit `arch`.
- **Why:** The selector change is shared infrastructure. It needs its own narrow tests so future selection refactors do not regress `arch` product behavior accidentally.
- **Alternatives considered:**
  - Keep only the runtime regression — rejected because it would still leave the shared selector contract implicit and harder to diagnose when broken.

### Lock the remaining governed metadata surface into RS-ARCH-07 tests
- **Chose:** Add a `packages/shared` workspace-metadata attack case and tighten the governed-root `arch_role` diagnostic wording in placement.
- **Why:** The code already enforced governed-root metadata misuse at the right layer, but the attack review correctly pointed out that one metadata table variant was not explicitly protected by tests, and the message was too narrowly worded around `auxiliary`.
- **Alternatives considered:**
  - Leave the missing workspace-metadata case untested because the code path is shared — rejected because this exact family already regressed around metadata surface boundaries before, and the extra test is cheap.
  - Move governed metadata misuse into `RS-ARCH-08` — rejected because it is an invalid required input, not benign auxiliary inventory.

### Rewrite the live arch plan to current shape instead of preserving stale scaffolding
- **Chose:** Update `.plans/todo/checks/rs/arch.md` to reference the self-hosted family workspace and shared `placement/` path, and fix the stale “five rules” / deleted-path language.
- **Why:** The audit goal was not only runtime correctness but also reducing cold-start drift. The live plan is supposed to be an active source of truth; stale implementation references in that file keep reintroducing confusion.
- **Alternatives considered:**
  - Leave the stale implementation sections alone because historical files were already marked superseded — rejected because this file is not historical; it is still the live rule inventory.

## Architectural Notes
The selection fix is intentionally shared, not family-local:
- explicit family requests are a runtime/product concern, so `family_selection` is the right owner
- the family still decides rule semantics based on config and routed facts
- auto-selection still respects config enablement, so the change does not widen normal default execution

This keeps the architecture split clean:
- `family_selection` owns requested-family resolution
- `placement` owns live-root parsing and structural metadata validation
- `RS-ARCH` owns reporting over those routed inputs

The governed-root metadata wording stays in `placement` because that layer sees both package and workspace metadata tables before classification, which is exactly where dead `arch_role` usage should be cut off.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — current family source of truth
- `.plans/todo/checks/rs/arch.md` — live rule inventory and implementation notes
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — shared requested-family logic
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — product-surface `arch` regressions
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — governed-root metadata enforcement point
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs` — fail-closed attack coverage
- `.worklogs/2026-03-29-220217-align-rs-arch-04-overlap-contract.md`
- `.worklogs/2026-03-29-220707-fix-rs-arch-07-governed-manifest-fail-closed.md`
- `.worklogs/2026-03-29-221125-add-rs-arch-06-app-scope-coverage.md`
- `.worklogs/2026-03-29-221616-tighten-rs-arch-suppression-and-governed-metadata.md`
- `.worklogs/2026-03-29-221740-mark-historical-arch-handoffs.md`

## Open Questions / Future Considerations
- `RS-ARCH` is now materially cleaner, but `family_selection` behavior should probably be documented in the shared `rs/README.md` explicitly so other families do not make conflicting assumptions about explicit requests.
- Top-level product verification can still be blocked by unrelated in-flight family work if those lanes introduce compile failures. The `arch` slice itself is now clean, but shared workspace churn still affects how easy it is to re-run whole-product checks.
- `libarch` remains unimplemented as a full family even though `RS-ARCH` already models its ownership and enablement semantics.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — shared requested-family behavior; explicit requests now bypass config filtering
- `apps/guardrail3/crates/app/rs/family_selection/src/selection_tests.rs` — narrow selector regressions for explicit requests vs ambient filtering
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — product-surface `arch` regressions including explicit inactive reporting
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — governed-root metadata fail-closed enforcement
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs` — metadata/input fail-closed attack coverage
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — current family-local source of truth
- `.plans/todo/checks/rs/arch.md` — current live rule inventory and implementation notes
- `.worklogs/2026-03-29-221616-tighten-rs-arch-suppression-and-governed-metadata.md` — prior `arch` tightening that this change builds on
- `.worklogs/2026-03-29-221740-mark-historical-arch-handoffs.md` — doc-layer cleanup immediately preceding this slice

## Next Steps / Continuation Plan
1. If the next pass stays on `arch`, document explicit requested-family override behavior in the shared `apps/guardrail3/crates/app/rs/README.md` so the contract is visible outside the family README and tests.
2. If product-wide verification matters again, re-run:
   - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-selection`
   - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-arch --lib`
   - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-runtime arch_runtime_ -- --nocapture`
   - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family arch --inventory --format json`
3. Continue the broader Rust-family stabilization work in the other lanes (`clippy`, `deny`, `deps`, `garde`) without reintroducing nested workspaces or stale family-plan paths.
