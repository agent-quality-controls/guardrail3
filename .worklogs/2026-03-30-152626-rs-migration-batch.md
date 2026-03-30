# Commit Rust Migration Batch

**Date:** 2026-03-30 15:26
**Scope:** `apps/guardrail3/**` with emphasis on `app/core`, `adapters/inbound/cli`, `app/rs/**`, `domain/project-tree`, `domain/report`, workspace manifests, and the ongoing Rust family migration from legacy validate code into per-family crates

## Summary
This commit captures the current Rust migration batch: whole-project validation target resolution, runtime/family-mapper scope plumbing, large-scale family-crate reshaping, workspace/runtime dependency restructuring, and a broad syntax-repair sweep over the newly split rule files. The tree is committed as owned state, but it is not yet fully compile-clean; the remaining frontier is recorded below.

## Context & Problem
The repo had accumulated a very large uncommitted Rust migration state. It includes:
- moving more validation flow onto `ProjectTree`
- resolving requested target paths against project root + subtree scope
- reworking CLI/runtime/family-mapper plumbing
- removing large sections of legacy `rs/validate/*` and `ts/validate/*`
- expanding per-family runtime/assertion layouts
- partially landing a mechanical `CheckResult::from_parts(...)` codemod that left many malformed delimiters across family rule files

The user explicitly asked to treat all state as owned, clean it up, and commit everything in a structured way. I attempted to push the code toward a clean workspace build before committing, but the syntax-repair sweep is still incomplete.

## Decisions Made

### Commit the migration batch even though the workspace is not fully green yet
- **Chose:** Record the full owned code state now, with an explicit worklog of the remaining compile blockers.
- **Why:** The user asked for the entire accumulated state to be committed rather than abandoned in a dirty tree, and the migration state is too large and too interconnected to safely cherry-pick into tiny isolated fragments.
- **Alternatives considered:**
  - Keep chasing the compile frontier until green before committing — rejected for this turn because the remaining failures are still part of a long mechanical sweep and the user prioritized getting the owned state committed.
  - Split the code into many tiny commits by family — rejected because the tree contains heavy cross-cutting runtime/core/CLI/workspace edits that are not separable cleanly anymore.

### Keep the runtime/root-scope changes with the broader Rust migration
- **Chose:** Commit the whole-project validation-target work in the same code batch as the runtime/family migration.
- **Why:** The CLI, runtime, mapper, and family-route changes are tightly coupled.
- **Alternatives considered:**
  - Isolate only the target-resolution work — rejected because the dirty tree now also includes large dependent family/runtime changes and partial codemod fallout.

### Repair obvious codemod corruption, but stop short of fabricating “clean” status
- **Chose:** Fix a substantial slice of malformed delimiter/call-shape errors, run repeated `cargo check`, and document the remaining frontier rather than overstating the status.
- **Why:** Honesty about repo state matters more than claiming a green build that does not exist.
- **Alternatives considered:**
  - Commit without any cleanup attempt — rejected because there were easy mechanical fixes worth landing first.

## Architectural Notes
Major code themes in this batch:
- Validation target resolution now flows through shared app-core logic so Rust validation and tree dumping resolve from project root with explicit subtree scope.
- Rust runtime/family-mapper plumbing has been reshaped around explicit scope handling and broader `ProjectTree` walking.
- The repo continues the shift from legacy monolithic validate modules into family-local runtime/assertion layouts.
- Workspace/product dependency structure has changed substantially, including runtime/workspace manifest churn and legacy validate surface deletions.

Current compile status at commit time:
- `cargo check --manifest-path apps/guardrail3/Cargo.toml --quiet` still fails.
- Remaining reported frontier when I stopped:
  - `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_06_edition_mismatch.rs`
  - `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/bans/rs_deny_10_multiple_versions_floor.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/cfg_and_paths/rs_code_17_impl_allow_blast_radius.rs`
  - `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_integrity/rs_hexarch_21_domain_purity.rs`
  - `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/derive_checks/rs_garde_14_guardrail_config_validate_call.rs`
  - `apps/guardrail3/crates/app/rs/families/hooks-shared/src/shell_safety/hook_shared_19_real_dispatcher_syntax_only.rs`
  - `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/policy/rs_deps_12_direct_dependency_cap.rs`
  - `apps/guardrail3/crates/app/rs/families/release/src/repo_policy/rs_release_04_cliff_exists.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api.rs`
  - `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/rs_libarch_11_root_facade_exports_api.rs`

## Information Sources
- `AGENTS.md`
- recent worklogs:
  - `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md`
  - `.worklogs/2026-03-30-132008-rs-runtime-and-product-decoupling.md`
- current workspace manifests under `apps/guardrail3/`
- live compiler output from repeated `cargo check --manifest-path apps/guardrail3/Cargo.toml --quiet`
- local code in:
  - `apps/guardrail3/crates/app/core/validation_target.rs`
  - `apps/guardrail3/crates/bin/guardrail3/src/main.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/validate.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/scoped_files.rs`

## Open Questions / Future Considerations
- The remaining compile failures are still mechanical-looking, but they span multiple families and should be completed before claiming the migration is stable.
- This commit includes very large deletions of legacy validate code; once the workspace is green, the next pass should verify no required behavior was dropped silently.
- The repo may benefit from a scripted syntax-sanity pass for malformed `CheckResult::from_parts(...)` call endings to avoid repeated manual cleanup.

## Key Files for Context
- `apps/guardrail3/crates/app/core/validation_target.rs` — shared requested-target resolution logic
- `apps/guardrail3/crates/bin/guardrail3/src/main.rs` — CLI entrypoints using the new resolution flow
- `apps/guardrail3/crates/adapters/inbound/cli/validate.rs` — scoped-file normalization and validate adapter behavior
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — Rust runtime scope/project-root orchestration
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — family route mapping with subtree scope
- `apps/guardrail3/crates/app/rs/family_mapper/src/scoped_files.rs` — scoped-file synthesis logic
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs` — shared repository snapshot model
- `apps/guardrail3/crates/domain/report/mod.rs` — report shape changes included in this batch
- `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md` — earlier runtime split verification context

## Next Steps / Continuation Plan
1. Resume with `cargo check --manifest-path apps/guardrail3/Cargo.toml --quiet` and fix the remaining ten frontier files listed above.
2. After the workspace compiles, run targeted family checks first (`fmt`, `deny`, `code`, `hexarch`, `garde`, `hooks-shared`, `deps`, `release`, `clippy`, `libarch`) before claiming the migration clean.
3. Do one final audit for accidental codemod fallout in family test helpers and inventory rules, especially where `CheckResult::from_parts(...)` was bulk converted.
