# Replace Hexarch Ports Proxy Rule

**Date:** 2026-03-27 08:48
**Scope:** `.plans/todo/checks/rs/hexarch.md`, `apps/guardrail3/crates/app/rs/families/hexarch/README.md`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/source_facts.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance_tests/*`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_23_adapter_pub_trait.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_23_adapter_pub_trait_tests/*`

## Summary
Replaced `RS-HEXARCH-22`’s impl-count heuristic with a direct ports public-surface rule. The family now warns only when ports crates expose public free functions or public inherent methods on concrete types, while still allowing passive public types and trait impls. I also tightened source visibility handling so `pub` items inside private modules no longer count as public surface, and updated the adapter tests to match that interpretation.

## Context & Problem
After the previous `hexarch` attack round, `RS-HEXARCH-22` was still a proxy rule. It had already been narrowed away from raw impl-counting, but it still inferred “too much behavior” from syntax shape rather than enforcing an exact contract. The user explicitly asked what architecture would be both robust and workable, then asked to update the docs and make that happen.

The key design tension was:
- requiring every ports crate to define a `pub trait` is clean, but too rigid for DTO-only ports crates like the existing fixture’s `ports/inbound/api`
- allowing arbitrary public behavior in ports makes the boundary soft and pushes the checker back toward heuristics

## Decisions Made

### Make `RS-HEXARCH-22` a direct public-surface rule
- **Chose:** `RS-HEXARCH-22` now warns only for:
  - public free functions outside trait items
  - public inherent methods on concrete types
- **Why:** this enforces the architectural boundary directly instead of inferring suspiciousness from counts.
- **Alternatives considered:**
  - keep the old impl-heavy heuristic — rejected because it remained proxy-shaped and had already produced a live false positive
  - require at least one `pub trait` in every ports crate — rejected because it made passive DTO-only ports crates invalid, including the existing golden fixture

### Allow passive public types in ports
- **Chose:** keep public DTO/error/type declarations and trait impls legal in ports crates.
- **Why:** this is the workable middle ground. Ports can carry contract-adjacent passive types without turning into service/helper crates.
- **Alternatives considered:**
  - ban all public non-trait items in ports — rejected because it is architecturally pure but too rigid in practice
  - allow arbitrary inherent behavior on public types — rejected because that erodes the ports/adapters boundary quickly

### Count only actual public surface, not private-module items
- **Chose:** `source_facts.rs` now tracks a `public_module` chain. `pub trait`, `pub fn`, and public inherent methods count only when they are reachable through public modules from the crate root.
- **Why:** `pub` items inside private modules are not part of the crate’s public surface. Counting them would create false positives for both `RS-HEXARCH-22` and `RS-HEXARCH-23`.
- **Alternatives considered:**
  - keep counting every reachable parsed item — rejected because it confuses “syntactically public” with “publicly exported”
  - fully resolve re-exports before counting — rejected for now as too heavy for the immediate rule improvement

### Update adapter tests to the same public-surface definition
- **Chose:** changed adapter tests that expected private nested modules with `pub trait` to error. Those cases now require `pub mod`.
- **Why:** once source analysis switches to true public-surface semantics, those old test assumptions are wrong.
- **Alternatives considered:**
  - keep adapter rule on broader counting than ports rule — rejected because the family should have one coherent definition of public surface

## Architectural Notes
The new `RS-HEXARCH-22` contract is:
- passive public types in ports are fine
- public behavior in ports must live in trait definitions, not free functions or concrete-type inherent methods

That keeps the rule deterministic and non-proxy while still permitting practical contract-local types. It is also consistent with the broader boundary:
- ports define contracts and passive contract-local data
- adapters implement behavior
- private helper code is still allowed anywhere

The public-surface visibility change in `source_facts.rs` also quietly improves `RS-HEXARCH-23`, because adapter `pub trait` checks now align with actual exported surface instead of private module implementation detail.

## Information Sources
- `.plans/todo/checks/rs/hexarch.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/source_facts.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_23_adapter_pub_trait.rs`
- prior worklog `.worklogs/2026-03-26-224750-hexarch-attack-fixes.md`
- live repo verification:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate . --family hexarch --inventory --format json`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate /Users/tartakovsky/Projects/steady-parent --family hexarch --inventory --format json`

## Open Questions / Future Considerations
- The public-surface counting currently follows public module chains but does not do full re-export analysis. If a repo later hides or re-exports ports/adapters API in a tricky way, source-surface rules may need another refinement.
- `RS-HEXARCH-14` still has an open contract question around out-of-repo external path dependencies. `steady-parent` has such deps under `landing-dioxus`, and this round did not change that behavior.

## Key Files for Context
- `.plans/todo/checks/rs/hexarch.md` — current `RS-HEXARCH` inventory and rule wording
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — family-local architectural contract
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/source_facts.rs` — source-surface collection and public-module visibility handling
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance.rs` — direct ports public-surface rule
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance_tests/reachable_modules.rs` — visibility and reachable-module coverage for rule 22
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_23_adapter_pub_trait_tests/reachable_modules.rs` — matching public-surface semantics for adapters
- `.worklogs/2026-03-26-224750-hexarch-attack-fixes.md` — prior attack round that set up the remaining proxy-rule cleanup

## Next Steps / Continuation Plan
1. If the user still wants stricter ports contracts, the next ratchet is explicit re-export analysis, not more heuristics.
2. Decide the contract for `RS-HEXARCH-14` on out-of-repo path deps and encode that decision in the plan plus tests.
3. Re-run adversarial completeness passes on `deps` and `release`, which still have not had the same end-to-end verification depth as `test`, `arch`, and `hexarch`.
