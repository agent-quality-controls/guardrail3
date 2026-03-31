# Runtime Exactness Separation Proof

**Date:** 2026-03-31 20:43
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_integrity/rs_hexarch_20_dev_dependency_direction.rs`, `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs`, `apps/guardrail3/crates/app/rs/runtime/assertions/src/runtime.rs`

## Summary
Hardened the post-split verification around workspace-membership exactness ownership. The runtime test suite now proves that `hexarch` and `libarch` runs surface `RS-ARCH-12` from the `arch` section while the removed local exactness rule IDs remain absent, and the `hexarch` crate-local test ingress now mirrors the routed production surface instead of feeding rules the full raw tree.

## Context & Problem
The previous change moved workspace-membership exactness ownership into `RS-ARCH-12` and deleted the duplicate local rules from `hexarch` and `libarch`. That ownership shift was manually verified with lean CLI runs, but there were still two gaps:

1. The proof was not encoded as durable regression tests at the runtime layer.
2. The `hexarch` crate-local test harness still built a route and then called the family with `RsProjectSurface::from_tree(tree)`, which no longer matches production after routed-surface hardening.

The user asked to keep pushing until the split was actually separate and verified: `arch` enforces exactness, `hexarch` no longer enforces it, `libarch` does not enforce it, and that fact is proven instead of assumed.

## Decisions Made

### Encode the ownership split at runtime, not just in family-local tests
- **Chose:** Add runtime regression tests that run `hexarch` and `libarch` through the real orchestrator and assert `RS-ARCH-12` appears while the removed local rule IDs stay absent.
- **Why:** The runtime is the real product boundary. A family can be “clean” locally while the top-level dispatch still routes ownership incorrectly. The user specifically wanted proof that family runs are separated correctly.
- **Alternatives considered:**
  - Add only more `arch` family tests — rejected because `arch` correctness alone does not prove `hexarch` and `libarch` runs route ownership correctly.
  - Rely only on manual CLI runs — rejected because the proof would stay transient and easy to regress.

### Align `hexarch`’s crate-local test ingress with production routing
- **Chose:** Build a routed test surface in `hexarch` test helpers using route roots plus extra repo/config files, matching the runtime’s `hexarch_surface(...)` shape.
- **Why:** After the runtime hardening, `hexarch` no longer operates on arbitrary full-tree ingress in production. The old test harness was creating a route and then bypassing it by feeding the family the raw tree anyway.
- **Alternatives considered:**
  - Leave the old harness and only add runtime tests — rejected because it preserves a misleading family-local test entrypoint.
  - Rewrite the entire failing `hexarch` unit corpus immediately — rejected because the broad failure surface is a separate fixture-drift problem and not necessary to prove the exactness-owner split.

### Treat old rule IDs as banned output, not just deleted files
- **Chose:** Add assertion support for “IDs must be absent anywhere in the report” and use it in the new runtime tests.
- **Why:** Deleting files is not enough. The product-level concern is whether those IDs can still leak into output through stale wiring or hidden paths.
- **Alternatives considered:**
  - Grep-only proof — rejected because source absence does not protect against future accidental reintroduction through copied constants or test-only wiring.

## Architectural Notes
The important architectural boundary here is:

- `arch` owns repo-wide workspace-topology exactness
- `hexarch` owns app-local structure and dependency policy
- `libarch` owns layered package structure and facade semantics

The new runtime tests encode that boundary at the executable/report layer, which is stronger than unit-only coverage. The `hexarch` test-surface change also reinforces the broader routed-surface architecture: family tests should exercise the same shaped ingress as production wherever possible.

## Information Sources
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — production `hexarch_surface(...)` behavior
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — family-local test ingress
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` — runtime regression harness
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_12_declared_workspace_members_only_tests/mod.rs` — existing exactness coverage in `arch`
- `.worklogs/2026-03-31-200335-arch-workspace-membership-exactness.md` — previous ownership shift into `arch`
- `.worklogs/2026-03-31-203539-remove-libarch-exactness-duplicates.md` — previous duplicate removal from `libarch`
- Lean validator runs:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-hexarch -- rs validate apps/guardrail3 --family hexarch --format json | jq ...`
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-libarch -- rs validate apps/guardrail3 --family libarch --format json | jq ...`

## Open Questions / Future Considerations
- The broad `guardrail3-app-rs-family-hexarch --lib` corpus still has many failures, but those are not evidence that `hexarch` still owns workspace-membership exactness. They stem from older fixture/test assumptions that no longer match current routed-surface and legality semantics.
- Several historical `.plans/todo/...` documents still mention removed `RS-HEXARCH-07/09` and `RS-LIBARCH-05/06` ownership. They are historical notes, but if they are still used as active handoffs they should be marked superseded or updated.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` — runtime-level proof that `hexarch`/`libarch` runs surface `RS-ARCH-12` and not the removed duplicate IDs
- `apps/guardrail3/crates/app/rs/runtime/assertions/src/runtime.rs` — shared runtime assertions, including new absent-ID proof helper
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — family-local routed test ingress matching production
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_12_declared_workspace_members_only_tests/mod.rs` — `arch` exactness coverage for app and package workspaces
- `.worklogs/2026-03-31-200335-arch-workspace-membership-exactness.md` — original exactness ownership move into `arch`
- `.worklogs/2026-03-31-203539-remove-libarch-exactness-duplicates.md` — follow-up removal from `libarch`

## Next Steps / Continuation Plan
1. Decide whether to repair or retire the stale broad `hexarch` unit corpus. If repairing, start with the family-local fixture ingress and workspace-legality assumptions before touching individual rule expectations.
2. Sweep active handoff docs under `.plans/by_family/rs` and any actively used `.plans/todo/*` handoffs to ensure none still describe `hexarch` or `libarch` as owners of workspace-membership exactness.
3. Keep new separation proofs in place whenever future family ownership moves happen: add runtime-layer report assertions, not just family-local tests and manual CLI verification.
