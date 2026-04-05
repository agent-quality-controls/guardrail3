# Clean Up Older Package Test Paths And Feature Gating

**Date:** 2026-04-05 19:26
**Scope:** `packages/g3-toolchain-content-checks`, `packages/g3-fmt-content-checks`, `packages/g3-clippy-content-checks`, `packages/g3-deny-content-checks`, `packages/g3-cargo-content-checks`

## Summary
Cleaned up the older extracted content-check packages so they stop using the old `#[path = "../..._tests/mod.rs"]` test wiring and stop tripping the missing internal feature-gating checks on their runtime/assertions crates. The affected packages still have the broader shared cross-package dependency debt, but the test-path and gating issues requested by the user are now fixed.

## Context & Problem
After building `g3-deps-content-checks`, the user explicitly asked for a comparison against the older extracted packages and then asked for two categories of cleanup across those older packages:
- test path wiring
- feature gating

The validator review showed the same recurring pattern in the older extracted packages:
- rule modules still referenced sidecar test directories through `#[path = "../..._tests/mod.rs"]`
- several runtime crates still exported `run::check` without feature gating
- several assertions crates still had `default = ["checks"]` without an `all` feature even though the validator expects the `all` + `default = ["all"]` pattern

Those issues were already present in `toolchain`, `fmt`, `clippy`, `deny`, and `cargo`. The user explicitly said complexity was acceptable for now and wanted the path-hook and gating issues fixed first.

## Decisions Made

### Move older package tests under rule-local `tests/` directories
- **Chose:** for each affected package, move `*_tests/` directories into the sibling rule directory as `tests/`.
- **Why:** this removes the `#[path = "../..._tests/mod.rs"]` bypass and matches the cleaned-up `deps` package layout.
- **Alternatives considered:**
  - Leave the old sidecar directories and accept the validator warnings — rejected because the user explicitly asked for the test-path cleanup.
  - Collapse tests into inline `mod tests` blocks — rejected because the repo uses rule-local sidecar test directories, not large inline test bodies.

### Replace `#[path = ...]` hooks with normal module resolution
- **Chose:** update the affected `mod.rs` files to use plain `#[cfg(test)] mod tests;`.
- **Why:** this is the whole point of the test-directory move and removes the `RS-ARCH-09` findings.
- **Alternatives considered:**
  - Keep `#[path = ...]` and merely rename directories — rejected because it would preserve the exact complaint the user asked to remove.

### Add the missing `all` / `default = ["all"]` feature pattern where the validator still complained
- **Chose:** add `all` meta-features and `default = ["all"]` to the older runtime and assertions crates that still lacked them, and gate runtime `pub use run::check` exports with `#[cfg(feature = "checks")]`.
- **Why:** the repo’s validator treats ungated runtime exports and missing `all` features as incorrect facade structure.
- **Alternatives considered:**
  - Leave the gating debt in place and only fix test paths — rejected because the user explicitly called out feature gating too.
  - Add ad hoc feature names per crate — rejected because the existing extracted packages already standardize on `all = ["checks"]`.

### Fix cargo’s moved-fixture paths after the directory reshuffle
- **Chose:** update the `include_str!` paths in the cargo package tests to point at the new nested fixture location under `rs_cargo_01_workspace_lints/tests/fixtures`.
- **Why:** the directory move broke those relative includes.
- **Alternatives considered:**
  - Duplicate the fixture in each rule directory — rejected because that would create unnecessary test data duplication.

## Architectural Notes
This was an extracted-package hygiene pass, not a family-boundary change.

What changed structurally:
- older packages now match the `tests/` subdirectory pattern used in the cleaned-up deps package
- runtime crates now use the expected `checks` feature gate on exported `check` functions
- assertions crates that previously missed `all` now follow the same public-feature pattern as the better-behaved packages

What did **not** change:
- package boundaries
- rule ownership
- the broader shared dependency/facade rule debt (`RS-ARCH-05/06`) across extracted packages
- complexity findings the user explicitly said to ignore for now

## Information Sources
- `AGENTS.md`
- package-local `arch/code` validation runs for:
  - `packages/g3-toolchain-content-checks`
  - `packages/g3-fmt-content-checks`
  - `packages/g3-clippy-content-checks`
  - `packages/g3-deny-content-checks`
  - `packages/g3-cargo-content-checks`
- `.worklogs/2026-04-05-191423-deps-content-checks-architecture-cleanup.md`
- existing extracted package layouts under `packages/g3-*`

## Open Questions / Future Considerations
- The older extracted packages still have the broader shared dependency/facade complaints (`RS-ARCH-05/06`) that also affect newer packages.
- Several older packages still exceed local complexity thresholds, especially `clippy`, `deny`, and `cargo`.
- Some assertions `lib.rs` files still contain private `use ... as _` stubs and related facade debt; that was not part of the user’s requested cleanup.

## Key Files for Context
- `packages/g3-toolchain-content-checks/crates/runtime/src/rs_toolchain_02_channel_and_components/mod.rs` — example of the cleaned test wiring
- `packages/g3-fmt-content-checks/crates/runtime/Cargo.toml` — example runtime feature gating fix
- `packages/g3-clippy-content-checks/crates/assertions/Cargo.toml` — example assertions feature gating fix
- `packages/g3-deny-content-checks/crates/runtime/src` — largest test-directory migration surface
- `packages/g3-cargo-content-checks/crates/runtime/src/rs_cargo_01_workspace_lints/tests/fixtures/golden_workspace.toml` — fixture path that had to move with the cargo cleanup
- `.worklogs/2026-04-05-191423-deps-content-checks-architecture-cleanup.md` — immediate prior package cleanup that established the desired shape

## Next Steps / Continuation Plan
1. If the next cleanup pass targets extracted-package architecture again, focus on the remaining shared debt in `RS-ARCH-05/06` and the private `use ... as _` facade violations in assertions/types crates.
2. Keep complexity cleanup separate from this pass; the user explicitly deferred it.
3. When wiring the `deps` package into the app family, use the cleaned test-directory and feature-gated runtime pattern from this batch rather than reintroducing the old `#[path = ...]` test hooks.
