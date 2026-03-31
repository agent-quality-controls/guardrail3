# Mapper Family Consumption Hardening

**Date:** 2026-03-31 07:38
**Scope:** `apps/guardrail3/crates/app/rs/ownership`, `apps/guardrail3/crates/app/rs/family_mapper`, Rust family runtimes/tests for `toolchain`, `cargo`, `clippy`, `deny`, `deps`, and `release`

## Summary
Finished the missing half of the shared ownership-routing migration: the affected workspace-local families now actually consume the routed Cargo/family-file inventories instead of silently falling back to old root-only assumptions. The work also added adversarial tests and lean binary attacks so the new route contract is proven at runtime, not just in mapper unit tests.

## Context & Problem
The prior shared-ownership pass had wired `ownership` and `FamilyMapper`, but the family consumers were still inconsistent. Some families only consumed a subset of the routed file surface, some rederived legal roots from the old mapper filtering contract, and some rescanned raw tree shape even though the routed Cargo/family-file inventory already existed.

That left a real gap between the intended architecture and the running code:
- the mapper could discover and attach files correctly
- but the families were not all judging those attached files against the workspace topology they received
- so a green test suite could still hide “data structure exists but nobody enforces it” bugs

The user’s correction for this pass was the right one: local families must actually compare all relevant Cargo manifests and family-owned files against the routed workspace surface, rather than waiting for `arch` to hide bad shapes for them.

## Decisions Made

### Broadened mapper routes to expose all relevant local-family surfaces
- **Chose:** route workspace-local families through all discovered candidate roots plus all family-relevant files, instead of pre-filtering the roots down to only workspace-policy roots inside `FamilyMapper`.
- **Why:** if the mapper only passes “already accepted” roots, misplaced or nested family-owned files become invisible and the family never gets a chance to judge them.
- **Alternatives considered:**
  - Keep mapper-level workspace-root filtering and teach each family about “special misplaced candidates” separately — rejected because it recreates ad hoc blind spots.
  - Pass the full `ProjectTree` and let every family rediscover everything — rejected because it defeats the shared discovery contract.

### Moved legal-root selection back into the families that own the policy
- **Chose:** let `clippy` and `deny` compute top-level allowed workspace policy roots from the routed Cargo surface instead of trusting mapper root filtering.
- **Why:** once the mapper exposes all candidate roots, the family must distinguish “visible” from “allowed”; otherwise every nested workspace root turns into an allowed local policy root.
- **Alternatives considered:**
  - Reintroduce mapper-level workspace-policy filtering just for those families — rejected because it would hide the same misplaced surfaces again.

### Stopped `release` from rescanning the tree for Cargo manifests under routed roots
- **Chose:** collect release Cargo roots from routed `Cargo.toml` file attachments rather than walking directory structure under each routed root.
- **Why:** the routed file inventory is already the source of truth for which Cargo manifests are in scope. Rescanning the raw tree weakened the contract and could drift from mapper behavior.
- **Alternatives considered:**
  - Keep the raw tree scan and rely on tests to keep it “close enough” — rejected because it preserves exactly the architectural leak the user flagged.

### Hardened `cargo` against undeclared nested packages without mixing in the standalone-package policy decision
- **Chose:** keep current standalone-package linting behavior for now, but add explicit `RS-CARGO-14` failures when a live `Cargo.toml` sits under a workspace root without being declared in `[workspace].members`.
- **Why:** this addresses the immediate ownership/judgment gap without silently changing the broader standalone-package contract in the same patch.
- **Alternatives considered:**
  - Ban standalone-package policy roots entirely in the same change — rejected for this pass because it would have mixed two policy migrations and obscured whether the new route consumption was correct.
  - Ignore nested undeclared manifests until `arch` bans them — rejected because the family is supposed to judge visible Cargo surfaces now, not only after `arch` catches up.

