# Fix deps local path dependency parity

**Date:** 2026-04-05 20:34
**Scope:** packages/g3-deps-content-checks; apps/guardrail3 deps family facts/run/tests

## Summary
Fixed the remaining deps extraction parity gap around local path dependencies. The content package now receives discovered local Cargo path targets when they are available, keeps structural ownership for undeclared local packages under the workspace root, and the deps family now has a bridge test that pins that fail-closed behavior.

## Context & Problem
After wiring `g3-deps-content-checks` into the app, an adversarial attack found two local-path edge cases that were not pinned correctly. First, a local Cargo package under the workspace root but outside `[workspace].members` could double-fire as both a content failure and `RS-DEPS-11`; it should stay owned by the structural fail-closed rule only. Second, path dependencies can point at other local Cargo manifests, and when those manifests are available the content package should use the target crate's real `package.name` rather than the alias.

The package tests already covered some workspace and path behavior, but the local-path identity and fail-closed ownership cases were not fully represented. The app bridge also needed to hand the content package enough parsed-file context to resolve those identities without smuggling reduced policy types.

## Decisions Made

### Local path dependency identity stays file-based
- **Chose:** Extend the deps content package inputs with full parsed local path Cargo manifests plus their repo-relative `Cargo.toml` paths.
- **Why:** This keeps the package on the agreed architecture boundary: parsed files only. The package can resolve alias vs real crate identity only when the app has already discovered and parsed the local target manifests.
- **Alternatives considered:**
  - Passing derived local dependency names — rejected because it hides resolution logic in the app and recreates the same subset-type mistake we already backed out.
  - Leaving path dependencies alias-based — rejected because it breaks package identity semantics whenever a local Cargo target is actually available.

### Structural failures remain structural
- **Chose:** When a local path dependency points to a Cargo package under the workspace root that is not declared in `[workspace].members`, the content package stands down and the app keeps `RS-DEPS-11` as the sole owner.
- **Why:** That situation is an input trust-boundary failure, not a normal allowlist decision. The content package should not emit its own dependency-policy result on top of the structural failure.
- **Alternatives considered:**
  - Emitting both `RS-DEPS-05` and `RS-DEPS-11` — rejected because it duplicates ownership and weakens the fail-closed boundary.
  - Moving the structural error into the content package — rejected because malformed / unreadable / structurally illegal inputs belong in the app orchestrator.

### Family bridge coverage should match observable scope
- **Chose:** Add an app-family bridge test only for the undeclared in-tree local Cargo package case, and keep the external local Cargo package identity case pinned at the package level.
- **Why:** The family bridge can only assert behavior for files actually visible in the selected family view. A sibling manifest outside that selected scope is not a meaningful family-level invariant, but it is still a valid package-level invariant once the manifest is explicitly supplied.
- **Alternatives considered:**
  - Forcing a family-level assertion for an out-of-scope sibling manifest — rejected because it depends on view selection rather than the deps package contract.

## Architectural Notes
The final shape remains consistent with the current extraction rule:
- app deps family owns discovery, route selection, malformed input reporting, and structural failure ownership
- `g3-deps-content-checks` owns pure content checks over parsed files
- the package contract grew only by adding more parsed files that are actually needed for one local assertion

This batch does not migrate any new rule IDs. It hardens the existing `RS-DEPS-05/06/07/08/12` package path and keeps `RS-DEPS-11` as the structural fail-closed rule.

## Information Sources
- Existing deps family resolution logic in `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts/dependency_entries.rs`
- Current content package support logic in `packages/g3-deps-content-checks/crates/runtime/src/support.rs`
- Existing deps bridge tests in `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/policy/rs_deps_11_input_failures/tests/family_bridge.rs`
- Previous worklog `.worklogs/2026-04-05-200711-wire-deps-content-checks.md`

## Open Questions / Future Considerations
- External local Cargo manifests that live outside the selected family view are still not observable at the family layer. If deps routing later grows a broader scoped view for nested roots, that behavior should be revisited deliberately rather than inferred from package tests.
- `g3-deps-content-checks` still has the known oversized-input and sibling-directory complexity debt that was explicitly deferred earlier.

## Key Files for Context
- `packages/g3-deps-content-checks/crates/types/src/input.rs` — current public package contract for deps content checks
- `packages/g3-deps-content-checks/crates/runtime/src/support.rs` — dependency identity resolution, including local path Cargo handling
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts/dependency_entries.rs` — app-side discovery of local path Cargo manifests and structural dependency facts
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/run.rs` — app/package bridge for moved deps rules
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/policy/rs_deps_11_input_failures/tests/family_bridge.rs` — family-level ownership tests for migrated deps rules
- `.worklogs/2026-04-05-200711-wire-deps-content-checks.md` — prior deps package wiring worklog

## Next Steps / Continuation Plan
1. If more deps parity hardening is needed, add bridge coverage for `RS-DEPS-06`, `RS-DEPS-07`, and `RS-DEPS-12` through the family run path, not just package tests.
2. When returning to extraction sequencing, move on to the next planned content-check family (`garde`) unless the user explicitly wants more deps cleanup first.
3. If nested-root deps validation becomes important, decide whether the family view should include sibling manifests outside the selected root before adding any new assertions around that case.
