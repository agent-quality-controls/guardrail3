# Workspace Compile-Green Checkpoint

**Date:** 2026-03-30 17:26
**Scope:** `apps/guardrail3/crates/app/rs/**`, `apps/guardrail3/crates/app/ts/validate/**`, `apps/guardrail3/crates/app/core/validation_target.rs`, `apps/guardrail3/crates/bin/guardrail3/src/main.rs`, `apps/guardrail3/crates/adapters/**`, `apps/guardrail3/crates/domain/config/types.rs`, `.worklogs/2026-03-30-165132-compile-frontier-cleanup-checkpoint.md`

## Summary
This batch carries the remaining owned migration state to a compile-green workspace. The main work was collapsing repo-wide codemod fallout across Rust family runtimes/assertions/test-support and the still-built legacy TS validators, then finishing the root/scope migration plumbing so `cargo check --manifest-path apps/guardrail3/Cargo.toml` succeeds again.

## Context & Problem
After the runtime split and target-resolution refactor, the repo still had a large compile frontier made up mostly of mechanical damage rather than architectural failures. The dominant failure modes were:

- malformed `CheckResult::from_parts(...)` conversions with stray `)`, `},`, `.as_inventory()` placement, and broken function endings
- accessor migration scars such as repeated method-call garbage on `CheckResult`
- tests and test-support code still constructing private `ProjectTree` / `DirEntry` internals directly
- legacy TS validation files still being part of the workspace build, even though TS is not the active product direction

The user asked to make it compile and then to commit the whole owned state rather than treat the remaining dirty tree as someone else’s work.

## Decisions Made

### Carry the workspace to green instead of stopping at the Rust frontier
- **Chose:** keep fixing the built TS validator files after the Rust runtime side was mostly clean.
- **Why:** the user asked for the repo to compile, not just the active Rust path, and `guardrail3-app-ts` is still a workspace member that blocks `cargo check`.
- **Alternatives considered:**
  - Stop after Rust packages compiled — rejected because the workspace would still be broken.
  - Remove TS crates from the workspace build — rejected because that would be a product-surface change, not a compile cleanup.

### Rewrite the most corrupted helper files directly
- **Chose:** fully rewrite the worst TS helper file bodies when delimiter damage made surgical edits slower and riskier.
- **Why:** files like `tsconfig_check.rs` were too broadly corrupted by bad codemod output for reliable line-by-line repair.
- **Alternatives considered:**
  - Keep applying tiny delimiter fixes — rejected because the same file had multiple broken function endings and mixed struct-literal / builder-call damage.
  - Try another automated codemod — rejected because the existing damage already came from bad mechanical rewrites.

### Preserve the migration state as one owned batch
- **Chose:** commit the whole dirty tree plus the compile-green worklog in one checkpoint.
- **Why:** all current changes are part of the same owned migration stream: runtime split, root/scope refactor, accessor migration, syntax recovery, and workspace compile restoration.
- **Alternatives considered:**
  - Split into many small family commits now — rejected because the state is already highly cross-cutting and would take extra surgery without improving clarity.

## Architectural Notes
This commit does not introduce a new architecture. It stabilizes the already chosen one:

- runtime/family split remains intact
- whole-project walking plus scoped validation remains on the refactored path
- family-specific Rust crates still compile independently through the split runtime

The meaningful code themes in this checkpoint are:

- root/scope-aware validation plumbing in the Rust runtime and CLI path
- broad Rust family cleanup to current `CheckResult` accessor and constructor APIs
- test-support migration to private `ProjectTree` / `DirEntry` constructors
- legacy TS validator syntax repair so the workspace can build again

## Information Sources
- Live compiler frontier from repeated:
  - `cargo check --manifest-path apps/guardrail3/Cargo.toml --quiet`
  - `cargo check --manifest-path apps/guardrail3/Cargo.toml`
- Current `CheckResult` contract:
  - `apps/guardrail3/crates/domain/report/mod.rs`
- Current `ProjectTree` / `DirEntry` contract:
  - `apps/guardrail3/crates/domain/project-tree/src/lib.rs`
- Prior committed context:
  - `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md`
  - `.worklogs/2026-03-30-152626-rs-migration-batch.md`
  - `.worklogs/2026-03-30-165132-compile-frontier-cleanup-checkpoint.md`

## Open Questions / Future Considerations
- The workspace compiles, but this was a compile-restoration pass, not a behavior audit. The next serious pass should run targeted tests for the most-edited families and helper crates.
- Legacy TS validation still exists and still consumes maintenance even though Rust is the active direction. At some point the repo should decide whether those crates remain first-class or get moved out of the normal build path.
- There is clear evidence of at least one bad automated rewrite in the repo history. Future large API migrations should use smaller scoped codemods plus immediate compile checks to avoid another broad syntax-collapse event.

## Key Files for Context
- `apps/guardrail3/crates/app/core/validation_target.rs` — shared target resolution used by CLI/runtime after the root/scope refactor
- `apps/guardrail3/crates/bin/guardrail3/src/main.rs` — CLI entrypoints now flowing through the shared target-resolution path
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — Rust runtime orchestration and scoped validation behavior
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — family route mapping under subtree scope
- `apps/guardrail3/crates/app/rs/family_mapper/src/scoped_files.rs` — scoped file synthesis after the whole-project walker change
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs` — private constructor model that many tests/support crates had to migrate to
- `apps/guardrail3/crates/domain/report/mod.rs` — `CheckResult` API that drove most assertion and helper cleanup
- `apps/guardrail3/crates/app/ts/validate/architecture/ts_arch_checks.rs` — representative legacy TS codemod repair
- `apps/guardrail3/crates/app/ts/validate/source/ts_comment_checks.rs` — representative TS syntax/accessor repair
- `apps/guardrail3/crates/app/ts/validate/packages/tsconfig_check.rs` — rewritten TS helper body after delimiter corruption
- `.worklogs/2026-03-30-165132-compile-frontier-cleanup-checkpoint.md` — prior checkpoint describing the earlier compile frontier before this batch carried it to green

## Next Steps / Continuation Plan
1. Run targeted test/package verification on the most-edited Rust families instead of relying only on `cargo check`:
   - `clippy`
   - `deny`
   - `hexarch`
   - `libarch`
   - `release`
   - `test`
2. Verify the root/scope behavior end to end with real CLI runs against subtree targets, especially around `family_mapper` scoped-file synthesis.
3. Decide whether legacy TS validation should remain in the default workspace build. If not, do that as an explicit architecture/product change rather than as compile cleanup.
