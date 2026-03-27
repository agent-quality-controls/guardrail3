# Fix Clippy Policy Root Gaps

**Date:** 2026-03-27 21:16
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/{facts.rs,facts_tests/mod.rs,rs_clippy_01_coverage_tests/*,rs_clippy_14_library_global_state_tests/*}`

## Summary
Fixed two real `RS-CLIPPY` semantic holes found during adversarial review. Validation-root `clippy.toml` is now honored even when the validation root is not itself a Rust root, and standalone app roots under `apps/<name>` now inherit `rust.apps.<name>` profile and garde policy instead of silently falling back to generic defaults.

## Context & Problem
After finishing the `rs/clippy` sidecar extraction, the next phase was rule attack rather than more structural migration. The family plan says:

- validation-root `clippy.toml` is always an allowed policy root
- routed Rust units may be covered by an allowed ancestor `clippy.toml`
- profile-aware clippy policy should respect `rust.apps.*` / `rust.packages` policy

Code inspection of `facts.rs` exposed two contradictions:

1. root `clippy.toml` was only collected when `""` was present in routed Cargo roots, which means a repo root with no `Cargo.toml` could not act as the validation-root policy file at all
2. app-path resolution used workspace member directories only, so direct standalone app roots like `apps/libsite/Cargo.toml` could not resolve `rust.apps.libsite` policy

Those were not theoretical concerns. I added focused regressions first, and the validation-root coverage regression failed immediately while the standalone-app policy regression confirmed the second gap.

## Decisions Made

### Always treat the validation root as an allowed clippy policy root
- **Chose:** Always collect root `clippy.toml` / `.clippy.toml` when present, and always include `""` in the allowed policy-root set.
- **Why:** The family contract explicitly allows the validation root regardless of whether the repo root is also a Rust root. Gating root policy discovery on routed Cargo roots was a fail-open bug.
- **Alternatives considered:**
  - Keep root config tied to routed roots only — rejected because it contradicts the plan and produced a real uncovered-workspace false negative.
  - Move validation-root handling into `FamilyMapper` — rejected because this is still family-local config discovery inside routed scope, not Rust-root routing.

### Extend app-path resolution to direct `apps/<name>` roots
- **Chose:** Keep the existing workspace-member-based app resolution, but also map direct Cargo roots at `apps/<name>` to app names.
- **Why:** Standalone app roots are still routed Rust roots, and their Clippy policy needs to inherit `rust.apps.<name>` metadata. Without this, library/service profile and garde-aware ban baselines drift silently.
- **Alternatives considered:**
  - Infer all app policy only from workspace members — rejected because it misses legitimate standalone app roots.
  - Add app metadata to `RsClippyRoute` — rejected for this slice because the family can resolve this local policy question from routed root paths without broadening mapper scope.

### Prove both bugs with direct regressions
- **Chose:** Add one facts-level regression plus one coverage-rule regression for the validation-root gap, and one facts-level plus one rule-level regression for standalone app profile inheritance.
- **Why:** The attack result needed to become durable. These were rule-semantics bugs, not one-off fixture oddities.
- **Alternatives considered:**
  - Patch the code without new tests — rejected because this work is explicitly about hardening detector trust, not just changing behavior.

## Architectural Notes
- The fix stays within the intended family split:
  - `placement` still decides which Rust roots exist
  - `FamilyMapper` still decides which Rust roots reach `clippy`
  - `clippy::facts` still owns family-local policy discovery inside those routed roots
- Validation-root policy handling belongs in `facts.rs` because it is Clippy config discovery, not Rust-root routing.
- Direct `apps/<name>` path resolution is intentionally narrow. It does not try to reimplement app discovery globally; it only closes the gap for routed standalone app roots already in scope.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- `.plans/todo/checks/rs/clippy.md`
- `.worklogs/2026-03-27-210812-finish-clippy-sidecar-extraction.md`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_01_coverage.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_14_library_global_state.rs`
- nested-workspace verification:
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`

## Open Questions / Future Considerations
- The next likely `RS-CLIPPY` attack surface is still policy semantics rather than placement:
  - coverage / shadowing parity around root `.clippy.toml` precedence and ancestor selection
  - profile-sensitive rules that may still fail open if `guardrail3.toml` is malformed or partially missing
- The outer app workspace is still broken by unrelated in-flight `deny` work, so fresh top-level `rs validate --family test` on `clippy` still cannot be rerun from the root until that workspace issue is resolved.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — clippy policy-root discovery and profile/garde resolution
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts_tests/mod.rs` — direct facts regressions for validation-root policy and standalone app policy inheritance
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_01_coverage_tests/root_policy_without_root_cargo.rs` — coverage regression proving ancestor root policy works without root Cargo
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_14_library_global_state_tests/standalone_app_profile.rs` — rule-level regression proving standalone app profile inheritance
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — family contract being enforced
- `.worklogs/2026-03-27-210812-finish-clippy-sidecar-extraction.md` — prior checkpoint before semantic attack work

## Next Steps / Continuation Plan
1. Keep attacking `RS-CLIPPY` semantics, starting with remaining coverage/placement edges and then profile-sensitive rules (`13`, `16`, `17`, `19`, `20`).
2. Once the unrelated `deny` workspace break is gone, rerun top-level:
   - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`
   to confirm the family still passes `RS-TEST`.
3. Continue isolating clippy-only commits; do not fold the shared top-level workspace churn or deny-family migration into this stream.
