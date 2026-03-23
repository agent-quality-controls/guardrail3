# Tighten RS-DEPS Lockfile Scope

**Date:** 2026-03-23 11:06
**Scope:** `.plans/todo/checks/rs/deps.md`, `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/rs/checks/rs/deps/*`, `apps/guardrail3/tests/unit/project_walker_test.rs`

## Summary
Refined `rs/deps` after the first adversarial audit by moving lockfile and `.gitignore` policy from a single repo-root check to per-Rust-root checks. Also fixed the real walker/input gap by caching `.gitignore` files and added direct tests for nested lockfile masking and `workspace = true` extraction in build/dev dependency sections.

## Context & Problem
The first `rs/deps` implementation was structurally complete, but the adversarial review showed the weakest area was lockfile policy. `RS-DEPS-09` and `RS-DEPS-10` were only looking at the repo root, which under-checked nested Rust workspaces and packages. There was also a hidden integration bug: `.gitignore` content was not cached by the real project walker, so `RS-DEPS-10` only really worked in synthetic `ProjectTree` tests.

The user explicitly wanted a real implementation rather than a breadth-first half-step, so the family needed to enforce the intended scope correctly before moving on to the next family.

## Decisions Made

### Make Lockfile Policy Per Rust Root
- **Chose:** Reworked `LockfileFacts` from one root-level fact into a collection of per-Rust-root facts.
- **Why:** Nested Rust roots are independent validation boundaries. A single repo-root `Cargo.lock` check under-enforced apps/workspaces below the repo root.
- **Alternatives considered:**
  - Keep the repo-root-only model — rejected because it would silently miss missing nested lockfiles.
  - Move lockfile policy to another family — rejected because dependency reproducibility still belongs in `RS-DEPS`.

### Cache `.gitignore` in the Real Walker
- **Chose:** Added `.gitignore` to the cached config file set in `project_walker.rs`.
- **Why:** `RS-DEPS-10` depends on `.gitignore` content. Without caching, the rule only worked against hand-built trees and failed open in real walked projects.
- **Alternatives considered:**
  - Read `.gitignore` directly from filesystem in `rs/deps` — rejected because family code should not bypass `ProjectTree`.
  - Keep `.gitignore` root-only and synthetic-only — rejected because that would hide real lockfile masking.

### Check Ancestor `.gitignore` Files Relative to Each Rust Root
- **Chose:** Added per-root `.gitignore` evaluation using ancestor `.gitignore` paths and root-relative pattern matching.
- **Why:** A Rust root can be masked by its own `.gitignore` or an ancestor `.gitignore`. Root-only matching was too narrow and path-insensitive.
- **Alternatives considered:**
  - Fully implement all gitignore semantics — rejected for now; the current model is strong enough for this phase and materially better than the root-only check.
  - Inspect only the nearest `.gitignore` — rejected because ancestor ignores can still mask nested lockfiles.

### Add Missing Collected-Facts Tests For Build/Dev `workspace = true`
- **Chose:** Added real `ProjectTree`-based extraction tests for `workspace = true` in `[build-dependencies]` and `[dev-dependencies]`.
- **Why:** The runtime dependency rule already had this coverage, but the build/dev rules were only tested with synthetic prebuilt facts. That was enough to miss extraction regressions.
- **Alternatives considered:**
  - Leave build/dev tests as synthetic only — rejected because the shared extractor is exactly where cross-section drift would happen.

## Architectural Notes
This pass did not change the overall `rs/deps` family structure:
- `facts.rs` still owns discovery/policy normalization
- `mod.rs` still fans out minimal rule inputs
- each rule remains one production file with one rule-specific test file

The main architectural change is that `LockfileFacts` is now pluralized in `DepsFacts`:
- one fact per Rust root
- each fact carries:
  - root relative dir
  - specific `Cargo.lock` path
  - relevant `.gitignore` path if any
  - profile name for severity selection

That aligns `RS-DEPS-09/10` with the same per-root philosophy already used in other families.

## Information Sources
- `.plans/todo/checks/rs/deps.md` — intended `RS-DEPS-09/10` semantics
- `apps/guardrail3/crates/app/core/project_walker.rs` — actual walker caching behavior
- `apps/guardrail3/crates/domain/project_tree.rs` — available cached-content access model
- `apps/guardrail3/crates/app/rs/checks/rs/deps/facts.rs` — existing dependency and lockfile facts
- `apps/guardrail3/tests/unit/project_walker_test.rs` — walker regression test location
- `.worklogs/2026-03-23-104643-complete-deps-family.md` — previous `rs/deps` implementation checkpoint

## Open Questions / Future Considerations
- `RS-DEPS-11` is still broader than the narrow prose wording in the plan. It is fail-closed and still a single concern, but the plan may need another wording update if that breadth is kept intentionally.
- The `.gitignore` matching is materially better now but still not a full gitignore engine. If later audits find pattern-class bypasses, that logic should be tightened further.

## Key Files for Context
- `.plans/todo/checks/rs/deps.md` — canonical `RS-DEPS` contract after the lockfile-scope clarification
- `apps/guardrail3/crates/app/rs/checks/rs/deps/facts.rs` — per-root dependency and lockfile fact collection
- `apps/guardrail3/crates/app/rs/checks/rs/deps/rs_deps_09_cargo_lock_present.rs` — per-root lockfile severity rule
- `apps/guardrail3/crates/app/rs/checks/rs/deps/rs_deps_10_gitignore_not_ignoring_cargo_lock.rs` — per-root `.gitignore` masking rule
- `apps/guardrail3/crates/app/core/project_walker.rs` — now caches `.gitignore`
- `apps/guardrail3/tests/unit/project_walker_test.rs` — regression coverage for `.gitignore` caching
- `.worklogs/2026-03-23-104643-complete-deps-family.md` — original `rs/deps` implementation context

## Next Steps / Continuation Plan
1. Commit this `rs/deps` tightening pass and keep `deps` as the completed reference for policy-root lockfile handling.
2. Move to `rs/garde` next: read its plan, old validation code, and any old tests before implementing anything.
3. Reconcile `rs/garde` semantics first, then implement the family with the same strict file/test structure used for `deps`.
