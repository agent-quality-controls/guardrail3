# Harden Walker Config Recovery

**Date:** 2026-04-01 20:27
**Scope:** `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/core/project_walker/tests.rs`, `apps/guardrail3/crates/app/core/project_walker/README.md`

## Summary
Expanded the walker’s ignored-file recovery surface to cover real Rust and planned TypeScript config/policy files while keeping ignored source files out. Then ran multiple attack rounds, fixed two under-recovery bugs and two over-recovery bugs, and tightened the tests so they prove both file presence and cached content.

## Context & Problem
The prior walker cut narrowed ignored-file recovery to config and policy files, but the exact restore set was still incomplete and partially wrong. The repo needed a broader config surface for Rust families such as release/test/hooks and for planned TypeScript families, but the broadening had to stay strict about two constraints:

- ignored source code must still stay out of global recovery
- hard-banned roots like `.claude/worktrees` must stay invisible regardless of file type

After expanding the restore set, adversarial test attacks exposed that some real repo-root config surfaces still did not recover, while some new matchers were too broad and started reintroducing invalid paths.

## Decisions Made

### Broaden recovery to real config and policy surfaces
- **Chose:** Added recovery support for additional Rust and TS config/policy files, including `rust-toolchain`, hook files, workflow YAML, `contentlayer.config.*`, `playwright.config.*`, and JSON TS config variants.
- **Why:** The shared walker must expose the config surfaces families and later TS structure work depend on, or legality/ownership can fail open before family code runs.
- **Alternatives considered:**
  - Keep the narrower list and let families rediscover missing files — rejected because that breaks the shared-boundary architecture.
  - Recover all ignored files outside banned roots — rejected because that reopens the earlier source-file leak.

### Remove non-config junk from the restore list
- **Chose:** Dropped `CLAUDE.md`, `LICENSE*`, and `.gitkeep` from the cached/recovered set.
- **Why:** Those are not shared config/policy discovery inputs. Keeping them in the walker blurs the contract and increases noise for no architectural gain.
- **Alternatives considered:**
  - Leave them in because they are harmless — rejected because the point of this pass is to make the restore set intentional rather than drifting.

### Narrow hook recovery to the real repo-root hook contract
- **Chose:** Recovery now accepts only the hook surfaces the hook family actually owns at repo root: `.githooks/pre-commit`, `hooks/pre-commit`, `.husky/pre-commit`, root `.githooks/pre-commit.d/*`, and root `.guardrail3/overrides/pre-commit.d/*`.
- **Why:** Attack repros showed the earlier matcher missed root `hooks/pre-commit` and incorrectly recovered nested `*/hooks/pre-commit` and nested local hook directories. That contradicted the hook family contract.
- **Alternatives considered:**
  - Match any path containing `/hooks/` or `.husky/` — rejected because it over-recovers nested app-local paths the shared hook family does not own.

### Narrow TS config recovery to `tsconfig*.json`
- **Chose:** Removed the broad `tsconfig.` prefix and replaced it with an explicit `name.starts_with("tsconfig") && name.ends_with(".json")` rule.
- **Why:** The TS plan contract is `tsconfig*.json`, not arbitrary `tsconfig.*`. Attack repros proved the broad prefix was incorrectly recovering `tsconfig.backup`, `tsconfig.app.yaml`, and `tsconfig.worker.ts`.
- **Alternatives considered:**
  - Keep `tsconfig.json` and `tsconfig.base.json` only — rejected because the TS plan explicitly allows nearest-local `tsconfig*.json`.
  - Keep the broad prefix and rely on later TS code to reject bad variants — rejected because the walker should not knowingly inject non-contract junk.

### Strengthen the tests to assert cached content and negative cases
- **Chose:** Added regression tests for root hook recovery, root hook directory recovery, nested non-root hook exclusion, `tsconfig*.json` recovery, and broad tool/policy content caching.
- **Why:** The first broad recovery test only proved `file_exists`, which would miss regressions where the walker left paths in structure but failed to cache their content.
- **Alternatives considered:**
  - Keep only positive presence tests — rejected because `ProjectTree`’s contract is both structure visibility and cached config content.

## Architectural Notes
The walker contract is now:

- use ignore rules as the baseline noise filter
- restore tracked ignored files because they are real repo state
- restore ignored config/policy/hook/tool files outside hard-banned roots
- never restore ignored source files globally
- keep hard-banned roots excluded from every phase

This remains a shared pre-family responsibility. Families still must not crawl the filesystem to compensate for missing config surfaces.

The hook portion is intentionally repo-root-specific because the current shared hook family only owns repo-root hook entry points and hook directories.

The TS config portion now aligns with the planned TS local-config ownership model: `tsconfig*.json` is part of the config surface, but non-JSON lookalikes are not.

## Information Sources
- `apps/guardrail3/crates/app/core/project_walker.rs` — live recovery predicates and hard-ban logic.
- `apps/guardrail3/crates/app/core/project_walker/tests.rs` — contract tests and attack-driven regressions.
- `apps/guardrail3/crates/app/core/project_walker/README.md` — walker contract documentation.
- `apps/guardrail3/crates/app/hooks/mod.rs` — actual repo-root hook surfaces the hook family consumes.
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs` — `ProjectTree` contract, especially cached config content.
- `apps/guardrail3/crates/app/rs/ownership/src/discover.rs` and Rust family READMEs — Rust config surfaces that must be visible.
- `.plans/todo/checks/ts/README.md` — TS nearest-local ownership contract, especially `tsconfig*.json`.
- Real `dump-tree` repros run against temp git repos for:
  - root `hooks/pre-commit`
  - nested non-root hook paths
  - `tsconfig.app.json`
  - invalid `tsconfig.*` lookalikes
  - banned `.claude/worktrees` paths
- `.worklogs/2026-04-01-193447-narrow-walker-recovery-surface.md` — prior walker narrowing pass this work builds on.

## Open Questions / Future Considerations
- The TypeScript restore set still has planned families whose exact config filenames are not yet fully pinned, especially type coverage, size-budget config, SEO/config routing surfaces, and some content-specific config files.
- If hooks are later redesigned as workspace-local or language-local rather than repo-root shared, the walker restore rules for hook surfaces will need to be revisited together with that architecture change.
- Some Rust and TS families still consume config surfaces indirectly through older runtime code; a future pass should cross-check the final restore set against all actual family mappers/runtime ingress again after the TS architecture work progresses.

## Key Files for Context
- `apps/guardrail3/crates/app/core/project_walker.rs` — the shared recovery and hard-ban rules.
- `apps/guardrail3/crates/app/core/project_walker/tests.rs` — the attack-derived walker contract tests.
- `apps/guardrail3/crates/app/core/project_walker/README.md` — current shared walker contract.
- `apps/guardrail3/crates/app/hooks/mod.rs` — the repo-root hook surfaces that justify the hook matcher.
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs` — the `ProjectTree` structure/content contract.
- `.plans/todo/checks/ts/README.md` — TS config ownership contract, especially local `tsconfig*.json`.
- `.worklogs/2026-04-01-193447-narrow-walker-recovery-surface.md` — previous pass that removed global ignored source recovery.

## Next Steps / Continuation Plan
1. Commit this walker hardening pass, then rerun any higher-level validations that depend on the broadened restore set if the next task touches Rust or TS family routing.
2. Continue auditing the remaining walker restore entries against actual family usage and planned TS file surfaces, with particular attention to type coverage, size-budget, content, i18n, and SEO config roots.
3. If future attack rounds find another over-broad matcher, prefer replacing string `contains(...)` patterns with exact root-relative ownership checks, the same way this pass corrected hook recovery.
