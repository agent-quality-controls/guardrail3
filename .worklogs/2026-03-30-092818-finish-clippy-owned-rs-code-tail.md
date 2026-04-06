# Finish Clippy-Owned RS-CODE Tail

**Date:** 2026-03-30 09:28
**Scope:** `.plans/todo/checks/rs/clippy/FIXES.md`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_08_too_many_lines_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_09_too_many_arguments_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_10_excessive_nesting_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_13_local_policy_root_baseline.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_14_library_global_state.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_19_cognitive_complexity_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_20_type_complexity_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_23_policy_context_parseable.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_24_forbid_clippy_conf_dir_override.rs`

## Summary
Closed the remaining clippy-owned repo-root `RS-CODE` debt and updated the clippy fixes document from an open backlog into a completed status record. The code change is deliberately narrow: keep the same sidecar layout, but express the `#[path]` justification in the exact syntax `RS-CODE-24` now requires.

## Context & Problem
After the clippy full-sweep cleanup, repo-root `RS-CLIPPY` was already clean on failures, but clippy-owned files still carried the last `RS-CODE` tail. The remaining findings were `RS-CODE-24` hits on test-only sidecar wiring inside clippy runtime files. Those were not real architectural problems anymore; they were formatting mismatches against the stricter same-line `// reason:` parser in `RS-CODE`.

At the same time, `.plans/todo/checks/rs/clippy/FIXES.md` still read like an active backlog even though the lane had already been swept. Leaving it that way would preserve false “unfinished” context for the next agent.

## Decisions Made

### Keep sidecar wiring, fix the reason syntax
- **Chose:** Keep the existing `#[path = "..."]` test-sidecar pattern in the clippy runtime files and move the `// reason:` comment onto the same line as the attribute.
- **Why:** These are intentional test-only sidecar modules, not ad hoc escapes. The issue was the exact explanation syntax expected by `RS-CODE-24`, not the existence of the sidecars themselves.
- **Alternatives considered:**
  - Remove `#[path]` wiring entirely — rejected because the family already uses rule-local sidecar directories intentionally.
  - Add more parsing leniency back into `RS-CODE-24` — rejected because that would reopen the rule we just tightened.

### Convert `clippy/FIXES.md` into a closure record
- **Chose:** Rewrite `FIXES.md` as an implemented-outcomes/status document instead of an open bug list.
- **Why:** The user explicitly asked for a whole-repo cleanup, and leaving a stale “still open” fix list after the lane is clean would be misleading.
- **Alternatives considered:**
  - Leave the old backlog untouched — rejected because it would no longer match the live code.
  - Delete the file — rejected because it still contains useful historical/architectural context about what was fixed.

## Architectural Notes
This commit does not relax `RS-CODE-24`. It does the opposite: it brings the clippy family into compliance with the stricter explanation channel by making the justification syntactically exact. The sidecar test pattern remains the same and stays local to each rule file.

The doc rewrite also establishes a useful pattern for other family-local `FIXES.md` files: once a sweep is closed, the file should become a status record rather than a zombie backlog.

## Information Sources
- `.worklogs/2026-03-30-091513-clippy-full-sweep-cleanup.md`
- `.plans/todo/checks/rs/clippy.md`
- `.plans/todo/checks/rs/clippy/FIXES.md`
- `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- Repo-root validation:
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --inventory --format json`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family clippy --inventory --format json`

## Open Questions / Future Considerations
- The clippy lane is clean on failures, but future work may still expand policy intentionally. That belongs in `clippy.md`, not back in this file as pseudo-bugs.
- Any future change to `RS-CODE-24` comment parsing should be checked against these sidecar files, because they now serve as real-world compliance examples.

## Key Files for Context
- `.plans/todo/checks/rs/clippy/FIXES.md` — closure record for the clippy sweep and the decisions that resolved the old backlog
- `.plans/todo/checks/rs/clippy.md` — live clippy family contract
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — family-local behavior and structure
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_24_forbid_clippy_conf_dir_override.rs` — representative rule affected by the sidecar path-attribute pattern
- `.worklogs/2026-03-30-091513-clippy-full-sweep-cleanup.md` — prior sweep context for the clippy lane

## Next Steps / Continuation Plan
1. Commit the remaining residual cleanup outside the clippy lane: domain module test relocation, fmt test-quality cleanup, arch formatting, and lockfile updates.
2. Commit the remaining untracked planning/docs files so the repo is clean instead of leaving local notes behind.
3. Run final adversarial checks against `code`, `clippy`, and `arch` after the last commit to confirm the live final tree still catches reintroduced bad shapes.