### Made `toolchain` consume routed Cargo ownership facts as part of its runtime contract
- **Chose:** include `Cargo.toml` in `toolchain` ownership discovery and use the routed Cargo facts when collecting toolchain policy roots/snapshots.
- **Why:** the family compares toolchain files against workspace policy roots, so it should receive the same routed Cargo ownership facts as the other workspace-local config families.
- **Alternatives considered:**
  - Keep `toolchain` as a toolchain-file-only route — rejected because it would leave it as the odd family out and hide the workspace/file comparison inside family-local path assumptions.

## Architectural Notes
- `ownership/src/discover.rs` now emits `Cargo.toml` facts for `toolchain` in addition to the other workspace-local families.
- `FamilyMapper` now exposes the full candidate surface to the local families instead of doing early workspace-policy-root pruning.
- `clippy` and `deny` now compute top-level allowed workspace policy roots locally from the routed Cargo facts, which keeps bad nested roots visible but forbidden.
- `release` now treats routed Cargo attachments as authoritative input for release Cargo-root discovery.
- `cargo` now surfaces undeclared nested package manifests through `RS-CARGO-14`.
- `deps` route-consumption hardening stayed in the tree and passed again under the broadened mapper contract; its placement-failure coverage remains aligned with the broader local-family route model.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-03-31-002143-shared-ownership-routing-migration.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `apps/guardrail3/crates/app/rs/ownership/src/discover.rs`
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs_tests/mod.rs`
- family runtimes/tests under:
  - `apps/guardrail3/crates/app/rs/families/toolchain`
  - `apps/guardrail3/crates/app/rs/families/cargo`
  - `apps/guardrail3/crates/app/rs/families/clippy`
  - `apps/guardrail3/crates/app/rs/families/deny`
  - `apps/guardrail3/crates/app/rs/families/deps`
  - `apps/guardrail3/crates/app/rs/families/release`
- Lean binary attack runs against temp fixtures for `cargo`, `release`, `toolchain`, `clippy`, and `deny`

## Open Questions / Future Considerations
- `cargo` still supports standalone-package policy roots even though the long-term repo topology direction is workspace-only. That should be revisited together with the `arch` migration so the policy change is explicit and testable.
- `deps` and `garde` still have family-local policy-selection logic layered on top of the broadened mapper route. They now pass again under the broadened route contract, but they are still candidates for further tightening if the ownership layer grows richer.
- `release` still has repo-root release-policy assumptions for shared files like `release-plz.toml` and `cliff.toml`; this patch only fixed Cargo-manifest surface discovery, not the broader release-policy location question.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/ownership/src/discover.rs` — shared ownership discovery for family-relevant files
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — mapper route construction after widening local-family visibility
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs_tests/mod.rs` — mapper-level proofs that local families still see misplaced candidates
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/discover.rs` — toolchain runtime consumption of routed Cargo ownership facts
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/discover.rs` — Cargo-family ownership checks for nested undeclared manifests
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — top-level allowed workspace-root selection inside clippy
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts.rs` — top-level allowed workspace-root selection inside deny
- `apps/guardrail3/crates/app/rs/families/release/src/facts/cargo_roots.rs` — routed-file-based release Cargo-root discovery
- `apps/guardrail3/crates/app/rs/families/release/src/repo_inventory/rs_release_12_input_failures_tests/scoped_cargo_surface.rs` — scoped CLI-equivalent release attack coverage
- `.worklogs/2026-03-31-002143-shared-ownership-routing-migration.md` — prior ownership-routing pass this work completed

## Next Steps / Continuation Plan
1. Move the generic workspace-topology bans into `arch` so the local families no longer need to tolerate the “standalone package is still policy-valid here” compromise.
2. Revisit `cargo` once `arch` is ready and explicitly remove standalone-package policy-root support, updating the remaining root-policy tests at the same time.
3. Tighten `deps` and `garde` policy-file ownership further if the shared ownership layer grows richer attachment facts for per-workspace policy files.
4. Decide whether `release` should stay repo-policy-root-aware for shared files or move fully to per-workspace release policy surfaces.
