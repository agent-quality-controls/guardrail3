# Harden Clippy Policy Context And Package Workspaces

**Date:** 2026-03-27 21:38
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/**`, `.plans/todo/checks/rs/clippy.md`

## Summary
Continued the adversarial `RS-CLIPPY` hardening pass instead of repo cleanup. This checkpoint does two things: it makes malformed active `guardrail3.toml` a first-class `RS-CLIPPY-23` failure, and it fixes a real profile-resolution gap where `rust.packages` policy was not being applied to package workspace roots, which weakened library-profile clippy checks on those roots.

## Context & Problem
The current task was to keep attacking the `clippy` family itself, not to reduce repo findings. Earlier work had already fixed root-policy coverage, type-ban ownership overlap, and macro-ban path correctness, but the family still had two architectural holes:

1. `guardrail3.toml` parse failures influenced active Clippy profile/garde policy, yet there was no dedicated rule surfacing that failure. Rules like `16` and `17` could quietly degrade to default policy assumptions instead of producing a clear policy-context failure.
2. `read_policy_map()` applied `rust.packages` only to the validation root and standalone package roots. Package workspace roots under `packages/*` still inherited the default profile instead of the package policy, which created a false-green path for library-only checks such as `RS-CLIPPY-14` and local-baseline checks such as `RS-CLIPPY-13`.

This work had to stay inside the nested `clippy` workspace because the outer `apps/guardrail3` workspace is still temporarily broken by the unrelated in-flight `deny` migration.

## Decisions Made

### Add a dedicated policy-context parse rule instead of silently folding parse failure into unrelated rules
- **Chose:** Introduce `RS-CLIPPY-23` as an explicit error when the active `guardrail3.toml` used for Clippy policy resolution is unreadable or malformed.
- **Why:** The family needs a clear fail-closed result for policy-context failure. Letting `facts.rs` fall back to defaults without a dedicated diagnostic would hide the real reason downstream profile-aware rules changed behavior.
- **Alternatives considered:**
  - Let threshold/profile rules fail closed individually on malformed `clippy.toml` only — rejected because that does not surface malformed `guardrail3.toml`, which drives profile and garde policy.
  - Treat malformed `guardrail3.toml` as out of scope for `RS-CLIPPY` — rejected because the family explicitly depends on that file for active policy context.

### Model “published library” at the policy-root level, not just the local package root
- **Chose:** Replace the old `package_publishable` signal with `published_library_policy`, computed from the routed root plus any publishable member crates beneath a workspace policy root.
- **Why:** `RS-CLIPPY-16` is about whether the active policy root is governing published library code. A library workspace with publishable member crates is a legitimate place to informationally tolerate `avoid-breaking-exported-api = true`, even though the workspace root itself has no `[package]`.
- **Alternatives considered:**
  - Keep using only the local root’s `[package.publish]` state — rejected because it misses published library workspaces and misclassifies their policy roots.
  - Drop the published-library exception entirely and always warn on `true` — rejected because it weakens the documented library policy nuance.

### Apply `rust.packages` policy to non-app workspace roots as well as standalone package roots
- **Chose:** Extend `read_policy_map()` so `rust.packages` is assigned to workspace roots that are not app roots, in addition to standalone package roots and the validation root.
- **Why:** Package workspaces are still package policy roots. Without this, local `packages/*/clippy.toml` files used the default profile and could false-green under library-only rules.
- **Alternatives considered:**
  - Keep package policy limited to standalone package roots — rejected because it produced a concrete false green on package workspaces.
  - Push package-workspace classification into `FamilyMapper` — rejected because this is still family-local policy resolution inside already-routed roots.

## Architectural Notes
- `placement` and `FamilyMapper` still own root scope and routing. This checkpoint stays inside family-local Clippy policy discovery.
- `facts.rs` now distinguishes two failure classes:
  - parse failure of the `clippy.toml` being checked
  - parse failure of active `guardrail3.toml` policy context
- `RS-CLIPPY-23` owns the second class so profile-aware rules do not need to duplicate it.
- Package workspace roots now inherit `rust.packages` consistently, which keeps `RS-CLIPPY-13`, `14`, and `16` aligned on the same resolved profile/garde context.

## Information Sources
- Family contract:
  - `apps/guardrail3/crates/app/rs/families/clippy/README.md`
  - `.plans/todo/checks/rs/clippy.md`
- Recent clippy worklogs:
  - `.worklogs/2026-03-27-210812-finish-clippy-sidecar-extraction.md`
  - `.worklogs/2026-03-27-211613-fix-clippy-policy-root-gaps.md`
  - `.worklogs/2026-03-27-212201-split-clippy-library-type-ban-ownership.md`
  - `.worklogs/2026-03-27-212709-fix-clippy-macro-ban-paths.md`
- Official Clippy config docs:
  - `https://doc.rust-lang.org/clippy/lint_configuration.html`
- Local tool probes:
  - temporary `cargo clippy` runs verifying `allow-expect-in-tests`, `allow-unwrap-in-tests`, and `avoid-breaking-exported-api` semantics
- Verification commands:
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib package_workspace_root_uses_rust_packages_profile_policy`
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib package_workspace_root_uses_rust_packages_library_profile`
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`

## Open Questions / Future Considerations
- The outer workspace is still broken by unrelated `deny` work, so top-level `guardrail3 rs validate ... --family test` for `clippy` still cannot be rerun from `apps/guardrail3`.
- `RS-CLIPPY-19` currently looks safe against the live Clippy key set, but it still depends on edit-distance heuristics. If the managed key set grows, parity against real Clippy top-level keys should be rechecked.
- There may still be rule-overlap noise between `RS-CLIPPY-13` and per-key rules (`16/17`) on incomplete local policy roots, but that is a contract question rather than a discovered detector bug.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — profile/garde resolution, package-workspace handling, and policy-context parse facts
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs` — `RS-CLIPPY-23` wiring and family orchestration
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_23_policy_context_parseable.rs` — dedicated policy-context fail-closed rule
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_14_library_global_state.rs` — rule affected by package-workspace profile resolution
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — fixtures for published-library workspaces and package workspaces
- `.plans/todo/checks/rs/clippy.md` — updated rule inventory including `RS-CLIPPY-23`
- `.worklogs/2026-03-27-211613-fix-clippy-policy-root-gaps.md` — previous policy-resolution fix that this builds on

## Next Steps / Continuation Plan
1. Once this checkpoint is committed, continue the adversarial pass on the remaining `RS-CLIPPY` rule surface, with emphasis on:
   - `RS-CLIPPY-19` typo heuristics against real Clippy keys
   - overlap/noise between `RS-CLIPPY-13` and `RS-CLIPPY-16/17`
   - any remaining fail-open path on malformed active policy context
2. Keep the work inside the nested `clippy` workspace until the outer workspace is healthy again. Use:
   - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
3. After the unrelated `deny` workspace conflict is resolved, rerun top-level validation:
   - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`
   - then run the same for `--family clippy` to confirm the family still self-hosts cleanly under the stricter semantics.
