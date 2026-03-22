# Clippy Audit Fixes

**Date:** 2026-03-22 16:49
**Scope:** `.plans/todo/checks/rs/clippy.md`, `.plans/by_file/rs/clippy-toml.md`, `apps/guardrail3/crates/app/rs/checks/rs/clippy/**`, `apps/guardrail3/crates/domain/modules/clippy/**`, `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`

## Summary
Applied the post-implementation audit fixes to the clippy family and canonical generator. This closed the remaining high-severity gaps around `.clippy.toml` coverage/shadowing, per-root policy resolution, garde-conditional baselines, and the over-broad `RS-CLIPPY-19` behavior, then updated the planning docs to reflect the tighter final behavior.

## Context & Problem
After the clippy family and canonical module were committed, an explicit plan-vs-code audit found several mismatches:
- `.clippy.toml` was outside the checker’s model even though the by-file design had already established that clippy resolves both filenames
- profile and garde handling were still mostly global instead of policy-root-specific
- the generator still always rendered garde bans
- `RS-CLIPPY-19` still risked false warnings on legitimate user-owned keys
- `RS-CLIPPY-16` tolerated `avoid-breaking-exported-api = true` too broadly

The point of the second pass was to verify the twelve-ish design decisions had actually made it into the implementation, not just the first happy-path version.

## Decisions Made

### Treat `.clippy.toml` as part of the real resolution surface
- **Chose:** Extend config discovery and placement enforcement to include both `clippy.toml` and `.clippy.toml`.
- **Why:** Clippy resolves both, so ignoring `.clippy.toml` leaves a real shadowing/bypass hole.
- **Alternatives considered:**
  - Keep validating only `clippy.toml` — rejected because it leaves a known coverage gap.

### Bind profile/garde policy to actual allowed roots
- **Chose:** Resolve policy settings against actual allowed workspace/package roots instead of generic leaf-name heuristics.
- **Why:** The checker must validate each local policy root against its own resolved baseline, not a repo-global guess.
- **Alternatives considered:**
  - Keep one global profile name — rejected because mixed service/library monorepos would validate against the wrong baseline.
  - Match only by final path segment everywhere — rejected because it was still too loose and not anchored to actual allowed roots.

### Thread garde enablement through both generator and checker
- **Chose:** Make the canonical generator and checker both support garde-disabled baselines.
- **Why:** The by-file design already described garde-conditional clippy bans. Leaving the generator and checker unconditional would over-enforce a config the project explicitly allows to be disabled.
- **Alternatives considered:**
  - Leave garde as always-on hardening — rejected because it would directly contradict the documented `checks.garde = false` contract.

### Narrow RS-CLIPPY-19 to typo-like managed-key mistakes
- **Chose:** Change `RS-CLIPPY-19` from “warn on keys outside a small hardcoded allowlist” to “warn on keys that look like typos of guardrail-managed keys”.
- **Why:** The frozen policy was to avoid false positives on arbitrary user-owned clippy keys while still catching mistakes like `disalowed-methods`.
- **Alternatives considered:**
  - Try to encode every known clippy key — rejected for now as too brittle and larger than needed for the project’s immediate policy.
  - Keep the old short allowlist — rejected because it still false-warned too broadly.

### Tighten published-library handling for avoid-breaking-exported-api
- **Chose:** Only downgrade `avoid-breaking-exported-api = true` to `Info` when the root is both library-profile and publishable by Cargo metadata.
- **Why:** The prior implementation treated every library-profile root as if it were a published library, which was looser than the agreed rule.
- **Alternatives considered:**
  - Always treat library roots as informational — rejected because many library-profile roots are still internal.
  - Always warn even for publishable libraries — rejected because the policy deliberately left a narrow info-level exception.

## Architectural Notes
This pass reinforced the intended relationship between the new family and the canonical module:
- facts layer models the real resolution surface
- canonical generator and checker both honor garde enablement
- policy-root-local validation uses resolved per-root settings
- tests are adversarial and specifically target bypass/shadow scenarios

The only remaining mild compromise is `RS-CLIPPY-19`: it now follows the policy much more closely, but it is still heuristic because it detects typo-like managed-key mistakes instead of importing a full upstream clippy key universe.

## Information Sources
- Explorer audit findings from agent `019d1667-1501-7133-8607-005107a96669`
- `.plans/todo/checks/rs/clippy.md`
- `.plans/by_file/rs/clippy-toml.md`
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/clippy_support.rs`
- `apps/guardrail3/crates/domain/modules/clippy/render.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`
- `.worklogs/2026-03-22-162849-clippy-checker-family.md`

## Open Questions / Future Considerations
- `RS-CLIPPY-19` is now “managed-key typo” detection rather than a full upstream clippy-key registry. That is a deliberate compromise, but if the project later wants exact upstream coverage, it should come from a generated known-key source rather than another hand-maintained list.
- The repo still has a large amount of unrelated dirty state outside the clippy line of work. This worklog and commit cover only the clippy audit fixes.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs` — config discovery, policy-root binding, and coverage facts
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/clippy_support.rs` — managed baseline helpers and typo-detection logic
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/clippy_tests.rs` — adversarial tests for `.clippy.toml`, local library roots, and garde-disabled baselines
- `apps/guardrail3/crates/domain/modules/clippy/render.rs` — canonical clippy generator now honors garde enablement
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs` — generate path threads profile/garde into `build_clippy_toml`
- `.plans/todo/checks/rs/clippy.md` — updated policy wording for `.clippy.toml`, current code location, and RS-CLIPPY-19 semantics
- `.worklogs/2026-03-22-162849-clippy-checker-family.md` — previous clippy family implementation checkpoint

## Next Steps / Continuation Plan
1. Commit these audit fixes as a focused clippy follow-up commit.
2. If another audit pass reports no high-severity gaps, treat the clippy family as the current reference implementation for the remaining Rust families.
3. Only after that decide whether to sweep the unrelated dirty worktree into separate commits, because that backlog is outside the clippy migration scope.
