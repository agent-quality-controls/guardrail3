# Tighten RS-TEST Route Boundaries And Runtime Test Entrypoints

**Date:** 2026-03-26 17:22
**Scope:** `.plans/todo/checks/rs/test.md`, `apps/guardrail3/crates/app/rs/families/test/**`, `apps/guardrail3/crates/app/rs/families/cargo/**`, `apps/guardrail3/crates/app/rs/families/arch/**`, `apps/guardrail3/crates/app/rs/families/hexarch/**`, `apps/guardrail3/Cargo.lock`

## Summary
Tightened `RS-TEST-03` so assertions crates can no longer hide route construction through `FamilyMapper` / `placement`, then moved family-local test execution entrypoints into runtime crates so `rs/test`, `cargo`, and `arch` stay green under the stricter rule. Applied the same runtime-entrypoint cleanup to `hexarch` as well, which removed the assertions-side loophole and left only real sidecar-boundary violations in that family.

## Context & Problem
An adversarial pass against the rewritten cargo family showed that the previous green `RS-TEST` result was still partly artificial. The first fix tightened `RS-TEST-18` to reject mapper wiring in `test_support`, but cargo still passed by relocating the exact same route-construction logic into its assertions crate. The live README contract already said assertions must stay reusable semantic proof helpers and must not own orchestration, so the validator was wrong.

The same pattern turned out not to be cargo-specific. `rs/test` itself, plus `arch` and `hexarch`, were all constructing routed family input from their assertions crates. Fixing only cargo would have left the checker self-hosting story inconsistent and would have kept the same architectural leak live in other families.

At the same time, the cargo family workspace split itself was still sitting as a dirty tree rather than a committed checkpoint. The current work now depends on that local workspace shape, so this commit folds that family rewrite into the repo history instead of leaving it as a floating uncommitted refactor.

## Decisions Made

### Tighten `RS-TEST-03` instead of relaxing the contract
- **Chose:** Extended `RS-TEST-03` to reject assertions-module imports of route-construction infrastructure crates and `FamilyMapper`-built routed inputs.
- **Why:** The contract already banned assertions-side orchestration in substance; the validator was just not enforcing it. Tightening the rule exposed real false greens immediately.
- **Alternatives considered:**
  - Leave assertions-side route construction allowed implicitly — rejected because it defeats the runtime/assertions boundary and keeps the same loophole everywhere.
  - Move the prohibition into `RS-TEST-18` — rejected because the issue is not generic `test_support`; it is the assertions boundary itself.

### Move fixture execution entrypoints into runtime public API
- **Chose:** Added runtime-owned helper entrypoints such as `check_test_tree(...)` so assertions crates call runtime public API instead of rebuilding routes themselves.
- **Why:** Assertions are allowed to use runtime public API. They are not supposed to know how placement, selection, or mapping work. Runtime is the correct place to host a family-local test execution helper.
- **Alternatives considered:**
  - Keep route construction in assertions and weaken the new rule — rejected because that preserves the architectural smell.
  - Put route construction back into `test_support` — rejected because `test_support` is explicitly generic-only and `RS-TEST-18` now rejects that too.

### Apply the same boundary fix across all affected families immediately
- **Chose:** Reworked `rs/test`, `cargo`, `arch`, and `hexarch` in the same checkpoint.
- **Why:** Once `RS-TEST-03` was tightened, all four families showed the same backdoor. Fixing only one would have left the checker family itself red and would have produced a half-truth checkpoint.
- **Alternatives considered:**
  - Fix only `cargo` — rejected because `rs/test` would fail on itself and `arch` / `hexarch` would stay falsely structured.
  - Fix `cargo` and `rs/test` only — rejected because `arch` was small enough to clean up in the same pass, and `hexarch` benefited from shedding the same noise before further work.

### Treat remaining `hexarch` findings as real work, not validator noise
- **Chose:** Stop the architectural cleanup once `hexarch` no longer leaked route construction through assertions, even though it still fails `RS-TEST`.
- **Why:** After the cleanup, `hexarch` is down to concrete sidecar sibling-module violations. Those are actual family fixes, not another validator loophole.
- **Alternatives considered:**
  - Continue directly into all `hexarch` sidecar rewrites in the same checkpoint — rejected because that is a separate family migration slice with its own risk and reading burden.

## Architectural Notes
The important architectural shift is:

`ProjectTree -> placement -> FamilyMapper -> runtime::check_test_tree(...) -> assertions helpers`

Assertions crates no longer know how to build routes. They consume runtime public test helpers instead.

This is now true for:
- `rs/test`
- `rs/cargo`
- `rs/arch`
- `rs/hexarch`

The stricter `RS-TEST-03` also changed the meaning of “green”:
- `rs/test` passes on itself again after the runtime-entrypoint change
- `cargo` passes `RS-TEST` again after the same cleanup
- `arch` passes `RS-TEST` again after the same cleanup
- `hexarch` still fails, but now for sidecar sibling-module imports/calls rather than hidden route infrastructure

So the validator is now removing a real false-green class instead of merely reshuffling where helper wiring lives.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/README.md` — contract for assertions/runtime/test_support boundaries
- `.plans/todo/checks/rs/test.md` — current RS-TEST rule reminders and gotchas
- `.worklogs/2026-03-26-155851-tighten-rs-test-boundaries.md` — previous checkpoint that tightened `RS-TEST-03` for sidecar `super::...` escapes and `RS-TEST-18` for test-support route construction
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — live implementation of the tightened assertions-boundary rule
- `apps/guardrail3/crates/app/rs/families/{test,cargo,arch,hexarch}/crates/runtime/src/lib.rs` — new runtime-owned test execution entrypoints
- `apps/guardrail3/crates/app/rs/families/{test,cargo,arch,hexarch}/crates/assertions/src/*` — call sites moved off assertions-side route construction

## Open Questions / Future Considerations
- `hexarch` still has `36` real `RS-TEST-03` findings from internal sidecars importing sibling production modules. That family is not structurally clean yet.
- `RS-TEST` still does not deterministically catch every “semantic assertions duplicated outside assertions” case for internal sidecars. Cargo likely remains somewhat looser than the written README ideal even after the route-boundary cleanup.
- `test_support` genericness is still a sharper contract in prose than in deterministic enforcement. The cargo fixture-constant question is still open if we want `RS-TEST-18` to enforce that more aggressively.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — new prohibition on assertions-side route infrastructure
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — runtime-owned `rs/test` fixture entrypoint
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lib.rs` — runtime-owned cargo fixture entrypoint
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs` — runtime-owned arch fixture entrypoint
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — runtime-owned hexarch fixture entrypoint
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/dependency_facts.rs` — hexarch helper moved off assertions-side route construction
- `.worklogs/2026-03-26-155851-tighten-rs-test-boundaries.md` — prior RS-TEST boundary-tightening context

## Next Steps / Continuation Plan
1. Start from `apps/guardrail3/crates/app/rs/families/hexarch` and fix the remaining `36` `RS-TEST-03` sidecar violations. The current validator output points to concrete files under `crates/runtime/src/*_tests/*.rs`.
2. For each offending `hexarch` sidecar, move sibling-production helper access behind owned assertions helpers or owned module-local test helpers so the sidecar stops importing/calling sibling production modules directly.
3. Re-run:
   - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch`
   - `guardrail3 rs validate apps/guardrail3/crates/app/rs/families/hexarch --family test --inventory --format json`
4. After `hexarch` is structurally green, repeat the adversarial pass and decide whether `RS-TEST-17/18` need another deterministic tightening for internal sidecar semantic-assertion ownership.
