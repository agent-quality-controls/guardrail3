# Inbound CLI Crate Promotion

**Date:** 2026-03-25 10:40
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/Cargo.lock`, `apps/guardrail3/crates/adapters/inbound/cli`, `apps/guardrail3/crates/lib.rs`

## Summary
Promoted `crates/adapters/inbound/cli` into a real workspace crate and rewired the root facade to re-export that crate instead of owning the module tree directly. Normalized the CLI module imports so they now depend on promoted app/domain crates rather than reaching back through `crate::app` / `crate::domain`.

## Context & Problem
The workspace split plan explicitly lists `adapters/inbound/cli` as a later-phase real member, but only after the command-text owner and Rust generator/check/diff surfaces were normalized. Those prerequisites are now in place:
- `app/commands` owns shared command/help text
- `app/rs/generate` owns the Rust write set

That left the inbound CLI as one of the biggest remaining root-owned surfaces. Even after the earlier splits, the CLI files still imported app/domain functionality through the root facade, which meant the module tree was still monolithic in practice. The next meaningful architectural step was to make the CLI a real crate and force its dependencies to point one-way into the promoted owners.

## Decisions Made

### Promote the whole current CLI module tree as one crate
- **Chose:** Add a real `guardrail3-adapters-inbound-cli` manifest at `crates/adapters/inbound/cli/Cargo.toml` and use the existing `mod.rs` as the crate root.
- **Why:** The current tree already has a coherent module boundary and file layout. Reusing it as the crate root keeps the change structural rather than turning it into a file-move churn batch.
- **Alternatives considered:**
  - Promote only the smallest subset (`cli.rs`, `help_gen.rs`, `validate.rs`, `check.rs`) first — rejected because the current tree compiled cleanly as one crate once the direct imports were normalized, and a half-promotion would preserve an awkward split ownership inside the same directory.
  - Leave the CLI under the root facade and add a facade crate over it — rejected because that would not change the real ownership boundary.

### Re-export the promoted CLI crate from the root facade instead of keeping a local module
- **Chose:** Change `crates/lib.rs` from `pub mod cli;` to `pub use guardrail3_adapters_inbound_cli as cli;`.
- **Why:** This keeps existing `guardrail3::adapters::inbound::cli::...` callsites stable while removing the root crate as the source owner of the CLI implementation.
- **Alternatives considered:**
  - Update all callers to import `guardrail3_adapters_inbound_cli` directly in the same batch — rejected because the re-export keeps product-entry compatibility while still moving implementation ownership to the new member crate.
  - Keep both the local module and the external crate temporarily — rejected because dual ownership would make the split ambiguous and fragile.

### Normalize CLI imports to direct app/domain crates
- **Chose:** Replace remaining root-facade backedges like `crate::app::core`, `crate::domain::modules`, and `crate::domain::report` with direct crate imports.
- **Why:** A real CLI crate cannot keep depending on the root `guardrail3` facade without recreating the monolith or risking cycles. The plan explicitly calls for inbound CLI dependencies to consume app/domain crates directly.
- **Alternatives considered:**
  - Add temporary helper re-exports so the old imports still compile — rejected because that would hide the dependency direction problem instead of fixing it.
  - Leave only `generate.rs` or `map.rs` on root imports — rejected because that would still block the crate build.

### Accept package-level underscore imports in the root crate to satisfy `unused-crate-dependencies`
- **Chose:** Add underscore imports in `crates/lib.rs` for dependencies that moved off the root implementation path but still remain package dependencies.
- **Why:** The root package still owns the binary target and integration tests, so package-level dependency wiring is broader than the root library implementation. Underscore imports keep the package compiling cleanly without undoing the ownership move.
- **Alternatives considered:**
  - Remove those dependencies from the root package immediately — rejected because some are still needed by the bin target or compatibility re-exports.
  - Reintroduce root implementation usage just to satisfy the lint — rejected because that would reverse the split.

## Architectural Notes
This batch changes the dependency shape in an important way:
- `adapters/inbound/cli` is now a real member crate
- the root facade is now only a re-export surface for that implementation
- CLI coverage/map/init/generate/help/check modules consume promoted crates directly

That means the root library no longer owns the CLI implementation and the CLI no longer needs to reach “up” into the facade to find app/domain functionality. This is the intended one-way flow from the plan.

I intentionally did not move `main.rs` onto direct CLI-crate imports in this batch. Keeping the root re-export in place makes the ownership change smaller and keeps the product-entry surface stable while the next splits continue behind it.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — phase ordering and inbound CLI promotion constraints
- `.worklogs/2026-03-25-102654-app-commands-owner-cutover.md` — prior prerequisite batch for command/help ownership
- `.worklogs/2026-03-25-103628-rs-generate-owner-cutover.md` — prior prerequisite batch for Rust write-set ownership
- `apps/guardrail3/crates/adapters/inbound/cli/*.rs` — direct dependency normalization targets
- `apps/guardrail3/crates/lib.rs` — root facade re-export change
- Subagent exploration from Linnaeus during this batch — confirmed the likely dependency surface and blockers before promotion

## Open Questions / Future Considerations
- The CLI crate is real now, but the root facade still re-exports it. Future cleanup can move more product-entry code to direct crate imports once compatibility pressure is lower.
- `generate.rs` still owns mixed-stack TS generation and mixed hook output, while `app/rs/generate` owns the Rust write set. That remaining split is intentional for now but still marks a future boundary to sharpen.
- Root-level tests still import the root facade broadly, so the full compile-time benefit of this CLI promotion is not realized until more tests move onto smaller crates.
- `domain/modules/guide.rs` remains legacy duplication outside the new `app/commands` owner.

## Key Files for Context
- `apps/guardrail3/crates/adapters/inbound/cli/Cargo.toml` — real manifest for the promoted inbound CLI crate
- `apps/guardrail3/crates/adapters/inbound/cli/mod.rs` — crate root for the promoted CLI member
- `apps/guardrail3/crates/adapters/inbound/cli/generate.rs` — mixed-stack generator path after direct crate import normalization
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs` — TS init path now depending directly on promoted crates
- `apps/guardrail3/crates/adapters/inbound/cli/map.rs` — map/coverage path now depending directly on `app_core`
- `apps/guardrail3/crates/lib.rs` — root facade now re-exporting the CLI crate
- `apps/guardrail3/Cargo.toml` — workspace membership and package dependency wiring
- `.worklogs/2026-03-25-103628-rs-generate-owner-cutover.md` — prerequisite batch immediately before this one

## Next Steps / Continuation Plan
1. Keep draining root-facade usage from product-entry code, starting with direct imports in `main.rs` where the new CLI/app crates can now be referenced without going through `guardrail3::...`.
2. Revisit the remaining mixed-stack surfaces in CLI generation: decide whether TS generation also deserves its own owner crate or whether mixed CLI generation should stay as the adapter boundary.
3. Continue dismantling root test aggregators and broad root-facade tests so the new member crates translate into real faster test/build loops instead of only structural correctness.
