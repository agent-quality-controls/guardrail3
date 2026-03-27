# Harden Clippy Policy Context

**Date:** 2026-03-27 21:40
**Scope:** `.plans/todo/checks/rs/clippy.md`, `apps/guardrail3/crates/app/rs/families/clippy/{README.md,clippy.toml,crates/assertions/src/*,crates/runtime/src/*,test_support/src/*}`

## Summary
Hardened `rs/clippy` against a real fail-open hole: malformed active `guardrail3.toml` policy metadata was silently collapsing to default profile/garde behavior instead of surfacing a clippy failure. Added `RS-CLIPPY-23` for that case, propagated policy-context parse errors through facts, and made the profile/garde-dependent rules stop deriving expectations from broken metadata. This checkpoint also folds in the already-live `allow-panic-in-tests = false` policy and the missing published-library workspace coverage tests so the family’s fixtures, tests, and docs are internally consistent again.

## Context & Problem
After the earlier `rs/clippy` attack fixes, the remaining suspicious surface was no longer sidecar structure; it was policy-context correctness:

- `facts.rs` was reading `guardrail3.toml` directly for profile/garde resolution
- malformed or missing cached content for `guardrail3.toml` silently fell back to default profile/garde behavior
- that meant profile-sensitive rules like `RS-CLIPPY-04/05/06/07/13/14/16` could produce false greens or misleading results while the active policy context was broken

This contradicted both the family README and the plan, which already said the family should fail closed when active inputs are unreadable or malformed.

At the same time, the clippy worktree already contained a coherent but uncommitted sub-slice:

- `allow-panic-in-tests = false` had been added to the family root config and support helpers
- `RS-CLIPPY-17` tests had been updated for that managed key
- published-library workspace helpers/tests had been added for `RS-CLIPPY-14` and `RS-CLIPPY-16`

Leaving those changes out would keep the family half-updated and the nested workspace green only by accident.

## Decisions Made

### Add a dedicated clippy policy-context failure rule
- **Chose:** Add `RS-CLIPPY-23` as an explicit error when active `guardrail3.toml` policy metadata is unreadable or malformed.
- **Why:** One explicit family-owned failure is clearer than silently letting profile-sensitive rules infer defaults from broken metadata.
- **Alternatives considered:**
  - Let each affected rule invent its own parse-error message — rejected because that would duplicate the same input failure many times.
  - Keep the old silent fallback — rejected because it creates real false greens in profile/garde-sensitive rules.

### Propagate policy-context parse errors through facts and short-circuit dependent rules
- **Chose:** Store `policy_context_parse_error` in `ClippyFacts` and each `ClippyConfigFacts`, then make the profile/garde-dependent rules (`04`, `05`, `06`, `07`, `13`, `14`, `16`) return early when that failure is present.
- **Why:** Those rules need trustworthy profile/garde context. If the context is broken, `RS-CLIPPY-23` should own the failure instead of letting dependent rules misclassify.
- **Alternatives considered:**
  - Suppress every config rule on policy-context failure — rejected because rules like `RS-CLIPPY-17` do not depend on `guardrail3.toml` and can still evaluate meaningfully.
  - Leave dependent rules running alongside `RS-CLIPPY-23` — rejected because they can still emit misleading results from defaulted profile/garde values.

### Keep `RS-CLIPPY-17` independent of guardrail policy context
- **Chose:** Do not short-circuit `RS-CLIPPY-17` on malformed `guardrail3.toml`.
- **Why:** Its managed booleans are fixed Clippy policy knobs and do not depend on profile/garde resolution.
- **Alternatives considered:**
  - Treat `RS-CLIPPY-17` like the profile-sensitive rules and suppress it — rejected because that would hide real misconfiguration unrelated to profile resolution.

