# Finish Clippy Sidecar Extraction

**Date:** 2026-03-27 21:08
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/{README.md,crates/assertions/src/*,crates/runtime/src/*}`

## Summary
Finished the large `rs/clippy` sidecar migration that was still leaving the family structurally behind the stabilized Rust families. The family now routes test execution through owner-module helpers plus sibling assertions modules across `RS-CLIPPY-01..22`, and the old runtime-local `test_support.rs` shim was deleted after its responsibilities were moved into owner modules and `facts.rs`.

## Context & Problem
The earlier `rs/clippy` workspace repair and first extraction rounds made the family buildable and green under `RS-ARCH` / `RS-CLIPPY`, but `RS-TEST` debt was still dominated by exactly the patterns the project is trying to stamp out:

- runtime sidecars reaching into a private runtime-local `test_support` module
- rule tests asserting `CheckResult` semantics inline instead of through sibling assertions modules
- parity tests leaning on runtime-private helpers like `clippy_support`

At the start of this slice, the `04..08` cluster was already in progress locally, and `12..20` plus `01` were still old-shape. At the same time, another agent’s in-flight `deny` workspace migration temporarily broke the outer app workspace, so fresh top-level `cargo run -p guardrail3 -- rs validate ...` was unavailable. That meant the trustworthy local loop for this slice had to be the nested `clippy` workspace tests rather than top-level validator reruns.

## Decisions Made

### Finish the migration cluster-by-cluster instead of waiting for a fresh top-level validator run
- **Chose:** Complete the clippy family’s sidecar/assertions extraction inside its nested workspace first, even though the outer workspace is temporarily broken by unrelated `deny` work.
- **Why:** The structural debt was local to the family and could be verified by the family’s own unit tests. Waiting for the outer workspace to recover would stall the migration without improving correctness.
- **Alternatives considered:**
  - Pause until top-level `rs validate --family test` is runnable again — rejected because the remaining clippy work was concrete and local.
  - Try to “fix around” the `deny` agent’s workspace state — rejected because that would risk trampling unrelated work.

### Replace runtime-local helper plumbing with owner-module helpers plus sibling assertions
- **Chose:** Add or extend `run_for_tests(...)` helpers in the owning runtime rule files and move semantic result expectations into per-rule modules under `crates/assertions/src`.
- **Why:** That matches the stabilized-family pattern already used in `test`, `arch`, `cargo`, `hexarch`, and `code`, and it removes the direct `RS-TEST-03` / `RS-TEST-16` style leaks from clippy sidecars.
- **Alternatives considered:**
  - Keep a shared runtime-local helper module and only rename imports — rejected because it preserves the boundary leak under a different spelling.
  - Push runtime-aware execution helpers into the sibling `test_support` crate — rejected because generic `test_support` should not own route/facts semantics.

### Delete the runtime-local `test_support.rs` shim instead of preserving it as a convenience layer
- **Chose:** Move test-only routing/facts helpers into `facts.rs` and owner modules, then delete `crates/runtime/src/test_support.rs`.
- **Why:** The shim had become dead architecture debt. Leaving it in place would keep the family half-migrated and make future `RS-TEST` failures harder to interpret.
- **Alternatives considered:**
  - Keep the shim until a future validator pass proves it is unnecessary — rejected because the local code already no longer needed it.
  - Rename the shim to dodge `RS-TEST` naming pressure — rejected because that would be a cosmetic bypass, not a boundary fix.

### Keep parity tests, but remove their dependence on runtime-private helpers
- **Chose:** Rewrite parity tests to use domain-module exports, direct TOML inspection, or tiny local helper functions instead of `super::super::super::clippy_support`.
- **Why:** These parity tests are still useful, but they should not rely on runtime-private internals that the family is explicitly trying to encapsulate.
- **Alternatives considered:**
  - Delete parity tests outright — rejected because they still protect baseline-generation drift.
  - Move parity logic into assertions modules — rejected because parity is about generated config surfaces, not result semantics.

## Architectural Notes
- `facts.rs` now owns the test-only route and config-input bridge:
  - `collect_for_tests(tree)`
  - `config_input_for_tests(facts, rel_path)`
- `rs_clippy_01_coverage.rs` now owns the full-family test entrypoint and fixture-copy helper for its sidecars, instead of depending on a runtime-local shim.
- `crates/assertions` now has rule-local modules for:
  - `RS-CLIPPY-01`
  - `RS-CLIPPY-04..08`
  - `RS-CLIPPY-12..20`
  - the threshold cluster and `facts` from earlier work
- The family README was updated to stop claiming the runtime shim still exists and to acknowledge the current verification constraint: outer-workspace `RS-TEST` reruns are blocked until the unrelated `deny` workspace break is gone.

## Information Sources
- Prior clippy worklogs:
  - `.worklogs/2026-03-27-202045-stabilize-rs-clippy-workspace.md`
  - `.worklogs/2026-03-27-202948-extract-clippy-threshold-assertions.md`
  - `.worklogs/2026-03-27-204331-clippy-boundary-cleanup.md`
- Family contract:
  - `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- Current stabilized-family patterns used as precedent:
  - `apps/guardrail3/crates/app/rs/families/test/README.md`
  - `apps/guardrail3/crates/app/rs/families/code/README.md`
- Core implementation files touched:
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_01_coverage.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/lib.rs`
- Verification commands:
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy-assertions --lib`
  - `cargo metadata --manifest-path apps/guardrail3/Cargo.toml --no-deps` (still fails because `families/deny` currently introduces a second workspace root)

## Open Questions / Future Considerations
- A fresh `RS-TEST` validator run for the clippy family still needs to happen once the unrelated `deny` workspace migration stops breaking outer-workspace Cargo metadata.
- If that fresh validator run still reports residual `RS-TEST` debt, the next likely place to look is not sidecar structure anymore; it will probably be proof-site recognition details or a stale README/validator assumption.
- `apps/guardrail3/Cargo.toml` and `apps/guardrail3/crates/app/rs/Cargo.toml` remain modified in the shared worktree, but they are outside this commit because they are part of the overlapping deny work.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — updated family contract and current verification status
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — now owns the test-only route/config bridge
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_01_coverage.rs` — now owns the full-family test helper path for coverage tests
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/lib.rs` — current assertions surface across the migrated rule clusters
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/rs_clippy_01_coverage.rs` — specimen for migrated coverage assertions
- `.worklogs/2026-03-27-202045-stabilize-rs-clippy-workspace.md` — workspace baseline context
- `.worklogs/2026-03-27-202948-extract-clippy-threshold-assertions.md` — first working extraction pattern
- `.worklogs/2026-03-27-204331-clippy-boundary-cleanup.md` — immediate precursor to this extraction

## Next Steps / Continuation Plan
1. Once the outer app workspace is healthy again, rerun:
   - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`
   and record the actual remaining `RS-TEST` buckets, if any.
2. If clippy is green under `RS-TEST`, switch focus from structural migration to adversarial `RS-CLIPPY` rule review:
   - attack coverage/placement (`01`, `12`, `13`)
   - attack policy correctness (`14..20`)
   - attack fail-closed behavior on malformed active `clippy.toml`
3. Keep `deny` isolated: do not fold the shared workspace-manifest churn from `apps/guardrail3/Cargo.toml` or `apps/guardrail3/crates/app/rs/Cargo.toml` into clippy commits unless that work is explicitly merged and verified.
