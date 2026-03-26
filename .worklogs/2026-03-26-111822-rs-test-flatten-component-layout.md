# RS-TEST Flatten Self-Hosting Component Layout

**Date:** 2026-03-26 11:18
**Scope:** `apps/guardrail3/crates/app/rs/families/test/**`, `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`

## Summary
Removed the synthetic `crates/family/` wrapper from the `rs/test` family and taught discovery to accept a direct self-hosted shape at `crates/runtime` and `crates/assertions`. The family now matches the README’s intended component interpretation more closely and still validates cleanly on itself.

## Context & Problem
The prior self-hosting refactor had placed the family crates at `families/test/crates/family/runtime` and `families/test/crates/family/assertions`. That satisfied the validator’s old assumption that tested components always look like `crates/<component>/{runtime,assertions}`, but it introduced an unnecessary fake component name. The user correctly pointed out that the `test` family root itself is the component, so the extra nesting was design drift caused by discovery logic rather than a real architectural requirement.

## Decisions Made

### Extend discovery instead of preserving the synthetic folder
- **Chose:** Update `collect_components(...)` so it accepts both:
  - normal nested components at `crates/<name>/runtime` + `crates/<name>/assertions`
  - direct self-hosted crates at `crates/runtime` + `crates/assertions`
- **Why:** This removes the need for a fake `family` component name while keeping compatibility with existing nested component fixtures used by the rule tests.
- **Alternatives considered:**
  - Keep `crates/family/...` and document it as special — rejected because it contradicts the README intent and the user’s design expectation.
  - Rewrite discovery to only support the direct shape — rejected because the family also validates fixture repos that intentionally use `crates/<name>/...`.

### Treat the family root as the component identity for the direct shape
- **Chose:** For direct `crates/runtime` + `crates/assertions`, set the discovered component root to the family root rather than `crates/`.
- **Why:** The component is the family itself, not the internal `crates` container directory.
- **Alternatives considered:**
  - Use `crates/` as the component root — rejected because it would preserve the same conceptual mismatch under a different name.

### Move the actual crates and fix Cargo wiring in one checkpoint
- **Chose:** Physically move runtime/assertions to `families/test/crates/{runtime,assertions}` and immediately update workspace members plus downstream dependency paths.
- **Why:** This keeps the tree and manifests coherent at every checked state.
- **Alternatives considered:**
  - Land discovery support first and defer the move — rejected because it would leave the repo in a half-migrated state with the old synthetic shape still on disk.

## Architectural Notes
The `rs/test` family discovery is now slightly more general: it no longer conflates “componentized” with “must have a named child under `crates/`”. That makes the family’s own self-hosting layout cleaner without weakening the rule model for normal fixture repos. Existing fixture repos using `crates/demo/runtime` and `crates/demo/assertions` continue to work unchanged.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `apps/guardrail3/Cargo.toml`
- `apps/guardrail3/crates/app/rs/Cargo.toml`
- Prior worklogs:
  - `.worklogs/2026-03-26-101556-rs-test-exact-self-hosting.md`
  - `.worklogs/2026-03-26-110506-rs-test-assertions-contract.md`
  - `.worklogs/2026-03-26-110932-rs-test-self-fix-assertions.md`

## Open Questions / Future Considerations
- Other families may eventually want the same direct self-hosting component shape. If so, the generalized discovery pattern should probably be reused rather than reimplemented family-by-family.
- The README still describes the generic target shape as `crates/x/{runtime,assertions}`. That is still correct for normal components, but a future doc tweak could explicitly mention that a self-hosted family may use `crates/runtime` + `crates/assertions` when the family root itself is the component.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs` — discovery logic for nested and direct component layouts
- `apps/guardrail3/crates/app/rs/families/test/Cargo.toml` — local workspace members for the flattened family shape
- `apps/guardrail3/Cargo.toml` — top-level workspace members updated to the flattened paths
- `apps/guardrail3/crates/app/rs/Cargo.toml` — runtime dependency on the family crate updated to the flattened path
- `.worklogs/2026-03-26-101556-rs-test-exact-self-hosting.md` — earlier checkpoint that introduced the synthetic `family` wrapper

## Next Steps / Continuation Plan
1. If the same “component is the family root” issue exists in other self-hosted families, apply the same direct `crates/runtime` + `crates/assertions` shape and reuse the discovery pattern from `facts.rs`.
2. Keep an eye on test fixture assumptions when changing discovery: the direct-shape support must not break nested `crates/<name>/...` repos used throughout the rule suite.
3. Consider a small README clarification so future refactors do not reintroduce a fake extra component name just to satisfy discovery.
