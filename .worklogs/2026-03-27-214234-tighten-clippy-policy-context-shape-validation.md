# Tighten Clippy Policy Context Shape Validation

**Date:** 2026-03-27 21:42
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/{facts.rs,rs_clippy_16_avoid_breaking_exported_api_tests/malformed_policy_context.rs,rs_clippy_23_policy_context_parseable_tests/*}`, `apps/guardrail3/crates/domain/modules/clippy/{mod.rs,settings.rs}`

## Summary
Closed another `RS-CLIPPY` fail-open path. `RS-CLIPPY-23` now treats semantically invalid active `guardrail3.toml` fields as policy-context failures instead of silently falling back to defaults, and the domain Clippy settings export now includes `allow-panic-in-tests` so the canonical generator and runtime expectations stay aligned.

## Context & Problem
After the previous checkpoint, `RS-CLIPPY-23` correctly reported syntax-level parse failures and missing `guardrail3.toml` content. But the shape contract was still too weak:

- `guardrail3.toml` with invalid TOML syntax failed closed
- `guardrail3.toml` with wrong field types, such as `profile.name = 7`, parsed successfully and then silently collapsed to default profile/garde behavior

That is still malformed active policy context. It meant profile-aware rules like `RS-CLIPPY-16` could quietly lose their real inputs without `RS-CLIPPY-23` taking ownership.

While checking the runtime expectations, I also found that `allow-panic-in-tests` had already become a managed runtime/config key, but the domain settings export was still incomplete in the index. That needed to be committed with the same slice so the canonical generator and runtime helpers stayed consistent.

## Decisions Made

### Treat wrong-type policy fields as malformed active context
- **Chose:** Validate the subset of `guardrail3.toml` that `RS-CLIPPY` actually consumes:
  - `profile.name`
  - `rust.checks.garde`
  - `rust.apps.*.{type|profile}`
  - `rust.apps.*.checks.garde`
  - `rust.packages.{type|profile}`
  - `rust.packages.checks.garde`
- **Why:** These are the fields that determine Clippy profile and garde policy. If they are the wrong type, the family no longer has valid policy context and should fail closed under `RS-CLIPPY-23`.
- **Alternatives considered:**
  - Keep treating only TOML syntax errors as malformed — rejected because it leaves a real semantic fail-open path.
  - Add a full schema validator for all of `guardrail3.toml` here — rejected because the family should validate only the subset it actually depends on.

### Make dependent rules defer cleanly to `RS-CLIPPY-23`
- **Chose:** Keep the profile-aware rules quiet when `policy_context_parse_error` is present, and add regression coverage proving `RS-CLIPPY-16` stays silent on wrong-type policy context.
- **Why:** Once `RS-CLIPPY-23` owns active-policy failure, downstream rules should not pile on misleading fallback-based diagnostics.
- **Alternatives considered:**
  - Let downstream rules continue to run on defaulted values — rejected because that hides the real problem and creates noisy false interpretations.

### Commit the `allow-panic-in-tests` domain constant export with this slice
- **Chose:** Include the already-prepared domain module updates that add `ALLOW_PANIC_IN_TESTS` to the exported settings surface.
- **Why:** Runtime `g3rs-clippy/avoid-breaking-exported-api` and local-baseline checks already depend on this managed key. Leaving the domain settings export behind would keep the generated baseline and runtime expectations out of sync.
- **Alternatives considered:**
  - Leave the domain-module staging for a later commit — rejected because this slice is about policy-context correctness, and the managed test-relaxation setting is part of that same contract.

## Architectural Notes
- `RS-CLIPPY-23` now owns “invalid active policy context” in the semantic sense, not just “bad TOML bytes.”
- The family still validates only its own consumed subset of `guardrail3.toml`; it does not try to become the repo-wide schema checker.
- `facts.rs` remains the single place where active Clippy profile/garde policy is resolved from `guardrail3.toml`, so this validation belongs there.

## Information Sources
- Current family contract:
  - `apps/guardrail3/crates/app/rs/families/clippy/README.md`
  - `.plans/todo/checks/rs/clippy.md`
- Prior checkpoint:
  - `.worklogs/2026-03-27-213841-harden-clippy-policy-context-and-package-workspaces.md`
- Key implementation files:
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_23_policy_context_parseable.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api.rs`
  - `apps/guardrail3/crates/domain/modules/clippy/settings.rs`
- Verification commands:
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib errors_when_guardrail_policy_context_has_invalid_types`
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib yields_no_result_when_policy_context_shape_is_invalid`
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`

## Open Questions / Future Considerations
- The typo-heuristic rule `RS-CLIPPY-19` still looks sound against the current Clippy config key set, but it remains heuristic and should be rechecked if the managed key set expands again.
- The outer workspace is still blocked by unrelated `deny` work, so top-level `guardrail3 rs validate ...` remains deferred.
- There may still be contract-noise overlap between `g3rs-clippy/local-policy-root` and the specific boolean rules, but no concrete detector bug was found in that overlap during this slice.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — active policy-context parsing, validation, and profile/garde resolution
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_23_policy_context_parseable.rs` — dedicated policy-context failure rule
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_23_policy_context_parseable_tests/shape_error.rs` — regression for wrong-type active policy fields
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api_tests/malformed_policy_context.rs` — proves downstream rule ownership stays quiet when `23` should own the failure
- `apps/guardrail3/crates/domain/modules/clippy/settings.rs` — canonical managed test-relaxation settings including `allow-panic-in-tests`
- `.worklogs/2026-03-27-213841-harden-clippy-policy-context-and-package-workspaces.md` — previous clippy policy-context checkpoint

## Next Steps / Continuation Plan
1. Continue the adversarial `RS-CLIPPY` review with focus on remaining heuristic surfaces:
   - `RS-CLIPPY-19` against current upstream key inventory
   - any overlap/noise between `g3rs-clippy/local-policy-root` and `RS-CLIPPY-16/17`
2. Keep using the nested clippy workspace for verification until the outer workspace is healthy again:
   - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
3. After the `deny` migration stops breaking the top-level workspace, rerun family-level validator checks from `apps/guardrail3` and record the actual `RS-TEST` / `RS-CLIPPY` status there.
