# Finish Clippy Completeness Pass

**Date:** 2026-03-22 17:01
**Scope:** `apps/guardrail3/crates/app/core/discover.rs`, `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`, `apps/guardrail3/crates/app/rs/checks/rs/clippy/**`, `.plans/todo/checks/rs/clippy.md`, `.plans/by_file/rs/clippy-toml.md`

## Summary
Closed the last non-`RS-CLIPPY-19` completeness gaps in the clippy implementation. The checker and generator now share the same app-root resolution helper, and the plan/docs explicitly frame `RS-CLIPPY-19` as a temporary heuristic typo detector instead of pretending it is a fully exact upstream-key validator.

## Context & Problem
After the audit-fix pass, one remaining issue was still real: the checker and generator were aligned only by duplicated path-inference logic rather than by one shared implementation. The user also clarified the bar for “complete”:
- `RS-CLIPPY-19` may remain temporary if that is stated explicitly
- everything else should be actually fixed, not hand-waved as “good enough”

So this pass focused on removing the duplicated root-resolution logic and tightening the documentation around the one intentionally temporary part.

## Decisions Made

### Share app-root resolution between generator and checker
- **Chose:** Extract `resolve_app_paths_from_member_dirs(...)` into `app/core/discover.rs` and make both the generator and clippy facts layer use it.
- **Why:** The previous state still had mirrored heuristics in two places. Even though they had been made consistent, that was not a finished architecture.
- **Alternatives considered:**
  - Leave the same heuristic duplicated in both places — rejected because it would reintroduce drift risk later.
  - Pull the entire generator helper into the checker — rejected because the checker should not depend on CLI-generation plumbing.

### Make the temporary nature of RS-CLIPPY-19 explicit
- **Chose:** Keep `RS-CLIPPY-19` as a managed-key typo detector and update the docs to say exactly that.
- **Why:** The user explicitly allowed this to remain temporary as long as it was documented honestly.
- **Alternatives considered:**
  - Pretend it is a complete unknown-key validator — rejected because that would misstate the implementation.
  - Block completion on importing a full upstream clippy key registry — rejected for now because the user allowed the temporary version.

## Architectural Notes
With this pass, the remaining clippy architecture looks like this:
- canonical generator baseline in `domain/modules/clippy/**`
- checker family in `app/rs/checks/rs/clippy/**`
- shared app-path resolution in `app/core/discover.rs`

That removes the last duplicated inference path that mattered for correctness. The only still-temporary behavior is `RS-CLIPPY-19`, and that is now explicitly documented as such.

## Information Sources
- Follow-up user clarification about completeness and `RS-CLIPPY-19`
- Explorer re-audit finding that the remaining issue was checker/generator path-resolution duplication
- `apps/guardrail3/crates/app/core/discover.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs`
- `.worklogs/2026-03-22-164943-clippy-audit-fixes.md`

## Open Questions / Future Considerations
- If the project later wants `RS-CLIPPY-19` to be fully exact, it should use a real upstream-known-key source rather than extending the heuristic by hand.
- The repo still has a large unrelated dirty backlog outside the clippy line of work. This commit intentionally does not sweep that state.

## Key Files for Context
- `apps/guardrail3/crates/app/core/discover.rs` — shared app-root resolution helper
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs` — generator now consumes the shared helper
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs` — checker now consumes the shared helper
- `.plans/todo/checks/rs/clippy.md` — explicit temporary framing for `RS-CLIPPY-19`
- `.plans/by_file/rs/clippy-toml.md` — by-file design and remaining override-policy notes
- `.worklogs/2026-03-22-164943-clippy-audit-fixes.md` — prior audit-fix checkpoint

## Next Steps / Continuation Plan
1. Commit this final clippy completeness pass separately.
2. Treat clippy as complete except for the explicitly temporary `RS-CLIPPY-19` exactness question.
3. Move to the next Rust family or, if requested, start grouping and committing the unrelated dirty backlog separately from the clippy work.