### Fold in the already-live `allow-panic-in-tests` and published-library workspace coverage changes
- **Chose:** Include the dirty-but-coherent clippy changes that were already in the worktree: managed `allow-panic-in-tests = false`, updated key parity tests, and the missing workspace publishability regressions/helpers.
- **Why:** They match the family’s actual semantics and were already part of the green nested clippy workspace. Leaving them out would keep the family internally inconsistent.
- **Alternatives considered:**
  - Cherry-pick only the new `RS-CLIPPY-23` code and ignore the rest — rejected because the current test surface and fixture policy would drift immediately.
  - Revert the in-worktree clippy changes and isolate only the policy-context fix — rejected because that would be silently undoing local clippy work without a user request.

## Architectural Notes
- `placement` and `FamilyMapper` are still untouched. The new failure path stays fully inside family-local config discovery.
- `facts.rs` now has a clear split:
  - discover routed clippy configs
  - resolve profile/garde policy context from `guardrail3.toml`
  - propagate policy-context failure explicitly instead of inventing defaults
- `RS-CLIPPY-23` is family-level input hygiene, not config-content linting.
- Published-library policy remains a family-local fact. The new tests ensure both:
  - publishable standalone package roots
  - publishable library workspaces with publishable members
  are classified consistently by `RS-CLIPPY-16`.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- `.plans/todo/checks/rs/clippy.md`
- Prior clippy worklogs:
  - `.worklogs/2026-03-27-210812-finish-clippy-sidecar-extraction.md`
  - `.worklogs/2026-03-27-211613-fix-clippy-policy-root-gaps.md`
  - `.worklogs/2026-03-27-212201-split-clippy-library-type-ban-ownership.md`
  - `.worklogs/2026-03-27-212709-fix-clippy-macro-ban-paths.md`
- Key implementation files:
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_17_test_relaxations.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_23_policy_context_parseable.rs`
- Verification:
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy-assertions --lib`

## Open Questions / Future Considerations
- The outer app workspace is still broken by unrelated `deny` migration work, so top-level `guardrail3 rs validate ... --family test` for `clippy` still cannot be rerun from `apps/guardrail3/Cargo.toml`.
- The next likely `rs/clippy` attack surfaces are:
  - whether `RS-CLIPPY-19` has any false positives against real user-owned Clippy keys that are close to managed keys
  - whether any remaining profile-sensitive rules still derive from policy facts they should instead treat as explicit input failures
- The cargo/clippy ownership boundary should still be rechecked after this checkpoint, but current local reading suggests `RS-CARGO` already owns the raw lint-baseline enforcement for `expect_used`, `unwrap_used`, and `disallowed_macros`.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — family-local Clippy config discovery plus new policy-context failure propagation
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/inputs.rs` — config and policy-context input surfaces
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs` — orchestration order including the new `RS-CLIPPY-23`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_23_policy_context_parseable.rs` — dedicated fail-closed rule for malformed active `guardrail3.toml`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api.rs` — profile-sensitive rule now guarded by policy-context validity
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_17_test_relaxations.rs` — managed test-relaxation policy, including `allow-panic-in-tests`
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — workspace/package publishability fixtures for library-policy tests
- `.plans/todo/checks/rs/clippy.md` — updated family contract and rule inventory
- `.worklogs/2026-03-27-212709-fix-clippy-macro-ban-paths.md` — previous semantic correctness checkpoint before this fail-closed pass

## Next Steps / Continuation Plan
1. Once the unrelated `deny` workspace break is gone, rerun top-level family validation for clippy:
   - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`
   - then the same for `--family clippy`
2. Continue adversarial `rs/clippy` review on the remaining semantic edges:
   - `RS-CLIPPY-19` false-positive sampling against real non-managed Clippy keys
   - cargo/clippy ownership split around raw lint enforcement vs `clippy.toml` semantics
3. After clippy is stable enough, move to the next easy high-leverage family in the queue (`toolchain`/`fmt` handoffs are already prepared; `deny` is in progress elsewhere).
