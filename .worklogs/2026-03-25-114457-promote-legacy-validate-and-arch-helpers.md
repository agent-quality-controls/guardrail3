# Promote Legacy Validate And Arch Helpers

**Date:** 2026-03-25 11:44
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/mod.rs`, `apps/guardrail3/crates/app/rs/validate`, `apps/guardrail3/crates/app/arch-helpers`, `apps/guardrail3/crates/app/ts`, `apps/guardrail3/crates/lib.rs`

## Summary
Promoted the legacy Rust validator tree under `app/rs/validate` into its own real workspace crate and then extracted the shared `arch_helpers` module into a separate owner crate. This removes a live root-owned source-file backedge from both legacy Rust validation and TypeScript validation while keeping the workspace compiling at crate scope.

## Context & Problem
The current split has most Rust families as real crates, but `app/rs/validate` still lived inside the root library tree as a giant legacy module surface. That meant a substantial chunk of Rust validation code was still coupled to the root package rather than compiling as an independent crate. After the user clarified the goal again, the priority was crate decoupling and compile boundaries, not test relayering.

While promoting `app/rs/validate`, one immediate problem surfaced: it depended on `app/arch_helpers.rs` through the old root module tree. Leaving that as a `#[path]` include would technically compile, but it would preserve a fake boundary instead of creating a clean owner crate. The right move was to promote that helper too.

## Decisions Made

### Promote `app/rs/validate` as a legacy compatibility crate
- **Chose:** Added `crates/app/rs/validate/Cargo.toml`, made it a workspace member, and changed the root `app::rs` facade to `pub use guardrail3_app_rs_legacy_validate as validate;`.
- **Why:** This isolates the remaining monolithic legacy Rust validator surface behind a real Cargo crate, which is exactly the kind of compile boundary the workspace split is supposed to create.
- **Alternatives considered:**
  - Keep `app/rs/validate` as root-owned modules — rejected because it leaves one of the biggest remaining Rust compile surfaces inside the monolith.
  - Rewrite the whole legacy validator to direct family crates first — rejected because it is larger than the current goal and would slow the boundary split.

### Keep legacy namespace compatibility via narrow shims
- **Chose:** Added small `domain` and `app` shim modules in `app/rs/validate/mod.rs` to rebind legacy paths onto promoted crates.
- **Why:** Most of the legacy source still assumes `crate::domain::*` and `crate::app::*`. Small shims keep the source compiling without re-editing every file before the crate boundary exists.
- **Alternatives considered:**
  - Rewrite all legacy imports immediately — rejected because it would balloon the change and make the ownership cut harder to verify.
  - Leave the code under the root facade and defer the crate entirely — rejected because that preserves the existing compile bottleneck.

### Extract `app/arch_helpers` into a real owner crate
- **Chose:** Created `crates/app/arch-helpers` and moved the old `app/arch_helpers.rs` contents into `src/lib.rs`.
- **Why:** Both TS validation and legacy RS validation were still reaching into a root-owned helper file. Turning it into a crate removes that shared source-file backedge and keeps the boundary honest.
- **Alternatives considered:**
  - Use `#[path]` from both crates — rejected because it is exactly the fake-crate pattern this refactor is supposed to eliminate.
  - Leave it only in the root facade and import through `guardrail3::app::arch_helpers` — rejected because it keeps the facade as a broad internal owner instead of a thin adapter.

### Preserve root package checks with underscore imports
- **Chose:** Added underscore imports in `crates/lib.rs` for package-level deps now used by the bin target or newly promoted crates.
- **Why:** The workspace enforces `unused_crate_dependencies = "deny"`. The root package still needs some deps for the bin target and compatibility facade even when the lib target does not reference them directly.
- **Alternatives considered:**
  - Remove the deps from the root package immediately — rejected because several are still used by the bin target and integration surface.
  - Ignore the lint for the root package — rejected because it would hide real ownership drift.

## Architectural Notes
This commit does not try to redesign legacy validation or fix test attribution. It is a crate-boundary move only.

After this change:
- `app/rs/validate` is a real workspace member and compiles on its own.
- `app/arch-helpers` is a real workspace member and compiles on its own.
- the root `guardrail3` facade now reexports both through crate dependencies instead of owning the source files directly.
- TypeScript was touched only where necessary to stop consuming the deleted root helper file.

The remaining structural debt is now more obvious:
- `app/rs/validate` is isolated, but still internally monolithic.
- the root facade is thinner than before, but still wider than the end-state plan.
- root-level tests and legacy compatibility paths still exist, but they are no longer the justification for keeping these sources in the root package.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — current workspace split plan and end-state expectations.
- `apps/guardrail3/crates/app/rs/mod.rs` — current Rust app facade.
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy validator entry and namespace assumptions.
- `apps/guardrail3/crates/app/arch_helpers.rs` — shared helper that still tied TS and legacy RS validation to the root package.
- `apps/guardrail3/crates/lib.rs` — root facade and package-level dependency pattern.
- `.worklogs/2026-03-25-111802-narrow-root-test-imports.md` — prior state immediately before this decoupling pass.

## Open Questions / Future Considerations
- `app/rs/validate` is now a real crate, but it still contains a large number of legacy submodules. The next useful improvement is cutting more live callers over to family/runtime owners so this crate shrinks rather than simply moving intact.
- `app/ts/validate` still exists as a module tree inside the `app-ts` crate. That is lower priority given the current Rust-only roadmap, but it remains structurally similar legacy debt.
- The root package still owns broad facade exports. Those should continue shrinking as callers move to direct crate imports.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — workspace members and root package dependency graph.
- `apps/guardrail3/crates/app/rs/mod.rs` — Rust app facade now reexporting runtime and legacy validate crates.
- `apps/guardrail3/crates/app/rs/validate/Cargo.toml` — new legacy validator crate manifest.
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — compatibility shim root for legacy validator code.
- `apps/guardrail3/crates/app/arch-helpers/Cargo.toml` — new shared helper crate manifest.
- `apps/guardrail3/crates/app/arch-helpers/src/lib.rs` — canonical shared arch-helper owner.
- `apps/guardrail3/crates/lib.rs` — root facade after the crate promotion.
- `.worklogs/2026-03-25-111802-narrow-root-test-imports.md` — prior worklog immediately before this change.

## Next Steps / Continuation Plan
1. Audit remaining root-owned Rust compatibility surfaces that still compile as source trees rather than crates, starting with `app/rs/checks` and any remaining shared helpers pulled through the root facade.
2. Keep routing live callers away from `app/rs/validate/*` onto `app-rs-runtime` and family crates so the new legacy-validate crate becomes a shrinking compatibility island instead of a permanent second monolith.
3. Continue checking crate-local compile boundaries directly with `cargo check -p <crate> --lib` before relying on broader workspace proof, so new splits are validated as real independent targets.
4. Avoid more test-architecture churn until the crate split is far enough along that LLM-driven test cleanup can happen against narrow owner crates instead of the root harness.
