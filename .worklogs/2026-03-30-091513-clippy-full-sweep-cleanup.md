# Finish Clippy Full-Sweep Cleanup

**Date:** 2026-03-30 09:16
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/**`, `apps/guardrail3/crates/domain/modules/clippy/mod.rs`

## Summary
Completed the remaining clippy full-sweep cleanup by removing clippy-owned `RS-CODE` errors/warnings, splitting the long facts and test-support files into smaller helper modules, and converting the clippy test-matrix sidecars away from warning-producing `#[path]` wiring. The clippy family validator now returns zero errors and zero warnings, and clippy-owned code-family results are reduced to info-only inventory.

## Context & Problem
The follow-up brief in `.plans/todo/check_review/test_hardening/36-clippy-full-sweep-agent-brief.md` required the clippy lane to converge completely, not just the family rules themselves. After the earlier fail-closed hardening work, the remaining debt was repo-root `RS-CODE` pressure inside clippy-owned files:

- `facts.rs` and the clippy `test_support` crate were over the file-length limit
- the domain clippy module still tripped the top-level import-count rule
- the scenario-matrix `mod.rs` files in clippy test directories emitted a large `RS-CODE-24` warning fanout because they used `#[path]` to point at same-directory sibling files

The required end state was not “tests pass locally”; it was the full convergence bundle from the brief: clippy validator clean, clippy-owned code-family error/warn count at zero, docs consistent with the new file layout, and a post-fix attack pass returning clean.

## Decisions Made

### Split facts ownership by concern instead of suppressing file-length guardrails
- **Chose:** Keep [`facts.rs`](apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs) as the public facts surface and move cargo discovery/config discovery/policy resolution into `facts/cargo.rs`, `facts/configs.rs`, and `facts/policy.rs`.
- **Why:** The long-file finding was real. Splitting by discovery concern preserved the family orchestrator shape while making the code easier to review.
- **Alternatives considered:**
  - Leave `facts.rs` as-is and accept the repo-root code-rule debt — rejected because it violated the brief directly.
  - Add ad hoc allows for the long file — rejected because it weakens the repo-root guardrails instead of tightening the architecture.

### Convert clippy test matrix sidecars to standard module resolution
- **Chose:** Remove same-directory `#[path = "..."]` attributes from the clippy test-matrix `mod.rs` files and rely on normal `mod foo;` declarations.
- **Why:** The `#[path]` usage was unnecessary for same-directory sibling files and produced a large warning fanout under `RS-CODE-24`.
- **Alternatives considered:**
  - Keep `#[path]` and live with warning inventory — rejected because the brief required clippy-owned code-family findings to converge to zero non-info results.
  - Change the repo-root code rule instead of the clippy files — rejected because the clippy files were the cleaner fix and the brief scoped ownership here.

### Reduce the clippy domain-module surface without changing rule behavior
- **Chose:** Simplify [`mod.rs`](apps/guardrail3/crates/domain/modules/clippy/mod.rs) so it exposes a smaller surface and uses standard test module wiring instead of the previous path-based test hook.
- **Why:** The top-level import-count rule was pointing at a real API-surface smell. Narrowing the surface fixed the repo-root check and made the module easier to reason about.
- **Alternatives considered:**
  - Keep the existing flat export surface and try to game the count rule — rejected because that would preserve the clutter rather than improve the module boundary.
  - Move every consumer immediately to deep submodule imports — rejected because it was more disruptive than the lane required.

### Split the clippy test-support crate along actual helper responsibilities
- **Chose:** Make [`lib.rs`](apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs) a thin re-export surface over smaller helper modules for fixture trees, filesystem/temp-dir support, and TOML mutation helpers.
- **Why:** This removed the second long-file violation while keeping the test API stable for the clippy runtime crate.
- **Alternatives considered:**
  - Keep one giant helper file with an allow — rejected because it would just move the exception burden into test infrastructure.
  - Push the helpers back into runtime-private modules — rejected because the shared test-support crate was already the right architecture; it just needed decomposition.

## Architectural Notes
The resulting shape is closer to the checker architecture the repo is aiming for:

- `facts.rs` remains the orchestrator-facing facts surface
- helper discovery code now lives in narrow files under `facts/`
- rule-specific test matrices still live next to the rule, but now use normal Rust module resolution when there is no real path escape
- the clippy test-support crate keeps a stable crate-root API while hiding the implementation split behind re-exports

This was a structural tightening, not a policy change. The rule outputs are the same where they should be the same; the difference is that clippy-owned files no longer need repo-root code-rule exceptions or warning inventory to express that behavior.

## Information Sources
- `.plans/todo/check_review/test_hardening/36-clippy-full-sweep-agent-brief.md`
- `.worklogs/2026-03-30-042157-rs-clippy-fix-hardening.md`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-domain-modules --lib`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family clippy --format json`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json`
- fresh post-fix test-attack subagent pass over the clippy-owned lane

## Open Questions / Future Considerations
- The remaining clippy-owned code-family output is intentional `info` inventory for the `unreachable!()` in `rs_clippy_17_test_relaxations.rs`; it is not a convergence blocker, but it is the next obvious place to revisit if the repo later decides to tighten macro inventory policy.
- The repo still has unrelated dirty files outside the clippy lane; this commit should stay scoped to the clippy cleanup and not sweep those changes in.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — family facts surface and orchestrator entry point
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts/cargo.rs` — cargo-root discovery extracted from `facts.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts/configs.rs` — clippy config and cargo-config override discovery
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts/policy.rs` — guardrail policy resolution and publishability logic
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — public test-support surface after the split
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/fixtures.rs` — shared ProjectTree fixture builders
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/fs_ops.rs` — temp-dir/file-system helpers used by tests
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/toml_edit.rs` — shared TOML mutation helpers for attack scenarios
- `apps/guardrail3/crates/domain/modules/clippy/mod.rs` — narrowed clippy domain-module export surface
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — updated file layout and architecture notes for the split
- `.worklogs/2026-03-30-042157-rs-clippy-fix-hardening.md` — prior hardening work this cleanup builds on

## Next Steps / Continuation Plan
1. If more clippy work starts later, read the prior hardening worklog and this one first, then start from `facts.rs`, `facts/configs.rs`, and the `test_support` crate surface.
2. If the repo later tightens code-family rules again, re-run `--family code` filtered to clippy-owned files before making policy changes so warning/error ownership stays local.
3. Keep future clippy tests on standard module resolution when the file is already in the same directory; only use `#[path]` where there is an actual non-standard boundary being crossed.
