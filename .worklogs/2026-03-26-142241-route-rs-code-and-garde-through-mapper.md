# Route Rs Code And Garde Through Mapper

**Date:** 2026-03-26 14:22
**Scope:** `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/lib.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`, `apps/guardrail3/crates/app/rs/families/code/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/code/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/code/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/code/src/discover.rs`, `apps/guardrail3/crates/app/rs/families/code/src/test_support.rs`, `apps/guardrail3/crates/app/rs/families/code/src/rs_code_30_input_failures_tests/*`, `apps/guardrail3/crates/app/rs/families/garde/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/garde/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/garde/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/garde/src/test_support.rs`, `apps/guardrail3/crates/app/rs/families/garde/src/garde_facts_tests.rs`, `apps/guardrail3/crates/app/rs/families/garde/src/rs_garde_*_tests/*`, `apps/guardrail3/Cargo.lock`

## Summary
Moved `rs/code` and `rs/garde` onto typed mapper routes and stopped passing raw `scoped_files` from `runtime.rs` into either family. Both families now take routed roots plus mapper-resolved scoped files, and their tests build routes through the same placement + mapper path instead of keeping a shadow local entrypoint.

## Context & Problem
The Rust architecture plan in `apps/guardrail3/crates/app/rs/README.md` says shared root scope should be collected once, mapped into family-specific inputs, and then consumed by family-local orchestrators. After routing `arch`, `test`, and `hexarch`, `code` and `garde` were still on the old path:

- `runtime.rs` passed raw `scoped_files`
- the families crawled `Cargo.toml` roots directly from `ProjectTree`
- file/config ownership still came from “everything in the tree” instead of mapped roots

That meant the mapper was not actually the family-input boundary for source-check families.

## Decisions Made

### Add one shared routed source-root shape for code and garde
- **Chose:** Introduce `RsScopedRootView` + `RsScopedSourceRoute`, exported as `RsCodeRoute` and `RsGardeRoute`.
- **Why:** Both families need the same high-level input: routed Rust roots, classification metadata, and mapper-filtered staged-file scope. A shared route shape avoids inventing two near-identical family-specific structs.
- **Alternatives considered:**
  - Separate `RsCodeRoute` and `RsGardeRoute` structs — rejected because they would immediately duplicate fields and mapper code.
  - Reuse plain `RsRootView` plus raw `scoped_files` — rejected because that leaves the family boundary underspecified and keeps staged-file routing half inside `runtime.rs`.

### Keep family-local normalization, but route-driven root ownership
- **Chose:** Preserve each family’s own parsing and fact-building logic, but replace `dirs_with_file("Cargo.toml")` discovery with iteration over routed roots.
- **Why:** The families still need to parse manifests, config files, and Rust source for their own semantics. What had to move out was deciding which roots are in scope.
- **Alternatives considered:**
  - Push more family semantics into the mapper — rejected because mapper should remain a projection layer, not become a second orchestrator.
  - Leave local Cargo crawling in place and only change runtime signatures — rejected because it preserves the old bug class under a new API.

### Bind code-family files and config comments to routed roots
- **Chose:** `rs/code` now filters Rust files and exception-comment configs to those owned by routed roots before building facts.
- **Why:** Without this, switching to routed roots would still leave `code` semantically scoped to “all files in the tree,” which is the old ownership model in disguise.
- **Alternatives considered:**
  - Only change the family signature and keep all-file scanning — rejected because it would make the route input cosmetic rather than authoritative.

### Preserve garde’s config gating while removing root crawling
- **Chose:** Restore `garde`’s policy-map gating against `guardrail3.toml`, but key it off routed roots instead of rediscovered roots.
- **Why:** A direct route cut initially regressed a test where `[rust.packages.checks].garde = false` should suppress the root workspace. The regression showed that route migration cannot drop existing family policy behavior.
- **Alternatives considered:**
  - Let mapper-only family enablement replace garde’s internal policy map — rejected because the current mapper does not encode garde’s root-local policy semantics precisely enough yet.
  - Delete the failing test as obsolete — rejected because the behavior is still part of the family contract.

## Architectural Notes
This slice changes the source-check path to:

`ProjectTree -> placement -> FamilyMapper -> RsCodeRoute / RsGardeRoute -> family facts -> rule inputs`

What moved out of the families:
- live Cargo-root discovery
- raw staged-file filtering at the runtime boundary

What stayed inside the families:
- manifest parsing
- policy/config interpretation
- Rust source parsing
- per-rule fan-out

The current mapper still applies family/root enablement policy, and `runtime.rs` still keeps post-family applicability filtering. This commit improves the family-input boundary, but the broader runtime/mapper policy split is still not fully cleaned up.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md` — target architecture for placement, family selection, and mapper responsibilities
- `apps/guardrail3/crates/app/rs/runtime.rs` — current Rust orchestration entrypoint
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` and `rs.rs` — typed route layer
- `apps/guardrail3/crates/app/rs/families/code/src/facts.rs` — prior local Cargo-root crawl and policy map
- `apps/guardrail3/crates/app/rs/families/garde/src/facts.rs` — prior local Cargo-root crawl, policy map, and active-root derivation
- `.worklogs/2026-03-26-141017-route-rs-hexarch-through-mapper.md` — previous routed-family checkpoint

## Open Questions / Future Considerations
- `cargo` and `release` still own Cargo-root discovery locally.
- `runtime.rs` still applies a second applicability filter after families run.
- `FamilyMapper` still owns policy decisions that the README says should move before mapping.
- `code` and `garde` tests now go through mapper-built routes, but the mapper itself still lacks dedicated contract tests.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — current target architecture
- `apps/guardrail3/crates/app/rs/runtime.rs` — dispatch layer still carrying residual applicability logic
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — routed family input shapes
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — route construction for Rust families
- `apps/guardrail3/crates/app/rs/families/code/src/lib.rs` — routed code family entrypoint
- `apps/guardrail3/crates/app/rs/families/code/src/facts.rs` — code-family route-aware ownership and fact collection
- `apps/guardrail3/crates/app/rs/families/garde/src/lib.rs` — routed garde family entrypoint
- `apps/guardrail3/crates/app/rs/families/garde/src/facts.rs` — garde route-aware root selection and policy gating
- `.worklogs/2026-03-26-141017-route-rs-hexarch-through-mapper.md` — prior routed-family migration context

## Next Steps / Continuation Plan
1. Migrate `rs/cargo` onto a typed mapper route so policy-root families stop crawling `Cargo.toml` roots locally.
2. Migrate `rs/release` onto a routed input shape, reusing the same shared-scope boundary rather than rediscovering publishable crates from `ProjectTree`.
3. After the remaining Cargo-root families are migrated, remove the post-hoc applicability filter from `runtime.rs` and move the residual policy decisions out of `FamilyMapper`.
4. Add dedicated tests under `family_mapper` and `family_selection` that prove root routing, staged-file filtering, and family agreement without relying on `runtime.rs`.
