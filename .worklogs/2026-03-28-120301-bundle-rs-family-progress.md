# Bundle RS Family Progress

**Date:** 2026-03-28 12:03
**Scope:** `apps/guardrail3/Cargo.lock`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/app/rs/families/code/**`, `apps/guardrail3/crates/app/rs/families/hexarch/**`, `apps/guardrail3/crates/app/rs/families/test/**`, `apps/guardrail3/crates/app/rs/families/garde/**`, `.plans/todo/family-stabilization-handoffs/deps.md`, `.plans/todo/family-stabilization-handoffs/garde.md`, `apps/guardrail3/crates/app/rs/families/deny/README.md`

## Summary
Bundled the current in-flight Rust family work into one coherent checkpoint now that the app-level workspace boundary is fixed again. The tree now builds from `apps/guardrail3`, the heavy `code` and `hexarch` family hardening passes are green, and `garde` has been promoted into the self-hosted family shape with passing library tests.

## Context & Problem
The worktree had accumulated several overlapping lines of Rust-family work: a large `RS-CODE` adversarial expansion, continued `RS-HEXARCH` hardening, the `RS-TEST` adjustments needed after family-root workspace removal, and fresh `RS-GARDE` stabilization. Before bundling any of that, the important question was whether the app was still integration-broken.

That integration question is now answered: `apps/guardrail3` once again has a healthy top-level Cargo workspace and live `hexarch` validation is clean. With the structural workspace hole closed, it made sense to sort through the remaining source edits, remove generated debris, verify representative families, and commit the current bundle rather than leaving hundreds of tracked source edits hanging unrecorded.

## Decisions Made

### Commit the current Rust-family bundle as one checkpoint
- **Chose:** Commit the full remaining source worktree together instead of trying to carve it back into several smaller retroactive commits.
- **Why:** The edits are already interdependent at the app level: workspace wiring, runtime dispatch, family-local tests, and new family structure all need to coexist in a buildable tree. Splitting them after the fact would add risk without improving architectural clarity.
- **Alternatives considered:**
  - Re-split into separate commits for `code`, `hexarch`, and `garde` — rejected because the worktree was already interleaved and the user explicitly asked to commit everything.
  - Keep the tree dirty and continue family-by-family without a checkpoint — rejected because too much verified work would remain unrecorded.

### Verify representative top-level and family-level health before bundling
- **Chose:** Verify the app build plus the most relevant changed families instead of attempting an unbounded full-repo test sweep.
- **Why:** The goal was to confirm that the current source set is coherent and buildable. The most informative checks were the top-level `guardrail3` build, the workspace-boundary-sensitive `hexarch` family, the heavily changed `code` family, the `RS-TEST` family that encodes the packaging discipline, the `cargo` family for workspace policy, and the newly stabilized `garde` family.
- **Alternatives considered:**
  - Run every package test in the workspace — rejected because it would add time without materially changing confidence for this checkpoint.
  - Commit after only `cargo build` — rejected because the family-heavy changes warranted actual family test coverage.

### Remove only generated artifacts, not in-flight source work
- **Chose:** Delete untracked family `target/` directories and stray family `Cargo.lock` files, while leaving all modified and newly created source files intact.
- **Why:** The build was already healthy; the real clutter was generated output. Cleaning only generated artifacts made the final diff reflect actual source progress.
- **Alternatives considered:**
  - Leave generated artifacts in the worktree and commit around them — rejected because they obscured the real source state.
  - Aggressively clean or revert modified tracked files — rejected because those edits were intentional in-flight work and the user asked to commit everything.

## Architectural Notes
This checkpoint should be read as a bundled state update, not a final architecture verdict. The important structural baseline now in place is:

- `apps/guardrail3` is again the active app-root workspace for live builds and validation.
- `RS-HEXARCH` and `RS-TEST` are both operating against the post-nested-workspace family container shape.
- `RS-CODE` has undergone a large adversarial broadening pass, including stronger owned assertions and many real-tree attack cases.
- `RS-GARDE` is now moving in the same self-hosted family direction as the other stabilized families, with runtime/assertions/test-support crates present and testable from the main workspace.

The remaining open design question is still the larger internal package grammar under the app workspace. This commit does not settle that debate; it only checkpoints the now-buildable and heavily tested current state.

## Information Sources
- `git diff --stat` and `git status --short` at bundle time
- `.worklogs/2026-03-28-110213-finish-hexarch-workspace-boundary.md`
- verification commands:
  - `cargo build --manifest-path apps/guardrail3/Cargo.toml -p guardrail3`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --bin guardrail3`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-cargo --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-code --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-garde --lib`

## Open Questions / Future Considerations
- `RS-CODE` and `RS-HEXARCH` now carry large hardening/test surfaces; future follow-up should prune or reorganize only if that can be done without weakening rule ownership clarity.
- `RS-GARDE` is now compiled and tested in its split shape, but its family-level docs and self-validation status should be reviewed as a separate stabilization pass rather than assumed complete from this bundle alone.
- The broader app-internal package grammar question remains open even though the workspace-boundary loophole is closed.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/Cargo.toml` — workspace-level Rust-family wiring after the current family bundle
- `apps/guardrail3/crates/app/rs/runtime.rs` — top-level Rust family dispatch over the current family set
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs` — current `RS-CODE` family orchestrator after the attack-driven broadening pass
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — current `RS-HEXARCH` family orchestrator over the post-boundary-fix rule set
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — family-container discovery logic that now works without fake parent workspaces
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/lib.rs` — newly split `RS-GARDE` runtime entrypoint
- `.plans/todo/family-stabilization-handoffs/garde.md` — handoff context for the new `garde` family work
- `.worklogs/2026-03-28-110213-finish-hexarch-workspace-boundary.md` — prior checkpoint that restored the top-level workspace baseline

## Next Steps / Continuation Plan
1. Use the now-clean top-level app workspace as the truth source for live family validation runs: `test`, `cargo`, `clippy`, and `code` are the next families to review from the app root rather than from isolated nested-family commands.
2. Review `RS-GARDE` at the same stabilization level as `fmt` and `deny`: README accuracy, self-validation under `arch` / `test` / `garde`, and attack coverage against its plan.
3. Revisit the broader internal package-grammar debate from the now-stable baseline, without reopening the already-fixed workspace-boundary loophole.
