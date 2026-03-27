# Baseline Clippy Family Documentation

**Date:** 2026-03-27 19:58
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/README.md`

## Summary
Added the first family README for `RS-CLIPPY` so the family has an explicit contract before the self-host migration starts. The doc captures the placement/mapper split, current old-shape layout, target runtime/assertions/test_support shape, and the known baseline state that still needs to be stabilized.

## Context & Problem
`RS-CODE` has now had several adversarial hardening rounds and is green on itself again, so the next active family is `RS-CLIPPY`. That family had no README at all, which meant there was no local source of truth describing what it owns versus what belongs to `placement`, `FamilyMapper`, `RS-CARGO`, `RS-CODE`, and `RS-TEST`. Before migrating the family to the self-hosted workspace shape, the contract needed to be written down.

## Decisions Made

### Document `RS-CLIPPY` before reshaping it
- **Chose:** Add a family README that describes ownership, current structure, target structure, and the current baseline.
- **Why:** The recent Rust-family work has shown that migrations go better once each family has a local contract file, especially when the family no longer owns root discovery and now sits behind shared scope plus typed routing.
- **Alternatives considered:**
  - Skip docs and start moving files immediately — rejected because the family boundary would stay implicit and drift-prone.
  - Only rely on the top-level `apps/guardrail3/crates/app/rs/README.md` — rejected because that file describes shared Rust architecture, not `RS-CLIPPY`’s family-local contract.

### Treat `RS-CLIPPY` as a policy family inside routed roots
- **Chose:** State explicitly that `RS-CLIPPY` owns Clippy policy discovery and validation only inside routed roots.
- **Why:** This matches the architecture already pushed through `placement` and `FamilyMapper`, and prevents `clippy` from silently re-owning root discovery.
- **Alternatives considered:**
  - Let the family decide which Rust roots are live — rejected because that repeats the boundary bug already removed from other families.
  - Collapse Cargo lint-table policy into `RS-CLIPPY` — rejected because that is owned by `RS-CARGO`.

## Architectural Notes
The README deliberately separates:
- shared Rust scope (`placement`)
- family selection / typed routing (`FamilyMapper`)
- family-local Clippy policy discovery

It also records the intended self-host target:
- `crates/runtime`
- `crates/assertions`
- `test_support`

That mirrors the stabilized family pattern already used by `test`, `arch`, `cargo`, `hexarch`, and `code`.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust architecture and FamilyMapper plan
- `apps/guardrail3/crates/app/rs/families/code/README.md` — latest stabilized family README pattern
- `apps/guardrail3/crates/app/rs/families/test/README.md` — stricter self-host/test-ownership contract
- live `clippy` family files under `apps/guardrail3/crates/app/rs/families/clippy/src`
- live family status from local runs:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
  - `guardrail3 rs validate apps/guardrail3/crates/app/rs/families/clippy --family clippy --inventory --format json`
  - `guardrail3 rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`
  - `guardrail3 rs validate apps/guardrail3/crates/app/rs/families/clippy --family arch --inventory --format json`

## Open Questions / Future Considerations
- The family still has one self-hit under `RS-CLIPPY-01`; the README records it but does not resolve it.
- The family is still single-crate and still fails `RS-TEST`; the README is only the baseline, not the migration.
- There is also unrelated in-progress `fmt` workspace churn in the tree; that was intentionally not folded into this checkpoint.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — family-local contract and migration target
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust placement/selection/mapper contract
- `apps/guardrail3/crates/app/rs/families/clippy/src/lib.rs` — current old-shape runtime entrypoint
- `apps/guardrail3/crates/app/rs/families/clippy/src/test_support.rs` — current pre-migration test support boundary
- `apps/guardrail3/crates/app/rs/families/code/README.md` — specimen for the stabilized README structure
- `.worklogs/2026-03-27-194344-rs-code-attack-round-3.md` — latest `RS-CODE` hardening checkpoint before switching focus to `clippy`

## Next Steps / Continuation Plan
1. Re-run `RS-CLIPPY`, `RS-TEST`, and `RS-ARCH` on `apps/guardrail3/crates/app/rs/families/clippy` after any workspace unblock fixes so the baseline is fresh.
2. Fix the immediate `RS-CLIPPY-01` self-hit in the smallest possible way, unless deeper audit shows the rule is wrong.
3. Convert `apps/guardrail3/crates/app/rs/families/clippy` from the old single-crate shape into the stabilized self-host pattern:
   - root workspace `Cargo.toml`
   - `crates/runtime`
   - `crates/assertions`
   - `test_support`
4. Make the family pass `RS-TEST` before treating any repo-wide `RS-CLIPPY` findings as trustworthy.
5. After the structural migration, run the same adversarial rule review pattern used on `RS-CODE` and `RS-TEST`.
