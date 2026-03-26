# Route Rs Release, Clippy, Deny, And Deps Through Mapper

**Date:** 2026-03-26 14:48
**Scope:** `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/{lib.rs,rs.rs,views.rs}`, `apps/guardrail3/crates/app/rs/families/{release,clippy,deny,deps}/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/{release,clippy,deny,deps}/src/*`, `apps/guardrail3/Cargo.lock`

## Summary
Moved `rs/release`, `rs/clippy`, `rs/deny`, and `rs/deps` onto typed mapper routes so those families no longer decide their own live Cargo-root universe by crawling `ProjectTree`. The remaining root-selection boundary now lives in shared placement plus `FamilyMapper`, while the families keep only their own parsing and rule fan-out logic.

## Context & Problem
The `apps/guardrail3/crates/app/rs/README.md` plan says Rust root scope should be collected once, selected once, and mapped into typed family inputs before family-local discovery runs. Earlier slices had already routed `arch`, `test`, `hexarch`, `code`, `garde`, and `cargo`, but several families still reimplemented the same bug class internally:

- `release` locally collected Cargo roots before deriving publishable crates
- `clippy` locally collected Cargo roots before deciding workspace and standalone policy roots
- `deny` locally collected Cargo roots before deciding validation/workspace/package policy roots
- `deps` locally collected Cargo roots twice, once for workspaces and once for standalone members

That kept the family-input boundary inconsistent and meant root-discovery drift could still happen family by family.

## Decisions Made

### Route the remaining root-owning families through the existing mapper layer
- **Chose:** Added mapper outputs for `release`, `clippy`, `deny`, and `deps`, and changed each family entrypoint to accept a typed route instead of discovering roots itself.
- **Why:** The architecture already had the right shared scope pieces (`placement`, `family_selection`, `FamilyMapper`). Extending that path to the remaining crawler families removes duplicated root ownership logic without rewriting family semantics.
- **Alternatives considered:**
  - Leave these families on local crawling until a larger orchestrator rewrite — rejected because it preserves the exact drift the README is trying to eliminate.
  - Push more family parsing into placement or mapper — rejected because that would move family semantics out of the family instead of just fixing ownership boundaries.

### Keep route shapes minimal and root-only
- **Chose:** Used root-only routes for these families, reusing the same route class pattern already used by `cargo` and `release`.
- **Why:** None of these families needed staged files, overlaps, or extra mapper metadata. They only needed the set of routed Cargo roots they are allowed to reason about.
- **Alternatives considered:**
  - Create richer family-specific route structs for each family — rejected because it adds type noise without giving the family any new authority or needed data.
  - Reuse raw placement facts directly — rejected because that weakens the mapper boundary.

### Make config-file discovery respect routed ownership, not just root parsing
- **Chose:** For `clippy` and `deny`, restricted config scanning to files that live under routed root trees, and stopped implicitly treating `""` as always in scope.
- **Why:** Replacing `collect_cargo_roots(...)` alone would have been incomplete. Both families also scanned config files across the entire tree, which would have let unrouted roots leak back into the family through a side door.
- **Alternatives considered:**
  - Leave config scanning global and only route Cargo roots — rejected because it makes the mapper cut cosmetic.
  - Restrict config scanning only to exact routed roots, not descendants — rejected because it would miss the forbidden nested config cases these families intentionally detect.

### Preserve family tests by routing test helpers without enabling mapper-side policy filtering
- **Chose:** Updated family test helpers to build routes through `placement + FamilyMapper`, but passed `config = None` for those helpers.
- **Why:** These family tests are asserting family semantics, not end-to-end runtime enablement policy. Using the mapper while ignoring config keeps the new boundary under test without accidentally routing out fixture roots due to `guardrail3.toml`.
- **Alternatives considered:**
  - Make family tests honor mapper-side config gating — rejected because it silently changes the meaning of existing family fixtures.
  - Keep a hidden direct raw-tree test-only entrypoint — rejected because it preserves the old boundary in another code path.

## Architectural Notes
This commit extends the live path to:

