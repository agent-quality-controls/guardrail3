# Advance RS-CODE Path And Expect Cleanup

**Date:** 2026-03-29 22:11
**Scope:** `apps/guardrail3/crates/app/core`, `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs`, `apps/guardrail3/crates/app/rs/families/{cargo,clippy,deny,garde,hexarch,release,toolchain}`, `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`, `apps/guardrail3/crates/domain/modules/{clippy,cspell,eslint}.rs`, `apps/guardrail3/crates/domain/report/mod.rs`

## Summary
Pushed `RS-CODE` down further by converting the remaining bare `#[path = ...]` sites into explicit, reasoned uses and by replacing the last large bucket of placeholder `.expect(...)` strings in `garde`, `project_walker`, and a small `hexarch` fixture. After this pass, repo-root `RS-CODE` is down to `75` errors, with `RS-CODE-24` fully off the error line and `RS-CODE-32` reduced from `275` errors to `30`.

## Context & Problem
The previous allow-removal slices got `RS-CODE-04` under control, but the family was still blocked by two large structural buckets:
- `RS-CODE-24`: mostly justified `#[path = ...]` usages in test-sidecar wiring that were still missing same-line `// reason:` comments
- `RS-CODE-32`: low-signal `.expect("write")`, `.expect("mkdir")`, `.expect("cleanup")`, `.expect("symlink")`, and similar placeholder strings across `garde` and hexarch/core test fixtures

At this point, continuing to chase more `#[allow(...)]` removals would have been lower leverage than clearing those two real correctness/documentation buckets.

## Decisions Made

### Convert justified `#[path]` usage into explicit warnings instead of leaving them as errors
- **Chose:** Add same-line `// reason:` comments to the remaining app-local, cargo-family, clippy-family, and deny-family `#[path = ...]` sites that are structurally necessary.
- **Why:** These are not bypasses. They are legitimate sidecar/test fixture wiring and should be represented as warned inventory rather than repo-blocking errors.
- **Alternatives considered:**
  - Remove the `#[path]` usage entirely — rejected because many of these files are intentionally split sidecars or colocated test modules.
  - Leave them bare and continue treating them as errors — rejected because the checker already defines the documented-warning path for justified uses.

### Bulk-upgrade repeated fixture `.expect(...)` messages
- **Chose:** Rewrite the repeated `garde` fixture setup strings (`parent`, `mkdir`, `write`, `cleanup`, plus local/shared variants) to explicit failure messages, and do the same for the `project_walker` / `project_walker_lossless` / `hexarch` symlink cases.
- **Why:** The weak-message bucket was almost entirely repetitive fixture plumbing. A consistent message upgrade preserves behavior and removes the error without adding noise.
- **Alternatives considered:**
  - Introduce broad helper wrappers first — rejected because the immediate problem was message quality, and the repeated call sites were already clear enough to rewrite directly.
  - Ignore the bucket until later — rejected because it was the dominant remaining live error source.

### Keep this checkpoint focused on `RS-CODE`
- **Chose:** Exclude unrelated dirty files (`Cargo.lock`, `hooks-rs`, `project-tree`, AST assertions, pre-existing code-family test files, etc.) from the commit even though they remain in the worktree.
- **Why:** The repo already has unrelated in-flight lanes. Mixing them into this checkpoint would make the worklog and commit unusably broad.
- **Alternatives considered:**
  - Commit the entire dirty tree — rejected because it would destroy traceability.
  - Revert unrelated files first — rejected because those changes are not ours to discard.

## Architectural Notes
This pass continues the cleanup policy from the earlier `RS-CODE` work:
- a justified structural escape hatch should be explicitly documented in place
- a weak test failure message should become precise, not merely longer
- repetitive fixture setup can be upgraded consistently without changing the underlying test model

The practical effect is that `RS-CODE` is no longer dominated by path-comment debt or generic test setup messages. The remaining errors are concentrated in real buckets:
- residual `RS-CODE-32`
- direct filesystem boundary usage (`RS-CODE-15`)
- oversized files (`RS-CODE-09`)
- a few small tail rules

## Information Sources
- Live repo-root validation:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json`
- Targeted package tests:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-garde --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-core --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
- Prior worklogs:
  - `.worklogs/2026-03-29-220421-continue-rs-code-allow-removal.md`
  - `.worklogs/2026-03-29-215038-continue-rs-code-allow-removal.md`
- Subagent guidance on:
  - top `RS-CODE-32` offender files and safe message-upgrade patterns

## Open Questions / Future Considerations
- `RS-CODE-32` still has `30` live errors, now mostly in other hexarch test files and a tiny garde tail. Those should be finished before shifting away from message quality.
- `RS-CODE-15` still has `26` errors and is the next likely structural bucket after the `expect(...)` cleanup is complete.
- `RS-CODE-09` still has `13` oversize-file errors in production code and will require actual refactors, not comment or string cleanups.
- The worktree still contains substantial unrelated modifications that were intentionally excluded from this commit.

## Key Files for Context
- `apps/guardrail3/crates/app/core/project_walker_tests.rs` — representative upgrade from placeholder fixture messages to precise setup diagnostics.
- `apps/guardrail3/crates/app/core/project_walker_lossless_tests.rs` — lossless golden tests with previously-weak `strip` / `ft` messages.
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/rs_garde_ast_03_enum_derive_validate_tests/false_positives.rs` — representative high-count `RS-CODE-32` target.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_config_01_workspace_lints_tests/mod.rs` — representative `cases.rs` `#[path]` justification pattern.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api_tests/mod.rs` — representative test-matrix `#[path]` justification pattern.
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_09_ban_baseline_complete_tests/mod.rs` — representative deny-family `#[path]` justification pattern.
- `.worklogs/2026-03-29-220421-continue-rs-code-allow-removal.md` — immediately previous `RS-CODE-04` reduction slice that this checkpoint builds on.

## Next Steps / Continuation Plan
1. Re-run repo-root `RS-CODE` and list the remaining `RS-CODE-32` files explicitly; finish that bucket first so the family has a clean message-quality baseline.
2. After `RS-CODE-32` is zero, move to `RS-CODE-15` and audit direct `std::fs` usage by deciding which call sites belong in real filesystem-boundary crates versus helper code.
3. Then tackle `RS-CODE-09` by splitting the remaining oversize production files rather than suppressing the rule.
4. Keep committing in small `RS-CODE` slices, never bundling in the unrelated `hooks-rs`, `project-tree`, or lockfile drift.
