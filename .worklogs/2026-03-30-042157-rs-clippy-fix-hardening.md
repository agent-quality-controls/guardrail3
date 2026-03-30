# Repair RS-CLIPPY Fail-Closed Contract

**Date:** 2026-03-30 04:22
**Scope:** `.plans/todo/checks/rs/clippy.md`, `.plans/todo/checks/rs/clippy/FIXES.md`, `apps/guardrail3/crates/app/rs/families/clippy/**`, `apps/guardrail3/crates/domain/modules/clippy/**`

## Summary
Repaired the full RS-CLIPPY backlog from `FIXES.md`: fail-open routing and cargo-config holes, malformed ban parsing, wrong-type scalar diagnostics, generator/runtime drift, assertion weakness, and missing sidecar coverage. Added a single-owner parseability rule for allowed Clippy configs, expanded the adversarial sidecar matrix, and synced the written contract to the repaired 25-rule family.

## Context & Problem
RS-CLIPPY had several real hardening gaps despite a green suite. Malformed routed `Cargo.toml` and malformed applicable `.cargo/config*` could weaken enforcement, ban parsing silently dropped bad shapes, threshold rules fanned one parse error into many, and pure-layer service handling had drifted away from the runtime contract. The tests also under-proved several branches: multiplicity, malformed-input ownership, same-root precedence at nested roots, negative published-library classification, and string-form ban behavior.

The user explicitly asked to fix the entire RS-CLIPPY backlog from `.plans/todo/checks/rs/clippy/FIXES.md`, keep working until the whole family was repaired, and verify the result with post-fix test attacks.

## Decisions Made

### Single-owner parseability for active Clippy configs
- **Chose:** add `RS-CLIPPY-25` to own parseability of allowed `clippy.toml` / `.clippy.toml`.
- **Why:** malformed allowed configs were previously reported redundantly by dependent rules. A single-owner parseability rule is stricter and less noisy.
- **Alternatives considered:**
  - Keep per-rule parse-error reporting — rejected because it created fanout noise and obscured ownership.
  - Push parseability into one of the threshold rules — rejected because parseability is family-wide, not threshold-specific.

### Fail closed on malformed routed Cargo roots
- **Chose:** keep ownership inside RS-CLIPPY facts collection and surface failures through `RS-CLIPPY-01` for coverage and `RS-CLIPPY-12` for placement.
- **Why:** the bug was real now, and the clippy family already reparses routed manifests for coverage/placement-sensitive semantics.
- **Alternatives considered:**
  - Wait for a broader placement/family-mapper refactor — rejected because it would leave a live bypass.
  - Treat malformed routed roots as “not a root” — rejected because that was the fail-open behavior being fixed.

### Pure-layer service semantics do not belong in Clippy baseline generation
- **Chose:** remove pure-layer service-specific global-state additions from canonical Clippy generation and keep library-only handling in RS-CLIPPY.
- **Why:** pure-layer semantics are architectural, not Clippy-profile semantics. This makes generator and runtime agree without teaching Clippy about hexarch concerns.
- **Alternatives considered:**
  - Teach runtime expectations about pure-layer service roots — rejected because it would further couple Clippy to architecture policy.
  - Leave generator/runtime drift and document it — rejected because it would preserve false “extra ban” results and baseline mismatch.

### Harden tests around exactness, malformed shapes, and cross-rule behavior
- **Chose:** replace weak assertion patterns with exact message/count checks and add sidecars for malformed-input ownership, wrong-type branches, nested precedence, and string-form behavior.
- **Why:** the original suite passed while still allowing real blind spots. The repaired suite should prove the family contract directly.
- **Alternatives considered:**
  - Keep broad set-style assertions and add a few more golden tests — rejected because multiplicity and extra findings would still slip through.
  - Add only runtime logic fixes without new sidecars — rejected because the same regressions could reappear undetected.

## Architectural Notes
The family now uses a clearer malformed-input ownership model:
- `RS-CLIPPY-23` owns `guardrail3.toml` parseability for profile/garde resolution.
- `RS-CLIPPY-24` owns applicable cargo-config override surfaces that can redirect Clippy discovery.
- `RS-CLIPPY-25` owns parseability of allowed Clippy config files.
- `RS-CLIPPY-01` and `RS-CLIPPY-12` fail closed when routed `Cargo.toml` is malformed and coverage/placement cannot be resolved.

Shared ban parsing now returns both valid entries and malformed messages, which lets completeness, extra-inventory, and quality rules all fail closed without silently dropping bad shapes. Domain-module exports now include exact type-path helpers so parity tests can stay tied to canonical generator order instead of reconstructing expected inventories ad hoc.

## Information Sources
- `.plans/todo/checks/rs/clippy/FIXES.md` — repair backlog and acceptance bars.
- `.plans/todo/checks/rs/clippy.md` — family contract and rule inventory.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/*` — runtime behavior and sidecars.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/*` — assertion semantics and exactness requirements.
- `apps/guardrail3/crates/domain/modules/clippy/*` — canonical generator/runtime parity source.
- Post-fix adversarial explorer audits in this session — used to catch the late `RS-CLIPPY-25` proof gap and one remaining parity cleanup.

## Open Questions / Future Considerations
- If routed Cargo-root parseability becomes a shared Rust-family concern later, the ownership currently implemented in RS-CLIPPY may be worth hoisting into a broader placement/family-mapper layer.
- RS-CLIPPY now has strong sidecar coverage; future rule additions should preserve the same one-rule/one-sidecar-family pattern and the single-owner malformed-input contract.

## Key Files for Context
- `.plans/todo/checks/rs/clippy/FIXES.md` — the backlog that drove the repair set.
- `.plans/todo/checks/rs/clippy.md` — the synced RS-CLIPPY rule contract and ownership model.
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — family ownership and current baseline summary.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — routing/coverage/placement facts and fail-closed cargo-root handling.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs` — canonical ban parsing and scalar type classification helpers.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs` — family orchestration and single-owner rule dispatch.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_25_config_parseable.rs` — allowed-config parseability ownership.
- `apps/guardrail3/crates/domain/modules/clippy/types.rs` — canonical type-path exports and profile-order helpers.
- `apps/guardrail3/crates/domain/modules/clippy/render.rs` — canonical Clippy baseline generation after pure-layer drift removal.

## Next Steps / Continuation Plan
1. If new RS-CLIPPY policy changes are requested, start by updating `domain/modules/clippy` and then repair parity/tests immediately rather than letting generator/runtime drift accumulate.
2. If the repo later centralizes routed Cargo-root parsing, revisit whether the `RS-CLIPPY-01/12` fail-closed logic should move into shared placement/family-mapper facts.
3. Keep post-fix attack review as part of future RS-CLIPPY change cycles; the late `RS-CLIPPY-25` proof gap showed the value of a final skeptical pass even after green tests.
