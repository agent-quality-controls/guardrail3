# Freeze G3RS Package Structure

**Date:** 2026-04-06 13:23
**Scope:** `.plans/2026-04-06-g3rs-package-structure.md`

## Summary
Added a concrete package-structure plan for the new `g3rs` line. The plan freezes the outer package shape for `workspace-crawl`, `*-ingestion`, and the already extracted `*-checks` packages so the next implementation phase can follow one consistent scaffold instead of inventing different layouts per package.

## Context & Problem
The previous plan established the high-level runtime stack:

- `g3rs-workspace-crawl`
- `g3rs-*-ingestion`
- `g3rs-*-checks`

That still left an implementation gap: there was no committed source of truth for how those packages should actually be laid out on disk. The repo already has good extracted package specimens, especially `g3rs-garde-ast-checks` and `g3rs-fmt-config-checks`, and the user explicitly wanted the new packages to follow the way current packages are built rather than inventing a brand-new shape.

## Decisions Made

### Use the current `g3rs-*` extracted package layout as the baseline
- **Chose:** make the new crawl and ingestion packages mirror the existing extracted package pattern: root facade plus `crates/types`, `crates/runtime`, `crates/assertions`, with `README.md`, `TODO.md`, and package-local `Cargo.lock`.
- **Why:** the existing checks packages already compile, test, and publish with that shape. Reusing it minimizes structural drift.
- **Alternatives considered:**
  - Create a lighter custom layout for crawl/ingestion — rejected because it would produce a second package idiom immediately.
  - Collapse new packages to one crate each — rejected because it would violate the just-agreed stage/type separation.

### Keep checks-style rule directories only for checks packages
- **Chose:** preserve the existing rule-directory runtime layout only for `*-checks` packages, while crawl and ingestion organize their runtime crates by operational units like `crawl.rs`, `select.rs`, `parse.rs`, and `ingest.rs`.
- **Why:** crawl and ingestion are not rule inventories. Faking rule directories there would blur their role and create meaningless `RS-*` structure where none exists.
- **Alternatives considered:**
  - Force one-rule-per-directory structure onto crawl/ingestion too — rejected because those packages are stage-oriented, not rule-oriented.
  - Put everything for crawl/ingestion into one `run.rs` — rejected because it would make the first packages too monolithic.

### Make the feature split role-specific but uniform
- **Chose:** keep facade `api` features and let runtime crates export one role-specific logic feature: `crawl`, `ingest`, or `checks`.
- **Why:** this keeps the dependency story explicit and aligns with the earlier decision that packages should depend on each other through `types`, while the orchestrator alone pulls in `logic`.
- **Alternatives considered:**
  - Reuse `checks` everywhere — rejected because it hides the role distinction between the stages.
  - Skip feature splits and export everything directly — rejected because it weakens the package-stage boundary.

## Architectural Notes
- The structure plan intentionally distinguishes:
  - package shape: shared across all `g3rs-*` packages
  - runtime module shape: different between `checks` and `crawl`/`ingestion`
- `g3rs-workspace-crawl` remains the only shared pre-family package.
- `g3rs-*-ingestion` stays family/surface-specific and depends on crawl types plus checks-input types.
- Checks packages remain the current pure validation boundary and are not changed by this plan beyond being named as the structural baseline.

## Information Sources
- `.plans/2026-04-06-g3rs-workspace-crawl-and-ingestion.md` — prior stage architecture plan
- `packages/g3rs-garde-ast-checks/` — representative extracted AST package specimen
- `packages/g3rs-fmt-config-checks/` — representative extracted config package specimen
- `.worklogs/2026-04-06-125408-g3rs-workspace-crawl-plan.md` — prior worklog for the crawl/ingestion architecture plan
- current repository layout and package manifests under `packages/`

## Open Questions / Future Considerations
- The exact crawl output types are still placeholders until `g3rs-workspace-crawl` is actually scaffolded.
- It is still open whether all crawl/ingestion packages truly need an `assertions` crate, but the plan keeps it for consistency unless the first scaffold proves it useless.
- The future minimal `g3rs` app is still intentionally deferred until crawl plus at least a couple of ingestion packages exist.

## Key Files for Context
- `.plans/2026-04-06-g3rs-package-structure.md` — concrete package-shape source of truth
- `.plans/2026-04-06-g3rs-workspace-crawl-and-ingestion.md` — higher-level stage architecture
- `packages/g3rs-garde-ast-checks/Cargo.toml` — representative package root manifest
- `packages/g3rs-garde-ast-checks/crates/runtime/src/lib.rs` — representative runtime crate entrypoint layout
- `packages/g3rs-garde-ast-checks/crates/types/src/lib.rs` — representative public types crate
- `packages/g3rs-fmt-config-checks/Cargo.toml` — representative config-check package root manifest
- `.worklogs/2026-04-06-112700-rename-g3rs-config-ast-packages.md` — rename baseline for the new `g3rs-*` line

## Next Steps / Continuation Plan
1. Commit this structure plan.
2. Scaffold `packages/g3rs-workspace-crawl` exactly to this shape:
   - root facade
   - `crates/types`
   - `crates/runtime`
   - `crates/assertions`
   - `README.md`, `TODO.md`, `Cargo.lock`
3. Start with a minimal but real crawl contract:
   - explicit workspace root
   - discovered entries
   - ignore state
   - basic queries
4. Get `cargo test --workspace --manifest-path packages/g3rs-workspace-crawl/Cargo.toml` green before moving to the first ingestion package.
