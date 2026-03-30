# Finish Non-Clippy RS-CODE Cleanup

**Date:** 2026-03-30 05:10
**Scope:** `apps/guardrail3/crates/{adapters/inbound/cli,app/core,app/rs,domain/modules,domain/report}/**`, `apps/guardrail3/crates/app/rs/families/{cargo,code,deny,deps,garde,hexarch,hooks-rs,hooks-shared,release,test}/**`

## Summary
Finished the remaining non-clippy `RS-CODE` sweep at repo root. The work removed or refactored the last non-clippy code-family findings instead of documenting them away, split oversized helper/runtime files into smaller internal modules, and revalidated the repo so that the only remaining `RS-CODE` errors belong to the separately-owned `clippy` lane.

## Context & Problem
After the code-family correctness backlog from `families/code/FIXES.md` was closed, repo-root `RS-CODE` still had ordinary-source debt outside the code family itself. The user explicitly asked to keep pushing the remaining `RS-CODE` issues to zero for every lane except the active clippy-owned handoff, and to prefer removing `allow`/path/import/fs debt over annotating it.

At the start of this slice, the remaining non-clippy backlog had already been reduced heavily, but the worktree still contained a large uncommitted cleanup sweep spread across cargo, deny, release, test, garde, hooks, and shared helper crates. The repo also had unrelated dirty `clippy`, `fmt`, `Cargo.lock`, and plan-doc lanes in flight from other agents, so this commit needed to isolate only the non-clippy `RS-CODE` work.

## Decisions Made

### Keep clippy-owned RS-CODE debt out of this commit
- **Chose:** treat every `RS-CODE-24` hit under `families/clippy/**` as out of scope and leave those files untouched.
- **Why:** the user had a separate clippy handoff and other agents were actively editing that lane.
- **Alternatives considered:**
  - Bundle clippy path-attr fixes here — rejected because it would conflict with the dedicated clippy sweep and muddy ownership.
  - Stop when repo-root `RS-CODE` was not literally zero — rejected because the user explicitly scoped the exception to the clippy-owned handoff.

### Prefer structural removal over new reasons/exemptions
- **Chose:** keep shrinking import surfaces, splitting oversized files, removing dead helper scaffolding, and replacing same-dir `#[path = ...]` wiring with normal module declarations.
- **Why:** most remaining findings were ordinary code-shape debt, not legitimate policy exceptions.
- **Alternatives considered:**
  - Add more `// reason:` comments or tolerate the warning inventory — rejected because the user explicitly wanted refactors/removals where possible.
  - Weaken `RS-CODE` thresholds or exemptions — rejected because this lane was about proving the existing checks still bite after cleanup.

### Use the built binary for repo-root proof while unrelated compile blockers existed
- **Chose:** use `apps/guardrail3/target/debug/guardrail3` for repeated repo-root `RS-CODE` validation.
- **Why:** unrelated in-flight `clippy` and `hooks-shared` refactors temporarily blocked top-level rebuilds, but the source-level validator binary was already available and sufficient to prove the live `RS-CODE` state.
- **Alternatives considered:**
  - Fix the unrelated compile blockers first — rejected because that would cross ownership boundaries.
  - Stop validating until the whole workspace rebuilt — rejected because the user asked for continuous progress.

### Treat the final proof step as adversarial, not just green tests
- **Chose:** run a temp-repo attack pass after the cleanup instead of relying only on green package tests.
- **Why:** the user explicitly asked for test attacks, and the important proof here was that the repo is clean because the debt is actually gone, not because the check softened.
- **Alternatives considered:**
  - Only rerun repo-root validation — rejected because that would not prove reintroduced debt is still caught.
  - Attack every touched file shape — rejected because the slice was repo-root debt cleanup, not a new family-correctness lane.

## Architectural Notes
This sweep did not change `RS-CODE` rule behavior. It changed source layout so the existing rules pass for real:
- same-dir test/helper modules now use normal module declarations instead of warning-heavy `#[path = ...]`
- cargo runtime test suites moved from flat `*_tests/{mod.rs,cases.rs}` ownership to per-rule child directories
- deny, garde, release, test, hooks-rs, hooks-shared, deps, and hexarch helper surfaces were split into smaller internal modules to get under file-size/import thresholds without changing rule ownership
- `ast_helpers` and related shared helpers now use narrower module-qualified visitor calls instead of broad import fans
- direct `panic!`/`std::fs` cleanup stayed on the source side rather than through checker exceptions

The result is that repo-root `RS-CODE` now reports only clippy-owned `RS-CODE-24` errors, which is the expected remaining lane.

## Information Sources
- `.worklogs/2026-03-29-232925-finish-rs-code-fixes-tail.md` — prior code-family checkpoint clarifying that repo-root debt remained after family correctness was closed.
- `.worklogs/2026-03-29-233650-close-rs-code-proof-gaps.md` — prior proof-gap closure and the narrowness requirement for code-family commits.
- `apps/guardrail3/crates/app/rs/families/code/FIXES.md` — background for what was already considered code-family correctness versus ordinary repo-root debt.
- Live repo-root validation via `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family code --inventory --format json`.
- Targeted package tests run during the sweep:
  - `guardrail3-app-rs-ast`
  - `guardrail3-app-rs-family-garde`
  - `guardrail3-app-rs-family-release`
  - `guardrail3-app-rs-family-test`
  - `guardrail3-app-rs-family-deny`
  - `guardrail3-app-rs-family-cargo`

## Open Questions / Future Considerations
- Repo-root `RS-CODE` is not globally zero yet; the remaining `RS-CODE-24` errors all live under `families/clippy/**` and should be handled only in the dedicated clippy lane.
- The repo still has unrelated dirty work in `clippy`, `fmt`, `Cargo.lock`, and plan docs. Those were intentionally excluded from this commit.
- If repo-root `RS-CODE` is revisited after the clippy sweep lands, the next likely task is simply to confirm the final zero state and remove any stale temporary proof notes, not to redesign the rules.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs` — final shared-helper import cleanup that removed the last non-clippy import-count warning.
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/parse.rs` — final garde import cleanup that removed the other surviving non-clippy warning.
- `apps/guardrail3/crates/app/rs/families/release/src/lib.rs` — same-dir module declaration cleanup that the adversarial pass re-broke on purpose to prove `RS-CODE-24` still fires.
- `apps/guardrail3/crates/app/rs/families/release/src/release_support/workflows.rs` — representative oversized-file split from the repo-root cleanup lane.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — representative runtime split from the test-family side of this sweep.
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/hook_rs_13_cargo_dupes_excludes.rs` — representative hook-rule size cleanup.
- `.worklogs/2026-03-29-232925-finish-rs-code-fixes-tail.md` — prior checkpoint separating code-family correctness from repo-root source debt.
- `.worklogs/2026-03-29-233650-close-rs-code-proof-gaps.md` — prior code-family proof closure that this repo-root sweep built on.

## Next Steps / Continuation Plan
1. Finish the dedicated clippy lane and clear the remaining `families/clippy/**` `RS-CODE-24` errors there rather than reopening this commit.
2. After the clippy lane lands, rerun repo-root `RS-CODE --inventory` and confirm literal zero errors/warnings across the full app root.
3. Only after the final clippy cleanup, consider a small follow-up commit if any merge fallout reintroduces import-count/path-attr warnings outside clippy.
