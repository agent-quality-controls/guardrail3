# Trim RS-CODE Allow Surface In Rust Families

**Date:** 2026-03-29 21:13
**Scope:** apps/guardrail3/crates/app/rs/families/{cargo,deps,test,release}

## Summary
Reduced the repo-root `RS-CODE-03` bucket by removing copied `#[allow(unused_imports)]` / `#[allow(dead_code)]` scaffolding across the Rust family test harnesses instead of papering it over with more reasons. The pass tightened `cargo` case-table exceptions, trimmed unused sidecar imports in `test`, `deps`, and `release`, and deleted dead helper seams that were being carried by template copy rather than real callers.

## Context & Problem
After fixing `RS-DENY`, the next large repo-root debt was `RS-CODE`, dominated by `RS-CODE-03`: item-level `#[allow(...)]` without reason comments. Agent audits agreed that most of this surface was not legitimate architectural exception handling. It was copied test harness residue:
- `*_tests/mod.rs` files carrying `unused_imports` to make child modules compile
- child test files importing more assertion helpers than they used
- production rule files carrying `dead_code` test helpers that no current sidecar referenced
- cargo case tables bundling a real lowercase-name exception with a bogus dead-code suppression

The goal for this pass was to remove those suppressions wherever possible rather than simply moving them from `RS-CODE-03` into `RS-CODE-04` with weak reasons.

## Decisions Made

### Remove copied test harness suppressions instead of documenting them
- **Chose:** trim unused sidecar imports and helper seams in `test`, `deps`, and `release` until the affected family tests compiled cleanly without the undocumented allows.
- **Why:** the audits showed these were mostly mechanical template residue, not real exceptions. Deleting them is strictly better than inventing reasons.
- **Alternatives considered:**
  - add `// reason:` comments everywhere — rejected because it would preserve junk seams and turn obvious debt into sanctioned inventory noise
  - revert to the old suppressed shape — rejected because it would leave the main `RS-CODE-03` bucket essentially unchanged

### Keep cargo case-table exceptions, but make the real exception explicit
- **Chose:** restore `dead_code` on `cases.rs` fixture fragments and add a concrete reason comment, while removing the unnecessary `unused_imports` suppressions.
- **Why:** the case tables intentionally hold shared TOML/config fragments with lowercase names, and some fragments are used unevenly across scenarios. That is a legitimate declarative-fixture exception.
- **Alternatives considered:**
  - remove `dead_code` from all case tables — rejected after the cargo package failed to compile under `-D dead-code`
  - keep the old bare `#[allow(dead_code, non_upper_case_globals)]` — rejected because it remained undocumented and indistinguishable from cargo-cult suppression

### Trim release helper surfaces to actual callers
- **Chose:** remove unused per-rule `mod.rs` assertion aliases and delete copied `run_family` helpers where the sidecars actually call `run_tree` or do not call the helper at all; restore `run_family` only for `rs_release_12_input_failures`, where the sidecars truly use it.
- **Why:** `release` had the largest copied helper template surface. The family tests showed which helpers were actually needed.
- **Alternatives considered:**
  - keep all copied helpers and document them — rejected because the compile and audit evidence showed many were truly dead
  - rewrite the whole release test layout in one pass — rejected for this slice because the immediate problem was allow-surface reduction, not a fresh family migration

## Architectural Notes
This pass keeps the existing runtime/assertions/test-support family split intact. The change is in test seam hygiene:
- `mod.rs` shims should expose only names child sidecars actually use
- child sidecars should import only the assertion helpers they actually call
- rule files should expose only test helpers that have real callers in sibling sidecars

That matches the repo’s broader intent: companion test crates are allowed, but they should not become excuse surfaces for dead code and unused import suppressions.

## Information Sources
- Agent audit reports from Sartre, Descartes, and Helmholtz on `RS-CODE-03` concentrations and anti-patterns
- repo-root `RS-CODE` live runs via `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json`
- family package tests:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-cargo --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deps --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-release --lib`
- targeted `cargo fix --tests --allow-dirty --allow-staged` runs for `guardrail3-app-rs-family-test` and `guardrail3-app-rs-family-release`

## Open Questions / Future Considerations
- `RS-CODE` still has large remaining debt outside this slice, especially `RS-CODE-24`, `RS-CODE-32`, and non-family `RS-CODE-03` fallout in other areas.
- Some `release` and `deps` files were already dirty before this pass; the next step should separate pure `RS-CODE` cleanup from broader `RS-RELEASE` / `RS-DEPS` semantics work.
- `crates/domain/project-tree/src/lib.rs` still has live `RS-CODE-05` debt outside this commit.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src` — clearest example of trimming imported assertion surface instead of documenting unused imports
- `apps/guardrail3/crates/app/rs/families/release/src` — biggest copied helper surface; shows the dead helper removal approach
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/*_tests/cases.rs` — declarative fixture exception pattern with explicit reason comment
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src` — smaller runtime-helper cleanup that now compiles without the dead-code suppressions
- `.worklogs/2026-03-29-210129-fix-rs-deny-placement-and-baseline.md` — immediately preceding cleanup that removed the live `RS-DENY` blockers
- `.worklogs/2026-03-29-204728-restore-inventory-contracts.md` — earlier family stabilization work that explains the current finished-family baseline

## Next Steps / Continuation Plan
1. Re-run repo-root `RS-CODE` and record the new per-rule counts, then target the remaining `RS-CODE-03` concentration outside these four families.
2. Tackle `RS-CODE-24` next: most remaining repo-root errors are now in `#[path = ...]` sites without explicit same-line `// reason:` comments.
3. Clean `RS-CODE-32` test `.expect(...)` messages after the structural allow cleanup is stable, because those edits are mostly independent string-quality fixes.
4. Keep `RS-RELEASE` semantics work separate from future `RS-CODE` sweeps unless a test seam change clearly serves both.