`ProjectTree -> placement -> family_selection -> FamilyMapper -> typed route -> family facts -> rule inputs`

For the migrated families:
- `release` still owns repo-level release files, workflow parsing, README reads, and tool checks
- `clippy` still owns policy/profile interpretation from `guardrail3.toml` and config parsing
- `deny` still owns deny-config precedence and profile interpretation
- `deps` still owns workspace member-pattern validation, allowlist policy resolution, and dependency parsing

What moved out:
- deciding which Cargo roots exist for those families
- deciding which Cargo roots are in scope for those families

After this slice, the only remaining `Cargo.toml` directory scan inside `apps/guardrail3/crates/app/rs/families` is in `hexarch/src/dependency_facts.rs`, where it is still being used for inner dependency inference rather than top-level family root routing. The larger remaining architecture mismatch is now the policy split between `family_selection`, `FamilyMapper`, and `runtime.rs`, especially the extra applicability filtering in `runtime.rs`.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md` — target shared-scope and mapper architecture
- `apps/guardrail3/crates/app/rs/runtime.rs` — current Rust runtime dispatch and residual applicability logic
- `apps/guardrail3/crates/app/rs/family_mapper/src/{lib.rs,rs.rs,views.rs}` — routed family input layer
- `apps/guardrail3/crates/app/rs/families/release/src/{lib.rs,facts.rs,test_support.rs}` — prior local root discovery in release
- `apps/guardrail3/crates/app/rs/families/clippy/src/{lib.rs,facts.rs,test_support.rs}` — prior local root and config discovery in clippy
- `apps/guardrail3/crates/app/rs/families/deny/src/{lib.rs,facts.rs,test_support.rs}` — prior local root and config discovery in deny
- `apps/guardrail3/crates/app/rs/families/deps/src/{lib.rs,facts.rs,test_support.rs}` — prior local root discovery in deps
- `.worklogs/2026-03-26-142843-route-rs-cargo-through-mapper.md` — previous routed Cargo-root family checkpoint

## Open Questions / Future Considerations
- `runtime.rs` still owns a second applicability filter after families run, which means family enablement policy is not yet fully single-sourced.
- `FamilyMapper` still computes family/root enablement and the arch reporting flag, which the README says should eventually move out of the mapper.
- `hexarch` still scans `Cargo.toml` paths internally for dependency context. That may be acceptable as inner discovery, but it should be reviewed against the updated “family input vs inner semantics” boundary so the exception is explicit rather than accidental.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — architecture contract for shared scope, selection, and mapper responsibilities
- `apps/guardrail3/crates/app/rs/runtime.rs` — current runtime entrypoint and remaining applicability layer
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — route construction for all currently migrated Rust families
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — routed family input shapes and aliases
- `apps/guardrail3/crates/app/rs/families/release/src/facts.rs` — route-driven release-family fact collection
- `apps/guardrail3/crates/app/rs/families/clippy/src/facts.rs` — routed clippy-family root and config ownership
- `apps/guardrail3/crates/app/rs/families/deny/src/facts.rs` — routed deny-family root and config ownership
- `apps/guardrail3/crates/app/rs/families/deps/src/facts.rs` — routed deps-family root ownership with local member-pattern validation retained
- `.worklogs/2026-03-26-142843-route-rs-cargo-through-mapper.md` — previous routed family migration context

## Next Steps / Continuation Plan
1. Re-read `apps/guardrail3/crates/app/rs/runtime.rs`, `family_selection/src/selection.rs`, and `family_mapper/src/rs.rs` together and decide the final home for per-family/per-root enablement policy.
2. Remove the extra result-side applicability filter from `runtime.rs` once the pre-family selection/mapping path is authoritative.
3. Review `apps/guardrail3/crates/app/rs/families/hexarch/src/dependency_facts.rs` and explicitly decide whether its remaining `Cargo.toml` scan is valid inner discovery or another root-ownership leak.
4. Add dedicated mapper contract tests so routed-root ownership is proven directly instead of only through family tests.
