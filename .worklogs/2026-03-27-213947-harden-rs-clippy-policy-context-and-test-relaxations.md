# Backfill Clippy Hardening Context

**Date:** 2026-03-27 21:39
**Scope:** `apps/guardrail3/crates/domain/modules/clippy/{mod.rs,settings.rs}`, `.worklogs/2026-03-27-213947-harden-rs-clippy-policy-context-and-test-relaxations.md`

## Summary
Backfilled the missing follow-up context for the already-landed `rs/clippy` policy-context hardening and synced the canonical domain-module exports to the live code. This keeps the clippy attack stream documented and ensures the top-level clippy module re-exports every managed setting the runtime now enforces.

## Context & Problem
While continuing the adversarial `rs/clippy` review, `HEAD` advanced to `01b87ad` (`harden rs clippy policy context`). That commit landed the runtime-side fixes first:

- fail-closed policy-context handling (`RS-CLIPPY-23`)
- published-library workspace handling for `RS-CLIPPY-16`
- `allow-panic-in-tests = false` enforcement inside `g3rs-clippy/avoid-breaking-exported-api`

But the repo was left with two follow-up problems:

1. the commit had no worklog, which violates the repo’s worklog rule
2. the canonical `domain/modules/clippy` exports were still slightly behind the live runtime behavior

So this follow-up commit is not “more clippy code.” It is the documentation and source-of-truth sync that makes the already-landed hardening legible and durable.

## Decisions Made

### Add the missing detailed worklog instead of rewriting history
- **Chose:** Commit `.worklogs/2026-03-27-214025-harden-clippy-policy-context.md` now as the detailed technical record for the already-landed clippy hardening.
- **Why:** The repo rule is explicit that commits need worklogs, and the cleanest recovery is to add the missing record immediately rather than trying to silently ignore it.
- **Alternatives considered:**
  - Leave the previous clippy commit undocumented — rejected because it breaks the repo’s stated operating rule and makes future clippy work harder to recover.
  - Rewrite or amend the earlier commit — rejected because the instructions explicitly say not to amend unless requested.

### Export the new canonical bool from `domain/modules/clippy`
- **Chose:** Export `ALLOW_PANIC_IN_TESTS` from `apps/guardrail3/crates/domain/modules/clippy/mod.rs`.
- **Why:** The runtime and tests already depend on the settings module’s `allow-panic-in-tests` baseline. The top-level domain module should expose that setting the same way it already exposes the other managed booleans.
- **Alternatives considered:**
  - Keep `ALLOW_PANIC_IN_TESTS` private to `settings.rs` — rejected because that leaves the canonical module surface incomplete relative to the generated config contract.

## Architectural Notes
- This is a source-of-truth sync commit, not a new runtime-semantics commit.
- The real runtime behavior for:
  - policy-context fail-closed handling
  - library-workspace publishability
  - test panic relaxation enforcement
  already lives at `HEAD` in the clippy family.
- The plan file remains the human-readable contract, while `domain/modules/clippy` remains the canonical generated-settings source used by family fixtures and parity checks.

## Information Sources
- `git show --stat --name-only 3f69dc3`
- `.plans/todo/checks/rs/clippy.md`
- `apps/guardrail3/crates/domain/modules/clippy/{mod.rs,settings.rs}`
- Prior clippy worklogs:
  - `.worklogs/2026-03-27-210812-finish-clippy-sidecar-extraction.md`
  - `.worklogs/2026-03-27-211613-fix-clippy-policy-root-gaps.md`
  - `.worklogs/2026-03-27-212201-split-clippy-library-type-ban-ownership.md`
  - `.worklogs/2026-03-27-212709-fix-clippy-macro-ban-paths.md`

## Open Questions / Future Considerations
- The outer workspace is still blocked by the in-flight `deny` migration, so the next clippy checkpoint still needs top-level `guardrail3 rs validate ...` reruns once Cargo metadata is healthy again.
- `RS-CLIPPY-19` still deserves more adversarial sampling against real non-managed Clippy keys, even though the current implementation looks acceptable against the active key set.

## Key Files for Context
- `.worklogs/2026-03-27-214025-harden-clippy-policy-context.md` — detailed record of the runtime-side clippy hardening already landed at `HEAD`
- `apps/guardrail3/crates/domain/modules/clippy/settings.rs` — canonical generated bool settings
- `apps/guardrail3/crates/domain/modules/clippy/mod.rs` — exported canonical surface
- `.worklogs/2026-03-27-212709-fix-clippy-macro-ban-paths.md` — prior semantic checkpoint before the policy-context hardening

## Next Steps / Continuation Plan
1. Once the unrelated `deny` workspace break is gone, rerun top-level clippy family validation from `apps/guardrail3/Cargo.toml`.
2. Continue adversarial `rs/clippy` review on the remaining semantic edges, especially `RS-CLIPPY-19` and any remaining overlap between local policy-root completeness and per-key rules.
3. After clippy is stable enough, return to the next family in the user’s chosen sequence (`deny`, then `deps`, then `garde`), while keeping the easy-family handoff files for `toolchain` and `fmt` ready for parallel agents.
