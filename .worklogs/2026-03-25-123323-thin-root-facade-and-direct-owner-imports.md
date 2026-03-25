# Thin Root Facade And Direct Owner Imports

**Date:** 2026-03-25 12:33
**Scope:** `apps/guardrail3/crates/lib.rs`, `apps/guardrail3/crates/app/rs/validate/**`, `apps/guardrail3/crates/app/ts/**`, `apps/guardrail3/tests/**`

## Summary
Removed the remaining root-facade import paths from the live `apps/guardrail3` codebase and root test suite. The legacy Rust validator crate and the TS crate now import their real owners directly, and the root library has been collapsed into a thin package shell instead of re-exporting broad `domain/app/adapters` namespaces.

## Context & Problem
The crate split had already made the Rust family crates and most shared surfaces real workspace members, but there was still a compatibility layer problem:

- `guardrail3` still exposed wide `domain`, `app`, and `adapters` trees from the root library.
- `guardrail3-app-rs-legacy-validate` still defined crate-local `domain` / `app` shim modules and relied on `crate::domain::*` / `crate::app::core::*`.
- `guardrail3-app-ts` still did the same with local `domain` / `app` shims.
- The root `apps/guardrail3/tests` suite still imported `guardrail3::domain::*` and `guardrail3::app::ts::validate::*`, which kept the root facade alive as a de facto owner.

That shape was structurally wrong even if the split compiled. It let callers keep reaching through fake ownership layers instead of the real crates we created specifically to make the system independently compilable.

## Decisions Made

### Remove internal shim namespaces from compatibility crates
- **Chose:** Replace `crate::domain::report`, `crate::domain::config`, and `crate::app::core` usages inside `app/rs/validate` and `app/ts` with direct imports from `guardrail3_domain_report`, `guardrail3_domain_config`, and `guardrail3_app_core`.
- **Why:** These crates are already real owners. Keeping private shim trees inside promoted crates only preserves the old monolith shape under a different spelling.
- **Alternatives considered:**
  - Keep the shim modules for convenience — rejected because it preserves fake local ownership and makes future extraction harder.
  - Move the old code into the root crate again — rejected because it would reverse the split.

### Move root tests to direct crate imports
- **Chose:** Repoint remaining root tests away from `guardrail3::domain::*` and `guardrail3::app::ts::validate::*` onto direct dependency crates such as `guardrail3_domain_report`, `guardrail3_domain_config`, and `guardrail3_app_ts`.
- **Why:** The root test harness should not be the thing keeping facade APIs alive. Tests can depend on the same real crates as production code.
- **Alternatives considered:**
  - Leave root tests on `guardrail3::...` until a later test-only pass — rejected because that keeps the root package wide and blocks the thin-facade end state.
  - Move these tests into other crates immediately — rejected for now because the immediate goal was decoupling compile boundaries, not redesigning test architecture.

### Collapse the root library to a package shell
- **Chose:** Remove the `pub mod domain`, `pub mod app`, and `pub mod adapters` re-export trees from `apps/guardrail3/crates/lib.rs`, leaving the root library as a dependency shell with explicit underscore imports for package-level dependencies.
- **Why:** This is the actual Phase-1/Phase-2 architecture direction: the root package remains present, but it no longer masquerades as the owner of internal domain/app/adapter surfaces.
- **Alternatives considered:**
  - Keep the public reexports and call the root "thin enough" — rejected because callers had already been cut off and the reexports were now pure compatibility debt.
  - Remove the root library entirely — rejected because the package still needs a lib target and dependency surface for the bin target and integration tests.

## Architectural Notes
- This batch does not invent any new crates. It tightens the ownership model of the crates already promoted.
- `guardrail3-app-rs-legacy-validate` is still a compatibility crate, but it now depends on real owners directly instead of recreating mini-root namespaces.
- `guardrail3-app-ts` remains legacy in roadmap terms, but its ownership edges are now cleaner and no longer keep root facade paths alive.
- The root package is now much closer to the intended "product shell" shape: broad public namespace reexports are gone, and the package only keeps dependency markers needed for `unused_crate_dependencies = deny`.

## Information Sources
- `AGENTS.md`
- `.plans/todo/check_review/test_hardening/30-workspace-split-phase1-agent-brief.md`
- `.plans/todo/check_review/test_hardening/33-hexarch-layered-test-architecture-note.md`
- `.worklogs/2026-03-25-120443-delete-dead-root-rs-check-trees.md`
- `.worklogs/2026-03-25-121859-cut-root-tests-off-legacy-facades.md`
- `.worklogs/2026-03-25-122326-remove-arch-helpers-facade-shims.md`
- `rg -n "guardrail3::|crate::domain::|crate::app::core|crate::app::ts::validate|pub mod domain|pub mod app" apps/guardrail3/crates apps/guardrail3/tests`
- `cargo check --manifest-path apps/guardrail3/Cargo.toml --workspace --lib`
- `cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-legacy-validate --lib`
- `cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-ts --lib`

## Open Questions / Future Considerations
- The root `cargo test --manifest-path apps/guardrail3/Cargo.toml --test unit --no-run` target still drags the heavy `hooks-rs` compile boundary. This batch did not solve that compile-cost problem; it only proved that the import/ownership cut itself is clean.
- `app/rs/validate` still exists as a compatibility crate. The next real architectural gain is to keep reducing the reasons anyone needs it.
- The broader root test topology is still oversized. Tests are now less coupled to the root facade, but the root harness still aggregates too much.

## Key Files for Context
- `apps/guardrail3/crates/lib.rs` — root package shell after removing broad public reexports
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy Rust validator crate after removing local `domain/app` shims
- `apps/guardrail3/crates/app/ts/mod.rs` — TS crate after removing local `domain/app` shims
- `apps/guardrail3/crates/app/ts/validate/mod.rs` — TS validator entrypoint now importing direct crate owners
- `apps/guardrail3/tests/unit/ts_source_scan_test.rs` — representative root test now importing `guardrail3_app_ts` and `guardrail3_domain_report` directly
- `.worklogs/2026-03-25-121859-cut-root-tests-off-legacy-facades.md` — previous step removing root tests from `app::rs::validate`
- `.worklogs/2026-03-25-122326-remove-arch-helpers-facade-shims.md` — previous step removing the last `arch_helpers` facade use sites

## Next Steps / Continuation Plan
1. Keep draining live callers off `guardrail3-app-rs-legacy-validate` by identifying which root tests or helper paths still depend on it and moving those imports to the real family crates where possible.
2. Reassess `apps/guardrail3/tests/unit.rs` now that the root facade is gone from callers, and start removing or relocating the worst remaining root test clusters without redesigning the full layered test architecture yet.
3. Investigate the `hooks-rs` compile hotspot as the main remaining blocker to fast root-harness proof loops. The goal is not a test redesign here; it is to reduce the heavy crate boundary that still dominates `--test unit --no-run`.
