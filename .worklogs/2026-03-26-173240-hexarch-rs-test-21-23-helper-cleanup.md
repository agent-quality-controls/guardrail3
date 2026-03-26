# Hexarch RS-TEST Sidecar Helper Cleanup

**Date:** 2026-03-26 17:32
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_21_domain_purity.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_21_domain_purity_tests/**`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance_tests/**`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_23_adapter_pub_trait.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_23_adapter_pub_trait_tests/**`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config.rs`

## Summary
Removed the remaining `RS-TEST-03` sidecar escape hatches in hexarch rules 21, 22, and 23 by moving their test setup into local helper functions on the owned rule modules. The sidecars now call those helpers instead of importing `dependency_facts`, `inputs`, or `source_facts` directly.

## Context & Problem
`rs validate ... --family test` on `apps/guardrail3/crates/app/rs/families/hexarch` was still flagging RS-TEST-03 violations in the 21/22/23 sidecar trees because the tests were constructing their own dependency/source facts by reaching into sibling production modules. The goal was to keep behavior unchanged while forcing the test surface through rule-owned helpers.

## Decisions Made

### Local test helpers instead of sidecar ownership
- **Chose:** Added `#[cfg(test)]` helper entrypoints on the rule modules:
  - `run_domain_purity_case(...)`
  - `run_source_case(...)` for ports and adapter source rules
- **Why:** This keeps the sidecars from importing sibling production modules directly while preserving the existing assertions and fixture shape.
- **Alternatives considered:**
  - Moving the test logic into assertions crates only — rejected because these tests still need direct rule-module coverage and would have widened the boundary more than necessary.
  - Loosening `RS-TEST-03` — rejected because the issue was real; the tests were still constructing routes/facts from the wrong layer.

### Narrow helper shapes
- **Chose:** Exposed small helper APIs that take primitive inputs or a `ProjectTree`, then build the owned rule input internally.
- **Why:** This avoided reintroducing `dependency_facts`, `inputs`, or `source_facts` imports in the sidecars and kept the helper surface limited to the exact test scenarios.
- **Alternatives considered:**
  - Passing full sibling fact structs through the sidecars — rejected because that would preserve the same coupling with a thinner wrapper.

## Architectural Notes
The key architectural constraint is that the sidecar tests should exercise the owned rule module, not reconstruct its internals. For `RS-TEST-03`, that means the module may own test-only fixture assembly, but the test files themselves must stay within the module boundary. This keeps the validator honest and prevents the test tree from becoming a back door into the dependency/source discovery layers.

## Information Sources
- Existing hexarch runtime modules and sidecar tests under `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/`
- The `RS-TEST-03` contract in `apps/guardrail3/crates/app/rs/families/test/README.md`
- Prior worklog `2026-03-26-172253-tighten-rs-test-route-boundaries-and-runtime-test-entrypoints.md`

## Open Questions / Future Considerations
- `cargo test --lib` for the full hexarch crate still reports unrelated failures in rules 24/25. Those are outside this slice and were not modified.
- If the next pass wants the full crate test suite green, the 24/25 broad-attacks need separate investigation.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_21_domain_purity.rs` — new helper API for domain-purity sidecars.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance.rs` — new helper API for ports trait-dominance sidecars.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_23_adapter_pub_trait.rs` — new helper API for adapter public-trait sidecars.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_21_domain_purity_tests/` — migrated sidecars using the owned helper.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance_tests/` — migrated sidecars using the owned helper.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_23_adapter_pub_trait_tests/` — migrated sidecars using the owned helper.
- `.worklogs/2026-03-26-172253-tighten-rs-test-route-boundaries-and-runtime-test-entrypoints.md` — prior routing-boundary cleanup that set up this pass.

## Next Steps / Continuation Plan
1. If the goal is only RS-TEST validator compliance for 21/22/23, stop here and commit this slice.
2. If the goal is a fully green hexarch crate test run, investigate the unrelated 24/25 failures in a separate pass.
3. Re-run `rs validate` on the hexarch family root after any further edits to make sure `RS-TEST-03` stays clean.
