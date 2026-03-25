# Cut Root Tests Off Legacy Facades

**Date:** 2026-03-25 12:18
**Scope:** `apps/guardrail3/crates/app/rs/mod.rs`, `apps/guardrail3/tests/**`

## Summary
Moved the remaining root test callers off the root `guardrail3::app::rs::validate` and root outbound-fs facades, updated the root test suite to the family-based Rust CLI/config contract, and then removed the `app::rs::validate` public reexport from the root Rust facade. This is the next real decoupling step after the crate split: tests no longer force the old compatibility export to stay public.

## Context & Problem
After the crate split and legacy-validator promotion, there were no production callers left on `guardrail3::app::rs::validate`. The remaining internal blockers were root tests:
- unit tests importing `guardrail3::app::rs::validate::*`
- TS/RS arch fixture helpers still importing root outbound-fs paths
- CLI/property/adversarial tests still using removed grouped Rust flags and old config keys

That meant the root facade was still wider than necessary for internal reasons, not product reasons. If the branch is meant to converge on honest crate ownership, those tests need to move first.

## Decisions Made

### Move root tests to direct crate owners
- **Chose:** Replaced root test imports of `guardrail3::app::rs::validate::*` with direct imports from `guardrail3_app_rs_legacy_validate`, and replaced remaining root outbound-fs imports with `guardrail3_adapters_outbound_fs`.
- **Why:** These tests are internal callers. They should depend on the real owner crates, not keep the root compatibility facade alive.
- **Alternatives considered:**
  - Keep using the root facade in tests for convenience — rejected because it preserves exactly the coupling the split is trying to remove.
  - Delete the tests instead — rejected because these are still valid compatibility/behavior checks even if the longer-term test architecture is imperfect.

### Remove the root `app::rs::validate` reexport
- **Chose:** Deleted `pub use guardrail3_app_rs_legacy_validate as validate;` from `apps/guardrail3/crates/app/rs/mod.rs`.
- **Why:** Once internal callers moved off it, the root Rust facade no longer needed to expose the legacy validator tree as part of `app::rs`.
- **Alternatives considered:**
  - Leave the reexport around as a convenience alias — rejected because it would keep the root facade broader than necessary with no active internal consumer.

### Update root tests and fixtures to the family-based Rust contract
- **Chose:** Updated CLI/adversarial/property tests and the golden `guardrail3.toml` fixture from grouped Rust flags/keys (`--architecture`, `--code`, `--tests`, `architecture`, `tests`) to the current family model (`--family ...`, `hexarch`, `test`, etc.).
- **Why:** The root test suite should validate the live CLI/config contract, not lock in removed grouped-domain behavior.
- **Alternatives considered:**
  - Revert these tests and leave stale expectations behind — rejected because it would preserve false failures against a runtime contract we already changed intentionally.

## Architectural Notes
This commit is not test-relayering. It is caller migration:
- root tests now call real crate owners directly
- the root `app::rs` facade is narrower
- the remaining legacy-validate surface is a crate dependency, not a root public namespace

This is exactly the kind of cleanup that makes the workspace split durable instead of purely structural.

## Information Sources
- `rg -n "guardrail3::app::rs::validate"` across `apps/guardrail3/tests` — identified the remaining internal blockers.
- `apps/guardrail3/crates/app/rs/mod.rs` — root Rust facade that still reexported legacy validate.
- `apps/guardrail3/tests/cli_tests.rs` — root CLI coverage for the family-based contract.
- `apps/guardrail3/tests/adversarial_categories.rs` — adversarial coverage of family selection and config behavior.
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/guardrail3.toml` — golden config fixture still using removed grouped keys.
- `.worklogs/2026-03-25-120739-direct-outbound-traits-in-legacy-validators.md` — prior direct-owner cleanup inside legacy validator/TS compatibility code.
- `.worklogs/2026-03-25-120443-delete-dead-root-rs-check-trees.md` — prior deletion of dead root check trees.

## Open Questions / Future Considerations
- The root `unit` test target is still an expensive compile boundary. `cargo test --test unit --no-run` spent a long time compiling `guardrail3-app-rs-family-hooks-rs` without surfacing diagnostics, so I stopped treating it as useful proof for this batch.
- Two non-test shim users still exist in live code: `app/rs/validate/arch/rs_arch_01/helpers.rs` and `app/ts/validate/ts_arch_checks.rs` still reach `crate::app::arch_helpers`. That is the next small compatibility edge to clean if the goal is to narrow the root facade even further.
- Root tests still exist as a broad topology. This commit only removes their dependence on the legacy root facades; it does not redesign the overall root harness.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/mod.rs` — root Rust facade after removing the legacy-validate reexport.
- `apps/guardrail3/tests/cli_tests.rs` — root CLI contract tests for family-based validation.
- `apps/guardrail3/tests/adversarial_categories.rs` — adversarial family-selection coverage.
- `apps/guardrail3/tests/unit/test_hex_arch_checks.rs` — representative direct-import migration from root facade to real owner crates.
- `apps/guardrail3/tests/unit/rs_test_checks_test.rs` — direct-import migration onto `guardrail3_app_rs_legacy_validate`.
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/guardrail3.toml` — root fixture updated to family keys.
- `.worklogs/2026-03-25-120739-direct-outbound-traits-in-legacy-validators.md` — preceding legacy-validator cleanup.
- `.worklogs/2026-03-25-121051-sync-split-docs-config-and-plans.md` — preceding doc/config synchronization for the family-based model.

## Next Steps / Continuation Plan
1. Audit whether `guardrail3_app_rs_legacy_validate` is still needed as a root package dependency or only as a direct test dependency marker in `crates/lib.rs`.
2. Clean the remaining `crate::app::arch_helpers` shim use in `app/rs/validate/arch/rs_arch_01/helpers.rs` and `app/ts/validate/ts_arch_checks.rs`, then reassess whether the root `app::arch_helpers` reexport can narrow.
3. Keep using narrow crate/workspace compile checks for proof; avoid treating the monolithic root `unit` harness as the primary validation signal until that topology is split further.
